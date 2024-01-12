use ratatui::prelude::Style;
use tui_textarea::TextArea;


pub fn input_field<'a>() -> TextArea<'a> {
    // creates a new input field
    let mut text_input = TextArea::default();
    text_input.set_cursor_line_style(Style::default());

    text_input
}

pub fn password_field<'a>() -> TextArea<'a> {
    // creates a new password input field
    let mut password_input = TextArea::default();

    password_input.set_cursor_line_style(Style::default());
    password_input.set_mask_char('\u{2022}'); //U+2022 BULLET (•)

    password_input
}