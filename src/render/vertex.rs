use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout};

use crate::core::Color;

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: Color,
    pub uv: [f32; 2],
}
unsafe impl Pod for Vertex {
}

impl Vertex {
    pub const DEFAULT_DESC: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex>() as BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            VertexAttribute {
                offset: size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: (size_of::<[f32; 3]>() + size_of::<Color>()) as BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x2,
            },
        ],
    };
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
        nucley::Vertex::new($a, $b, $c, None, None)
    };
    ($a:expr, $b:expr, $c:expr, $col:expr) => {
        crate::Vertex::new($a, $b, $c, Some($col), None)
    };
    ($a:expr, $b:expr, $c:expr, $col:expr, $uv:expr) => {
        crate::Vertex::new($a, $b, $c, Some($col), Some($uv))
    };
}
