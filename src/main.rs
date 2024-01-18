use std::error::Error;

use crate::{app::App, terminal::*, types::*};
use chrono::prelude::Utc;

mod aes_impl;
mod app;
mod app_db_conn;
mod base64_enc_dec;
mod db_interface;
mod event;
mod file_manager;
mod key_processor;
mod logger;
mod password;
mod terminal;
mod types;
mod ui;


fn main() -> std::result::Result<(), Box<dyn Error>> {
    // main function to setup app and run
    logger::init_logger(&format!("RustwortManager_{}.log", Utc::now().format("%Y%m%d_%H%M%S"))); //Init logger; works globally
    let mut terminal = setup_terminal()?;
    let app = App::new();

    let result = app.run(&mut terminal);

    restore_terminal(terminal)?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}
