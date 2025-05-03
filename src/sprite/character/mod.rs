/// Defines a set of animated behaviors for a character entity.
///
/// The `Character` trait abstracts the set of possible stateful animations that
/// a character can perform. Each behavior corresponds to a distinct animation
/// sequence (such as walking, standing idle, etc.), and is returned as a mutable
/// reference to an object implementing the [`Animation`] trait.
///
/// This trait is designed to be flexible for both 2D terminal-based rendering and
/// more complex systems where behavior-driven animation selection is necessary.
///
/// ## Usage
/// Typically, a `Character` is owned by a game state or actor controller and is
/// queried based on user input or AI decisions to drive what animation should be
/// played. For example:
///
/// ```ignore
/// let walk = character.side_walk();
/// walk.play(...);
/// ```
///
/// ## Notes
/// - The animations are returned as trait objects (`&mut dyn Animation`) to allow
///   dynamic dispatch and polymorphic behavior.
/// - Implementors are expected to manage and reuse animation instances internally
///   to avoid unnecessary allocations.
///
/// ## Example Implementations
/// See `Knight` or other concrete structs that embed `Sprite`-based animations.
pub(crate) mod character;
pub(crate) mod knight;
