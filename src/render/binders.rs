use std::{borrow::Borrow, collections::HashMap};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
};

use crate::GpuCtx;

/// Provides wgpu binding data for shaders.
pub trait ToBinder {
    fn get_layout(&self, index: u32) -> BindGroupLayoutEntry;
    fn get_group(&self, index: u32) -> BindGroupEntry;
    fn debug_name(&self) -> &'static str;
}
impl std::fmt::Debug for dyn ToBinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.debug_name())
    }
}

/// Group of uniforms and buffers that are binded to the GPU. If location or binder type does not match with the binded shader it will panic.
pub struct Binder {
    bindings: HashMap<u32, Box<dyn ToBinder>>,
}
impl Binder {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Add or replace a new bind based on its location.
    pub fn bind(&mut self, index: u32, binding: Box<dyn ToBinder>) {
        self.bindings.insert(index, binding);
    }

    /// Create GPU bindings to link with the render pipeline.
    pub fn bake(&self, wnd: &GpuCtx) -> (BindGroupLayout, BindGroup) {
        let layout_entries = self
            .bindings
            .iter()
            .map(|(id, to_bind)| to_bind.as_ref().borrow().get_layout(*id))
            .collect::<Vec<_>>();
        let mut bind_entries = Vec::new();
        for (i, bind) in self.bindings.iter() {
            let cell = bind.as_ref();
            let borrowed_tobind = cell.borrow();
            println!("Loc: {:?} - Bind: {:?}", i, bind);
            bind_entries.push((*i, borrowed_tobind));
        }
        let groups = bind_entries
            .iter()
            .map(|(i, x)| x.get_group(*i))
            .collect::<Vec<_>>();

        let bgl = wnd
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });

        let bg = wnd.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bgl,
            entries: &groups,
            label: None,
        });

        (bgl, bg)
    }
}
