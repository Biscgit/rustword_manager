use std::{
    error::Error,
    ops::ControlFlow,
    time::Duration,
};
use crossterm::event;
use crate::{app::{App, states::LoginState}};

 mod manager;
 mod login;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handles events like resizing window and key presses every 100ms
    // processes depending on current app state and display
    if event::poll(Duration::from_millis(100))? {
        return match app.vault_state.state {
            LoginState::Unlocked => { manager::handle_events(app) }
            _ => { login::handle_events(app) }
        };
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
