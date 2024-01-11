use ratatui::Frame;
use crate::app::App;

pub(crate) mod manager;
pub(self) mod login;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    match app.vault_unlocked {
        true => {manager::draw_ui(frame, app)}
        false => {login::draw_ui(frame, app)}
    }
}