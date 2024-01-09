use std::{
    error::Error,
    ops::ControlFlow,
    sync::mpsc,
    thread,
    time::{Duration},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};

use crate::app::App;


// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum _Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}


// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.page_index.index {
                0 => {
                    match key.code {
                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),
                        KeyCode::Up => app.entries_list.previous(),
                        KeyCode::Down => app.entries_list.next(),
                        KeyCode::Char('q') => { return Ok(ControlFlow::Break(())); }
                        _ => {}
                    }
                }
                1 => {
                    match key.code {
                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),
                        KeyCode::Up => app.template_names.previous(),
                        KeyCode::Down => app.template_names.next(),
                        KeyCode::Enter => app.select_template(),
                        KeyCode::Char('q') => { return Ok(ControlFlow::Break(())); }
                        _ => {}
                    }
                }
                2 => {
                    match key.code {
                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),
                        KeyCode::Char('q') => { return Ok(ControlFlow::Break(())); }
                        _ => {}
                    }
                }
                _ => unreachable!()
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}