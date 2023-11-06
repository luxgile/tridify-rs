use glam::{Affine3A, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A};

use crate::{GpuDataLayout, InputLayoutGroup, InputType};

/// Representation for position, rotation and scale.
#[derive(Clone, Copy)]
pub struct Transform {
    affine: Affine3A,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            affine: Affine3A::from_scale_rotation_translation(scale, rotation, position),
        }
    }

    pub fn get_pos(&self) -> Vec3 { self.affine.translation.into() }
    pub fn set_pos(&mut self, pos: Vec3) { self.affine.translation = pos.into() }

    //FIXME: Not very optimal
    pub fn get_rot(&self) -> Quat { self.affine.to_scale_rotation_translation().1 }
    pub fn set_rot(&mut self, rot: Quat) {
        let (s, r, t) = self.affine.to_scale_rotation_translation();
        self.affine = Affine3A::from_scale_rotation_translation(s, rot, t);
    }

    //FIXME: Not very optimal
    pub fn get_scale(&self) -> Vec3 { self.affine.to_scale_rotation_translation().0 }
    pub fn set_scale(&mut self, scale: Vec3) {
        let (s, r, t) = self.affine.to_scale_rotation_translation();
        self.affine = Affine3A::from_scale_rotation_translation(scale, r, t);
    }

    pub fn local_translate(&mut self, movement: Vec3) {
        let local_move = self.affine.transform_vector3(movement);
        self.translate(local_move);
    }
    pub fn translate(&mut self, movement: Vec3) {
        self.affine.translation += Vec3A::from(movement);
    }

    //TODO: Test if this actually works
    pub fn rotate(&mut self, rotation: Quat) {
        let rot_matrix = Mat3A::from_quat(rotation);
        self.affine.matrix3 *= rot_matrix;
    }

    /// Create transform based only on position. Rotation and scale will have default values.
    pub fn from_pos(position: Vec3) -> Self { Transform::new(position, Quat::IDENTITY, Vec3::ONE) }

    /// Create transform based on position and view direction.
    pub fn from_look_to(eye: Vec3, forward: Vec3, up: Vec3) -> Self {
        Self {
            affine: Affine3A::look_to_lh(eye, forward, up),
        }
    }

    /// Create transform based on position and view point.
    pub fn from_look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self {
            affine: Affine3A::look_at_lh(eye, center, up),
        }
    }

    pub fn build_matrix(&self) -> Mat4 { Mat4::from(self.affine) }
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
