use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use crate::app::stateful_list::StatefulList;
use crate::ui::fields::{input_field, password_field};

pub struct SingleValue<T> {
    pub value: T,
}

pub struct IndexManager {
    // manages page indexes
    pub index: usize,
    pub size: usize,
}

impl IndexManager {
    pub fn new(size: usize) -> IndexManager {
        // create new with specific size
        IndexManager { index: 0, size }
    }

    pub fn page_up(&mut self) {
        // moves page up by one, use rem_euclid to always get positive values
        self.index = (self.index + 1).rem_euclid(self.size);
    }

    pub fn page_down(&mut self) {
        // moves page down by one, fix for possible negative value
        self.index = (self.index as isize - 1).rem_euclid(self.size as isize) as usize;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    // template struct which gets created from json
    pub deletable: bool,
    pub name: String,
    pub elements: Vec<TemplateElement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateElement {
    // elements from json list
    pub name: String,
    pub private: bool,
}

pub struct EditableTextFields<'a> {
    // struct to hold all editable text fields, because they
    // need to be accessible in the event handling
    pub password_input: TextArea<'a>,
    pub search_bar: TextArea<'a>,
    pub edit_fields: Option<StatefulList<TextArea<'a>>>,
}

impl<'a> EditableTextFields<'a> {
    pub fn new() -> EditableTextFields<'a> {
        // initialises default fields
        EditableTextFields {
            password_input: password_field(),
            search_bar: input_field(),
            edit_fields: None,
        }
    }
}
