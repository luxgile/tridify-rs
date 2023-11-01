use std::path::Path;
use tobj;

use crate::{Brush, Color, GpuCtx, Shape, Vertex, VertexBuffer, VertexBufferBuilder};

pub struct ModelShape {
    meshes: Vec<MeshShape>,
    // brushes: Vec<Pallete>,
}

impl ModelShape {
    pub fn from_path(gpu: &GpuCtx, path: Path) -> Self {
        let obj_load = tobj::load_obj(path.as_os_str(), &tobj::GPU_LOAD_OPTIONS);
        let (models, mats) =
            obj_load.expect(format!("Failed to load OBJ file at {:?}", path.as_os_str()).as_str());
        let mats = mats.expect("Failed to load MTL file");
        let meshes = models
            .iter()
            .map(|m| {
                let verts = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        Vertex {
                            pos: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            uv: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                            color: Color::WHITE, //TODO: Support color from OBJ
                        }
                    })
                    .collect::<Vec<_>>();
                let buffer = VertexBufferBuilder {
                    vertices: verts,
                    indices: m.mesh.indices,
                    index_id_counter: 0,
                }
                .build_buffers(gpu);
                MeshShape {
                    mat_idx: m.mesh.material_id.unwrap_or(0) as u32,
                    vertex: buffer,
                }
            })
            .collect::<Vec<_>>();
        ModelShape { meshes: meshes }
    }
}

pub struct MeshShape {
    mat_idx: u32,
    vertex: VertexBuffer,
}

// impl Shape for MeshShape {
//     fn get_vertex_buffer(&self) -> &VertexBuffer { &self.vertex }
// }
