use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout};

use crate::core::Color;

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Default)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: Color,
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}
unsafe impl Pod for Vertex {}

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
            VertexAttribute {
                offset: (size_of::<[f32; 6]>() + size_of::<Color>()) as BufferAddress,
                shader_location: 3,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    };
    #[must_use]
    #[inline]
    pub fn x(&self) -> f32 {
        self.pos[0]
    }
    #[must_use]
    #[inline]
    pub fn y(&self) -> f32 {
        self.pos[1]
    }
}
