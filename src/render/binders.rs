use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    collections::HashMap,
    error::Error,
    fmt::Display,
    rc::Rc,
};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    ShaderStages,
};

use crate::{AssetRef, Graphics, Texture, TextureSize};

pub trait ToBinder {
    fn get_layout(&self, index: u32) -> BindGroupLayoutEntry;
    fn get_group(&self, index: u32) -> BindGroupEntry;
}

pub struct Binder {
    bindings: HashMap<u32, Box<dyn ToBinder>>,
}
impl Binder {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    pub fn bind(&mut self, index: u32, binding: Box<dyn ToBinder>) {
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
                entries: &bind_entries
                    .iter()
                    .map(|(i, x)| x.get_group(*i))
                    .collect::<Vec<_>>(),
                label: None,
            });

        (bgl, bg)
    }
}
