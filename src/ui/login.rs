use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Style},
    style::Stylize,
    widgets::{Block, Borders, BorderType, Padding, Paragraph},
};
use tui_textarea::TextArea;

use crate::{
    app::App,
    app_states::LoginState,
    password::validate_password_strength,
};


pub fn password_field<'a>() -> TextArea<'a> {
    // needs to be accessible from the events -> storing in app state
    let mut password_input = TextArea::default();

    password_input.set_cursor_line_style(Style::default());
    password_input.set_mask_char('\u{2022}'); //U+2022 BULLET (•)

    password_input
}

pub fn input_field<'a>() -> TextArea<'a> {
    let mut text_input = TextArea::default();
    text_input.set_cursor_line_style(Style::default());

    text_input
}


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // center layout
    let area = frame.size();
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Length(area.height / 2 - 3),
            Constraint::Length(3),
            Constraint::Length(area.height / 2 - 3)
        ],
    ).split(area);
    let center_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ],
    ).split(main_layout[2]);

    // draw title
    frame.render_widget(
        Paragraph::new("Welcome to the RustwordManager!")
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
            ),
        main_layout[0],
    );

    match app.vault_state.state {
        LoginState::Login |
        LoginState::IncorrectLogin => login_with_password(frame, app, center_layout[1]),
        LoginState::Register |
        LoginState::NewVaultConfirmMatch |
        LoginState::NewVaultConfirmNoMatch => register_password(frame, app, center_layout[1]),
        _ => unreachable!()
    }
}

fn login_with_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    let password_field = &mut app.text_fields.password_input;
    password_field.set_placeholder_text("Please enter your password");

    if app.vault_state.state == LoginState::Login {
        password_field.set_style(Style::default().fg(Color::LightYellow));
        password_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .fg(Color::LightYellow)
                .padding(Padding::horizontal(1))
                .title("Enter password")
        );
    } else {
        password_field.set_style(Style::default().fg(Color::LightRed));
        password_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .fg(Color::LightRed)
                .padding(Padding::horizontal(1))
                .title("Invalid Password! Try again!")
        );
    }

    frame.render_widget(password_field.widget(), area);
}

fn register_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    match app.vault_state.state {
        // new vault password create
        LoginState::Register => {
            first_password(frame, app, area)
        }
        LoginState::NewVaultConfirmMatch | LoginState::NewVaultConfirmNoMatch => {
            confirm_password(frame, app, area)
        }
        _ => unreachable!()
    }
}

fn first_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    let mut pw_field = &mut app.text_fields.password_input;
    pw_field.set_placeholder_text("Please enter a strong password");


    // set design depending on validation of password strength
    if let Some(error) = validate_password_strength(&mut pw_field) {
        pw_field.set_style(Style::default().fg(Color::LightRed));
        pw_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .fg(Color::LightRed)
                .padding(Padding::horizontal(1))
                .title(error)
        );
    } else {
        pw_field.set_style(Style::default().fg(Color::LightGreen));
        pw_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::LightGreen)
                .padding(Padding::horizontal(1))
                .title("Valid Password")
        );
    }

    frame.render_widget(pw_field.widget(), area);
}

fn confirm_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    let pw_field = &mut app.text_fields.password_input;
    pw_field.set_placeholder_text("Please confirm your password");

    match app.vault_state.state {
        LoginState::NewVaultConfirmMatch => {
            pw_field.set_style(Style::default().fg(Color::LightGreen));
            pw_field.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::LightGreen)
                    .padding(Padding::horizontal(1))
                    .title("Press Enter to confirm")
            );
        }
        LoginState::NewVaultConfirmNoMatch => {
            pw_field.set_style(Style::default().fg(Color::LightRed));
            pw_field.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::LightRed)
                    .padding(Padding::horizontal(1))
                    .title("Password do not match!")
            )
        }
        _ => unreachable!()
    }

    frame.render_widget(pw_field.widget(), area);
}