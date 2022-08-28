pub use glam::*;

pub mod math {
    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat3, Mat4, Quat, UVec2, UVec3, UVec4,
        Vec2, Vec3, Vec4,
    };
}

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
