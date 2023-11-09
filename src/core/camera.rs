use glam::{Affine3A, Mat4};

use crate::{GpuBuffer, GpuCtx, ToGpuBuf, Transform};

/// Projection representation using field of view and aspect ratio.
#[derive(Clone)]
pub struct Projection {
    pub aspect: f32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Projection {
    pub fn new(aspect: f32, fov: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect,
            fov,
            znear,
            zfar,
        }
    }
    pub fn build_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(self.fov.to_radians(), self.aspect, self.znear, self.zfar)
    }
}
impl Default for Projection {
    fn default() -> Self {
        Self {
            aspect: 16.0 / 9.0,
            fov: 65.0,
            zfar: 100.0,
            znear: 0.1,
        }
    }
}

/// Representation of camera to simplify matrices calculation
#[derive(Default, Clone)]
pub struct Camera {
    pub view: Transform,
    pub proj: Projection,
}

impl Camera {
    pub fn new(view: Transform, proj: Projection) -> Self { Self { view, proj } }

    pub fn build_camera_matrix(&self) -> Mat4 {
        self.proj.build_matrix()
            * Mat4::from(Affine3A::look_to_lh(self.view.position, self.view.forward(), self.view.up()))
    }
}

impl ToGpuBuf for Camera {
    fn build_buffer(&self, gpu: &GpuCtx) -> crate::GpuBuffer {
        GpuBuffer::init(
            gpu,
            bytemuck::cast_slice(&self.build_camera_matrix().to_cols_array()),
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        )
    }
}
