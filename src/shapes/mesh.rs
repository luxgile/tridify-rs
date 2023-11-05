use std::path::Path;
use tobj;

use crate::{
    Asset, Brush, BrushDesc, Color, GpuCtx, Painter, Palette, Renderable, Sampler, Shape, Texture,
    Vertex, VertexBuffer, VertexBufferBuilder,
};

pub struct Model {
    meshes: Vec<MeshShape>,
    palettes: Vec<PbrPalette>,
}

impl Model {
    pub fn from_path(gpu: &GpuCtx, path: &Path) -> Self {
        let obj_load = tobj::load_obj(path.as_os_str(), &tobj::GPU_LOAD_OPTIONS);
        let (models, mats) =
            obj_load.expect(format!("Failed to load OBJ file at {:?}", path.as_os_str()).as_str());
        let mats = mats.expect("Failed to load MTL file");
        let palettes = mats
            .iter()
            .map(|m| {
                let mut palette = PbrPalette::new(gpu);
                if let Some(tex) = m.diffuse_texture.as_ref() {
                    let diffuse = Texture::from_path(gpu, Path::new(&tex.to_string()));
                    palette.set_diffuse(diffuse);
                }
                palette
            })
            .collect::<Vec<_>>();
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
                    indices: m.mesh.indices.clone(),
                    index_id_counter: 0,
                }
                .build_buffers(gpu);
                MeshShape {
                    mat_idx: m.mesh.material_id.unwrap_or(0) as usize,
                    vertex: buffer,
                }
            })
            .collect::<Vec<_>>();
        Model { meshes, palettes }
    }
}

impl Renderable for Model {
    fn get_shape_pal_pair(&self, index: usize) -> Option<(&dyn Shape, &dyn crate::Palette)> {
        if index >= self.meshes.len() {
            return None;
        }
        let mesh = &self.meshes[index];
        Some((mesh, &self.palettes[mesh.mat_idx]))
    }
}

pub struct MeshShape {
    mat_idx: usize,
    vertex: VertexBuffer,
}

impl Shape for MeshShape {
    fn get_vbuffer(&self) -> &VertexBuffer { &self.vertex }
}

pub struct PbrPalette {
    brush: Brush,
    diffuse: Texture,
    sampler: Sampler,
    needs_update: bool,
}

impl PbrPalette {
    pub fn new(gpu: &GpuCtx) -> Self {
        let brush = Brush::from_source(
            BrushDesc::default(),
            gpu,
            include_str!("pbr.wgsl").to_string(),
        )
        .unwrap();
        let diffuse = Texture::new_placerholder(gpu);
        let sampler = Sampler::new_default(gpu);
        Self {
            brush,
            diffuse,
            sampler,
            needs_update: true,
        }
    }

    pub fn set_diffuse(&mut self, texture: Texture) {
        self.diffuse = texture;
        self.needs_update = true;
    }
}
impl Asset for PbrPalette {
    fn needs_update(&self) -> bool { self.needs_update }
    fn update(&mut self, gpu: &GpuCtx) {
        self.brush.clear_bindings();
        // self.brush.bind(0, 0, self.camera_view.clone());
        // self.brush.bind(1, 0, self.diffuse.clone());
        // self.brush.bind(1, 1, self.sampler.clone());
        self.brush.update(gpu);
        self.needs_update = false;
    }
}
impl Painter for PbrPalette {
    fn get_brush(&self) -> &Brush { &self.brush }
}
impl Palette for PbrPalette {
}
