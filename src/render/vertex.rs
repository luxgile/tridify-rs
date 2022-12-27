use glam::Vec3;

use crate::core::Color;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: Color,
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, c: Option<Color>, uv: Option<[f32; 2]>) -> Self {
        Self {
            pos: [x, y, z],
            color: c.unwrap_or(Color::WHITE),
            uv: uv.unwrap_or([0.0, 0.0]),
        }
    }
    pub fn from_vec(v: Vec3, c: Option<Color>, uv: Option<[f32; 2]>) -> Self {
        Self {
            pos: [v.x, v.y, v.z],
            color: c.unwrap_or(Color::WHITE),
            uv: uv.unwrap_or([0.0, 0.0]),
        }
    }
    #[must_use]
    #[inline]
    pub fn x(&self) -> f32 { self.pos[0] }
    #[must_use]
    #[inline]
    pub fn y(&self) -> f32 { self.pos[1] }
}

#[macro_export]
macro_rules! vertex {
    ($a:expr, $b:expr, $c:expr) => {
        crate::Vertex::new($a, $b, $c, None, None)
    };
    ($a:expr, $b:expr, $c:expr, $col:expr) => {
        crate::Vertex::new($a, $b, $c, Some($col), None)
    };
    ($a:expr, $b:expr, $c:expr, $col:expr, $uv:expr) => {
        crate::Vertex::new($a, $b, $c, Some($col), Some($uv))
    };
}
