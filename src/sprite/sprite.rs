use crate::renderer::Frame;

/// A `Sprite` consists of two or more `Frame`s, where each `Frame` represents a
/// visual state of the sprite
pub(crate) trait Sprite {
    fn frames(&self) -> &Vec<Frame>;
    fn frame_pos(&self) -> usize;
    fn timer(&self) -> f32;
    fn frame_pos_mut(&mut self) -> &mut usize;
    fn timer_mut(&mut self) -> &mut f32;
}
