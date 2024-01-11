use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Color, Style},
    style::Stylize,
    widgets::{Block, Borders, BorderType, Padding},
};
use tui_textarea::TextArea;
use crate::{
    app::App,
    password::validate_password_strength,
};


pub fn password_field<'a>() -> TextArea<'a> {
    // needs to be accessible from the events -> storing in app state
    let mut password_input = TextArea::default();

    password_input.set_cursor_line_style(Style::default());
    password_input.set_mask_char('\u{2022}'); //U+2022 BULLET (•)
    password_input.set_placeholder_text("Please enter your password");

    password_input
}


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    // center layout
    let area = frame.size();
    let center_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10)
        ],
    ).split(Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(area.height / 2 - 2),
            Constraint::Length(3),
            Constraint::Length(area.height / 2 - 1)
        ],
    ).split(area)[1]);

    match app.vault_setup {
        true => login_with_password(frame, app, center_layout[1]),
        false => new_password(frame, app, center_layout[1]),
    }
}

fn login_with_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    let mut password_field = &mut app.text_fields.password_input;
    password_field.set_style(Style::default().fg(Color::LightYellow));
    password_field.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .fg(Color::LightYellow)
            .padding(Padding::horizontal(1))
            .title("Enter password")
    );

    frame.render_widget(password_field.widget(), area);
}

fn new_password<'a>(frame: &mut Frame, app: &'a mut App, area: Rect) {
    let mut password_field = &mut app.text_fields.password_input;
    // set design depending on validation of password strength
    if let Some(error) = validate_password_strength(&mut password_field) {
        password_field.set_style(Style::default().fg(Color::LightRed));
        password_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .fg(Color::LightRed)
                .padding(Padding::horizontal(1))
                .title(error)
        );
    } else {
        password_field.set_style(Style::default().fg(Color::LightGreen));
        password_field.set_block(
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::LightGreen)
                .padding(Padding::horizontal(1))
                .title("Valid Password")
        );
    }

    frame.render_widget(password_field.widget(), area);
}