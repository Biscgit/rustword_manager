use std::error::Error;
use std::ops::ControlFlow;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => app.items.previous(),
                KeyCode::Down => app.items.next(),
                KeyCode::Char('q') => { return Ok(ControlFlow::Break(())); }
                _ => {}
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}