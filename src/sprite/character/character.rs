use ratatui::prelude::Backend;

use crate::animator::Animation;

/// The `Character` trait is for state actions that a character can perform
pub(crate) trait Character<B: Backend> {
    /// The character's idle animation.
    fn idle(&mut self) -> &mut dyn Animation<B>;
    /// The character's side walk animation.
    fn side_walk(&mut self) -> &mut dyn Animation<B>;
    /// The character's front walk animation.
    fn front_walk(&mut self) -> &mut dyn Animation<B>;
    /// The character's back walk animation.
    fn back_walk(&mut self) -> &mut dyn Animation<B>;
}
