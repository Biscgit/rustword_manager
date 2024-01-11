use std::{
    error::Error,
    ops::ControlFlow,
};
use std::hint::unreachable_unchecked;
use crossterm::event::{self, Event, KeyCode};
use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handle key presses and other events
    if let Event::Key(key) = event::read()? {
        // match inputs depending on currently displayed page
        // default events
        match key.code {
            KeyCode::Tab => app.page_index.page_up(),
            KeyCode::BackTab => app.page_index.page_down(),

            KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

            _ => {
                // page specific events
                match app.page_index.index {
                    0 => {
                        match key.code {
                            KeyCode::Up => app.entries_list.previous(),
                            KeyCode::Down => app.entries_list.next(),

                            _ => {}
                        }
                    }
                    1 => {
                        match app.page_selected {
                            false => match key.code {
                                KeyCode::Up => app.template_names.previous(),
                                KeyCode::Down => app.template_names.next(),

                                KeyCode::Enter => app.select_template(),

                                _ => {}
                            }
                            true => match key.code {
                                KeyCode::Up => { app.text_fields.edit_fields.as_mut().unwrap().previous(); },
                                KeyCode::Down | KeyCode::Enter => { app.text_fields.edit_fields.as_mut().unwrap().next(); },
                                _ => {
                                    let index = app.text_fields.edit_fields.as_ref().unwrap().current().clone().unwrap();
                                    app.text_fields.edit_fields.as_mut().unwrap().items[index].input(key);
                                }
                            }
                        }
                    }
                    2 => {}
                    _ => unreachable!()
                }
            }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}