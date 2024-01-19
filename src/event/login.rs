use crate::password::validate_password_strength;
use crate::{
    app::{states::LoginState, App},
    ui::fields::password_field,
};
use crossterm::event::{self, Event, KeyCode};
use std::{error::Error, ops::ControlFlow};

pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handels events when logging in or registering
    if let Event::Key(key) = event::read()? {
        // check for special overall functions
        match key.code {
            // quit application
            KeyCode::Esc => {
                if app.login_count > 0 {
                    log::warn!("Exit with {} failed login attempts.", app.login_count); //If program is quit after failed logins, this will be logged!
                }
                return Ok(ControlFlow::Break(()));
            }
            // toggle character visibility
            KeyCode::Tab | KeyCode::BackTab => {
                let field = &mut app.text_fields.password_input;
                if field.mask_char().is_none() {
                    field.set_mask_char('\u{2022}');
                } else {
                    field.clear_mask_char();
                }
            }

            // otherwise check for state
            _ => {
                match app.vault_state.state {
                    LoginState::Login | LoginState::IncorrectLogin => match key.code {
                        KeyCode::Enter => app.unlock_vault(),
                        _ => {
                            app.text_fields.password_input.input(key);
                            app.vault_state.state = LoginState::Login;
                        }
                    },
                    // creating new vault and first password input
                    LoginState::Register => match key.code {
                        KeyCode::Enter => {
                            let pw_field = &mut app.text_fields.password_input;

                            if validate_password_strength(pw_field).0.is_none() {
                                app.vault_state.set_password(
                                    app.text_fields.password_input.lines()[0].clone(),
                                );
                                app.vault_state.state = LoginState::NewVaultConfirmNoMatch;
                                app.text_fields.password_input = password_field();
                            }
                        }
                        _ => {
                            app.text_fields.password_input.input(key);
                        }
                    },
                    LoginState::NewVaultConfirmMatch | LoginState::NewVaultConfirmNoMatch => {
                        match key.code {
                            KeyCode::Enter => {
                                if app
                                    .vault_state
                                    .clone()
                                    .check_pw(&app.text_fields.password_input.lines()[0]) {
                                    app.setup_vault();
                                }
                            }
                            _ => {
                                app.text_fields.password_input.input(key);
                                if app
                                    .vault_state
                                    .clone()
                                    .check_pw(&app.text_fields.password_input.lines()[0]) {
                                    app.vault_state.state = LoginState::NewVaultConfirmMatch;
                                } else {
                                    app.vault_state.state = LoginState::NewVaultConfirmNoMatch;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
