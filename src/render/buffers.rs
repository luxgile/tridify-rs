// use crate::core::*;
// use crate::{vertex, Color, Vec3, Vertex, Window};

// ///Queue of shapes to be drawn. All shapes added to the same batch will be drawn at the same time using the same brush.
// #[derive(Default, Debug)]
// pub struct ShapeBatch {
//     vertices: Vec<Vertex>,
//     indices: Vec<u32>,
//     index_count: u32,
// }

// impl ShapeBatch {
//     ///Add a triangle to the batch specifying its 3 vertices
//     pub fn add_triangle(&mut self, v: [Vertex; 3]) -> &mut Self {
//         let index = self.index_count;
//         self.vertices.push(v[0]);
//         self.indices.push(index);
//         self.vertices.push(v[1]);
//         self.indices.push(index + 1);
//         self.vertices.push(v[2]);
//         self.indices.push(index + 2);
//         self.index_count += 3;
//         self
//     }

//     pub fn add_rect(&mut self, rect: &Rect, color: Color) {
//         self.add_2d_square(rect.center().extend(0.), rect.size.x, rect.size.y, color);
//     }

//     ///Add a square on axis XY to the batch specifying the center, width, height and color.
//     pub fn add_2d_square(&mut self, center: Vec3, w: f32, h: f32, color: Color) {
//         //Adding vertices
//         let hw = w / 2.0;
//         let hh = h / 2.0;
//         self.vertices
//             .push(vertex!(center.x - hw, center.y - hh, 0., color, [0.0, 0.0]));
//         self.vertices
//             .push(vertex!(center.x + hw, center.y - hh, 0., color, [1.0, 0.0]));
//         self.vertices
//             .push(vertex!(center.x - hw, center.y + hh, 0., color, [0.0, 1.0]));
//         self.vertices
//             .push(vertex!(center.x + hw, center.y + hh, 0., color, [1.0, 1.0]));

//         //Adding indices
//         let index = self.index_count;
//         self.indices.push(index);
//         self.indices.push(index + 1);
//         self.indices.push(index + 2);
//         self.indices.push(index + 2);
//         self.indices.push(index + 1);
//         self.indices.push(index + 3);

//         self.index_count += 4;
//     }

//     ///Add a square to the batch specifying the center, width, height and color.
//     pub fn add_square(
//         &mut self, center: Vec3, up: Vec3, normal: Vec3, w: f32, h: f32, color: Color,
//     ) {
//         //Adding vertices
//         let right = up.cross(normal).normalize();
//         let hw = w / 2.0;
//         let hh = h / 2.0;

//         self.vertices.push(Vertex::from_vec(
//             center - right * hw - up * hh,
//             Some(color),
//             Some([0.0, 0.0]),
//         ));
//         self.vertices.push(Vertex::from_vec(
//             center + right * hw - up * hh,
//             Some(color),
//             Some([1.0, 0.0]),
//         ));
//         self.vertices.push(Vertex::from_vec(
//             center - right * hw + up * hh,
//             Some(color),
//             Some([0.0, 1.0]),
//         ));
//         self.vertices.push(Vertex::from_vec(
//             center + right * hw + up * hh,
//             Some(color),
//             Some([1.0, 1.0]),
//         ));

//         //Adding indices
//         let index = self.index_count;
//         self.indices.push(index);
//         self.indices.push(index + 1);
//         self.indices.push(index + 2);
//         self.indices.push(index + 2);
//         self.indices.push(index + 1);
//         self.indices.push(index + 3);

//         self.index_count += 4;
//     }

//     ///Add a cube to the batch specifying the center, orientation, size and color.
//     pub fn add_cube(&mut self, center: Vec3, orientation: Quat, scale: Vec3, color: Color) {
//         let hw = scale.x / 2.0;
//         let hh = scale.y / 2.0;
//         let hd = scale.z / 2.0;
//         let right = orientation * Vec3::X;
//         let up = orientation * Vec3::Y;
//         let forw = orientation * Vec3::Z;
//         self.add_square(center + right * hw, up, right, scale.y, scale.z, color);
//         self.add_square(center - right * hw, up, -right, scale.y, scale.z, color);
//         self.add_square(center + up * hh, forw, up, scale.x, scale.z, color);
//         self.add_square(center - up * hh, forw, -up, scale.x, scale.z, color);
//         self.add_square(center + forw * hd, up, forw, scale.x, scale.y, color);
//         self.add_square(center - forw * hd, up, -forw, scale.x, scale.y, color);
//     }

//     ///Create buffers based on current batch data.
//     pub fn bake_buffers(&self, wnd: &impl Window) -> ShapeBuffer {
//         let vertex_buffer = glium::VertexBuffer::new(wnd.display(), &self.vertices).unwrap();
//         let index_buffer = glium::IndexBuffer::new(
//             wnd.display(),
//             glium::index::PrimitiveType::TrianglesList,
//             &self.indices,
//         )
//         .unwrap();
//         ShapeBuffer {
//             vertex_buffer,
//             index_buffer,
//         }
//     }

//     /// Get a reference to the shape batch's vertices.
//     #[must_use]
//     pub fn vertices(&self) -> &[Vertex] { self.vertices.as_ref() }

//     /// Get a mutable reference to the shape batch's vertices.
//     #[must_use]
//     pub fn vertices_mut(&mut self) -> &mut Vec<Vertex> { &mut self.vertices }
// }

// ///Buffers created from the batch and prepared to be sent directly to the GPU
// #[derive(Debug)]
pub struct ShapeBuffer {}

// impl ShapeBuffer {
//     /// Get a reference to the shape buffer's vertex buffer.
//     #[must_use]
//     pub fn vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> { &self.vertex_buffer }

//     /// Get a reference to the shape buffer's index buffer.
//     #[must_use]
//     pub fn index_buffer(&self) -> &glium::IndexBuffer<u32> { &self.index_buffer }
// }
