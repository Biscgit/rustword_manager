use std::{
    error::Error,
    ops::ControlFlow
};

use crate::app::App;

pub(self) mod manager;
pub(self) mod login;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    match app.vault_unlocked {
        true => manager::handle_events(app),
        false => login::handle_events(app),
    }
}
