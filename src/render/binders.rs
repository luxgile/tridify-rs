use wgpu::{
    BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages,
};

use crate::{Graphics, TextureSize};

pub enum BinderPart {
    Sampler,
    Texture(TextureSize),
}
impl BinderPart {
    fn to_entry(&self, index: u32) -> BindGroupLayoutEntry {
        match self {
            BinderPart::Sampler => BindGroupLayoutEntry {
                binding: index,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            BinderPart::Texture(size) => BindGroupLayoutEntry {
                binding: index,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: size.get_wgpu_view_dimension(),
                    multisampled: false,
                },
                count: None,
            },
        }
    }
}

pub struct Binder {
    layout: BindGroupLayout,
}
impl Binder {
    pub fn new(graphics: &impl Graphics, parts: &[BinderPart]) -> Self {
        let mut entries = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            entries.push(part.to_entry(i as u32));
        }
        let layout = graphics
            .get_device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            });
        Self { layout }
    }
    pub fn set_bind(&mut self, index: u32, )
}
