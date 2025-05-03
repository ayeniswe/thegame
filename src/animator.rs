//! The `Animation` trait allows sprites to be animated over time using a sequence of
//! frames, each with an optional duration. It includes support for:
//! - Time-based frame progression (`delta`)
//! - Mirroring transformations (`MirrorDirection`)
//! - Dynamic on-screen positioning (`Coordinate`)
//!
//! This trait is automatically implemented for any type that implements `Sprite`.
//!
//! ## Responsibilities
//! - Tracks animation progress based on frame durations and game delta time
//! - Applies optional vertical or horizontal mirroring to rendered frames
//! - Draws each pixel in the current frame at the given offset on the screen
//!
//! ## Frame Timing
//! If a frame does not define an explicit `duration`, a default duration is used,
//! calculated as an even slice of 1 second (i.e., `1.0 / frame_count`).
//!
//! ## Mirroring
//! Mirroring operations are performed relative to the width or height of the current
//! frame, not the overall sprite. This ensures correct flipping in-place.
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::prelude::*;

/// A trait for animating a `Sprite` on a terminal interface.
pub trait Animation<S: Screen>: Sprite {
    /// Plays the animation frame-by-frame with optional mirroring and position offset.
    fn play(
        &mut self,
        screen: Arc<Mutex<S>>,
        delta: f32,
        mirror: MirrorDirection,
        offset: Coordinate,
    ) -> Result<(), WindowError> {
        // Total time to show the frame (or default to evely used interval)
        let duration = self.frames()[self.frame_pos()]
            .duration
            .unwrap_or_else(|| Duration::from_secs_f32(1.0 / self.frames().len() as f32))
            .as_secs_f32();

        // Coordinate frame intervals with game frame rate
        *self.timer_mut() += delta;
        if self.timer() >= duration {
            *self.timer_mut() -= duration;
            *self.frame_pos_mut() = (self.frame_pos() + 1) % self.frames().len()
        }

        let mut screen_lock = screen
            .lock()
            .map_err(|e| WindowError::ScreenLockError(e.to_string()))?;

        screen_lock.clear()?;

        let frame = &self.frames()[self.frame_pos()];
        for p in &frame.pixels {
            // Ignores the mirror direction value since the value must be covered by
            // the frames dimensions
            match mirror {
                MirrorDirection::FlipVertical => p.draw(
                    &mut *screen_lock,
                    MirrorDirectionValue::FlipVertical(frame.width),
                    offset.clone(),
                ),
                MirrorDirection::FlipHorizontal => p.draw(
                    &mut *screen_lock,
                    MirrorDirectionValue::FlipHorizontal(frame.height),
                    offset.clone(),
                ),
                MirrorDirection::None => p.draw(
                    &mut *screen_lock,
                    MirrorDirectionValue::None,
                    offset.clone(),
                ),
            }
        }

        screen_lock.render()?;

        Ok(())
    }
}
impl<S: Screen, T: Sprite> Animation<S> for T {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{
        layout::{Coordinate, MirrorDirection},
        mock::{MockCharacter, MockScreen},
        sprite::character::character::Character,
    };

    #[test]
    fn test_animation_frame_advance() {
        let mut sprite = MockCharacter::new();
        let screen = Arc::new(Mutex::new(MockScreen::new(50, 50)));

        // Each call to `play` will advance by one frame if delta exceeds 1 / frames.len()
        // So if delta is 1.0 and 3 frames: 1/3 per frame → will advance
        sprite
            .idle()
            .play(
                screen.clone(),
                0.4,
                MirrorDirection::None,
                Coordinate::default(),
            )
            .unwrap();

        // Check that frame does not advance from 0
        assert_eq!(sprite.idle().frame_pos(), 0);

        sprite
            .idle()
            .play(
                screen.clone(),
                1.0,
                MirrorDirection::None,
                Coordinate::default(),
            )
            .unwrap();

        // Check that frame advanced from 0 → 1
        assert_eq!(sprite.idle().frame_pos(), 1);

        sprite
            .idle()
            .play(
                screen.clone(),
                1.0,
                MirrorDirection::None,
                Coordinate::default(),
            )
            .unwrap();
        assert_eq!(sprite.idle().frame_pos(), 0); // loops anad start aniamtion over
    }
}
