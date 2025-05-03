//! A module for handling 2D coordinates, movement directions, and mirroring transformations.
//!
//! This module includes definitions for a 2D coordinate system (`Coordinate`), movement directions
//! (`Direction`), and mirroring transformations (`MirrorDirection`). It provides useful methods
//! for working with pixel-based grids and transformations commonly used in rendering and manipulation
//! of terminal UI elements.
//!
//! # Key Structures:
//! - **Coordinate**: Represents a 2D position on a grid (with `x` for horizontal and `y` for vertical).
//! - **Direction**: Defines movement along either the vertical or horizontal axis, measured in pixels.
//! - **MirrorDirection**: Represents the transformation to mirror a coordinate either horizontally or vertically.
//!
//! # Key Features:
//! - **Coordinate Operations**: Supports addition, addition assignment, and scalar multiplication for easy manipulation of coordinates.
//! - **Direction Enum**: Provides an easy way to define movement or alignment along vertical or horizontal axes.
//! - **Mirroring Transformation**: Offers the ability to mirror coordinates across an axis, useful for flipped rendering or effects.
//!
//! # Example Usage:
//! ```rust
//! // Creating a coordinate
//! let point = Coordinate { x: 10.0, y: 5.0 };
//!
//! // Moving the point along both axes
//! let movement = Coordinate { x: 2.0, y: -1.0 };
//! let new_point = point + movement;
//! assert_eq!(new_point, Coordinate { x: 12.0, y: 4.0 });
//!
//! // Mirroring a coordinate along the X axis
//! let mirrored = Coordinate::mirror(10, 20); // Assuming width is 20
//! assert_eq!(mirrored, 10); // Mirrored position (20 - 10)
//! ```
use std::ops::{Add, AddAssign, Mul};

/// Represents a 2D position on the pixels grid.
///
/// `Coordinate` defines a location using `x` (horizontal) and `y` (vertical)
/// values in character cell units.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Default)]
pub struct Coordinate {
    /// Horizontal position (columns).
    pub x: f32,
    /// Vertical position (rows).
    pub y: f32,
}
impl From<(f32, f32)> for Coordinate {
    fn from(value: (f32, f32)) -> Self {
        Coordinate {
            x: value.0,
            y: value.1,
        }
    }
}
impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Mul<f32> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Represents a direction in the pixels coordinate system.
#[derive(Clone, Copy)]
pub enum Direction {
    /// Movement or alignment along the vertical (Y) axis, measured in pixels.
    Vertical(u16),
    /// Movement or alignment along the horizontal (X) axis, measured in pixels.
    Horizontal(u16),
}

/// Represents a mirroring transformation across an axis in the pixels coordinate system.
#[derive(Clone)]
pub enum MirrorDirectionValue {
    /// Flip across the horizontal axis, affecting the vertical (Y) direction.
    FlipHorizontal(u16),
    /// Flip across the vertical axis, affecting the horizontal (X) direction.
    FlipVertical(u16),
    None,
}
/// Represents a mirroring transformation across an axis in the pixels coordinate system.
pub enum MirrorDirection {
    /// Flip across the horizontal axis
    FlipHorizontal,
    /// Flip across the vertical axis
    FlipVertical,
    None,
}
