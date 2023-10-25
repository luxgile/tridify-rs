use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout};

use crate::{
    core::Color,
    input_layout::{InputLayoutGroup, InputType},
};

pub trait VertexDataLayout {
    fn get_layout() -> InputLayoutGroup;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Default)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: Color,
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}
unsafe impl Pod for Vertex {}

impl VertexDataLayout for Vertex {
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
