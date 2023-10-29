mod mesh_shape;
mod skybox_shape;

pub use mesh_shape::*;
pub use skybox_shape::*;

use glam::{Quat, Vec3};

use crate::{Color, GpuCtx, Texture, VertexBuffer, VertexBufferBuilder};

pub trait Shape {
    fn get_vertex_buffer(&self) -> &VertexBuffer;
}
