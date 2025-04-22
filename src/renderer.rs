use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};
use std::{ops::Range, time::Duration};

use crate::{
    layout::{
        Column, Coordinate, Direction, MirrorDirection, Row, PIXEL_MAX_HEIGHT, PIXEL_MAX_WIDTH,
    },
    palette::ColorScheme,
    Mirrorable,
};

/// The `Renderer` trait representing a renderable UI element.
trait Renderer {
    /// Renders the UI element onto the given `ratatui::Frame`.
    fn render(&self, f: &mut ratatui::Frame);
}

/// A container for terminal-rendered `Pixel`s.
///
/// The `Frame` struct represents a renderable frame that consists of multiple
/// `Pixel` elements.
#[derive(Clone, Debug)]
pub(crate) struct Frame {
    /// A collection of `Pixel` that make up this frame.
    pub(crate) pixels: Vec<Pixel>,
    pub(crate) height: u16,
    pub(crate) width: u16,
}
impl Frame {
    /// Creates a new `Frame` with the given pixels
    pub(crate) fn new(pixels: Vec<Pixel>) -> Self {
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        // Find max H and max W size of frame to allow
        // manipulation for later use
        for p in &pixels {
            for rect in &p.pixels {
                width = width.max(rect.1.x);
                height = height.max(rect.1.y);
            }
        }

        Self {
            pixels,
            height,
            width,
        }
    }
}

/// A single logical pixel in a terminal-based rendering context.
///
/// Uses an 8-bit color palette for styling. Each `Pixel` is rendered as one or
/// more terminal cells depending on the `ColorScheme`. Coordinates are specified
/// in terminal cell units, but a single `Pixel` may span multiple cells.
#[derive(Clone, Debug)]
pub(crate) struct Pixel {
    pixels: Vec<(Color, Rect)>,
}
impl Pixel {
    pub(crate) fn new(color: ColorScheme, x: Column, y: Row) -> Self {
        Self {
            pixels: InternalPixel::new(color, x, y).to_cells(),
        }
    }
    pub(crate) fn len(&self) -> usize {
        self.pixels.len()
    }
    /// Returns the Y coordinate (`Row`) of the pixel at the given index, if it exists.
    pub(crate) fn row_pos(&self, index: usize) -> Option<Row> {
        if let Some(rect) = self.pixels.get(index) {
            return Some(rect.1.y);
        }
        None
    }
    /// Returns the X coordinate (`Column`) of the pixel at the given index, if it exists.
    pub(crate) fn column_pos(&self, index: usize) -> Option<Column> {
        if let Some(rect) = self.pixels.get(index) {
            return Some(rect.1.x);
        }
        None
    }
    /// Changes the color of the pixel at the specified index.
    ///
    /// Returns the previous color if change was successful
    pub(crate) fn change_color(&mut self, index: usize, color: Color) -> Option<Color> {
        if let Some(p) = self.pixels.get_mut(index) {
            p.0 = color;
            return Some(p.0);
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
                    rect.y = new_pos;
                    return Some(rect.y);
                }
                Direction::Horizontal(new_pos) => {
                    rect.x = new_pos;
                    return Some(rect.x);
                }
            };
        }
        None
    }
    /// Renders this `Pixel` to the given terminal by drawing all the avaliable pixels
    /// with optional mirroring and position offset.
    pub(crate) fn render(
        &self,
        f: &mut ratatui::Frame,
        mirror: MirrorDirection,
        offset: Coordinate,
    ) {
        for i in &self.pixels {
            let block = Block::new().style(Style::default().bg(i.0));

            // Applied mirror transformation if applicable
            let area = match mirror {
                MirrorDirection::FlipVertical(max_width) => Rect::new(
                    Pixel::mirror(i.1.x, max_width),
                    i.1.y,
                    PIXEL_MAX_WIDTH,
                    PIXEL_MAX_HEIGHT,
                ),
                MirrorDirection::FlipHorizontal(max_height) => Rect::new(
                    i.1.x,
                    Pixel::mirror(i.1.y, max_height),
                    PIXEL_MAX_WIDTH,
                    PIXEL_MAX_HEIGHT,
                ),
                MirrorDirection::None => i.1,
            };

            // Apply directional offset of movements
            // SILENT FAILURE IF OVERFLOW
            let area = Rect {
                x: (offset.x + area.x as f32) as u16,
                y: (offset.y + area.y as f32) as u16,
                width: PIXEL_MAX_WIDTH,
                height: PIXEL_MAX_HEIGHT,
            };

            f.render_widget(block, area);
        }
    }
}
impl Mirrorable for Pixel {}

pub(crate) struct InternalPixel {
    /// Background color or pattern using an 8-bit palette.
    color: ColorScheme,
    /// Top-left position of the pixel on the terminal grid.
    coord: Coordinate,
}
impl InternalPixel {
    /// Creates a new `InternalPixel` with the given color scheme and grid position.
    pub(crate) fn new(color: ColorScheme, x: Column, y: Row) -> Self {
        Self {
            color,
            coord: Coordinate::from((x as f32, y as f32)),
        }
    }
    /// The `Rect` (terminal cell) position offset from the pixel's base coordinate
    /// based on the given pattern direction.
    ///
    /// For vertical patterns, this adds a horizontal offset (and compensates for 2x1 terminal
    /// pixels by adjusting the X-axis). For horizontal patterns, it moves down the Y-axis.
    fn pattern_to_rect(&self, dir: &Direction, offset: u16) -> Rect {
        match dir {
            // SILENT FAILURE IF OVERFLOW
            Direction::Horizontal(_) => Rect::new(
                self.coord.x as u16 + (offset * 2),
                self.coord.y as u16,
                PIXEL_MAX_WIDTH,
                PIXEL_MAX_HEIGHT,
            ),
            // SILENT FAILURE IF OVERFLOW
            Direction::Vertical(_) => Rect::new(
                self.coord.x as u16,
                self.coord.y as u16 + offset,
                PIXEL_MAX_WIDTH,
                PIXEL_MAX_HEIGHT,
            ),
        }
    }
    fn extract_range(dir: &Direction) -> Range<u16> {
        match dir {
            Direction::Vertical(rng) => 0..*rng,
            Direction::Horizontal(rng) => 0..*rng,
        }
    }
    /// Converts this `InternalPixel` into a list of terminal cells (`Color`, `Rect` pairs)
    fn to_cells(&self) -> Vec<(Color, Rect)> {
        match &self.color {
            ColorScheme::Standard(color) => vec![(
                *color,
                // SILENT FAILURE IF OVERFLOW
                Rect::new(
                    self.coord.x as u16,
                    self.coord.y as u16,
                    PIXEL_MAX_WIDTH,
                    PIXEL_MAX_HEIGHT,
                ),
            )],
            ColorScheme::CheckPattern(check_pattern) => {
                let mut pixels = Vec::new();
                for i in InternalPixel::extract_range(&check_pattern.range) {
                    // Alt colors starting with the first color specified always
                    let color = if i % 2 == 0 {
                        check_pattern.a
                    } else {
                        check_pattern.b
                    };
                    pixels.push((color, self.pattern_to_rect(&check_pattern.range, i)))
                }
                pixels
            }
            ColorScheme::Stroke(stroke) => {
                let mut pixels = Vec::new();
                for i in InternalPixel::extract_range(&stroke.range) {
                    pixels.push((stroke.color, self.pattern_to_rect(&stroke.range, i)));
                }
                pixels
            }
        }
    }
}
