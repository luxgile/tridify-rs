use bytemuck::Pod;
use glam::{Quat, Vec3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

use crate::{Color, GpuBuffer, GpuCtx, Rect, Transform, Vertex};

// pub struct Shape {
//     pub meshes: Vec<Mesh>,
// }

pub struct InstanceBufferBuilder<T> {
    data: Vec<T>,
}
impl<T> InstanceBufferBuilder<T> {
    pub fn len(&self) -> usize { self.data.len() }
    pub fn new() -> Self { InstanceBufferBuilder { data: Vec::new() } }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn push_instance(&mut self, data: T) { self.data.push(data); }
}
impl<T: bytemuck::Pod> InstanceBufferBuilder<T> {
    pub fn bake(&self, gpu: &GpuCtx) -> InstanceBuffer {
        let buffer = GpuBuffer::init(
            gpu,
            bytemuck::cast_slice(&self.data),
            wgpu::BufferUsages::VERTEX, //TODO: This might actually be INSTANCE
        );
        InstanceBuffer {
            buffer,
            length: self.len() as u32,
        }
    }
}

impl<T: bytemuck::Pod> Default for InstanceBufferBuilder<T> {
    fn default() -> Self { Self::new() }
}

pub struct InstanceBuffer {
    pub buffer: GpuBuffer,
    pub length: u32,
}

// ///Buffers created from the batch and prepared to be sent directly to the GPU
// #[derive(Debug)]
pub struct VertexBuffer {
    pub vertex_buffer: GpuBuffer,
    pub index_buffer: GpuBuffer,
    pub index_len: u32,
}

// ///Queue of shapes to be drawn. All shapes added to the same batch will be drawn at the same time using the same brush.
#[derive(Default, Debug)]
pub struct VertexBufferBuilder<T> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
    pub index_id_counter: u32,
}

impl<T: Pod> VertexBufferBuilder<T> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            index_id_counter: 0,
        }
    }

    ///Create buffers based on current batch data.
    pub fn build_buffers(&self, gpu: &GpuCtx) -> VertexBuffer {
        let vertex_buffer = GpuBuffer::init(
            gpu,
            bytemuck::cast_slice(&self.vertices),
            BufferUsages::VERTEX,
        );
        let index_buffer = GpuBuffer::init(
            gpu,
            bytemuck::cast_slice(&self.indices),
            BufferUsages::INDEX,
        );
        VertexBuffer {
            vertex_buffer,
            index_buffer,
            index_len: self.indices.len() as u32,
        }
    }
}

impl VertexBufferBuilder<Vertex> {
    ///Add a triangle to the batch specifying its 3 vertices
    pub fn add_triangle(&mut self, v: [Vertex; 3]) -> &mut VertexBufferBuilder<Vertex> {
        let index = self.index_id_counter;
        self.vertices.push(v[0]);
        self.indices.push(index);
        self.vertices.push(v[1]);
        self.indices.push(index + 1);
        self.vertices.push(v[2]);
        self.indices.push(index + 2);
        self.index_id_counter += 3;
        self
    }

    ///Add a square using a Rect as input
    pub fn add_rect(&mut self, rect: &Rect, color: Color) -> &mut VertexBufferBuilder<Vertex> {
        self.add_2d_square(rect.center().extend(0.), rect.size.x, rect.size.y, color);
        self
    }

    ///Add a square on axis XY to the batch specifying the center, width, height and color.
    pub fn add_2d_square(
        &mut self, center: Vec3, w: f32, h: f32, color: Color,
    ) -> &mut VertexBufferBuilder<Vertex> {
        //Adding vertices
        let hw = w / 2.0;
        let hh = h / 2.0;
        self.vertices.push(Vertex {
            pos: [center.x - hw, center.y - hh, 0.],
            color,
            uv: [0.0, 0.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: [center.x + hw, center.y - hh, 0.],
            color,
            uv: [1.0, 0.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: [center.x - hw, center.y + hh, 0.],
            color,
            uv: [0.0, 1.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: [center.x + hw, center.y + hh, 0.],
            color,
            uv: [1.0, 1.0],
            ..Default::default()
        });

        //Adding indices
        let index = self.index_id_counter;
        self.indices.push(index);
        self.indices.push(index + 1);
        self.indices.push(index + 2);
        self.indices.push(index + 2);
        self.indices.push(index + 1);
        self.indices.push(index + 3);

        self.index_id_counter += 4;
        self
    }

    ///Add a square to the batch specifying the center, width, height and color.
    pub fn add_square(
        &mut self, center: Vec3, up: Vec3, normal: Vec3, w: f32, h: f32, color: Color,
    ) -> &mut VertexBufferBuilder<Vertex> {
        //Adding vertices
        let right = up.cross(normal).normalize();
        let hw = w / 2.0;
        let hh = h / 2.0;

        self.vertices.push(Vertex {
            pos: (center - right * hw - up * hh).into(),
            color,
            uv: [0.0, 0.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: (center + right * hw - up * hh).into(),
            color,
            uv: [1.0, 0.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: (center - right * hw + up * hh).into(),
            color,
            uv: [0.0, 1.0],
            ..Default::default()
        });
        self.vertices.push(Vertex {
            pos: (center + right * hw + up * hh).into(),
            color,
            uv: [1.0, 1.0],
            ..Default::default()
        });

        //Adding indices
        let index = self.index_id_counter;
        self.indices.push(index);
        self.indices.push(index + 2);
        self.indices.push(index + 1);
        self.indices.push(index + 1);
        self.indices.push(index + 2);
        self.indices.push(index + 3);

        self.index_id_counter += 4;
        self
    }

    ///Add a cube to the batch specifying the center, orientation, size and color.
    pub fn add_cube(
        &mut self, center: Vec3, orientation: Quat, scale: Vec3, color: Color,
    ) -> &mut VertexBufferBuilder<Vertex> {
        let hw = scale.x / 2.0;
        let hh = scale.y / 2.0;
        let hd = scale.z / 2.0;
        let right = orientation * Vec3::X;
        let up = orientation * Vec3::Y;
        let forw = orientation * Vec3::Z;
        self.add_square(center + right * hw, up, right, scale.y, scale.z, color);
        self.add_square(center - right * hw, up, -right, scale.y, scale.z, color);
        self.add_square(center + up * hh, forw, up, scale.x, scale.z, color);
        self.add_square(center - up * hh, forw, -up, scale.x, scale.z, color);
        self.add_square(center + forw * hd, up, forw, scale.x, scale.y, color);
        self.add_square(center - forw * hd, up, -forw, scale.x, scale.y, color);
        self
    }

    pub fn add_inv_cube(
        &mut self, center: Vec3, orientation: Quat, scale: Vec3, color: Color,
    ) -> &mut VertexBufferBuilder<Vertex> {
        let hw = scale.x / 2.0;
        let hh = scale.y / 2.0;
        let hd = scale.z / 2.0;
        let right = orientation * Vec3::X;
        let up = orientation * Vec3::Y;
        let forw = orientation * Vec3::Z;
        self.add_square(center + right * hw, up, -right, scale.y, scale.z, color);
        self.add_square(center - right * hw, up, right, scale.y, scale.z, color);
        self.add_square(center + up * hh, forw, -up, scale.x, scale.z, color);
        self.add_square(center - up * hh, forw, up, scale.x, scale.z, color);
        self.add_square(center + forw * hd, up, -forw, scale.x, scale.y, color);
        self.add_square(center - forw * hd, up, forw, scale.x, scale.y, color);
        self
    }
}
