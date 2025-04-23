use crossbeam::channel::{unbounded, SendError, Sender};
use crossterm::event::{read, Event, KeyCode};
use log::{debug, error, info, warn};
use std::{collections::HashMap, sync::Arc, thread};
use thiserror::Error;

use crate::{layout::Coordinate, sync::Subscriber};

/// Centralizes terminal gameplay key input handling, mapping keys to in-game
/// actions and broadcasting them to all subscribed components.
pub(crate) struct GameInputHandler {
    mapping: HashMap<GameInput, KeyCode>,
    subscribers: Vec<Sender<Coordinate>>,
}
impl GameInputHandler {
    /// Spawns a background thread to listen for key events and publish movement coordinates.
    /// Enables decoupled, real-time input handling for interactive components.
    pub(crate) fn start(self: Arc<Self>) {
        thread::spawn(move || loop {
            match read() {
                Ok(Event::Key(event)) => {
                    let coordinate = match event.code {
                        KeyCode::Left => Coordinate { x: -1.0, y: 0.0 },
                        KeyCode::Right => Coordinate { x: 1.0, y: 0.0 },
                        KeyCode::Up => Coordinate { x: 0.0, y: -1.0 },
                        KeyCode::Down => Coordinate { x: 0.0, y: 1.0 },
                        _ => continue,
                    };
                    debug!("{:?}", coordinate);
                    
                    if let Err(e) = self.publish(coordinate) {
                        error!("{}", e);
                    }
                }
                Ok(_) => {
                    warn!("{}", GameInputError::NonKeyEvent);
                }
                Err(e) => {
                    error!("{}", GameInputError::IOReadError(e));
                }
            }
        });
    }
    pub(crate) fn subscribe(&mut self, subscriber: &mut dyn Subscriber<Coordinate>) {
        let (tx, rx) = unbounded::<Coordinate>();
        self.subscribers.push(tx);
        subscriber.subscribe(rx);
    }
    pub(crate) fn publish(&self, coordinate: Coordinate) -> Result<(), GameInputError> {
        for sub in &self.subscribers {
            sub.send(coordinate)?
        }
        Ok(())
    }
    pub(crate) fn get_keymap(&self, input: &GameInput) -> &KeyCode {
        self.mapping.get(input).unwrap()
    }
    pub(crate) fn update_keymap(&mut self, input: &GameInput, key: KeyCode) {
        *self.mapping.get_mut(input).unwrap() = key
    }
}
impl Default for GameInputHandler {
    fn default() -> Self {
        Self {
            mapping: [
                (GameInput::PlayerMoveUp, KeyCode::Up),
                (GameInput::PlayerMoveLeft, KeyCode::Left),
                (GameInput::PlayerMoveRight, KeyCode::Right),
                (GameInput::PlayerMoveDown, KeyCode::Down),
            ]
            .into(),
            subscribers: Vec::default(),
        }
    }
}

/// Stores a comprehensive list of all input actions
#[derive(PartialEq, Eq, Hash)]
pub(crate) enum GameInput {
    PlayerMoveUp,
    PlayerMoveLeft,
    PlayerMoveRight,
    PlayerMoveDown,
}

#[derive(Debug, Error)]
enum GameInputError {
    #[error("unable to read input event (IO failure): {0}")]
    IOReadError(#[from] std::io::Error),
    #[error("unable to send key input update to one or more subscribers: {0}")]
    GameInputBroadcastError(#[from] SendError<Coordinate>),
    #[error("received key input with no associated action")]
    NoKeyAction,
    #[error("received non-key event")]
    NonKeyEvent,
}
