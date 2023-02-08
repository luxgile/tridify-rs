use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    collections::HashMap,
    error::Error,
    rc::Rc,
};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    ShaderStages,
};

use crate::{AssetRef, Graphics, Texture, TextureSize};

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

pub trait ToBinder {
    fn get_layout(&self, index: u32) -> BindGroupLayoutEntry;
    fn get_group(&self, index: u32) -> BindGroupEntry;
}

pub struct Binder {
    bindings: HashMap<u32, Rc<RefCell<dyn ToBinder>>>,
}
impl Binder {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    pub fn bind(&mut self, index: u32, binding: Rc<RefCell<dyn ToBinder>>) {
        self.bindings.insert(index, binding);
    }

    pub fn bake<'a>(&'a self, graphics: &impl Graphics) -> (BindGroupLayout, BindGroup) {
        let layout_entries = self
            .bindings
            .iter()
            .map(|(id, to_bind)| to_bind.as_ref().borrow().get_layout(*id))
            .collect::<Vec<_>>();
        let mut bind_entries = Vec::new();
        for (i, bind) in self.bindings.iter() {
            let cell = bind.as_ref();
            let borrowed_tobind = cell.borrow();
            bind_entries.push((*i, borrowed_tobind));
        }

        let bgl = graphics
            .get_device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });

        let bg = graphics
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bgl,
                entries: &bind_entries.iter().map(|(i, x)| x.get_group(*i)).collect::<Vec<_>>(),
                label: None,
            });

        (bgl, bg)
    }
}
