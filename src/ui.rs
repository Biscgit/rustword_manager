use ratatui::Frame;
use crate::{app::{App, states::LoginState}};

mod manager;
mod login;
pub(crate) mod fields;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // called by application to draw current contents on screen
    // selects function depending on app state
    match app.vault_state.state {
        LoginState::Unlocked => { manager::draw_ui(frame, app) }
        _ => { login::draw_ui(frame, app) }
    }
}