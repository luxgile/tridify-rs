use std::rc::Rc;

use wgpu::{ShaderModel, ShaderStages};

use crate::{GpuCtx, ToBinder};

/// Representation on how a texture will be drawn into a shape.
pub struct Sampler {
    inner_sampler: Rc<wgpu::Sampler>,
}
impl Sampler {
    pub fn new_default(gpu: &GpuCtx) -> Self {
        Self {
            inner_sampler: Rc::new(gpu.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })),
        }
    }
}
impl ToBinder for Sampler {
    fn get_layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index,
            visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }
    }

    fn get_group(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::Sampler(&self.inner_sampler),
        }
    }

    fn debug_name(&self) -> &'static str { "Sampler" }
}

impl Clone for Sampler {
    fn clone(&self) -> Self {
        Self {
            inner_sampler: Rc::clone(&self.inner_sampler),
        }
    }
}
