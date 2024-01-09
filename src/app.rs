use serde::{Deserialize, Serialize};


use crate::{
    event::handle_events,
    stateful_list::StatefulList,
    ui::draw_ui,
    types::Terminal,
};


pub struct App<'a> {
    pub entries_list: StatefulList<(&'a str, usize)>,
    // pub selected_entry: json

    pub template_names: StatefulList<(&'a str, usize)>,
    pub templates: Vec<Template>,
    pub current_template: Option<usize>,

    pub page_index: PageManager,
}


impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
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
                        "elements": [
                          {"name":  "Username", "private":  false},
                          {"name":  "Password", "private":  true}
                        ]
                    }"#
                ).unwrap(),
                serde_json::from_str(
                    r#"{
                        "deletable": false,
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
                        "elements": [
                          {"name":  "Note", "private":  false}
                        ]
                    }"#
                ).unwrap(),
            ],
            current_template: None,
            page_index: PageManager::new(),
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
    }
}

pub struct PageManager {
    pub index: usize,
}

impl PageManager {
    pub fn new() -> PageManager {
        PageManager {
            index: 0
        }
    }

    pub fn page_up(&mut self) {
        self.index = (self.index + 1).rem_euclid(3);
    }

    pub fn page_down(&mut self) {
        // fix for possible negative value
        self.index = (self.index as isize - 1).rem_euclid(3) as usize;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    deletable: bool,
    elements: Vec<TemplateElement>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateElement {
    name: String,
    private: bool,
}