//! A module for rendering window-based pixel data and frame manipulation.
//!
//! This module provides the necessary structures and methods to represent
//! and manipulate pixel data (`Pixel`) and frames (`Frame`) for window-based
//! rendering, particularly suitable for retro-style games or window-based UIs.
//!
//! # Key Structures and Traits
//! - **`Renderer` Trait**: Defines the common interface for rendering UI elements.
//! - **`Frame` Struct**: Represents a window-rendered frame consisting of `Pixel` elements.
//! - **`Pixel` Struct**: Represents a single logical pixel in a window context, which may span multiple window cells.
//! - **`Mirrorable` Trait**: Allows `Pixel` to be mirrored during rendering, supporting both vertical and horizontal flips.
//!
//! # Frame Construction
//! - A `Frame` contains a collection of `Pixel` elements and is responsible for determining its own size and layout.
//! - Each `Pixel` contains a set of window coordinates and a color, which can be styled using `ColorScheme`.
//! - Frames can be created with optional durations for animation timing.
//!
//! # Pixel Creation
//! - `Pixel` supports multiple color schemes, including:
//!   - **Standard Color**: Single-color pixel.
//!   - **Check Pattern**: Alternating colors within a specified range (e.g., checkerboard pattern).
//!   - **Stroke**: A pattern where colors are applied in a stroke-like manner, based on direction.
//!
//! # Rendering and Drawing
//! - Pixels can be drawn onto a screen (implementing the `Screen` trait), with support for mirroring and positional offsets.
//! - Mirroring can be applied to create flipped versions of the pixel, either vertically or horizontally.
//!
//! # Example Usage
//! To create a `Frame` with a pixel:
//! ```rust
//! let pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 10, 20);
//! let frame = Frame::new(vec![pixel], None);
//! ```

use crate::prelude::*;
use std::{ops::Range, time::Duration};

/// A container for window-rendered `Pixel`s.
///
/// The `Frame` struct represents a renderable frame that consists of multiple
/// `Pixel` elements.
///
/// NOTE: `f32` in frame creation context should always be postive since
/// we coerce between `f32` and `u16`
#[derive(Clone, Debug)]
pub struct Frame {
    /// A collection of `Pixel` that make up this frame.
    pub(crate) pixels: Vec<Pixel>,
    pub(crate) height: u16,
    pub(crate) width: u16,
    pub(crate) duration: Option<Duration>,
}
impl Frame {
    /// Creates a new `Frame` with the given pixels
    pub(crate) fn new(pixels: Vec<Pixel>, duration: Option<Duration>) -> Self {
        let (width, height) = Frame::get_dimesions(&pixels);
        Self {
            pixels,
            height,
            width,
            duration,
        }
    }
    /// Calculates the maximum width and height based on pixel positions.
    fn get_dimesions(pixels: &Vec<Pixel>) -> (u16, u16) {
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        // Find max H and max W size of frame to allow
        // manipulation for later use
        for p in pixels {
            for rect in &p.pixels {
                width = width.max(rect.1.x as u16);
                height = height.max(rect.1.y as u16);
            }
        }
        (width, height)
    }
    /// Updates the stored width and height of the frame based on its pixel data.
    ///
    /// Should be called whenever modifications to pixel positions are made such as
    /// `Pixel::move_pos`
    pub(crate) fn resize(&mut self) {
        let (width, height) = Frame::get_dimesions(&self.pixels);
        self.height = height;
        self.width = width;
    }
}

/// A single logical pixel in a window-based rendering context.
///
/// Uses an 8-bit color palette for styling. Each `Pixel` is rendered as one or
/// more window cells depending on the `ColorScheme`. Coordinates are specified
/// in window cell units, but a single `Pixel` may span multiple cells.
#[derive(Clone, Debug)]
pub struct Pixel {
    pixels: Vec<(Color, Coordinate)>,
}
impl Pixel {
    pub(crate) fn new(color: ColorScheme, x: u16, y: u16) -> Self {
        let pixels = match color {
            ColorScheme::Standard(color) => vec![(
                color,
                Coordinate {
                    x: x.into(),
                    y: y.into(),
                },
            )],
            ColorScheme::CheckPattern(check_pattern) => {
                let mut pixels = Vec::new();
                for i in Pixel::extract_range(&check_pattern.range) {
                    // Alt colors starting with the first color specified always
                    let color = if i % 2 == 0 {
                        check_pattern.a
                    } else {
                        check_pattern.b
                    };
                    pixels.push((
                        color,
                        Pixel::pattern_to_coordinate(&check_pattern.range, x, y, i),
                    ))
                }
                pixels
            }
            ColorScheme::Stroke(stroke) => {
                let mut pixels = Vec::new();
                for i in Pixel::extract_range(&stroke.range) {
                    pixels.push((
                        stroke.color,
                        Pixel::pattern_to_coordinate(&stroke.range, x, y, i),
                    ));
                }
                pixels
            }
        };
        Self { pixels }
    }
    fn extract_range(dir: &Direction) -> Range<u16> {
        match dir {
            Direction::Vertical(rng) => 0..*rng,
            Direction::Horizontal(rng) => 0..*rng,
        }
    }
    /// The a new coordinate position based on offset from the pixel's base coordinate
    /// based on the given pattern direction.
    fn pattern_to_coordinate(dir: &Direction, x: u16, y: u16, offset: u16) -> Coordinate {
        match dir {
            Direction::Horizontal(_) => Coordinate {
                x: (x + offset).into(),
                y: y.into(),
            },
            Direction::Vertical(_) => Coordinate {
                x: x.into(),
                y: (y + offset).into(),
            },
        }
    }
    pub(crate) fn len(&self) -> usize {
        self.pixels.len()
    }
    /// Returns the Y coordinate of the pixel at the given index, if it exists.
    pub(crate) fn row_pos(&self, index: usize) -> Option<u16> {
        if let Some(rect) = self.pixels.get(index) {
            return Some(rect.1.y as u16);
        }
        None
    }
    /// Returns the X coordinate of the pixel at the given index, if it exists.
    pub(crate) fn column_pos(&self, index: usize) -> Option<u16> {
        if let Some(rect) = self.pixels.get(index) {
            return Some(rect.1.x as u16);
        }
        None
    }
    /// Changes the color of the pixel at the specified index.
    ///
    /// Returns the previous color if change was successful
    pub(crate) fn change_color(&mut self, index: usize, color: Color) -> Option<Color> {
        if let Some(p) = self.pixels.get_mut(index) {
            let old_color = p.0;
            p.0 = color;
            return Some(old_color);
        }
        None
    }
    /// Changes the position of the pixel at the specified index.
    ///
    /// Returns the previous position if change was successful
    pub(crate) fn move_pos(&mut self, index: usize, dir: Direction) -> Option<u16> {
        if let Some(p) = self.pixels.get_mut(index) {
            let rect = &mut p.1;
            match dir {
                Direction::Vertical(new_pos) => {
                    let old_pos = rect.y;
                    rect.y = new_pos.into();
                    return Some(old_pos as u16);
                }
                Direction::Horizontal(new_pos) => {
                    let old_pos = rect.x;
                    rect.x = new_pos.into();
                    return Some(old_pos as u16);
                }
            };
        }
        None
    }
    /// Mirroring coordinate point vertically/horizontally across axis
    fn mirror(x: u16, width_height: u16) -> u16 {
        width_height - x
    }
    /// Draws this `Pixel` to the given frame buffer by drawing all the avaliable pixels
    /// with optional mirroring and position offset.
    pub(crate) fn draw<S: Screen>(
        &self,
        screen: &mut S,
        mirror: MirrorDirectionValue,
        offset: Coordinate,
    ) {
        let screen_width = screen.width();
        let screen_height = screen.height();
        let screen_buffer = screen.frame_buffer();

        for pixel in &self.pixels {
            let (color, coordinate) = pixel;
            // Applied mirror transformation if applicable
            let area = match mirror {
                MirrorDirectionValue::FlipVertical(max_width) => Coordinate {
                    x: Pixel::mirror(coordinate.x as u16, max_width).into(),
                    y: coordinate.y,
                },
                MirrorDirectionValue::FlipHorizontal(max_height) => Coordinate {
                    x: coordinate.x,
                    y: Pixel::mirror(coordinate.y as u16, max_height).into(),
                },
                MirrorDirectionValue::None => *coordinate,
            };

            // Apply directional offset of movements
            let area = Coordinate {
                x: offset.x + area.x,
                y: offset.y + area.y,
            };

            // Stays in the screen bounds
            let x = area.x.round() as i32;
            let y = area.y.round() as i32;
            if x < 0 || y < 0 || x as u32 >= screen_width || y as u32 >= screen_height {
                continue;
            }

            // Row-major layout formula is used for RGB and RGBA support
            // since we only do power of two resolutions
            let idx = ((y as u32 * screen_width) + (x as u32)) as usize * 4;
            match color {
                Color::RGB(r, g, b) => {
                    screen_buffer[idx] = *r; // Red
                    screen_buffer[idx + 1] = *g; // Green
                    screen_buffer[idx + 2] = *b; // Blue
                    screen_buffer[idx + 3] = 255; // Alpha
                }
                Color::RGBA(r, g, b, a) => {
                    screen_buffer[idx] = *r; // Red
                    screen_buffer[idx + 1] = *g; // Green
                    screen_buffer[idx + 2] = *b; // Blue
                    screen_buffer[idx + 3] = *a; // Alpha
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{
        mock::MockScreen,
        palette::{CheckPattern, Stroke},
    };

    #[test]
    fn test_pixel_creation_with_standard_color() {
        let pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 10, 20);

        assert_eq!(pixel.len(), 1);
        assert_eq!(pixel.row_pos(0), Some(20));
        assert_eq!(pixel.column_pos(0), Some(10));
    }

    #[test]
    fn test_pixel_creation_with_check_pattern() {
        let check_pattern = ColorScheme::CheckPattern(CheckPattern {
            a: Color::RGB(255, 0, 0),
            b: Color::RGB(0, 255, 0),
            range: Direction::Horizontal(2),
        });

        let pixel = Pixel::new(check_pattern, 5, 5);

        assert_eq!(pixel.len(), 2);
        assert_eq!(pixel.row_pos(0), Some(5));
        assert_eq!(pixel.column_pos(0), Some(5));
        assert_eq!(pixel.row_pos(1), Some(5));
        assert_eq!(pixel.column_pos(1), Some(6));
    }

    #[test]
    fn test_pixel_creation_with_stroke() {
        let stroke = ColorScheme::Stroke(Stroke {
            color: Color::RGB(0, 0, 255),
            range: Direction::Vertical(2),
        });

        let pixel = Pixel::new(stroke, 5, 5);

        assert_eq!(pixel.len(), 2);
        assert_eq!(pixel.row_pos(0), Some(5));
        assert_eq!(pixel.column_pos(0), Some(5));
        assert_eq!(pixel.row_pos(1), Some(6));
        assert_eq!(pixel.column_pos(1), Some(5));
    }

    #[test]
    fn test_move_pos() {
        let mut pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 10, 20);
        assert_eq!(pixel.move_pos(0, Direction::Horizontal(15)), Some(10));
        assert_eq!(pixel.move_pos(0, Direction::Vertical(25)), Some(20));
    }

    #[test]
    fn test_change_color() {
        let mut pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 10, 20);
        assert_eq!(
            pixel.change_color(0, Color::RGB(0, 255, 0)),
            Some(Color::RGB(255, 0, 0))
        );
        assert_eq!(pixel.pixels[0].0, Color::RGB(0, 255, 0));
        assert_eq!(
            pixel.change_color(0, Color::RGB(0, 0, 255)),
            Some(Color::RGB(0, 255, 0))
        );
        assert_eq!(pixel.pixels[0].0, Color::RGB(0, 0, 255));
    }

    #[test]
    fn test_draw_rgb() {
        let screen = Arc::new(Mutex::new(MockScreen::new(50, 50)));
        let pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 5, 5);

        // Simulate drawing the pixel onto the screen
        pixel.draw(
            &mut *screen.lock().unwrap(),
            MirrorDirectionValue::None,
            Coordinate { x: 0.0, y: 0.0 },
        );

        // Check the pixel data in the screen buffer
        // RGBA means 4 bytes per pixel so calculation follows suit
        let screen = Arc::into_inner(screen).unwrap();
        let screen = screen.into_inner().unwrap();
        let idx = (5 * 50 + 5) as usize * 4; // Pixel at (5, 5)
        assert_eq!(screen.buffer[idx], 255); // Red channel
        assert_eq!(screen.buffer[idx + 1], 0); // Green channel
        assert_eq!(screen.buffer[idx + 2], 0); // Blue channel
        assert_eq!(screen.buffer[idx + 3], 255); // Alpha channel
    }

    #[test]
    fn test_draw_rgba() {
        let screen = Arc::new(Mutex::new(MockScreen::new(50, 50)));
        let pixel = Pixel::new(ColorScheme::Standard(Color::RGBA(255, 0, 0, 180)), 5, 5);

        // Simulate drawing the pixel onto the screen
        pixel.draw(
            &mut *screen.lock().unwrap(),
            MirrorDirectionValue::None,
            Coordinate { x: 0.0, y: 0.0 },
        );

        // Check the pixel data in the screen buffer
        // RGBA means 4 bytes per pixel so calculation follows suit
        let screen = Arc::into_inner(screen).unwrap();
        let screen = screen.into_inner().unwrap();
        let idx = (5 * 50 + 5) as usize * 4; // Pixel at (5, 5)
        assert_eq!(screen.buffer[idx], 255); // Red channel
        assert_eq!(screen.buffer[idx + 1], 0); // Green channel
        assert_eq!(screen.buffer[idx + 2], 0); // Blue channel
        assert_eq!(screen.buffer[idx + 3], 180); // Alpha channel
    }

    #[test]
    fn test_mirror_flip_vertical() {
        let screen = Arc::new(Mutex::new(MockScreen::new(50, 50)));
        let pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 5, 3);

        // Flip vertically at line 10
        pixel.draw(
            &mut *screen.lock().unwrap(),
            MirrorDirectionValue::FlipVertical(5),
            Coordinate { x: 0.0, y: 0.0 },
        );

        // Check the pixel's mirrored position
        let idx_original = (3 * 50 + 5) as usize * 4; // Pixel at (5, 3)
        let idx_mirrored = (3 * 50 + 0) as usize * 4; // Pixel at (0, 3)
        let screen = Arc::into_inner(screen).unwrap();
        let screen = screen.into_inner().unwrap();
        assert_eq!(screen.buffer[idx_original], 0); // Should not be original pixel
        assert_eq!(screen.buffer[idx_mirrored], 255); // Should be mirrored pixel
    }

    #[test]
    fn test_mirror_flip_horizontal() {
        let screen = Arc::new(Mutex::new(MockScreen::new(50, 50)));
        let pixel = Pixel::new(ColorScheme::Standard(Color::RGB(255, 0, 0)), 5, 3);

        // Flip vertically at line 10
        pixel.draw(
            &mut *screen.lock().unwrap(),
            MirrorDirectionValue::FlipHorizontal(3),
            Coordinate { x: 0.0, y: 0.0 },
        );

        // Check the pixel's mirrored position
        let idx_original = (3 * 50 + 5) as usize * 4; // Pixel at (5, 3)
        let idx_mirrored = (0 * 50 + 5) as usize * 4; // Pixel at (5, 0)
        let screen = Arc::into_inner(screen).unwrap();
        let screen = screen.into_inner().unwrap();
        assert_eq!(screen.buffer[idx_original], 0); // Should not be original pixel
        assert_eq!(screen.buffer[idx_mirrored], 255); // Should be mirrored pixel
    }
}
