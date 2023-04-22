use std::rc::Rc;

use wgpu::{util::DeviceExt, Buffer};

use crate::{GpuCtx, ToBinder, Window};

pub trait ToGpuBuf {
    fn build_buffer(&self, wnd: &GpuCtx) -> GpuBuffer;
}

/// Handle to a GPU buffer.
pub struct GpuBuffer {
    buffer: Rc<Buffer>,
}

impl GpuBuffer {
    /// Creates a new buffer with uninitialized data.
    fn new(wnd: &GpuCtx) -> Self {
        let buffer = wnd.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: todo!(),
            usage: todo!(),
            mapped_at_creation: todo!(),
        });
        Self {
            buffer: Rc::new(buffer),
        }
    }

    /// Creates a buffer with the given bytes.
    pub fn init(wnd: &GpuCtx, data: &[u8]) -> Self {
        let buffer = wnd
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                //TODO: User config
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            buffer: Rc::new(buffer),
        }
    }

    /// Update buffer GPU data with bytes provided.
    pub fn write(&mut self, wnd: &GpuCtx, data: &[u8]) {
        wnd.queue.write_buffer(&self.buffer, 0, data);
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

    fn debug_name(&self) -> &'static str { "GPU Buffer" }
}

impl Clone for GpuBuffer {
    fn clone(&self) -> Self {
        Self {
            buffer: Rc::clone(&self.buffer),
        }
    }
}
