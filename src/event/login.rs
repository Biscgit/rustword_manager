use std::{
    error::Error,
    ops::ControlFlow,
};
use crossterm::event::{self, Event, KeyCode};
use crate::{
    app::{App, LoginState},
    ui::login::password_field,
};


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Esc {
            return Ok(ControlFlow::Break(()));
        }

        match app.vault_state.state {
            LoginState::Login | LoginState::IncorrectLogin => match key.code {
                KeyCode::Enter => {
                    app.unlock_vault()
                }
                _ => {
                    app.text_fields.password_input.input(key);
                    app.vault_state.state = LoginState::Login;
                }
            }
            // creating new vault and first password input
            LoginState::NewVault => match key.code {
                KeyCode::Enter => {
                    app.vault_state.set_password(
                        &app.text_fields.password_input.lines()[0]
                    );
                    app.vault_state.state = LoginState::NewVaultConfirmNoMatch;
                    app.text_fields.password_input = password_field();
                }
                _ => {
                    app.text_fields.password_input.input(key);
                }
            }
            LoginState::NewVaultConfirmMatch | LoginState::NewVaultConfirmNoMatch => match key.code {
                KeyCode::Enter => {
                    if app.vault_state.clone().check_pw(
                        &app.text_fields.password_input.lines()[0]
                    ) {
                        app.setup_vault();
                    }
                }
                _ => {
                    app.text_fields.password_input.input(key);
                    if app.vault_state.clone().check_pw(
                        &app.text_fields.password_input.lines()[0]
                    ) {
                        app.vault_state.state = LoginState::NewVaultConfirmMatch;
                    } else {
                        app.vault_state.state = LoginState::NewVaultConfirmNoMatch;
                    }
                }
            }
            _ => {}
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
