use std::{
    error::Error,
    ops::ControlFlow,
};

use crossterm::event::{self, Event, KeyCode};
use crate::app::App;


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handles events when vault is unlocked
    if let Event::Key(key) = event::read()? {
        // match inputs depending on currently displayed page
        match app.page_index.index {
            0 => {
                match app.page_selected {
                    // credentials left side
                    false => match key.code {
                        KeyCode::Esc => { app.lock_vault(); }

                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),

                        KeyCode::Up => app.entries_list.previous(),
                        KeyCode::Down => app.entries_list.next(),

                        KeyCode::Enter => app.display_entry(),
                        KeyCode::Right | KeyCode::Left => app.select_entry(),

                        // fill input field if no matching action
                        _ => {
                            app.text_fields.search_bar.input(key);
                            app.entries_list.default_selected();
                        }
                    }
                    // credentials right side
                    true => match key.code {
                        KeyCode::Esc | KeyCode::Right | KeyCode::Left => { app.unselect_right(); }

                        // moves focus up or down on entries
                        KeyCode::Up | KeyCode::BackTab => { app.current_entry.as_mut().unwrap().previous(); }
                        KeyCode::Down | KeyCode::Tab => { app.current_entry.as_mut().unwrap().next(); }

                        KeyCode::Char('c') => {
                            let text = app.current_entry.as_ref().unwrap().current_item().unwrap().1.clone();
                            app.copy_to_clipboard(text);
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                match app.page_selected {
                    // insert left side
                    false => match key.code {
                        KeyCode::Esc => { app.lock_vault(); }

                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),

                        KeyCode::Up => app.templates.previous(),
                        KeyCode::Down => app.templates.next(),

                        KeyCode::Right => app.select_template(),
                        KeyCode::Enter => app.reset_input_fields(),

                        _ => {}
                    }
                    // insert right side
                    true => match key.code {
                        KeyCode::Esc => { app.unselect_right(); }

                        // moves focus up or down on entries
                        KeyCode::Up | KeyCode::BackTab => { app.text_fields.edit_fields.as_mut().unwrap().previous(); }
                        KeyCode::Down | KeyCode::Tab => { app.text_fields.edit_fields.as_mut().unwrap().next(); }

                        KeyCode::Enter => {
                            // select next or confirm button
                            let fields = app.text_fields.edit_fields.as_ref().unwrap();
                            if let Some(index) = fields.current_index() {
                                if index == fields.items.len() - 1 {
                                    app.save_entry();
                                } else {
                                    // fill with random credentials if empty
                                    if fields.current_item().unwrap().is_empty() {
                                        app.fill_random_password(index);
                                    }

                                    app.text_fields.edit_fields.as_mut().unwrap().next();
                                }
                            }
                        }

                        // fill focused field with user input
                        _ => {
                            let fields = app.text_fields.edit_fields.as_mut().unwrap();
                            if let Some(index) = fields.current_index() {
                                if index != fields.items.len() - 1 {
                                    app.text_fields.edit_fields.as_mut().unwrap().items[index].input(key);
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                // template page (under construction)
                match key.code {
                    KeyCode::Esc => { app.lock_vault(); }

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