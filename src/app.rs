use ratatui::{
    layout::Alignment,
    prelude::Style,
    widgets::{Block, BorderType, Borders},
};
use stateful_list::StatefulList;
use std::sync::{Arc, Mutex};

use self::{
    extras::*,
    states::{LoginState, LoginStates},
};
use crate::{
    app_db_conn::AppDBConnector,
    event::handle_events,
    file_manager::FileManager,
    key_processor::{derive_key, SecureStorage},
    password::generate_strong_password,
    types::{ClState, Terminal},
    ui::{
        draw_ui,
        fields::{input_field, password_field},
    },
};


mod clipboard_thread;
pub(crate) mod extras;
mod stateful_list;
pub(crate) mod states;

pub struct App<'a> {
    // App handling all states and storage of the application
    pub vault_state: LoginStates,
    pub text_fields: EditableTextFields<'a>,

    pub entries_list: StatefulList<String>,
    pub current_entry: Option<(String, StatefulList<(String, String, bool)>)>,
    pub delete_confirm: bool,

    pub templates: StatefulList<Template>,
    pub current_template: Option<usize>,
    pub insert_success: Option<bool>,

    pub page_index: IndexManager,
    pub page_selected: bool,

    pub clipboard: clipboard_thread::ClipboardManager,
    clip_copied: ClState,

    pub file_manager: &'a mut FileManager,
    db_manager: AppDBConnector,
    master_key: Option<SecureStorage>,
    pub login_count: u32,
}

impl<'a> App<'a> {
    pub fn new(file_manager: &'a mut FileManager) -> App<'a> {
        // creates a new with testing values
        let path = file_manager.create_path().unwrap();
        let copied = Arc::new(Mutex::new(SingleValue { value: None }));

        // return new instance of app
        App {
            vault_state: LoginStates::new(file_manager.check_db_exist()),
            text_fields: EditableTextFields::new(),

            entries_list: StatefulList::with_items(vec![]),
            current_entry: None,
            delete_confirm: false,

            templates: StatefulList::with_items(Vec::new()),
            current_template: None,
            insert_success: None,

            page_index: IndexManager::new(3),
            page_selected: false,

            clipboard: clipboard_thread::ClipboardManager::new(Arc::clone(&copied)),
            clip_copied: copied,
            file_manager,

            db_manager: AppDBConnector::new(path),
            master_key: None,
            login_count: 0,
        }
    }

    pub fn run(mut self, terminal: &mut Terminal) -> crate::Result<()> {
        // runs application forever until exited. Draws to the screen and handles events
        log::info!("Starting application view");

        loop {
            if handle_events(&mut self)?.is_break() {
                return Ok(());
            }

            terminal.draw(|f| draw_ui(f, &mut self))?;
        }
    }

    pub fn display_entry(&mut self) {
        // displays a selected entry in ui
        if let Some(item) = self.entries_list.current_item() {
            log::info!("Loading new entry to display");

            // get data from database
            let (template_name, elements) = self.db_manager.get_entry(
                item.clone(),
                self.master_key.as_mut().unwrap().get_contents(),
            );

            self.set_copied_state(None);

            // create list for ui renderer to interpret
            let template = self.templates
                .items
                .iter()
                .find(|t| t.db_name == template_name)
                .unwrap();

            self.current_entry = Some((template.name.clone(), StatefulList::with_items(template
                .elements[1..]
                .iter()
                .zip(elements)
                .map(|(temp, elem)| {
                    (temp.name.clone(), elem.1.clone(), temp.private)
                })
                .chain(std::iter::once((String::new(), String::new(), false)))
                .collect()
            )));
        }
    }

    pub fn select_entry(&mut self) {
        // push right side of entries page to focus
        if self.current_entry.is_some() {
            self.page_selected = true;
        }
    }

    pub fn reset_input_fields(&mut self) {
        // create inputs from template
        self.current_template = self.templates.current_index();

        // get current template
        let template: &Template = self
            .templates
            .items
            .get(self.current_template.unwrap())
            .unwrap();
        self.text_fields.edit_fields = Some(StatefulList::with_items(
            vec![input_field(); template.elements.len() + 1])
        );

        // fill fields with new formatted inputs from the template
        for (field, temp) in self
            .text_fields
            .edit_fields
            .as_mut()
            .unwrap()
            .items
            .iter_mut()
            .zip(&template.elements)
        {
            let mut placeholder = "Enter or paste credential".to_string();
            if temp.private {
                placeholder.push_str("\nPress Enter to generate secure password");
                field.set_mask_char('\u{2022}');
            }

            field.set_placeholder_text(placeholder);
            field.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(temp.name.clone()),
            )
        }

        // setup confirm button
        // (for simplicity the last field is also a text field disguised as a button)
        self.style_editable_confirm("Insert");

        // move focus to right side
        self.select_template();
    }

    pub fn style_editable_confirm(&mut self, text: &str) {
        // styles the confirm button if available of a template
        if let Some(fields) = self
            .text_fields
            .edit_fields
            .as_mut()
        {
            if fields.len() > 0 {
                // style a confirm button
                let mut confirm_button = input_field();

                confirm_button.insert_str(text);
                confirm_button.set_alignment(Alignment::Center);
                confirm_button.set_cursor_style(Style::default());
                confirm_button.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );

                let index = fields.len() - 1;
                fields.items[index] = confirm_button;
            }
        }
    }

    pub fn select_template(&mut self) {
        // push right side of templates page to focus
        if self.current_template.is_some() {
            self.page_selected = true;
        }
    }

    pub fn fill_random_password(&mut self, i: usize) {
        // fills selected field with a random password
        let field = &mut self.text_fields.edit_fields.as_mut().unwrap().items[i];
        field.insert_str(generate_strong_password(24));
    }

    pub fn unselect_right(&mut self) {
        // push left side and tabs to focus
        self.page_selected = false;
    }

    pub fn update_entries(&mut self) {
        // updates the currently cached names according to the set filter if set
        let filter = self.text_fields.search_bar.lines()[0].as_str();

        self.entries_list.set_items(
            self.db_manager.get_entry_names(filter)
                .iter()
                .map(|s| s.clone())
                .collect()
        );
    }

    pub fn unlock_vault(&mut self) {
        // unlocks existing vault
        // sets app state according to if password is correct

        let master_key = derive_key(
            self.text_fields.password_input.lines()[0].clone(),
            &self.file_manager.get_salt().unwrap(),
        );

        // login if password correct
        if self.db_manager.check_key_correct(master_key.clone()) {
            log::info!("Login successful after {} failed attempts.", self.login_count);
            self.login_count = 0;

            // unlock vault and clear password
            self.db_manager.connect_to_db(master_key.clone());
            self.master_key = Some(SecureStorage::new(master_key));

            self.vault_state.state = LoginState::Unlocked;
            self.text_fields.password_input = password_field();

            // load entries and templates
            self.templates.set_items(self.db_manager.get_all_templates());
            self.update_entries();
            log::info!("Loaded templates from database");

        } else {
            self.vault_state.state = LoginState::IncorrectLogin;
            self.login_count += 1;
        }
    }

    pub fn setup_vault(&mut self) {
        // creates a new vault with entered credential
        // get key and clear fields
        let master_key = self.text_fields.password_input.lines()[0].clone();

        self.vault_state.clear_password();
        self.text_fields.password_input = password_field();

        // setup database
        self.db_manager.create_new_db();

        // derive key and store securely in memory
        let master_key = derive_key(master_key, &self.file_manager.get_salt().unwrap());
        self.master_key = Some(SecureStorage::new(master_key.clone()));

        // set password to new key which needed the sqlite3 salt
        self.db_manager.set_db_key(master_key);

        // unlock vault and load templates
        self.templates.set_items(self.db_manager.get_all_templates());
        self.vault_state.state = LoginState::Unlocked;

        log::info!("Created new vault");
    }

    pub fn lock_vault(&mut self) {
        // disconnects from database and locks vault
        self.db_manager.disconnect_from_db();
        self.vault_state.state = LoginState::Login;

        // clear clipboard and clean search field on exiting
        self.clipboard.force_clear_clipboard();
        self.text_fields.search_bar = input_field();

        log::info!("Reset Login for vault");
    }

    pub fn save_entry(&mut self) {
        // tries to save a new entry to database
        if self.all_fields_filled() {
            let mut values: Vec<String> = self.text_fields.edit_fields
                .as_ref()
                .unwrap()
                .items
                .iter()
                .map(|t| t.lines()[0].clone())
                .collect();

            // remove button
            values.pop();

            // select correct database from template in which to insert
            let database_name = self.templates.get_ref(
                self.current_template.unwrap()
            ).unwrap().db_name.clone();
            let success = self.db_manager.insert_entry(
                database_name,
                values,
                self.master_key.as_mut().unwrap().get_contents(),
            );
            self.insert_success = Some(success);

            // display depending if insert worked or not
            if success {
                // load entries and clear fields
                self.update_entries();
                self.reset_input_fields();

                // apply field style
                let fields = self.text_fields.edit_fields.as_mut().unwrap();
                fields.state.select(Some(fields.len() - 1));

                self.style_editable_confirm("Insert successful!");
            } else {
                // apply field style
                let fields = self.text_fields.edit_fields.as_mut().unwrap();
                let last = fields.len() - 1;
                fields.state.select(Some(last));

                self.style_editable_confirm("Name exists!");
            }
        }
    }

    pub fn delete_entry(&mut self) {
        // deletes entry from view and database
        let current = self.entries_list.current_item().unwrap().clone();
        self.db_manager.delete_entry(current);

        // remove from view and update entries
        self.current_entry = None;
        self.page_selected = false;

        self.update_entries();
    }

    pub fn all_fields_filled(&self) -> bool {
        // checks if all template fields are filled
        for field in self.text_fields.edit_fields.as_ref().unwrap().items.iter() {
            if field.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn copy_to_clipboard(&mut self, text: String) {
        // copies a string to clipboard
        self.clipboard.copy_to_clipboard(text.as_str());
        self.set_copied_state(Some(
            self.current_entry
                .as_ref()
                .unwrap()
                .1
                .current_index()
                .unwrap(),
        ));
    }

    pub fn set_copied_state(&mut self, state: Option<usize>) {
        let mut copied_state = self.clip_copied.lock().unwrap();
        copied_state.value = state;
    }

    pub fn get_copied_state(&self) -> Option<usize> {
        // returns the copied state behind a mutex
        self.clip_copied.lock().unwrap().value
    }
}
