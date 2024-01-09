use crate::event::handle_events;
use crate::stateful_list::StatefulList;
use crate::ui::draw_ui;
use crate::types::Terminal;


pub struct App<'a> {
    pub entries_list: StatefulList<(&'a str, usize)>,
    pub template_list: StatefulList<(&'a str, usize)>,
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
            template_list: StatefulList::with_items(vec![
                ("Template1", 1),
                ("Template2", 1),
                ("Template3", 1),
            ]),
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