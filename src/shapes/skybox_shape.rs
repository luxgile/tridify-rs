use glam::{Quat, Vec3};

use crate::{Color, GpuCtx, Shape, VertexBuffer, VertexBufferBuilder};

pub struct SkyboxShape {
    vertex: VertexBuffer,
}

impl SkyboxShape {
    pub fn new(gpu: &GpuCtx) -> Self {
        let vertex = VertexBufferBuilder::new()
            .add_inv_cube(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE, Color::WHITE)
            .build_buffers(gpu);
        Self { vertex }
    }
}

impl Shape for SkyboxShape {
    fn get_vertex_buffer(&self) -> &VertexBuffer { &self.vertex }
}
