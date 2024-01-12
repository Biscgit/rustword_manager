use std::{
    error::Error,
    ops::ControlFlow,
};

use crossterm::event::{self, Event, KeyCode};
use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handle key presses and other events
    if let Event::Key(key) = event::read()? {
        // match inputs depending on currently displayed page
        // default events
        // page specific events
        match app.page_index.index {
            0 => {
                match key.code {
                    KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

                    KeyCode::Tab => app.page_index.page_up(),
                    KeyCode::BackTab => app.page_index.page_down(),

                    KeyCode::Up => app.entries_list.previous(),
                    KeyCode::Down => app.entries_list.next(),

                    _ => { app.text_fields.search_bar.input(key); }
                }
            }
            1 => {
                match app.page_selected {
                    false => match key.code {
                        KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),

                        KeyCode::Up => app.template_names.previous(),
                        KeyCode::Down => app.template_names.next(),

                        KeyCode::Right => app.select_template(),
                        KeyCode::Enter => app.reset_input_fields(),

                        _ => {}
                    }
                    true => match key.code {
                        KeyCode::Esc => { app.unselect_right(); }

                        KeyCode::Up | KeyCode::BackTab => { app.text_fields.edit_fields.as_mut().unwrap().previous(); }
                        KeyCode::Down | KeyCode::Tab => { app.text_fields.edit_fields.as_mut().unwrap().next(); }

                        KeyCode::Enter => {
                            let fields = app.text_fields.edit_fields.as_ref().unwrap();
                            if let Some(index) = fields.current() {
                                if index == fields.items.len() - 1 {
                                    app.save_entry();
                                } else {
                                    app.text_fields.edit_fields.as_mut().unwrap().next();
                                }
                            }
                        }
                        _ => {
                            let fields = app.text_fields.edit_fields.as_mut().unwrap();
                            if let Some(index) = fields.current() {
                                if index != fields.items.len() - 1 {
                                    app.text_fields.edit_fields.as_mut().unwrap().items[index].input(key);
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                match key.code {
                    KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

                    KeyCode::Tab => app.page_index.page_up(),
                    KeyCode::BackTab => app.page_index.page_down(),

                    _ => {}
                }
            }
            _ => unreachable!()
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}