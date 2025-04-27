use ratatui::style::Color;

use crate::layout::Direction;

pub(crate) const DARK_BROWN: Color = Color::Indexed(130);
pub(crate) const LIGHT_BROWN: Color = Color::Indexed(137);
pub(crate) const MIDNIGHT: Color = Color::Indexed(232);
pub(crate) const LIGHT_GRAY: Color = Color::Indexed(248);

/// Defines the color styling for a `Pixel`.
#[derive(Clone)]
pub(crate) enum ColorScheme {
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
#[derive(Clone)]
pub(crate) struct CheckPattern {
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
#[derive(Clone)]
pub(crate) struct Stroke {
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
