use std::{
    error::Error,
    ops::ControlFlow
};
use crossterm::event::{self, Event, KeyCode};
use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Esc => { return Ok(ControlFlow::Break(())); }
            KeyCode::Enter => {
                app.unlock_vault()
            }
            _ => {
                app.text_fields.password_input.input(key);
            }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
