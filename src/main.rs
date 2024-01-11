mod ui;
mod app;
mod app_states;
mod stateful_list;
mod event;
mod types;
mod terminal;
mod password;


use std::error::Error;

use crate::{
    app::App,
    terminal::*,
    types::*,
};


fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let app = App::new();

    let result = app.run(&mut terminal);

    restore_terminal(terminal)?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}
