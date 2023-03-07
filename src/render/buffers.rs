use std::error::Error;

use glam::{Quat, Vec3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};

use crate::{vertex, Color, Rect, Vertex, WindowCtx};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub tris: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, tris: Vec<u32>) -> Self { Self { vertices, tris } }
}

// ///Buffers created from the batch and prepared to be sent directly to the GPU
// #[derive(Debug)]
pub struct ShapeBuffer {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_len: u32,
}

// ///Queue of shapes to be drawn. All shapes added to the same batch will be drawn at the same time using the same brush.
#[derive(Default, Debug)]
pub struct ShapeBatch {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub index_id_counter: u32,
}

impl ShapeBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            index_id_counter: 0,
        }
    }

    ///Create buffers based on current batch data.
    pub fn bake_buffers(&self, ctx: &WindowCtx) -> ShapeBuffer {
        let device = &ctx.device;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.vertices),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        });
        ShapeBuffer {
            vertex_buffer,
            index_buffer,
            index_len: self.indices.len() as u32,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut ShapeBatch {
        self.vertices.extend(&mesh.vertices);
        self.indices.extend(&mesh.tris);
        self.index_id_counter += mesh.vertices.len() as u32;
        self
    }

    ///Add a triangle to the batch specifying its 3 vertices
    pub fn add_triangle(&mut self, v: [Vertex; 3]) -> &mut ShapeBatch {
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

    pub fn add_rect(&mut self, rect: &Rect, color: Color) -> &mut ShapeBatch {
        self.add_2d_square(rect.center().extend(0.), rect.size.x, rect.size.y, color);
        self
    }

    ///Add a square on axis XY to the batch specifying the center, width, height and color.
    pub fn add_2d_square(&mut self, center: Vec3, w: f32, h: f32, color: Color) -> &mut ShapeBatch {
        //Adding vertices
        let hw = w / 2.0;
        let hh = h / 2.0;
        self.vertices
            .push(vertex!(center.x - hw, center.y - hh, 0., color, [0.0, 0.0]));
        self.vertices
            .push(vertex!(center.x + hw, center.y - hh, 0., color, [1.0, 0.0]));
        self.vertices
            .push(vertex!(center.x - hw, center.y + hh, 0., color, [0.0, 1.0]));
        self.vertices
            .push(vertex!(center.x + hw, center.y + hh, 0., color, [1.0, 1.0]));

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
    ) -> &mut ShapeBatch {
        //Adding vertices
        let right = up.cross(normal).normalize();
        let hw = w / 2.0;
        let hh = h / 2.0;

        self.vertices.push(Vertex::from_vec(
            center - right * hw - up * hh,
            Some(color),
            Some([0.0, 0.0]),
        ));
        self.vertices.push(Vertex::from_vec(
            center + right * hw - up * hh,
            Some(color),
            Some([1.0, 0.0]),
        ));
        self.vertices.push(Vertex::from_vec(
            center - right * hw + up * hh,
            Some(color),
            Some([0.0, 1.0]),
        ));
        self.vertices.push(Vertex::from_vec(
            center + right * hw + up * hh,
            Some(color),
            Some([1.0, 1.0]),
        ));

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
    ) -> &mut ShapeBatch {
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
}
