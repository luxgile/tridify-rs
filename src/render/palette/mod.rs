use std::mem::size_of;

use glam::Vec3;
use wgpu::BufferUsages;

use crate::{
    Asset, Brush, BrushDesc, Camera, GpuBuffer, GpuCtx, Painter, Sampler, Texture, Transform,
};

pub trait Palette: Painter + Asset {}

pub struct SkyboxPalette {
    brush: Brush,
    diffuse: Texture,
    sampler: Sampler,
    camera_view: GpuBuffer,
    needs_update: bool,
}

impl SkyboxPalette {
    pub fn new(gpu: &GpuCtx) -> Self {
        let brush = Brush::from_source(
            BrushDesc::default(),
            gpu,
            include_str!("skybox.wgsl").to_string(),
        )
        .unwrap();
        let diffuse = Texture::new_placerholder(gpu);
        let sampler = Sampler::new_default(gpu);
        let camera_view = GpuBuffer::new(
            gpu,
            size_of::<Transform>() as u64,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        Self {
            brush,
            diffuse,
            sampler,
            camera_view,
            needs_update: true,
        }
    }

    pub fn set_diffuse_texture(&mut self, texture: Texture) {
        self.diffuse = texture;
        self.needs_update = true;
    }

    pub fn update_camera(&mut self, gpu: &GpuCtx, camera: &Camera) {
        let mut camera = camera.clone();
        camera.view.set_pos(Vec3::ZERO);
        let mvp = camera.build_camera_matrix();
        self.camera_view
            .write(gpu, bytemuck::cast_slice(&mvp.to_cols_array()));
    }
}

impl Painter for SkyboxPalette {
    fn get_brush(&self) -> &Brush {
        &self.brush
    }
}

impl Asset for SkyboxPalette {
    fn needs_update(&self) -> bool {
        self.needs_update
    }
    fn update(&mut self, gpu: &GpuCtx) {
        self.brush.clear_bindings();
        self.brush.bind(0, 0, self.camera_view.clone());
        self.brush.bind(1, 0, self.diffuse.clone());
        self.brush.bind(1, 1, self.sampler.clone());
        self.brush.update(gpu);
        self.needs_update = false;
    }
}
