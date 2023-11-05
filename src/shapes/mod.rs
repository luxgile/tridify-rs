mod mesh;
mod skybox;

pub use mesh::*;
pub use skybox::*;

use glam::{Quat, Vec3};

use crate::{Asset, Brush, Color, GpuCtx, Texture, VertexBuffer, VertexBufferBuilder};

pub trait Shape {
    fn get_vbuffer(&self) -> &VertexBuffer;
}

pub trait Painter {
    fn get_brush(&self) -> &Brush;
}

pub trait Palette: Painter + Asset {}

pub trait Renderable {
    fn get_shape_pal_pair(&self, index: usize) -> Option<(&dyn Shape, &dyn Palette)>;
    fn iter_pairs(&self) -> Vec<(&dyn Shape, &dyn Palette)> {
        let mut idx = 0;
        let mut pairs = Vec::new();
        let mut pair = self.get_shape_pal_pair(idx);
        while pair.is_some() {
            pairs.push(pair.unwrap());
            idx += 1;
            pair = self.get_shape_pal_pair(idx);
        }
        pairs
    }
}
