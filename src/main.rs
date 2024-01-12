use std::error::Error;

use crate::{
    app::App,
    terminal::*,
    types::*,
};

mod ui;
mod app;
mod event;
mod types;
mod terminal;
mod password;


fn main() -> std::result::Result<(), Box<dyn Error>> {
    // main function to setup app and run
    let mut terminal = setup_terminal()?;
    let app = App::new();

    let result = app.run(&mut terminal);

    restore_terminal(terminal)?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}
