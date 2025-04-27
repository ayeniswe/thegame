//! Represents an animated visual entity composed of multiple `Frame`s.
//!
//! A `Sprite` is the foundational abstraction for time-based visual changes.
//! Each sprite maintains an ordered collection of `Frame`s, where each frame
//! defines a visual state (e.g., a pose, a tile, or a frame of animation).
//!
//! This trait provides the low-level data and accessors necessary for animation
//! playback, but does not itself handle timing or rendering. For those features,
//! see the [`Animation`](crate::animation::Animation) trait.
//!
//! ## Responsibilities
//! - Stores a sequence of animation frames (`Frame`s)
//! - Tracks the currently visible frame
//! - Tracks animation time (`timer`)
//!
//! ## Usage
//! Most sprite implementations expose behaviors (e.g. `idle`, `run`, etc.)
//! that return a concrete type implementing this trait, allowing those behaviors
//! to be animated or drawn using the [`Animation::play`] method.
pub(crate) mod character;
pub(crate) mod sprite;
