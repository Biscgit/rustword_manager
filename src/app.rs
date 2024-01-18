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

    pub entries_list: StatefulList<(String, usize)>,
    pub current_entry: Option<StatefulList<(&'a str, &'a str, bool)>>,
    pub delete_confirm: bool,

    pub templates: StatefulList<Template>,
    pub current_template: Option<usize>,

    pub page_index: IndexManager,
    pub page_selected: bool,

    pub clipboard: clipboard_thread::ClipboardManager,
    clip_copied: ClState,

    pub file_manager: FileManager,
    db_manager: AppDBConnector,
    master_key: Option<SecureStorage>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        // creates a new with testing values
        let file_manager = FileManager::new();
        let path = file_manager.create_path().unwrap();
        let copied = Arc::new(Mutex::new(SingleValue { value: None }));

        App {
            vault_state: LoginStates::new(file_manager.check_db_exist()),
            text_fields: EditableTextFields::new(),

            entries_list: StatefulList::with_items(vec![]),
            current_entry: None,

            templates: StatefulList::with_items(vec![
                serde_json::from_str(
                    r#"{
                        "deletable": false,
                        "name": "Web Credential",
                        "elements": [
                          {"name":  "Name", "private":  false},
                          {"name":  "Username", "private":  false},
                          {"name":  "Password", "private":  true}
                        ]
                    }"#,
                ).unwrap(),
                serde_json::from_str(
                    r#"{
                        "deletable": false,
                        "name": "SSH-Keypair",
                        "elements": [
                          {"name":  "Name", "private":  false},
                          {"name":  "Website", "private":  false},
                          {"name":  "SSH-Public", "private":  false},
                          {"name":  "SSH-Private", "private":  true}
                        ]
                    }"#,
                ).unwrap(),
                serde_json::from_str(
                    r#"{
                        "deletable": false,
                        "name": "Note",
                        "elements": [
                          {"name":  "Name", "private":  false},
                          {"name":  "Note", "private":  false}
                        ]
                    }"#,
                ).unwrap(),
            ]),
            current_template: None,
            delete_confirm: false,

            page_index: IndexManager::new(3),
            page_selected: false,

            clipboard: clipboard_thread::ClipboardManager::new(Arc::clone(&copied)),
            clip_copied: copied,
            file_manager,

            db_manager: AppDBConnector::new(path),
            master_key: None,
        }
    }

    pub fn run(mut self, terminal: &mut Terminal) -> crate::Result<()> {
        // runs application forever until exited. Draws to the screen and handles events
        loop {
            if handle_events(&mut self)?.is_break() {
                return Ok(());
            }

            terminal.draw(|f| draw_ui(f, &mut self))?;
        }
    }

    pub fn display_entry(&mut self) {
        // ToDo: set entry from DB
        if self.entries_list.current_index().is_some() {
            self.current_entry = Some(StatefulList::with_items(vec![
                ("Title1", "Content1 and this is a very long content or password or idk", false),
                ("Title2", "Content2 is hidden", true),
                ("Title3", "Content3", false),
                ("Title3", "", true),
                ("", "", false),
            ]));

            self.set_copied_state(None);
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
        let confirm_button = self
            .text_fields
            .edit_fields
            .as_mut()
            .unwrap()
            .items
            .last_mut()
            .unwrap();

        // style confirm button
        confirm_button.insert_str("Insert");
        confirm_button.set_alignment(Alignment::Center);
        confirm_button.set_cursor_style(Style::default());
        confirm_button.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        // move focus to right side
        self.select_template();
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

    pub fn unlock_vault(&mut self) {
        // unlocks existing vault
        // sets app state according to if password is correct

        let master_key = derive_key(self.text_fields.password_input.lines()[0].clone());

        // login if password correct
        // if self.db_manager.connect_to_db(path, master_key.clone()) {
        if self.db_manager.check_key_correct(master_key.clone()) {
            // unlock vault and clear password
            self.db_manager.connect_to_db(master_key.clone());
            self.master_key = Some(SecureStorage::new(master_key));

            self.vault_state.state = LoginState::Unlocked;
            self.text_fields.password_input = password_field();

            // load entries
            self.update_entries();
        } else {
            self.vault_state.state = LoginState::IncorrectLogin;
        }
    }

    pub fn update_entries(&mut self) {
        let filter = self.text_fields.search_bar.lines()[0].as_str();

        self.entries_list.set_items(
            self.db_manager.get_entry_names(filter)
                .iter()
                .enumerate()
                .map(|(i, s)| (s.clone(), i))
                .collect()
        );
    }

    pub fn lock_vault(&mut self) {
        // disconnects from database and locks vault
        self.db_manager.disconnect_from_db();
        self.vault_state.state = LoginState::Login;

        // clear clipboard on exiting
        self.clipboard.force_clear_clipboard();
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
        let master_key = derive_key(master_key);
        self.master_key = Some(SecureStorage::new(master_key.clone()));

        // set password to new key which needed the sqlite3 salt
        self.db_manager.set_db_key(master_key);

        // unlock vault
        self.vault_state.state = LoginState::Unlocked;
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

            // ToDo: correct template
            let template_name = "tp_simple".to_string();
            self.db_manager.insert_entry(
                template_name,
                values,
                self.master_key.as_mut().unwrap().get_contents(),
            );

            // load entries and clear fields
            self.update_entries();
            self.reset_input_fields();
        }
    }

    pub fn delete_entry(&mut self) {
        // deletes entry from view and database
        // ToDo: delete entry from database

        // remove from view
        self.current_entry = None;
        self.page_selected = false;
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

    pub fn copy_to_clipboard(&mut self, text: &str) {
        // copies a string to clipboard
        // ToDo: thread to reset clipboard after time

        self.clipboard.copy_to_clipboard(text);
        self.set_copied_state(Some(
            self.current_entry
                .as_ref()
                .unwrap()
                .current_index()
                .unwrap(),
        ));
    }

    pub fn set_copied_state(&mut self, state: Option<usize>) {
        let mut copied_state = self.clip_copied.lock().unwrap();
        copied_state.value = state;
    }

    pub fn get_copied_state(&self) -> Option<usize> {
        self.clip_copied.lock().unwrap().value
    }

    pub fn update_shown_entries(&mut self, _filter: String) {
        // updates entries in list in relation to search filter
    }
}
