use ratatui::Frame;
use crate::{app::{App, states::LoginState}};

mod manager;
pub(crate) mod login;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    match app.vault_state.state {
        LoginState::Unlocked => { manager::draw_ui(frame, app) }
        _ => { login::draw_ui(frame, app) }
    }
}