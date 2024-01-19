use chrono::Utc;
use std::error::Error;

use crate::{app::App, terminal::*, types::*};
use crate::file_manager::FileManager;

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
    // main: programm start
    let mut file_manager = FileManager::new();

    // check if instance is already running
    if !file_manager.check_lock_set()? {
        // setup global logger
        logger::init_logger(file_manager.get_logger_path());
        log::info!("Setup global logger");

        let mut terminal = setup_terminal()?;
        let app = App::new(&mut file_manager);

        let result = app.run(&mut terminal);

        restore_terminal(terminal)?;

        if let Err(err) = result {
            eprintln!("{err:?}");
        }

        file_manager.release_file_lock()?;
    } else {
        eprintln!(
            "\nAn instance is already running\n\
            File 'home/RustwordManager/lock' is present\n"
        );
    }
    Ok(())
}
