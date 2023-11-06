use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout};

use crate::{
    core::Color,
    {GpuDataLayout, InputLayoutGroup, InputType},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Default)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: Color,
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}
unsafe impl Pod for Vertex {
}

impl Vertex {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z],
            ..Default::default()
        }
    }
    pub fn from_vec(pos: Vec3) -> Self {
        Self {
            pos: pos.to_array(),
            ..Default::default()
        }
    }
}

impl GpuDataLayout for Vertex {
    fn get_layout() -> InputLayoutGroup {
        let mut group = InputLayoutGroup::new_vertex();
        group
            .add_input(InputType::Vec3)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec2)
            .add_input(InputType::Vec3);
        group
    }
}
