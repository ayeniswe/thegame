use ratatui::prelude::Backend;

use crate::animator::Animation;

/// The `Character` trait is for state actions that a character can perform
pub(crate) trait Character<B: Backend> {
    /// The character's idle animation.
    fn idle(&mut self) -> &mut dyn Animation<B>;
    /// The character's walk animation.
    fn walk(&mut self) -> &mut dyn Animation<B>;
}
