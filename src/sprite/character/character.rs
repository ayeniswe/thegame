use crate::prelude::*;

/// The `Character` trait is for state actions that a character can perform
pub trait Character<S: Screen>: Send + 'static {
    /// The character's idle animation.
    fn idle(&mut self) -> &mut dyn Animation<S>;
    /// The character's side walk animation.
    fn side_walk(&mut self) -> &mut dyn Animation<S>;
    /// The character's front walk Animation<S>.
    fn front_walk(&mut self) -> &mut dyn Animation<S>;
    /// The character's back walk animation.
    fn back_walk(&mut self) -> &mut dyn Animation<S>;
}
