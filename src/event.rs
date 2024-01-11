use std::{
    error::Error,
    ops::ControlFlow,
    time::Duration,
};
use crossterm::event;
use crate::app::App;

pub(self) mod manager;
pub(self) mod login;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        return match app.vault_unlocked {
            true => { manager::handle_events(app) }
            false => { login::handle_events(app) }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
