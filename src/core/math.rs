use glam::*;

#[derive(Default)]
pub struct Rect {
    /// Bottom left from the rect
    pub pos: Vec2,
    pub size: Vec2,
}
impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self { Self { pos, size } }
    pub fn center(&self) -> Vec2 { self.pos + self.size / 2.0 }
}
