use crate::animator::Animation;
use crate::prelude::*;
use crate::renderer::{Frame, Pixel};
use crate::window::WindowError;

pub(crate) struct MockScreen {
    pub(crate) buffer: Vec<u8>,
    width: u32,
    height: u32,
}
impl MockScreen {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        MockScreen {
            buffer: vec![0; (width * height * 4) as usize], // RGBA buffer
            width,
            height,
        }
    }
}
impl Screen for MockScreen {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn frame_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    fn render(&mut self) -> Result<(), WindowError> {
        Ok(())
    }
    fn clear(&mut self) -> Result<(), WindowError> {
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct MockCharacter {
    idle: MockIdle,
    side_walk: MockSide,
    front_walk: MockFront,
    back_walk: MockBack,
    pub(crate) animation_trigerred: String,
}
impl MockCharacter {
    pub(crate) fn new() -> Self {
        Self {
            idle: MockIdle::new(),
            side_walk: MockSide::new(),
            front_walk: MockFront::new(),
            back_walk: MockBack::new(),
            animation_trigerred: String::default(),
        }
    }
}
// Mocked with no output since animation will play an empty
// frame container
impl Character<MockScreen> for MockCharacter {
    fn idle(&mut self) -> &mut dyn Animation<MockScreen> {
        self.animation_trigerred = "idle".into();
        &mut self.idle
    }
    fn side_walk(&mut self) -> &mut dyn Animation<MockScreen> {
        self.animation_trigerred = "side".into();
        &mut self.side_walk
    }

    fn front_walk(&mut self) -> &mut dyn Animation<MockScreen> {
        self.animation_trigerred = "front".into();
        &mut self.front_walk
    }

    fn back_walk(&mut self) -> &mut dyn Animation<MockScreen> {
        self.animation_trigerred = "back".into();
        &mut self.back_walk
    }
}
#[derive(Default)]
struct MockIdle {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl MockIdle {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        0,
                        1,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        1,
                        1,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
            ],
            ..Default::default()
        }
    }
}
#[derive(Default)]
struct MockSide {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl MockSide {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        0,
                        2,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        1,
                        2,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
            ],
            ..Default::default()
        }
    }
}
#[derive(Default)]
struct MockFront {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl MockFront {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        0,
                        3,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        1,
                        3,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
            ],
            ..Default::default()
        }
    }
}
#[derive(Default)]
pub(crate) struct MockBack {
    frames: Vec<Frame>,
    timer: f32,
    frame_pos: usize,
}
impl MockBack {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        0,
                        4,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
                Frame {
                    pixels: vec![Pixel::new(
                        ColorScheme::Standard(Color::RGB(0, 0, 255)),
                        1,
                        4,
                    )],
                    height: 5,
                    width: 5,
                    duration: None,
                },
            ],
            ..Default::default()
        }
    }
}
macro_rules! impl_sprite {
    ($struct_name:ident) => {
        impl Sprite for $struct_name {
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
    };
}
impl_sprite!(MockIdle);
impl_sprite!(MockSide);
impl_sprite!(MockFront);
impl_sprite!(MockBack);
