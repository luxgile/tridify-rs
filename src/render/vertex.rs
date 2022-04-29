use glium::implement_vertex;

use crate::core::Color;

implement_vertex!(Vertex, pos, color, uv);
#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    color: Color,
    uv: [f32; 2],
}

impl Vertex {
    /// Center       [ 0,  0],
    /// Top Right    [ 1,  1],
    /// Bottom Left  [-1, -1],
    pub fn from_viewport(x: f32, y: f32, c: Option<Color>, uv: Option<[f32; 2]>) -> Self {
        Self {
            pos: [x, y, 0.0],
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

    /// Get the vertex's color.
    #[must_use]
    pub fn color(&self) -> Color { self.color }
}

#[macro_export]
macro_rules! vertex {
    ($a:expr, $b:expr) => {
        crate::Vertex::from_viewport($a, $b, None, None)
    };
    ($a:expr, $b:expr, $c:expr) => {
        ldrawy::Vertex::from_viewport($a, $b, Some($c), None)
    };
    ($a:expr, $b:expr, $c:expr, $uv:expr) => {
        crate::Vertex::from_viewport($a, $b, Some($c), Some($uv))
    };
}
