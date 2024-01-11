use std::error::Error;
use std::ops::ControlFlow;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::app::App;

pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handle key presses and other events
    if event::poll(Duration::from_millis(100))? {
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
                                KeyCode::Enter => {}, // ToDo -> Unlock vault with password
                                _ => {app.text_fields.password_input.input(key); }
                            }
                        }
                        1 => {
                            match key.code {
                                KeyCode::Up => app.entries_list.previous(),
                                KeyCode::Down => app.entries_list.next(),

                                KeyCode::Right => app.page_side.page_up(),
                                KeyCode::Left => app.page_side.page_down(),

                                _ => {}
                            }
                        }
                        2 => {
                            match key.code {
                                KeyCode::Up => app.template_names.previous(),
                                KeyCode::Down => app.template_names.next(),

                                KeyCode::Right => app.page_side.page_up(),
                                KeyCode::Left => app.page_side.page_down(),

                                KeyCode::Enter => app.select_template(),

                                _ => {}
                            }
                        }
                        3 => {}
                        _ => unreachable!()
                    }
                }
            }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}