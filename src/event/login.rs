use std::error::Error;
use std::ops::ControlFlow;
use crate::app::App;

pub fn handle_events(app: &mut App) -> Result<ControlFlow<()>, Box<dyn Error>> {
    Ok(ControlFlow::Continue(()))
}