use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use crate::{
    stateful_list::StatefulList,
    ui::fields::{password_field, input_field}
};


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