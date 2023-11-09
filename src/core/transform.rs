use glam::{Affine3A, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A};

use crate::{GpuDataLayout, InputLayoutGroup, InputType};

/// Representation for position, rotation and scale.
#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn transform_dir(&self, dir: Vec3) -> Vec3 { self.build_matrix().transform_vector3(dir) }
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.build_matrix().transform_point3(point)
    }

    pub fn forward(&self) -> Vec3 { self.rotation.mul_vec3(-Vec3::Z) }
    pub fn up(&self) -> Vec3 { self.rotation.mul_vec3(-Vec3::Y) }
    pub fn right(&self) -> Vec3 { self.rotation.mul_vec3(-Vec3::X) }

    pub fn local_translate(&mut self, movement: Vec3) {
        let local_move = self.transform_dir(movement);
        self.translate(local_move);
    }
    pub fn translate(&mut self, movement: Vec3) { self.position += movement; }

    pub fn rotate(&mut self, rotation: Quat) { self.rotation *= rotation; }

    /// Create transform based only on position. Rotation and scale will have default values.
    pub fn from_pos(position: Vec3) -> Self { Transform::new(position, Quat::IDENTITY, Vec3::ONE) }

    /// Create transform based on position and view direction.
    pub fn from_look_to(eye: Vec3, forward: Vec3, up: Vec3) -> Self {
        let affine = Affine3A::look_to_lh(eye, forward, up);
        let (scale, rotation, position) = affine.to_scale_rotation_translation();
        Self {
            scale,
            rotation,
            position,
        }
    }

    /// Create transform based on position and view point.
    pub fn from_look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let affine = Affine3A::look_at_lh(eye, center, up);
        let (scale, rotation, position) = affine.to_scale_rotation_translation();
        Self {
            scale,
            rotation,
            position,
        }
    }

    pub fn build_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}
impl Default for Transform {
    fn default() -> Self { Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE) }
}
impl GpuDataLayout for Transform {
    fn get_layout() -> crate::InputLayoutGroup {
        let mut instance_layout_group = InputLayoutGroup::new_instance();
        instance_layout_group
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4);
        instance_layout_group
    }
}
