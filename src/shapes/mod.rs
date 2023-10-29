use glam::{Quat, Vec3};

use crate::{Color, GpuCtx, Texture, VertexBuffer, VertexBufferBuilder};

pub trait Shape {
    fn get_vertex_buffer(&self) -> &VertexBuffer;
}

pub struct SkyboxShape {
    vertex: VertexBuffer,
    pub light_dir: Vec3,
}

impl SkyboxShape {
    pub fn new(gpu: &GpuCtx) -> Self {
        let vertex = VertexBufferBuilder::new()
            .add_inv_cube(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE, Color::WHITE)
            .build_buffers(gpu);
        Self {
            vertex,
            light_dir: Vec3::NEG_Y,
        }
    }
}

impl Shape for SkyboxShape {
    fn get_vertex_buffer(&self) -> &VertexBuffer { &self.vertex }
}
