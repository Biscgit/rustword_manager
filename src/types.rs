use crate::app::extras::SingleValue;
use ratatui::backend::CrosstermBackend;
use std::sync::{Arc, Mutex};
use std::{error::Error, io::Stdout};

// often used types
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub type ClState = Arc<Mutex<SingleValue<Option<usize>>>>;
