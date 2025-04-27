use super::character::Character;

use crate::{
    palette::{
        CheckPattern, ColorScheme, Stroke, BLACK, DARK_BROWN, LIGHT_BROWN, LIGHT_GRAY, MIDNIGHT,
        RED, TRANSPARENT,
    },
    renderer::{Frame, Pixel},
};
use crate::prelude::*;

/// The default main character with predefined animations.
pub struct Knight {
    idle: Idle,
    side_walk: SideWalk,
    front_walk: FrontWalk,
    back_walk: BackWalk,
}
impl Knight {
    pub fn new() -> Self {
        Self {
            idle: Idle::new(),
            side_walk: SideWalk::new(),
            front_walk: FrontWalk::new(),
            back_walk: BackWalk::new(),
        }
    }
}
impl Character<GameWindowScreen> for Knight {
    fn idle(&mut self) -> &mut dyn Animation<GameWindowScreen> {
        &mut self.idle
    }
    fn side_walk(&mut self) -> &mut dyn Animation<GameWindowScreen> {
        &mut self.side_walk
    }
    fn front_walk(&mut self) -> &mut dyn Animation<GameWindowScreen> {
        &mut self.front_walk
    }
    fn back_walk(&mut self) -> &mut dyn Animation<GameWindowScreen> {
        &mut self.back_walk
    }
}

/// Idle animation builder
#[derive(Default)]
pub(crate) struct Idle {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl Idle {
    pub(crate) fn new() -> Self {
        let first = Frame::new(
            vec![
                // Helmet accessory
                Pixel::new(ColorScheme::Standard(RED), 2, 1),
                // Helmet
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    1,
                    2,
                ),
                // Eyes
                Pixel::new(
                    ColorScheme::CheckPattern(CheckPattern::new(
                        BLACK,
                        LIGHT_GRAY,
                        Direction::Horizontal(3),
                    )),
                    1,
                    3,
                ),
                // Body
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 0, 4),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    1,
                    4,
                ),
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 4, 4),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 0, 5),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    1,
                    5,
                ),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 4, 5),
                // Belt
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 1, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 2, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 3, 6),
                // Feet
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 1, 7),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 3, 7),
            ],
            None,
        );

        // Start arm stretch rotation
        let mut nth2 = first.clone();
        for pixel in &mut nth2.pixels.iter_mut().enumerate() {
            let (i, pixel) = pixel;
            if matches!(i, 6 | 12) {
                continue;
            }
            for j in 0..pixel.len() {
                let new_x = if i == 8 {
                    pixel.column_pos(j).unwrap().saturating_add(2)
                } else {
                    pixel.column_pos(j).unwrap().saturating_add(1)
                };
                pixel.move_pos(j, Direction::Horizontal(new_x));
            }
        }
        nth2.resize();

        // Middle of arm stretch rotation jump
        let mut nth3 = nth2.clone();
        nth3.pixels[6].move_pos(0, Direction::Vertical(3));
        nth3.pixels[6].move_pos(0, Direction::Vertical(3));
        nth3.pixels[8].move_pos(0, Direction::Vertical(3));
        nth3.pixels[13].move_pos(0, Direction::Horizontal(5));
        for pixel in &mut nth3.pixels {
            for idx in 0..pixel.len() {
                let new_y = pixel.row_pos(idx).unwrap().saturating_sub(1);
                pixel.move_pos(idx, Direction::Vertical(new_y));
            }
        }
        nth3.resize();

        // Climax of arm rotation raise
        let mut nth4 = nth2.clone();
        nth4.pixels[6].move_pos(0, Direction::Vertical(3));
        nth4.pixels[8].move_pos(0, Direction::Vertical(3));
        nth4.pixels[12].move_pos(0, Direction::Horizontal(2));
        nth4.resize();

        // Climax of arm rotation drop
        let mut nth5 = nth4.clone();
        nth5.pixels[6].move_pos(0, Direction::Vertical(5));
        nth5.pixels[8].move_pos(0, Direction::Vertical(5));
        nth5.resize();

        Self {
            frames: vec![first, nth2, nth3, nth4, nth5],
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

/// Sideways walking animation builder
#[derive(Default)]
pub(crate) struct SideWalk {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl SideWalk {
    pub(crate) fn new() -> Self {
        let first = Frame::new(
            vec![
                // Helmet accessory
                Pixel::new(ColorScheme::Standard(RED), 2, 1),
                // Helmet
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    1,
                    2,
                ),
                // Eyes
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 1, 3),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 2, 3),
                Pixel::new(ColorScheme::Standard(BLACK), 3, 3),
                // Body
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 0, 4),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    1,
                    4,
                ),
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 4, 4),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 0, 5),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    1,
                    5,
                ),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 4, 5),
                // Belt
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 1, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 2, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_GRAY), 3, 6),
                // Feet
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 1, 7),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 3, 7),
            ],
            None,
        );

        // Leg extend
        let mut nth2 = first.clone();
        for pixel in &mut nth2.pixels.iter_mut() {
            for i in 0..pixel.len() {
                let new_x = pixel.column_pos(i).unwrap().saturating_add(1);
                pixel.move_pos(i, Direction::Horizontal(new_x));
            }
        }
        nth2.pixels[9].change_color(0, LIGHT_BROWN);
        nth2.pixels[10].change_color(0, TRANSPARENT);
        nth2.pixels[14].move_pos(0, Direction::Horizontal(1));
        nth2.pixels[15].move_pos(0, Direction::Horizontal(5));
        nth2.pixels[15].move_pos(0, Direction::Vertical(6));
        nth2.resize();

        // Jump in air
        let mut nth3 = nth2.clone();
        nth3.pixels[9].change_color(0, MIDNIGHT);
        for pixel in &mut nth3.pixels.iter_mut().enumerate() {
            let (i, pixel) = pixel;
            if matches!(i, 15) {
                continue;
            }
            for idx in 0..pixel.len() {
                let new_y = pixel.row_pos(idx).unwrap().saturating_sub(1);
                pixel.move_pos(idx, Direction::Vertical(new_y));
            }
        }
        nth3.pixels
            .push(Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 6, 4));
        nth3.resize();

        // Coming down from jump
        let mut nth4 = first.clone();
        nth4.pixels[10].change_color(0, TRANSPARENT);
        nth4.pixels[14].move_pos(0, Direction::Horizontal(0));
        nth4.pixels[14].move_pos(0, Direction::Vertical(6));
        nth4.resize();

        // Cycle legs
        let mut nth5 = nth3.clone();
        for pixel in &mut nth5.pixels {
            for idx in 0..pixel.len() {
                let new_y = pixel.row_pos(idx).unwrap().saturating_add(1);
                pixel.move_pos(idx, Direction::Vertical(new_y));
            }
        }
        nth5.pixels[15].move_pos(0, Direction::Horizontal(3));
        nth5.resize();

        // Jump in air after cycle
        let mut nth6 = nth5.clone();
        for pixel in &mut nth6.pixels {
            for idx in 0..pixel.len() {
                let new_y = pixel.row_pos(idx).unwrap().saturating_sub(1);
                pixel.move_pos(idx, Direction::Vertical(new_y));
            }
        }
        nth6.pixels[16].change_color(0, TRANSPARENT);
        nth6.resize();

        // Coming down from jump
        let mut nth7 = first.clone();
        nth7.pixels[10].move_pos(0, Direction::Horizontal(5));
        nth7.pixels[10].move_pos(0, Direction::Vertical(4));
        nth7.pixels[15].change_color(0, TRANSPARENT);
        nth7.resize();

        Self {
            frames: vec![first, nth2, nth3, nth4, nth5, nth6, nth7],
            ..Default::default()
        }
    }
}
impl Sprite for SideWalk {
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

/// Front walking animation builder
#[derive(Default)]
pub(crate) struct FrontWalk {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl FrontWalk {
    pub(crate) fn new() -> Self {
        let first = Frame::new(
            vec![
                // Helmet accessory
                Pixel::new(ColorScheme::Standard(RED), 3, 0),
                // Helmet
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    2,
                    1,
                ),
                // Eyes
                Pixel::new(
                    ColorScheme::CheckPattern(CheckPattern::new(
                        BLACK,
                        LIGHT_GRAY,
                        Direction::Horizontal(3),
                    )),
                    2,
                    2,
                ),
                // Body
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 1, 3),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    2,
                    3,
                ),
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 5, 3),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 1, 4),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    2,
                    4,
                ),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 5, 4),
                // Belt
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    2,
                    5,
                ),
                // Feet
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 2, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 4, 6),
            ],
            None,
        );

        // Arm swing right
        let mut nth2 = first.clone();
        nth2.pixels[8].move_pos(0, Direction::Horizontal(6));
        nth2.pixels[11].change_color(0, TRANSPARENT);
        nth2.resize();

        // Leg cycle
        let mut nth3 = first.clone();
        nth3.pixels[10].change_color(0, TRANSPARENT);
        nth3.pixels[11].change_color(0, TRANSPARENT);

        // Arm swing left
        let mut nth4 = first.clone();
        nth4.pixels[6].move_pos(0, Direction::Horizontal(0));
        nth4.pixels[10].change_color(0, TRANSPARENT);
        nth4.resize();

        Self {
            frames: vec![first, nth2, nth3, nth4],
            ..Default::default()
        }
    }
}
impl Sprite for FrontWalk {
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

/// Back walking animation builder
#[derive(Default)]
pub(crate) struct BackWalk {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl BackWalk {
    pub(crate) fn new() -> Self {
        let first = Frame::new(
            vec![
                // Helmet accessory
                Pixel::new(ColorScheme::Standard(RED), 3, 0),
                // Helmet
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    2,
                    1,
                ),
                // Helmet
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    2,
                    2,
                ),
                // Body
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 1, 3),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    2,
                    3,
                ),
                Pixel::new(ColorScheme::Standard(DARK_BROWN), 5, 3),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 1, 4),
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(MIDNIGHT, Direction::Horizontal(3))),
                    2,
                    4,
                ),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 5, 4),
                // Belt
                Pixel::new(
                    ColorScheme::Stroke(Stroke::new(LIGHT_GRAY, Direction::Horizontal(3))),
                    2,
                    5,
                ),
                // Feet
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 2, 6),
                Pixel::new(ColorScheme::Standard(LIGHT_BROWN), 4, 6),
            ],
            None,
        );

        // Arm swing left
        let mut nth2 = first.clone();
        nth2.pixels[6].move_pos(0, Direction::Horizontal(0));
        nth2.pixels[10].change_color(0, TRANSPARENT);
        nth2.resize();

        // Leg cycle
        let mut nth3 = first.clone();
        nth3.pixels[10].change_color(0, TRANSPARENT);
        nth3.pixels[11].change_color(0, TRANSPARENT);

        // Arm swing right
        let mut nth4 = first.clone();
        nth4.pixels[8].move_pos(0, Direction::Horizontal(6));
        nth4.pixels[11].change_color(0, TRANSPARENT);
        nth4.resize();

        Self {
            frames: vec![first, nth2, nth3, nth4],
            ..Default::default()
        }
    }
}
impl Sprite for BackWalk {
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
