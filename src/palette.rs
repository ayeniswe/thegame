//! A module for defining and managing pixel colors and patterns.
//!
//! This module provides structures for defining different color schemes
//! and patterns that can be applied to pixels in a window-based rendering context.
//! It includes support for solid colors, checkered patterns, and stroked lines.
//!
//! # Key Structures
//! - **`Color` Enum**: Represents a color, either as an RGB value (three color channels)
//!   or an RGBA value (with transparency).
//! - **`ColorScheme` Enum**: Defines how the color is applied to a pixel, including:
//!   - `Standard`: A single, uniform color.
//!   - `CheckPattern`: A checkered pattern with alternating colors.
//!   - `Stroke`: A solid-colored line or stroke rendered in a specific direction.
//! - **`CheckPattern` Struct**: Defines a checkered pattern that alternates between two colors,
//!   either horizontally or vertically, over a specified range.
//! - **`Stroke` Struct**: Defines a solid-colored stroke that is rendered in a specific direction
//!   (either vertical or horizontal) for a given length.

//! # Color Definitions
//! Several common colors are predefined for convenience:
//! - `LIGHT_BROWN`, `MIDNIGHT`, `LIGHT_GRAY`, `DARK_BROWN`, `RED`, `BLACK`, and `TRANSPARENT`.
//! These colors can be used as `Color` values in various `ColorScheme` options.

//! # Example Usage
//! To create a pixel with a checkered pattern, use the `CheckPattern` and `ColorScheme::CheckPattern`:
//! ```rust
//! let checkered_pattern = CheckPattern::new(Color::RGB(255, 0, 0), Color::RGB(0, 0, 255), Direction::Horizontal(4));
//! let pixel = ColorScheme::CheckPattern(checkered_pattern);
//! ```
//! To create a pixel with a stroke, use the `Stroke` and `ColorScheme::Stroke`:
//! ```rust
//! let stroke = Stroke::new(Color::RGB(0, 255, 0), Direction::Vertical(5));
//! let pixel = ColorScheme::Stroke(stroke);
//! ```

use crate::prelude::*;

pub const LIGHT_BROWN: Color = Color::RGB(205, 133, 63);
pub const MIDNIGHT: Color = Color::RGB(8, 8, 8);
pub const LIGHT_GRAY: Color = Color::RGB(188, 188, 188);
pub const DARK_BROWN: Color = Color::RGB(139, 69, 19);
pub const RED: Color = Color::RGB(255, 0, 0);
pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0);

/// Defines the coloring of a pixel.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}
/// Defines the color styling for a `Pixel`.
#[derive(Clone, Copy)]
pub enum ColorScheme {
    /// A single uniform color.
    Standard(Color),
    /// A horizontally alternating color pattern.
    ///
    /// This simulates a checkered visual effect by alternating between
    /// two colors
    CheckPattern(CheckPattern),
    /// A solid stroke rendered in a specified direction and length.
    ///
    /// Used for creating vertical or horizontal lines.
    Stroke(Stroke),
}

/// A checkered pattern composed of two alternating colors.
///
/// The pattern repeats vertically or horizontally for a specified `range`, to create
/// a visual effect of alternating blocks.
#[derive(Clone, Copy)]
pub struct CheckPattern {
    /// The first color used in the alternating pattern.
    pub(crate) a: Color,
    /// The second color used in the alternating pattern.
    pub(crate) b: Color,
    /// The number of alternating segments to render.
    pub(crate) range: Direction,
}
impl CheckPattern {
    /// Creates a new `CheckPattern` with two alternating colors over the given range.
    pub fn new(a: Color, b: Color, range: Direction) -> Self {
        Self { a, b, range }
    }
}

/// A solid-colored stroke rendered across a directional range.
#[derive(Clone, Copy)]
pub struct Stroke {
    /// The color used for the stroke.
    pub(crate) color: Color,
    /// The direction and length of the stroke.
    pub(crate) range: Direction,
}
impl Stroke {
    /// Creates a new `Stroke` with the specified color and direction.
    pub fn new(color: Color, range: Direction) -> Self {
        Self { color, range }
    }
}
