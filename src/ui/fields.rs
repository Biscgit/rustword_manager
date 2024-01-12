use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Color, Style},
    style::Stylize,
    widgets::{Block, Borders, BorderType, Padding, Paragraph},
};
use tui_textarea::TextArea;

use crate::{
    app::{App, states::LoginState},
    password::validate_password_strength,
};


pub fn input_field<'a>() -> TextArea<'a> {
    let mut text_input = TextArea::default();
    text_input.set_cursor_line_style(Style::default());

    text_input
}

pub fn password_field<'a>() -> TextArea<'a> {
    // needs to be accessible from the events -> storing in app state
    let mut password_input = TextArea::default();

    password_input.set_cursor_line_style(Style::default());
    password_input.set_mask_char('\u{2022}'); //U+2022 BULLET (•)

    password_input
}