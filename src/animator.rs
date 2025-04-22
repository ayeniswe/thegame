use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use std::io::Stdout;

use crate::{
    layout::{Coordinate, MirrorDirection},
    sprite::sprite::Sprite,
};

/// A trait for animating a `Sprite` on a terminal interface.
pub(crate) trait Animation<B: Backend>: Sprite {
    /// Plays the animation frame-by-frame with optional mirroring and position offset.
    fn play(
        &mut self,
        terminal: &mut Terminal<B>,
        delta: f32,
        mirror: MirrorDirection,
        offset: Coordinate,
    ) {
        // Total time to show the frame
        let frame_interval = 1.0 / self.frames().len() as f32;

        // Coordinate frame intervals with game frame rate
        *self.timer_mut() += delta;
        if self.timer() >= frame_interval {
            *self.timer_mut() -= frame_interval;
            *self.frame_pos_mut() = (self.frame_pos() + 1) % self.frames().len()
        }

        let frame = &self.frames()[self.frame_pos()];
        let _ = terminal.draw(|f| {
            for p in &frame.pixels {
                // Ignores the mirror direction value since the value must be covered by
                // the frames dimensions
                match mirror {
                    MirrorDirection::FlipVertical(_) => p.render(
                        f,
                        MirrorDirection::FlipVertical(frame.width),
                        offset.clone(),
                    ),
                    MirrorDirection::FlipHorizontal(_) => p.render(
                        f,
                        MirrorDirection::FlipHorizontal(frame.height),
                        offset.clone(),
                    ),
                    MirrorDirection::None => p.render(f, mirror.clone(), offset.clone()),
                }
            }
        });
    }
}
impl<T: Sprite> Animation<CrosstermBackend<Stdout>> for T {}
