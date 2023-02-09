use wgpu::{util::DeviceExt, Buffer};

use crate::{Graphics, ToBinder};

use super::graphics;

pub trait ToGpuBuf {
    fn build_buffer(&self, graphics: &impl Graphics) -> GpuBuffer;
}

pub struct GpuBuffer {
    buffer: Buffer,
}

impl GpuBuffer {
    pub fn new(graphics: &impl Graphics) -> Self {
        let buffer = graphics
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: todo!(),
                usage: todo!(),
                mapped_at_creation: todo!(),
            });
        Self { buffer }
    }

    pub fn init(graphics: &impl Graphics, data: &[u8]) -> Self {
        let buffer = graphics
            .get_device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                //TODO: User config
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        Self { buffer }
    }

    pub fn write(&mut self, graphics: &impl Graphics, data: &[u8]) {
        graphics.get_queue().write_buffer(&self.buffer, 0, data);
    }
}

impl ToBinder for GpuBuffer {
    fn get_layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index,
            //TODO: User should be able to config this.
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                //TODO: User should be able to config this.
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn get_group(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: self.buffer.as_entire_binding(),
        }
    }
}
