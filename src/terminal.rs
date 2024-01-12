use std::{error::Error, io::stdout};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;

use crate::types::Terminal;


pub fn setup_terminal() -> Result<Terminal, Box<dyn Error>> {
    // helper method to setup terminal. See Ratatui Manuals
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<(), Box<dyn Error>> {
    // helper method to leave terminal. See Ratatui Manuals
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
