use crate::event::handle_events;
use crate::stateful_list::StatefulList;
use crate::ui::draw_ui;
use crate::types::Terminal;


pub struct App<'a> {
    pub items: StatefulList<(&'a str, usize)>,
}


impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
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
            ])
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