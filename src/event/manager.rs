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

            _ => {
                // page specific events
                match app.page_index.index {
                    0 => {
                        match key.code {
                            KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

                            KeyCode::Up => app.entries_list.previous(),
                            KeyCode::Down => app.entries_list.next(),

                            _ => {}
                        }
                    }
                    1 => {
                        match app.page_selected {
                            false => match key.code {
                                KeyCode::Esc => { return Ok(ControlFlow::Break(())); }

                                KeyCode::Up => app.template_names.previous(),
                                KeyCode::Down => app.template_names.next(),

                                KeyCode::Right => app.select_right(),
                                KeyCode::Enter => app.select_template(),

                                _ => {}
                            }
                            true => match key.code {
                                KeyCode::Esc => { app.unselect_template() }

                                KeyCode::Up => { app.text_fields.edit_fields.as_mut().unwrap().previous(); }
                                KeyCode::Down => { app.text_fields.edit_fields.as_mut().unwrap().next(); }
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
                    2 => {}
                    _ => unreachable!()
                }
            }
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}