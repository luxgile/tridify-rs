use glam::Mat4;

use crate::{GpuBuffer, Graphics, ToGpuBuf, Transform};

/// Projection representation using field of view and aspect ratio.
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
#[derive(Default)]
pub struct Camera {
    pub view: Transform,
    pub proj: Projection,
}

impl Camera {
    pub fn new(view: Transform, proj: Projection) -> Self { Self { view, proj } }

    pub fn build_camera_matrix(&self) -> Mat4 {
        let mat = self.proj.build_matrix() * self.view.build_matrix();
        mat
    }
}

impl ToGpuBuf for Camera {
    fn build_buffer(&self, graphics: &impl Graphics) -> crate::GpuBuffer {
        GpuBuffer::init(
            graphics,
            bytemuck::cast_slice(&self.build_camera_matrix().to_cols_array()),
        )
    }
}
