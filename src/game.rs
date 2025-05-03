//! Core game loop state responsible for rendering, updating, and input-driven movement logic.
//!
//! `GameState` orchestrates the runtime loop of a 2D character-based game:
//! - Processes input via a non-blocking channel
//! - Advances animations and draws characters to the screen
//! - Maintains a consistent framerate and applies delta time for smooth motion
//!
//! The system is designed to be modular by:
//! - Accepting any `Character` and `Screen` implementations
//! - Using crossbeam channels for decoupled asynchronous input reception
//!
//! # Key Responsibilities
//! - Drive frame updates (logic and rendering)
//! - Apply time-based player movement
//! - Ensure consistent frame pacing with sleep-based throttling
//!
//! # Example
//!
//! ```
//! let mut state = GameState::new(...);
//! state.start(); // begins the main game loop
//! ```
use crossbeam::channel::Receiver;
use log::error;
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, Instant},
};
use thiserror::Error;

use crate::prelude::*;

pub(crate) struct GameState<S: Screen, C: Character<S>> {
    input_handler: Option<Receiver<Coordinate>>,
    delta: f32,
    player: C,
    player_pos: Coordinate,
    player_speed: f32,
    screen: Arc<Mutex<S>>,
    fps: Duration,
}
impl<S: Screen, C: Character<S>> GameState<S, C> {
    pub(crate) fn new(
        fps: u64,
        player_speed: f32,
        player_pos: Coordinate,
        player: C,
        screen: Arc<Mutex<S>>,
    ) -> Self {
        Self {
            player,
            player_pos,
            player_speed,
            fps: Duration::from_micros(1_000_000 / fps),
            input_handler: None,
            delta: f32::default(),
            screen,
        }
    }
    pub(crate) fn start(mut self) {
        thread::spawn(move || {
            if let Some(rx) = self.input_handler.take() {
                loop {
                    let _ = self.update(rx.clone()).inspect_err(|e| error!("{}", e));
                }
            } else {
                error!("{}", GameStateError::NoInputHandlerError)
            }
        });
    }
    fn update(&mut self, rx: Receiver<Coordinate>) -> Result<(), WindowError> {
        // Track movement
        let input: Option<Coordinate> = if let Ok(inp) = rx.try_recv() {
            self.player_pos += inp * self.player_speed * self.delta;
            Some(inp)
        } else {
            None
        };

        // Frame animation
        let tick = Instant::now();
        match input {
            // Walk to Left
            Some(Coordinate { x: -1.0, .. }) => {
                self.player.side_walk().play(
                    self.screen.clone(),
                    self.delta,
                    MirrorDirection::FlipVertical,
                    self.player_pos,
                )?;
            }
            // Walk to Right
            Some(Coordinate { x: 1.0, .. }) => {
                self.player.side_walk().play(
                    self.screen.clone(),
                    self.delta,
                    MirrorDirection::None,
                    self.player_pos,
                )?;
            }
            // Walk Down
            Some(Coordinate { y: 1.0, .. }) => {
                self.player.front_walk().play(
                    self.screen.clone(),
                    self.delta,
                    MirrorDirection::None,
                    self.player_pos,
                )?;
            }
            // Walk Up
            Some(Coordinate { y: -1.0, .. }) => {
                self.player.back_walk().play(
                    self.screen.clone(),
                    self.delta,
                    MirrorDirection::None,
                    self.player_pos,
                )?;
            }
            _ => {
                self.player.idle().play(
                    self.screen.clone(),
                    self.delta,
                    MirrorDirection::None,
                    self.player_pos,
                )?;
            }
        }
        // Guarantee frames arent cut short and
        // exhaust their max view time
        let elapsed = tick.elapsed();
        if elapsed < self.fps {
            sleep(self.fps - elapsed)
        }

        // Keep frame-rate independent and consistent
        self.delta = Instant::now().duration_since(tick).as_secs_f32();

        Ok(())
    }
}
impl<S: Screen, C: Character<S>> Subscriber<Coordinate> for GameState<S, C> {
    fn subscribe(&mut self, rx: Receiver<Coordinate>) {
        self.input_handler = Some(rx);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game::GameState,
        layout::Coordinate,
        mock::{MockCharacter, MockScreen},
    };
    use crossbeam::channel;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    #[test]
    fn test_player_movement_applied() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: 1.0, y: -1.0 }).unwrap();
        gs.update(rx).unwrap();

        // The player should have moved right by 10 units
        assert_eq!(gs.player_pos, Coordinate { x: 10.0, y: -10.0 });
    }

    #[test]
    fn test_framerate_independence() {
        let (_, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 0.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        gs.update(rx).unwrap();

        assert!(gs.delta > 0.016 && gs.delta < 0.017);
    }

    #[test]
    fn test_idle_animation() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: 0.0, y: 0.0 }).unwrap();
        gs.update(rx).unwrap();
        
        assert_eq!(gs.player.animation_trigerred, "idle")
    }
    #[test]
    fn test_left_side_walk_animation() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: -1.0, y: 0.0 }).unwrap();
        gs.update(rx).unwrap();
        
        assert_eq!(gs.player.animation_trigerred, "side")
    }
    #[test]
    fn test_right_side_walk_animation() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: 1.0, y: 0.0 }).unwrap();
        gs.update(rx).unwrap();
        
        assert_eq!(gs.player.animation_trigerred, "side")
    }
    #[test]
    fn test_back_animation() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: 0.0, y: -1.0 }).unwrap();
        gs.update(rx).unwrap();
        
        assert_eq!(gs.player.animation_trigerred, "back")
    }
    #[test]
    fn test_front_animation() {
        let (tx, rx) = channel::unbounded();
        let mut gs = GameState {
            input_handler: Some(rx.clone()),
            delta: 1.0,
            player: MockCharacter::new(),
            player_pos: Coordinate::default(),
            player_speed: 10.0,
            screen: Arc::new(Mutex::new(MockScreen::new(50, 50))),
            fps: Duration::from_millis(16),
        };

        tx.send(Coordinate { x: 0.0, y: 1.0 }).unwrap();
        gs.update(rx).unwrap();
        
        assert_eq!(gs.player.animation_trigerred, "front")
    }
}

#[derive(Debug, Error)]
pub enum GameStateError {
    #[error("input handler not detected")]
    NoInputHandlerError,
}
