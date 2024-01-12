use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use crate::{
    stateful_list::StatefulList,
    ui::login::{input_field, password_field}
};


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
    pub search_bar: TextArea<'a>,
    pub edit_fields: Option<StatefulList<TextArea<'a>>>,
}

impl<'a> EditableTextFields<'a> {
    pub fn new() -> EditableTextFields<'a> {
        EditableTextFields {
            password_input: password_field(),
            search_bar: input_field(),
            edit_fields: None,
        }
    }
}