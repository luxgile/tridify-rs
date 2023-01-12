use std::{collections::HashMap, error::Error, rc::Rc};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages,
};

use crate::{Graphics, Texture, TextureSize};

use super::texture;

pub enum BinderPart {
    Sampler,
    Texture(Rc<Texture>),
}
impl BinderPart {
    fn to_layout(&self, index: u32) -> BindGroupLayoutEntry {
        match self {
            BinderPart::Sampler => BindGroupLayoutEntry {
                binding: index,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            BinderPart::Texture(texture) => BindGroupLayoutEntry {
                binding: index,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: texture.desc.size.get_wgpu_view_dimension(),
                    multisampled: false,
                },
                count: None,
            },
        }
    }
    fn to_entry(&self) -> BindGroupEntry {
        match self {
            BinderPart::Sampler => todo!(),
            BinderPart::Texture(texture) => wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
        }
    }
}

pub trait ToBinder {
    fn get_part(&self) -> BinderPart;
}

pub struct Binder {
    bind_group: Option<BindGroup>,
    parts: HashMap<u32, BinderPart>,
    needs_update: bool,
}
impl Binder {
    pub fn new(graphics: &impl Graphics) -> Self {
        Self {
            needs_update: true,
            parts: HashMap::new(),
            bind_group: None,
        }
    }
    pub fn set_bind(&mut self, index: u32, bind_part: &impl ToBinder) {
        let part = bind_part.get_part();
        self.parts.insert(index, part);
    }

    pub fn requires_update(&self) -> bool {
        self.needs_update
    }
    pub fn update(&mut self, graphics: &impl Graphics) -> Result<(), Box<dyn Error>> {
        let mut layout_entries = Vec::new();
        let mut wgpu_entries = Vec::new();
        for (i, part) in self.parts.iter().enumerate() {
            layout_entries.push(part.1.to_layout(i as u32));
            wgpu_entries.push(part.1.to_entry());
        }
        let layout = graphics
            .get_device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });

        self.bind_group = Some(graphics.get_device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &wgpu_entries,
                label: None,
            },
        ));

        Ok(())
    }
}
