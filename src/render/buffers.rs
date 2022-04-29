use crate::{vertex, Vertex, Window};

///Queue of shapes to be drawn. All shapes added to the same batch will be drawn at the same time using the same brush.
#[derive(Default)]
pub struct ShapeBatch {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    index_count: u32,
}

impl ShapeBatch {
    ///Add a triangle to the batch specifying its 3 vertices
    pub fn add_triangle(&mut self, v: [Vertex; 3]) {
        let index = self.index_count;
        self.vertices.push(v[0]);
        self.indices.push(index);
        self.vertices.push(v[1]);
        self.indices.push(index + 1);
        self.vertices.push(v[2]);
        self.indices.push(index + 2);
        self.index_count += 3;
    }

    ///Add a square to the batch specifying the center, width and height
    pub fn add_square(&mut self, c: Vertex, w: f32, h: f32) {
        //Adding vertices
        let hw = w / 2.0;
        let hh = h / 2.0;
        self.vertices
            .push(vertex!(c.x() - hw, c.y() - hh, c.color(), [0.0, 0.0]));
        self.vertices
            .push(vertex!(c.x() + hw, c.y() - hh, c.color(), [1.0, 0.0]));
        self.vertices
            .push(vertex!(c.x() - hw, c.y() + hh, c.color(), [0.0, 1.0]));
        self.vertices
            .push(vertex!(c.x() + hw, c.y() + hh, c.color(), [1.0, 1.0]));

        //Adding indices
        let index = self.index_count;
        self.indices.push(index);
        self.indices.push(index + 1);
        self.indices.push(index + 2);
        self.indices.push(index + 2);
        self.indices.push(index + 1);
        self.indices.push(index + 3);

        self.index_count += 4;
    }

    pub fn bake_buffers(&self, wnd: &Window) -> ShapeBuffer {
        let vertex_buffer = glium::VertexBuffer::new(wnd.display(), &self.vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            wnd.display(),
            glium::index::PrimitiveType::TrianglesList,
            &self.indices,
        )
        .unwrap();
        ShapeBuffer {
            vertex_buffer,
            index_buffer,
        }
    }
}

///Buffers created from the batch and prepared to be sent directly to the GPU
pub struct ShapeBuffer {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
}

impl ShapeBuffer {
    /// Get a reference to the shape buffer's vertex buffer.
    #[must_use]
    pub fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> { &self.vertex_buffer }

    /// Get a reference to the shape buffer's index buffer.
    #[must_use]
    pub fn index_buffer(&self) -> &glium::IndexBuffer<u32> { &self.index_buffer }
}
