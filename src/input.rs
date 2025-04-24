use crossbeam::channel::{unbounded, SendError, Sender};
use crossterm::event::{Event, KeyCode};
use log::{debug, error, warn};
use std::{collections::HashMap, sync::Arc, thread};
use thiserror::Error;

use crate::{layout::Coordinate, sync::Subscriber};

/// The `EventReader` that can delegate to real
/// event readers
trait EventReader: Send + Sync {
    /// Reads the next input event
    fn read_event(&self) -> Result<Event, std::io::Error>;
}

/// Real-time input from the terminal using `crossterm` backend
struct CrosstermEventReader;
impl EventReader for CrosstermEventReader {
    fn read_event(&self) -> Result<Event, std::io::Error> {
        crossterm::event::read()
    }
}

/// Centralizes terminal gameplay key input handling, mapping keys to in-game
/// actions and broadcasting them to all subscribed components.
pub(crate) struct GameInputHandler {
    mapping: HashMap<GameInput, KeyCode>,
    subscribers: Vec<Sender<Coordinate>>,
    event_reader: Arc<dyn EventReader>,
}
impl GameInputHandler {
    /// Spawns a background thread to listen for key events and publish movement coordinates.
    /// Enables decoupled, real-time input handling for interactive components.
    pub(crate) fn start(self: Arc<Self>) {
        thread::spawn(move || loop {
            match self.event_reader.read_event() {
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
            event_reader: Arc::new(CrosstermEventReader),
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
    #[error("received non-key event")]
    NonKeyEvent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::Receiver;
    use crossterm::event::{Event, KeyCode, KeyEvent};
    use logtest::Logger;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use std::{io, thread};

    struct TestSubscriber {
        pub received: Arc<Mutex<Vec<Coordinate>>>,
    }
    impl Subscriber<Coordinate> for TestSubscriber {
        fn subscribe(&mut self, rx: Receiver<Coordinate>) {
            let received = Arc::clone(&self.received);
            std::thread::spawn(move || {
                for msg in rx.iter() {
                    received.lock().unwrap().push(msg);
                }
            });
        }
    }

    #[test]
    fn test_all_log_errors() {
        let logger = Logger::start();

        // --- Produce IOReadError log ---
        struct SingleRightKeyReader;
        impl EventReader for SingleRightKeyReader {
            fn read_event(&self) -> Result<Event, io::Error> {
                Ok(Event::Key(KeyEvent::from(KeyCode::Right)))
            }
        }
        struct DropReceiverSubscriber;
        impl Subscriber<Coordinate> for DropReceiverSubscriber {
            fn subscribe(&mut self, rx: Receiver<Coordinate>) {
                drop(rx); // Drop the receiver immediately to simulate broken pipe
            }
        }

        let mut handler = GameInputHandler {
            mapping: GameInputHandler::default().mapping,
            subscribers: Vec::new(),
            event_reader: Arc::new(SingleRightKeyReader),
        };
        let mut subscriber = DropReceiverSubscriber;
        handler.subscribe(&mut subscriber);

        let handler = Arc::new(handler);
        handler.start();
        // Wait for the key event to be processed
        thread::sleep(Duration::from_millis(50));

        // --- Produce GameInputBroadcastError log ---
        struct FailingReader;
        impl EventReader for FailingReader {
            fn read_event(&self) -> Result<Event, io::Error> {
                Err(io::Error::new(io::ErrorKind::Other, "simulated failure"))
            }
        }

        let mut handler = GameInputHandler {
            mapping: GameInputHandler::default().mapping,
            subscribers: Vec::new(),
            event_reader: Arc::new(FailingReader),
        };
        let mut dummy_subscriber = TestSubscriber {
            received: Arc::new(Mutex::new(Vec::new())),
        };
        handler.subscribe(&mut dummy_subscriber);

        let handler = Arc::new(handler);
        handler.start();
        // Wait for the IO error to be processed
        thread::sleep(Duration::from_millis(50));

        // --- Produce NonKeyEvent log ---
        struct NonKeyEventReader;
        impl EventReader for NonKeyEventReader {
            fn read_event(&self) -> Result<Event, std::io::Error> {
                Ok(Event::Resize(80, 24)) // simulate non-key event
            }
        }

        let mut handler = GameInputHandler {
            mapping: GameInputHandler::default().mapping,
            subscribers: Vec::new(),
            event_reader: Arc::new(NonKeyEventReader),
        };
        let mut dummy_subscriber = TestSubscriber {
            received: Arc::new(Mutex::new(Vec::new())),
        };
        handler.subscribe(&mut dummy_subscriber);

        let handler = Arc::new(handler);
        handler.start();
        // Wait for the non-key event to be processed
        thread::sleep(Duration::from_millis(50));

        let logs: Vec<_> = logger.collect();
        assert!(
            logs.iter().any(|rec| {
                rec.level() == log::Level::Error
                    && rec.args().contains("unable to read input event")
            }),
            "Expected IOReadError to be logged"
        );
        assert!(
            logs.iter().any(|rec| {
                rec.level() == log::Level::Error
                    && rec.args().contains("unable to send key input update")
            }),
            "Expected GameInputBroadcastError to be logged"
        );
        assert!(
            logs.iter().any(|rec| {
                rec.level() == log::Level::Warn && rec.args().contains("received non-key event")
            }),
            "Expected NonKeyEvent to be logged"
        );
    }

    #[test]
    fn test_all_direction_key_inputs() {
        struct MockEventReader {
            events: Arc<Mutex<Vec<Event>>>,
        }
        impl EventReader for MockEventReader {
            fn read_event(&self) -> Result<Event, std::io::Error> {
                let mut evs = self.events.lock().unwrap();
                if let Some(e) = evs.pop() {
                    Ok(e)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "No more events",
                    ))
                }
            }
        }

        let expected_coords = vec![
            Coordinate { x: -1.0, y: 0.0 }, // Left
            Coordinate { x: 1.0, y: 0.0 },  // Right
            Coordinate { x: 0.0, y: -1.0 }, // Up
            Coordinate { x: 0.0, y: 1.0 },  // Down
        ];

        let key_events = vec![
            Event::Key(KeyEvent::from(KeyCode::Left)),
            Event::Key(KeyEvent::from(KeyCode::Right)),
            Event::Key(KeyEvent::from(KeyCode::Up)),
            Event::Key(KeyEvent::from(KeyCode::Down)),
        ];

        let events = Arc::new(Mutex::new(key_events.into_iter().rev().collect()));
        let mock_reader = Arc::new(MockEventReader {
            events: Arc::clone(&events),
        });

        let mut handler = GameInputHandler {
            mapping: GameInputHandler::default().mapping,
            subscribers: Vec::new(),
            event_reader: mock_reader,
        };

        let received = Arc::new(Mutex::new(Vec::new()));
        let mut subscriber = TestSubscriber {
            received: Arc::clone(&received),
        };

        handler.subscribe(&mut subscriber);
        let handler_clone = Arc::new(handler);
        handler_clone.start();
        // Wait a bit to let all key events process
        thread::sleep(Duration::from_millis(100));

        let result = received.lock().unwrap();
        assert_eq!(result.len(), expected_coords.len());

        for (expected, actual) in expected_coords.iter().zip(result.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_subscribe_and_publish() {
        let mut handler = GameInputHandler::default();
        let received = Arc::new(Mutex::new(Vec::new()));
        let mut test_subscriber = TestSubscriber {
            received: Arc::clone(&received),
        };

        handler.subscribe(&mut test_subscriber);
        let coordinate = Coordinate { x: 1.0, y: 0.0 };
        handler.publish(coordinate).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(50)); // let thread process

        let received_data = received.lock().unwrap();
        assert_eq!(received_data.len(), 1);
        assert_eq!(received_data[0], coordinate);
    }

    #[test]
    fn test_keymap_translation() {
        let handler = GameInputHandler::default();
        assert_eq!(handler.get_keymap(&GameInput::PlayerMoveUp), &KeyCode::Up);
        assert_eq!(
            handler.get_keymap(&GameInput::PlayerMoveLeft),
            &KeyCode::Left
        );
        assert_eq!(
            handler.get_keymap(&GameInput::PlayerMoveRight),
            &KeyCode::Right
        );
        assert_eq!(
            handler.get_keymap(&GameInput::PlayerMoveDown),
            &KeyCode::Down
        );
    }

    #[test]
    fn test_update_keymap() {
        let mut handler = GameInputHandler::default();
        handler.update_keymap(&GameInput::PlayerMoveUp, KeyCode::Char('w'));
        assert_eq!(
            handler.get_keymap(&GameInput::PlayerMoveUp),
            &KeyCode::Char('w')
        );
    }
}
