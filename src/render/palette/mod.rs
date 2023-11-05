use std::mem::size_of;

use glam::Vec3;
use wgpu::BufferUsages;

use crate::{
    Asset, Brush, BrushDesc, Camera, GpuBuffer, GpuCtx, Painter, Sampler, Texture, Transform,
};
