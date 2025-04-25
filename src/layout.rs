use std::ops::{Add, AddAssign, Mul};

/// A row index in the terminal grid (Y-axis).
pub(crate) type Row = u16;
/// A column index in the terminal grid (X-axis).
pub(crate) type Column = u16;

/// The maximum height of a visual "pixel" in terminal cells.
///
/// Set to 1 since terminal rows are a single character tall.
pub(crate) const PIXEL_MAX_HEIGHT: u16 = 1;
/// The maximum width of a visual "pixel" in terminal cells.
///
/// Set to 2 to simulate a square aspect ratio, accounting for the
/// typical 2:1 character width-to-height ratio in terminal fonts.
pub(crate) const PIXEL_MAX_WIDTH: u16 = 2;

/// Represents a 2D position on the terminal grid.
///
/// `Coordinate` defines a location using `x` (horizontal) and `y` (vertical)
/// values in character cell units.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Default)]
pub(crate) struct Coordinate {
    /// Horizontal position (columns).
    pub(crate) x: f32,
    /// Vertical position (rows).
    pub(crate) y: f32,
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

/// Represents a direction in the terminal coordinate system.
#[derive(Clone)]
pub(crate) enum Direction {
    /// Movement or alignment along the vertical (Y) axis, measured in rows.
    Vertical(Row),
    /// Movement or alignment along the horizontal (X) axis, measured in columns.
    Horizontal(Column),
}

/// Represents a mirroring transformation across an axis in the terminal coordinate system.
#[derive(Clone)]
pub(crate) enum MirrorDirection {
    /// Flip across the horizontal axis, affecting the vertical (Y) direction.
    FlipHorizontal(Column),
    /// Flip across the vertical axis, affecting the horizontal (X) direction.
    FlipVertical(Row),
    None,
}

/// A `Mirrorable` that can compute mirrored positions along the X or Y axis.
pub(crate) trait Mirrorable {
    /// Mirroring coordinate point vertically/horizontally across axis
    fn mirror(x: Column, width_height: u16) -> u16 {
        width_height - x
    }
}
