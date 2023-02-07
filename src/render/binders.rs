use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    error::Error,
    rc::Rc,
};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    ShaderStages,
};

use crate::{Graphics, Texture, TextureSize};


// #[derive(Clone)]
// pub enum BinderPart<'a> {
//     Sampler,
//     Texture(&'a Texture),
// }
// impl<'a> BinderPart<'a> {
//     fn to_layout(&self, index: u32) -> BindGroupLayoutEntry {
//         match self {
//             BinderPart::Sampler => BindGroupLayoutEntry {
//                 binding: index,
//                 visibility: ShaderStages::FRAGMENT,
//                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
//                 count: None,
//             },
//             BinderPart::Texture(texture) => BindGroupLayoutEntry {
//                 binding: index,
//                 visibility: ShaderStages::FRAGMENT,
//                 ty: wgpu::BindingType::Texture {
//                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
//                     view_dimension: texture.desc.size.get_wgpu_view_dimension(),
//                     multisampled: false,
//                 },
//                 count: None,
//             },
//         }
//     }
//     fn to_entry(&self) -> BindGroupEntry {
//         match self {
//             BinderPart::Sampler => todo!(),
//             BinderPart::Texture(texture) => wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: wgpu::BindingResource::TextureView(&texture.view),
//             },
//         }
//     }
// }

pub trait ToBinder  {
    fn get_layout(&self, index: u32) -> BindGroupLayoutEntry;
    fn get_group(&self, index: u32) -> BindGroupEntry;
}

pub struct Binder<'a> {
    pub bind_layout: Option<BindGroupLayout>,
    pub bind_group: Option<BindGroup>,
    layout_parts: HashMap<u32, BindGroupLayoutEntry>,
    entry_parts: HashMap<u32, BindGroupEntry<'a>>,
    needs_update: bool,
}
impl<'a> Binder<'a> {
    pub fn new() -> Self {
        Self {
            needs_update: true,
            bind_group: None,
            bind_layout: None,
            layout_parts: HashMap::new(),
            entry_parts: HashMap::new(),
        }
    }
    pub fn bind(&mut self, index: u32, bind_part: &'a impl ToBinder) {
        self.layout_parts.insert(index, bind_part.get_layout(index));
        self.entry_parts.insert(index, bind_part.get_group(index));
        self.needs_update = true;
    }

    pub fn needs_update(&self) -> bool {
        self.needs_update
    }
    pub fn update(&mut self, graphics: &impl Graphics) -> Result<(), Box<dyn Error>> {
        self.bind_layout = Some(graphics
            .get_device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &self.layout_parts.values().cloned().collect::<Vec<_>>(),
            }));

        self.bind_group = Some(graphics.get_device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.bind_layout.as_ref().unwrap(),
                entries: &self.entry_parts.values().cloned().collect::<Vec<_>>(),
                label: None,
            },
        ));

        Ok(())
    }
}
