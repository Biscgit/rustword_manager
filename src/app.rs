use arboard::Clipboard;
use ratatui::{
    layout::Alignment,
    prelude::Style,
    widgets::{Block, Borders, BorderType},
};
use stateful_list::StatefulList;

use self::{
    extras::*,
    states::{LoginState, LoginStates},
};
use crate::{
    event::handle_events,
    password::generate_strong_password,
    types::Terminal,
    ui::{draw_ui, fields::{input_field, password_field}},
};

pub(crate) mod states;
mod extras;
mod clipboard;
mod stateful_list;

pub struct App<'a> {
    // App handling all states and storage of the application

    pub vault_state: LoginStates,
    pub text_fields: EditableTextFields<'a>,

    pub entries_list: StatefulList<(&'a str, usize)>,
    pub current_entry: Option<StatefulList<(&'a str, &'a str)>>,
    pub delete_confirm: bool,

    pub templates: StatefulList<Template>,
    pub current_template: Option<usize>,

    pub page_index: IndexManager,
    pub page_selected: bool,

    pub clipboard: Clipboard,
    pub copied: Option<usize>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        // creates a new with testing values
        App {
            vault_state: LoginStates::new(),
            text_fields: EditableTextFields::new(),

            entries_list: StatefulList::with_items(vec![
                ("Item0", 0),
                ("Item1", 1),
                ("Item2", 2),
                ("Item3", 3),
                ("Item4", 4),
                ("Item5", 5),
                ("Item6", 6),
                ("Item7", 7),
                ("Item8", 8),
                ("Item9", 9),
            ]),
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
                    }"#
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
                    }"#
                ).unwrap(),
                serde_json::from_str(
                    r#"{
                        "deletable": false,
                        "name": "Note",
                        "elements": [
                          {"name":  "Name", "private":  false},
                          {"name":  "Note", "private":  false}
                        ]
                    }"#
                ).unwrap(),
            ]),
            current_template: None,
            delete_confirm: false,

            page_index: IndexManager::new(3),
            page_selected: false,

            clipboard: Clipboard::new().unwrap(),
            copied: None,
        }
    }

    pub fn run(mut self, terminal: &mut Terminal) -> crate::Result<()> {
        // runs application forever until exited. Draws to the screen and handles events
        loop {
            terminal.draw(|f| draw_ui(f, &mut self))?;
            if handle_events(&mut self)?.is_break() {
                return Ok(());
            }
        }
    }

    pub fn display_entry(&mut self) {
        // ToDo: set entry from DB
        if self.entries_list.current_index().is_some() {
            self.current_entry = Some(StatefulList::with_items(vec![
                ("Title1", "Content1 and this is a very long content or password or idk"),
                ("Title2", "Content2"),
                ("Title3", "Content3"),
                ("", ""),
            ]));

            self.copied = None;
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
        let template: &Template = self.templates.items.get(self.current_template.unwrap()).unwrap();
        self.text_fields.edit_fields = Some(StatefulList::with_items(
            vec![input_field(); template.elements.len() + 1])
        );

        // fill fields with new formatted inputs from the template
        for (field, temp) in self.text_fields.edit_fields
            .as_mut()
            .unwrap()
            .items
            .iter_mut()
            .zip(&template.elements)
        {
            let mut placeholder = "Enter or paste credential".to_string();
            if temp.private {
                placeholder.push_str("\nPress Enter to generate secure password");
            }

            field.set_placeholder_text(placeholder);
            field.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(temp.name.clone())
            )
        }

        // setup confirm button
        // (for simplicity the last field is also a text field disguised as a button)
        let confirm_button = self.text_fields.edit_fields.as_mut().unwrap().items.last_mut().unwrap();

        // style confirm button
        confirm_button.insert_str("Insert");
        confirm_button.set_alignment(Alignment::Center);
        confirm_button.set_cursor_style(Style::default());
        confirm_button.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
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
        field.insert_str(generate_strong_password(20));
    }

    pub fn unselect_right(&mut self) {
        // push left side and tabs to focus
        self.page_selected = false;
    }

    pub fn lock_vault(&mut self) {
        // disconnects from database and locks vault
        // ToDo: remove connection from DB etc.
        self.vault_state.state = LoginState::Login;
    }

    pub fn unlock_vault(&mut self) {
        // unlocks existing vault
        // sets app state according to if password is correct
        // ToDo: check password
        if self.text_fields.password_input.lines()[0] == "pass" {
            // unlock vault and clear password
            self.vault_state.state = LoginState::Unlocked;
            self.text_fields.password_input = password_field();
        } else {
            self.vault_state.state = LoginState::IncorrectLogin;
        }
    }

    pub fn setup_vault(&mut self) {
        // creates a new vault with entered credential

        // unlock vault and clear password
        self.vault_state.clear_password();
        self.vault_state.state = LoginState::Unlocked;
        self.text_fields.password_input = password_field();
    }

    pub fn save_entry(&mut self) {
        // tries to save a new entry to database
        if self.all_fields_filled() {
            // ToDo: send input to database

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

        self.copied = Some(self.current_entry.as_ref().unwrap().current_index().unwrap());
        self.clipboard.set_text(text).unwrap();
    }

    pub fn update_shown_entries(&mut self, _filter: String) {
        // updates entries in list in relation to search filter
    }
}
