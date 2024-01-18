use crate::app::{states::LoginState, App};
use ratatui::Frame;

pub(crate) mod fields;
mod login;
mod manager;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // called by application to draw current contents on screen
    // selects function depending on app state
    match app.vault_state.state {
        LoginState::Unlocked => manager::draw_ui(frame, app),
        _ => login::draw_ui(frame, app),
    }
}
