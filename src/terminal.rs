use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::{error::Error, io::stdout};

use crate::types::Terminal;

pub fn setup_terminal() -> Result<Terminal, Box<dyn Error>> {
    // helper method to setup terminal. See Ratatui Manuals
    initialize_panic_handler();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    log::info!("Setup Terminal");
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<(), Box<dyn Error>> {
    // helper method to leave terminal. See Ratatui Manuals
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    log::info!("Restored Terminal");
    Ok(())
}

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        log::error!("Programm has panic-ed! Exiting...");

        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
