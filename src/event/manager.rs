use std::{error::Error, ops::ControlFlow};

use crate::app::App;
use crossterm::event::{self, Event, KeyCode};


pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    // handles events when vault is unlocked
    if let Event::Key(key) = event::read()? {
        // match inputs depending on currently displayed page
        match app.page_index.index {
            0 => {
                match app.page_selected {
                    // credentials left side
                    false => match key.code {
                        KeyCode::Esc => {
                            app.lock_vault();
                        }

                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),

                        KeyCode::Up => app.entries_list.previous(),
                        KeyCode::Down => app.entries_list.next(),

                        KeyCode::Enter => {
                            app.display_entry();
                            app.select_entry();
                        }
                        KeyCode::Right => {
                            if app.text_fields.search_bar.is_empty() {
                                if app.current_entry.is_none() {
                                    app.display_entry();
                                }
                                app.select_entry();
                            } else {
                                app.text_fields.search_bar.input(key);
                                app.update_entries();
                            }
                        }

                        // fill input field if no matching action
                        _ => {
                            app.text_fields.search_bar.input(key);
                            app.update_entries();
                        }
                    },
                    // credentials right side
                    true => match key.code {
                        KeyCode::Esc | KeyCode::Right | KeyCode::Left => {
                            app.unselect_right();
                            app.delete_confirm = false;
                        }

                        // moves focus up or down on entries
                        KeyCode::Up => {
                            app.current_entry.as_mut().unwrap().1.previous();
                            app.delete_confirm = false;
                        }
                        KeyCode::Down => {
                            app.current_entry.as_mut().unwrap().1.next();
                            app.delete_confirm = false;
                        }

                        KeyCode::Tab | KeyCode::BackTab => {
                            let entry = app
                                .current_entry
                                .as_mut()
                                .unwrap()
                                .1
                                .current_item_mut()
                                .unwrap();
                            entry.2 = !entry.2;
                        }

                        KeyCode::Enter => {
                            let entries = app.current_entry.as_ref().unwrap();
                            if entries.1.current_index().unwrap() == entries.1.items.len() - 1 {
                                // delete entry when confirmed
                                if app.delete_confirm {
                                    app.delete_entry();
                                    app.delete_confirm = false;
                                } else {
                                    app.delete_confirm = true;
                                }
                            }
                        }

                        // copy by pressing "c"
                        KeyCode::Char('c') => {
                            let entries = app.current_entry.as_ref().unwrap();
                            if entries.1.current_index().unwrap() != entries.1.items.len() - 1 {
                                let text = app
                                    .current_entry
                                    .as_ref()
                                    .unwrap()
                                    .1
                                    .current_item()
                                    .unwrap()
                                    .1
                                    .clone();
                                app.copy_to_clipboard(text);
                            }
                        }
                        _ => {}
                    },
                }
            }
            1 => {
                match app.page_selected {
                    // insert left side
                    false => match key.code {
                        KeyCode::Esc => {
                            app.lock_vault();
                        }

                        KeyCode::Tab => app.page_index.page_up(),
                        KeyCode::BackTab => app.page_index.page_down(),

                        KeyCode::Up => app.templates.previous(),
                        KeyCode::Down => app.templates.next(),

                        KeyCode::Right => {
                            if app.current_template.is_some() {
                                app.select_template();
                            } else {
                                app.reset_input_fields();
                            }

                            // set view back to selected
                            app.templates.set_index(app.current_template.unwrap());
                        }
                        KeyCode::Enter => app.reset_input_fields(),

                        _ => {}
                    },
                    // insert right side
                    true => match key.code {
                        KeyCode::Esc => {
                            app.unselect_right();
                        }

                        // moves focus up or down on entries
                        KeyCode::Up => {
                            app.text_fields.edit_fields.as_mut().unwrap().previous();
                            if app.insert_success.unwrap_or(false) {
                                app.style_editable_confirm("Insert");
                            }
                        }
                        KeyCode::Down => {
                            app.text_fields.edit_fields.as_mut().unwrap().next();
                            if app.insert_success.unwrap_or(false) {
                                app.style_editable_confirm("Insert");
                            }
                        }

                        KeyCode::Tab | KeyCode::BackTab => {
                            let fields = app
                                .text_fields
                                .edit_fields
                                .as_mut()
                                .unwrap();

                            // toggle mask if not button and private
                            if fields.current_index().unwrap() != fields.items.len() - 1 {
                                let current_temp = app.templates.get_ref(app.current_template.unwrap()).unwrap();

                                if current_temp.elements[fields.current_index().unwrap()].private {
                                    let current_input = fields
                                        .current_item_mut()
                                        .unwrap();

                                    if current_input.mask_char().is_none() {
                                        current_input.set_mask_char('\u{2022}');
                                    } else {
                                        current_input.clear_mask_char();
                                    }
                                }
                            }
                        }

                        KeyCode::Enter => {
                            // select next or confirm button
                            let fields = app.text_fields.edit_fields.as_ref().unwrap();
                            if let Some(index) = fields.current_index() {
                                if index == fields.items.len() - 1 {
                                    app.save_entry();
                                } else {
                                    // fill with random credentials if empty and a private field
                                    let curr_temp = app.templates.current_item().unwrap();

                                    if fields.current_item().unwrap().is_empty()
                                        && curr_temp.elements[index].private {
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
                                    app.text_fields.edit_fields.as_mut().unwrap().items[index]
                                        .input(key);
                                }

                                if index == 0 {
                                    app.insert_success = None;
                                    app.style_editable_confirm("Insert");
                                }
                            }
                        }
                    },
                }
            }
            2 => {
                // template page (under construction)
                match key.code {
                    KeyCode::Esc => {
                        app.lock_vault();
                    }

                    KeyCode::Tab => app.page_index.page_up(),
                    KeyCode::BackTab => app.page_index.page_down(),

                    _ => {}
                }
            }
            _ => unreachable!(),
        }
    }

    // continue receiving input if nothing matches
    Ok(ControlFlow::Continue(()))
}
