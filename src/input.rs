use crossterm::event::{read, Event, KeyCode};
use thiserror::Error;

use crate::Coordinate;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("failed to read input event: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("unhandled key input")]
    UnhandledKey,
    #[error("failed to read key event")]
    ReadFailure,
}

/// Handles terminal key input and maps directional keys to coordinate movement.
pub(crate) struct InputHandler;
impl InputHandler {
    pub(crate) fn handler() -> Result<Coordinate, InputError> {
        if let Event::Key(e) = read()? {
            match e.code {
                KeyCode::Left => Ok(Coordinate { x: -1.0, y: 0.0 }),
                KeyCode::Right => Ok(Coordinate { x: 1.0, y: 0.0 }),
                KeyCode::Up => Ok(Coordinate { x: 0.0, y: -1.0 }),
                KeyCode::Down => Ok(Coordinate { x: 0.0, y: 1.0 }),
                _ => Err(InputError::UnhandledKey),
            }
        } else {
            Err(InputError::ReadFailure)
        }
    }
}
