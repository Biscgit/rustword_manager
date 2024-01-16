use std::{error::Error, io::Stdout};
use std::sync::{Arc, Mutex};
use ratatui::backend::CrosstermBackend;
use crate::app::extras::SingleValue;



// often used types
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub type ClState = Arc<Mutex<SingleValue<Option<usize>>>>;
