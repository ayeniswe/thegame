//! A module for handling player input in a game, abstracting raw key events into high-level actions
//! like movement in a 2D coordinate space.
//!
//! This module is responsible for:
//! - Mapping physical keys (e.g., `WASD` or arrow keys) to high-level game actions (e.g., `PlayerMoveUp`).
//! - Handling key press and release events to determine player actions, such as movement direction.
//! - Supporting remapping of keys for customizable controls.
//! - Translating key events into movement coordinates for game logic.
//!
//! # Example
//!
//! ```
//! use crate::layout::{Coordinate, GameInputHandler, GameInput};
//! use winit::keyboard::KeyCode;
//!
//! let mut input_handler = GameInputHandler::default();
//! input_handler.update_binding(&GameInput::PlayerMoveUp, KeyCode::W.into());
//! input_handler.update_binding(&GameInput::PlayerMoveLeft, KeyCode::A.into());
//! input_handler.update_binding(&GameInput::PlayerMoveRight, KeyCode::D.into());
//! input_handler.update_binding(&GameInput::PlayerMoveDown, KeyCode::S.into());
//!
//! // Test input for movement
//! let input = Input::PhysicalKey(PhysicalKeyInfo {
//!     state: ElementState::Pressed,
//!     code: KeyCode::W.into(),
//! });
//! let movement = input_handler.to_coordinate(input);
//! assert_eq!(movement, Some(Coordinate { x: 0.0, y: -1.0 }));
//! ```
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

/// Responsible for abstracting and centralizing input management for player controls.
///
/// `GameInputHandler` decouples raw key events from gameplay logic by mapping
/// low-level key codes to high-level game actions. This allows the game to remain modular
/// and adaptable, supporting remapping and cross-platform input handling with minimal friction.
pub(crate) struct GameInputHandler {
    binding: HashMap<GameInput, PhysicalKey>,
    mapping: HashSet<PhysicalKey>,
}
impl GameInputHandler {
    /// Converts a raw key event into a coordinate, if it matches a known input mapping.
    ///
    /// UI overlay and Player actions consume these coordinates
    pub(crate) fn to_coordinate(&mut self, key: Input) -> Option<Coordinate> {
        let coordinate = match key {
            Input::PhysicalKey(key) => {
                if key.state == ElementState::Pressed {
                    self.mapping.insert(key.code);
                } else {
                    self.mapping.remove(&key.code);
                    return None;
                }

                if self.is_held(&GameInput::PlayerMoveUp)
                    && self.is_held(&GameInput::PlayerMoveLeft)
                {
                    Some(Coordinate { x: -1.0, y: -1.0 })
                }
                // Left + Up
                else if self.is_held(&GameInput::PlayerMoveDown)
                    && self.is_held(&GameInput::PlayerMoveLeft)
                {
                    Some(Coordinate { x: -1.0, y: 1.0 })
                } else if self.is_held(&GameInput::PlayerMoveDown)
                    && self.is_held(&GameInput::PlayerMoveRight)
                {
                    Some(Coordinate { x: 1.0, y: 1.0 })
                } else if self.is_held(&GameInput::PlayerMoveUp)
                    && self.is_held(&GameInput::PlayerMoveRight)
                {
                    Some(Coordinate { x: 1.0, y: -1.0 })
                } else if self.is_held(&GameInput::PlayerMoveLeft) || key.code == KeyCode::ArrowLeft
                {
                    Some(Coordinate { x: -1.0, y: 0.0 })
                } else if self.is_held(&GameInput::PlayerMoveRight)
                    || key.code == KeyCode::ArrowRight
                {
                    Some(Coordinate { x: 1.0, y: 0.0 })
                } else if self.is_held(&GameInput::PlayerMoveUp) || key.code == KeyCode::ArrowUp {
                    Some(Coordinate { x: 0.0, y: -1.0 })
                } else if self.is_held(&GameInput::PlayerMoveDown) || key.code == KeyCode::ArrowDown
                {
                    Some(Coordinate { x: 0.0, y: 1.0 })
                } else {
                    None
                }
            }
        };

        coordinate
    }
    pub(crate) fn is_held(&self, input: &GameInput) -> bool {
        let binding = self.get_binding(input);
        self.mapping.get(binding).is_some()
    }
    pub(crate) fn get_binding(&self, input: &GameInput) -> &PhysicalKey {
        self.binding.get(input).unwrap()
    }
    pub(crate) fn update_binding(&mut self, input: &GameInput, key: PhysicalKey) {
        *self.binding.get_mut(input).unwrap() = key
    }
}
impl Default for GameInputHandler {
    fn default() -> Self {
        Self {
            binding: [
                (GameInput::PlayerMoveUp, PhysicalKey::Code(KeyCode::ArrowUp)),
                (
                    GameInput::PlayerMoveLeft,
                    PhysicalKey::Code(KeyCode::ArrowLeft),
                ),
                (
                    GameInput::PlayerMoveRight,
                    PhysicalKey::Code(KeyCode::ArrowRight),
                ),
                (
                    GameInput::PlayerMoveDown,
                    PhysicalKey::Code(KeyCode::ArrowDown),
                ),
            ]
            .into(),
            mapping: HashSet::new(),
        }
    }
}

/// Represents a high-level abstraction of user input events.
///
/// Used to decouple game logic from raw platform-specific input events.
#[derive(Debug, Clone)]
pub(crate) enum Input {
    PhysicalKey(PhysicalKeyInfo),
}
#[derive(Debug, Clone)]
pub(crate) struct PhysicalKeyInfo {
    pub(crate) state: ElementState,
    pub(crate) code: PhysicalKey,
}

/// Stores a comprehensive list of all accepted input actions
#[derive(PartialEq, Eq, Hash)]
pub(crate) enum GameInput {
    PlayerMoveUp,
    PlayerMoveLeft,
    PlayerMoveRight,
    PlayerMoveDown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::ElementState;

    #[test]
    fn test_to_coordinate_with_physical_key() {
        let mut handler = GameInputHandler::default();
        handler.update_binding(&GameInput::PlayerMoveUp, PhysicalKey::Code(KeyCode::KeyW));
        handler.update_binding(&GameInput::PlayerMoveLeft, PhysicalKey::Code(KeyCode::KeyA));
        handler.update_binding(
            &GameInput::PlayerMoveRight,
            PhysicalKey::Code(KeyCode::KeyD),
        );
        handler.update_binding(&GameInput::PlayerMoveDown, PhysicalKey::Code(KeyCode::KeyS));

        let test_cases = vec![
            // Press Left Arrow key (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                Some(Coordinate { x: -1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                None,
            ),
            // Press Right Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                Some(Coordinate { x: 1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                None,
            ),
            // Press Up Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                Some(Coordinate { x: 0.0, y: -1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                None,
            ),
            // Press Down Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                Some(Coordinate { x: 0.0, y: 1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                None,
            ),
            // Press UI Only Left Arrow key (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::ArrowLeft),
                }),
                Some(Coordinate { x: -1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::ArrowLeft),
                }),
                None,
            ),
            // Press UI Only Right Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::ArrowRight),
                }),
                Some(Coordinate { x: 1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::ArrowRight),
                }),
                None,
            ),
            // Press UI Only Up Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::ArrowUp),
                }),
                Some(Coordinate { x: 0.0, y: -1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::ArrowUp),
                }),
                None,
            ),
            // Press UI Only Down Arrow (and released)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::ArrowDown),
                }),
                Some(Coordinate { x: 0.0, y: 1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::ArrowDown),
                }),
                None,
            ),
            // Key not tracked
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::Power),
                }),
                None,
            ),
            // Press Left Arrow and Up Arrow (and released both)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                Some(Coordinate { x: -1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                Some(Coordinate { x: -1.0, y: -1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                None,
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                None,
            ),
            // Press Left Arrow and Down Arrow (and released both)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                Some(Coordinate { x: -1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                Some(Coordinate { x: -1.0, y: 1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                None,
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyA),
                }),
                None,
            ),
            // Press Right Arrow and Down Arrow (and released both)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                Some(Coordinate { x: 1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                Some(Coordinate { x: 1.0, y: 1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyS),
                }),
                None,
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                None,
            ),
            // Press Right Arrow and Up Arrow (and released both)
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                Some(Coordinate { x: 1.0, y: 0.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Pressed,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                Some(Coordinate { x: 1.0, y: -1.0 }),
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyW),
                }),
                None,
            ),
            (
                Input::PhysicalKey(PhysicalKeyInfo {
                    state: ElementState::Released,
                    code: PhysicalKey::Code(KeyCode::KeyD),
                }),
                None,
            ),
        ];

        for (input, expected_coord) in test_cases {
            let result = handler.to_coordinate(input.clone());
            assert_eq!(result, expected_coord, "Failed for {:?}", input);
        }
    }
}
