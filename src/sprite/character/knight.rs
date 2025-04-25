use ratatui::{prelude::CrosstermBackend, style::Color};
use std::{io::Stdout, time::Duration};

use super::character::Character;
use crate::{
    animator::Animation,
    layout::{Direction, Mirrorable},
    palette::{CheckPattern, ColorScheme, Stroke, DARK_BROWN, LIGHT_BROWN, LIGHT_GRAY, LIGHT_TAN},
    renderer::{Frame, Pixel},
    sprite::sprite::Sprite,
};

macro_rules! impl_character_default_for {
    ($backend:ty) => {
        impl Character<$backend> for Knight {
            fn idle(&mut self) -> &mut dyn Animation<$backend> {
                &mut self.idle
            }
            fn walk(&mut self) -> &mut dyn Animation<$backend> {
                &mut self.walk
            }
        }
    };
}

/// The default main character with predefined animations.
pub(crate) struct Knight {
    idle: Idle,
    walk: Walking,
}
impl Mirrorable for Knight {}
impl Knight {
    pub(crate) fn new() -> Self {
        Self {
            idle: Idle::new(),
            walk: Walking::new(),
        }
    }
}
impl_character_default_for!(CrosstermBackend<Stdout>);

/// Idle animation builder
#[derive(Default)]
pub(crate) struct Idle {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl Idle {
    pub(crate) fn new() -> Self {
        let first = Frame::new(vec![
            // Helmet accessory
            Pixel::new(ColorScheme::Standard(Color::Red), 4, 0),
            // Helmet
            Pixel::new(
                ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                2,
                1,
            ),
            // Eyes
            Pixel::new(
                ColorScheme::CheckPattern(CheckPattern::new(
                    Color::Black,
                    LIGHT_GRAY,
                    Direction::Horizontal(3),
                )),
                2,
                2,
            ),
            // Body
            Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 0, 3),
            Pixel::new(
                ColorScheme::CheckPattern(CheckPattern::new(
                    LIGHT_TAN,
                    Color::Black,
                    Direction::Horizontal(3),
                )),
                2,
                3,
            ),
            Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 8, 3),
            // Belt
            Pixel::new(ColorScheme::Standard(LIGHT_TAN), 0, 4),
            Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 2, 4),
            Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 4, 4),
            Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 6, 4),
            Pixel::new(ColorScheme::Standard(LIGHT_TAN), 8, 4),
            // Feet
            Pixel::new(ColorScheme::Standard(LIGHT_TAN), 2, 5),
            Pixel::new(ColorScheme::Standard(LIGHT_TAN), 6, 5),
        ]);

        // Makes helmet feather start blinking
        let mut nth2 = first.clone();
        nth2.pixels[0] = Pixel::new(ColorScheme::Standard(DARK_BROWN), 4, 0);

        // Make hero breathe and feather about finish blinking
        let mut last = nth2.clone();
        last.pixels[0] = Pixel::new(ColorScheme::Standard(Color::LightRed), 4, 0);
        let body = &mut last.pixels[4];
        body.change_color(1, LIGHT_TAN);

        Self {
            frames: vec![first, nth2, last],
            ..Default::default()
        }
    }
}
impl Sprite for Idle {
    fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }
    fn frame_pos(&self) -> usize {
        self.frame_pos
    }
    fn timer(&self) -> f32 {
        self.timer
    }
    fn frame_pos_mut(&mut self) -> &mut usize {
        &mut self.frame_pos
    }
    fn timer_mut(&mut self) -> &mut f32 {
        &mut self.timer
    }
}

/// Walking animation builder
#[derive(Default)]
pub(crate) struct Walking {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl Walking {
    pub(crate) fn new() -> Self {
        let first = Frame::new(vec![
            // Helmet accessory
            Pixel::new(ColorScheme::Standard(Color::Red), 2, 0),
            // Helmet
            Pixel::new(
                ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                0,
                1,
            ),
            // Eyes
            Pixel::new(
                ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(2))),
                0,
                2,
            ),
            Pixel::new(ColorScheme::Standard(Color::Black), 4, 2),
            // Body
            Pixel::new(
                ColorScheme::CheckPattern(CheckPattern::new(
                    LIGHT_TAN,
                    LIGHT_BROWN,
                    Direction::Horizontal(3),
                )),
                0,
                3,
            ),
            // Belt
            Pixel::new(
                ColorScheme::CheckPattern(CheckPattern::new(
                    LIGHT_GRAY,
                    LIGHT_TAN,
                    Direction::Horizontal(3),
                )),
                0,
                4,
            ),
            // Feet
            Pixel::new(ColorScheme::Standard(LIGHT_TAN), 2, 5),
        ]);

        // Extend body
        let mut nth2 = first.clone();
        for pixel in &mut nth2.pixels {
            for idx in 0..pixel.len() {
                let new_y = pixel.row_pos(idx).unwrap().saturating_sub(1);
                pixel.move_pos(idx, Direction::Vertical(new_y));
            }
        }
        nth2.pixels
            .push(Pixel::new(ColorScheme::Standard(LIGHT_TAN), 2, 5));

        // Start step
        let mut nth3 = nth2.clone();
        let pixel = &mut nth3.pixels[7];
        let pixel_col = pixel.column_pos(0).unwrap();
        let new_x = pixel_col.saturating_sub(2);
        pixel.move_pos(0, Direction::Horizontal(new_x));

        // Extend step
        let mut nth4 = nth3.clone();
        nth4.pixels
            .push(Pixel::new(ColorScheme::Standard(LIGHT_TAN), 4, 4));

        // Squash
        let mut nth5 = first.clone();
        let pixel = &mut nth5.pixels[6];
        let pixel_col = pixel.column_pos(0).unwrap();
        let new_x = pixel_col.saturating_sub(2);
        pixel.move_pos(0, Direction::Horizontal(new_x));

        // Both feet down
        let mut nth6 = nth5.clone();
        nth6.pixels
            .push(Pixel::new(ColorScheme::Standard(LIGHT_TAN), 4, 5));

        // Cycle feet
        let mut nth7 = nth6.clone();
        nth7.pixels[6].change_color(0, Color::Reset);

        // Feet rising
        let mut nth8 = nth2.clone();
        let pixel = &mut nth8.pixels[7];
        let pixel_col = pixel.column_pos(0).unwrap();
        let new_x = pixel_col.saturating_add(2);
        pixel.move_pos(0, Direction::Horizontal(new_x));
        nth8.pixels
            .push(Pixel::new(ColorScheme::Standard(LIGHT_TAN), 0, 4));

        // Cycle feet rising
        let mut nth9 = nth2.clone();
        let pixel = &mut nth9.pixels[7];
        let pixel_col = pixel.column_pos(0).unwrap();
        let new_x = pixel_col.saturating_add(2);
        pixel.move_pos(0, Direction::Horizontal(new_x));

        // Extend body
        let nth10 = nth2.clone();

        // Squeeze body back to initial start
        let nth11 = first.clone();
        Self {
            frames: vec![
                first, nth2, nth3, nth4, nth5, nth6, nth7, nth8, nth9, nth10, nth11,
            ],
            ..Default::default()
        }
    }
}
impl Sprite for Walking {
    fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }
    fn frame_pos(&self) -> usize {
        self.frame_pos
    }
    fn timer(&self) -> f32 {
        self.timer
    }
    fn frame_pos_mut(&mut self) -> &mut usize {
        &mut self.frame_pos
    }
    fn timer_mut(&mut self) -> &mut f32 {
        &mut self.timer
    }
}
