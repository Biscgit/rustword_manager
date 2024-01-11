use ratatui::layout::Alignment;
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders, BorderType};
use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use crate::{
    app_states::{LoginState, LoginStates},
    event::handle_events,
    stateful_list::StatefulList,
    ui::{draw_ui, login::{input_field, password_field}},
    types::Terminal,
};


pub struct App<'a> {
    pub vault_state: LoginStates,

    pub text_fields: EditableTextFields<'a>,

    pub entries_list: StatefulList<(&'a str, usize)>,
    // pub selected_entry: json

    pub template_names: StatefulList<(&'a str, usize)>,
    pub templates: Vec<Template>,
    pub current_template: Option<usize>,

    pub page_index: IndexManager,
    pub page_selected: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            vault_state: LoginStates::new(),

            text_fields: EditableTextFields::new(),
            entries_list: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 2),
            ]),
            template_names: StatefulList::with_items(vec![
                ("Simple Credential", 0),
                ("SSH-Keypair", 1),
                ("Note", 2),
            ]),
            templates: vec![
                serde_json::from_str(
                    r#"{
                        "deletable": false,
                        "name": "Web Credential",
                        "elements": [
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
                          {"name":  "Note", "private":  false}
                        ]
                    }"#
                ).unwrap(),
            ],
            current_template: None,
            page_index: IndexManager::new(3),
            page_selected: false,
        }
    }

    pub fn run(mut self, terminal: &mut Terminal) -> crate::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &mut self))?;
            if handle_events(&mut self)?.is_break() {
                return Ok(());
            }
        }
    }

    pub fn select_template(&mut self) {
        self.current_template = self.template_names.current();

        let template: &Template = &self.templates.get(self.current_template.unwrap()).unwrap();
        self.text_fields.edit_fields = Some(StatefulList::with_items(
            vec![input_field(); template.elements.len() + 1])
        );

        for (field, temp) in self.text_fields.edit_fields
            .as_mut()
            .unwrap()
            .items
            .iter_mut()
            .zip(&template.elements)
        {
            field.set_placeholder_text("Enter credential");
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

        confirm_button.insert_str("Insert!");
        confirm_button.set_alignment(Alignment::Center);
        confirm_button.set_cursor_style(Style::default());
        confirm_button.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        );

        // move focus to right side
        self.select_right();
    }

    pub fn select_right(&mut self) {
        // set selected to true for ui
        if self.current_template.is_some() {
            self.page_selected = true;
        }
    }

    pub fn unselect_template(&mut self) {
        self.page_selected = false;
    }

    pub fn unlock_vault(&mut self) {
        if self.text_fields.password_input.lines()[0] == "pass" {
            self.vault_state.state = LoginState::Unlocked;
        } else {
            self.vault_state.state = LoginState::IncorrectLogin;
        }
    }

    pub fn setup_vault(&mut self) {
        self.vault_state.state = LoginState::Unlocked
    }

    pub fn save_entry(&mut self) {
        // ToDo: check for fields and clear and insert into db
    }

    pub fn all_fields_filled(&self) -> bool {
        for field in self.text_fields.edit_fields.as_ref().unwrap().items.iter() {
            if field.is_empty() {
                return false;
            }
        }
        true
    }
}

pub struct IndexManager {
    pub index: usize,
    pub size: usize,
}

impl IndexManager {
    pub fn new(size: usize) -> IndexManager {
        IndexManager {
            index: 0,
            size,
        }
    }

    pub fn page_up(&mut self) {
        self.index = (self.index + 1).rem_euclid(self.size);
    }

    pub fn page_down(&mut self) {
        // fix for possible negative value
        self.index = (self.index as isize - 1).rem_euclid(self.size as isize) as usize;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub deletable: bool,
    pub name: String,
    pub elements: Vec<TemplateElement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateElement {
    pub name: String,
    pub private: bool,
}

pub struct EditableTextFields<'a> {
    pub password_input: TextArea<'a>,
    pub edit_fields: Option<StatefulList<TextArea<'a>>>,
}

impl<'a> EditableTextFields<'a> {
    pub fn new() -> EditableTextFields<'a> {
        EditableTextFields {
            password_input: password_field(),
            edit_fields: None,
        }
    }
}