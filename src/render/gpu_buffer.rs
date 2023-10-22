use std::rc::Rc;

use wgpu::{util::DeviceExt, Buffer};

use crate::{GpuCtx, ToBinder, Window};

pub trait ToGpuBuf {
    fn build_buffer(&self, wnd: &GpuCtx) -> GpuBuffer;
}

/// Handle to a GPU buffer.
pub struct GpuBuffer {
    //TODO: Make it private
    pub buffer: Rc<Buffer>,
}

impl GpuBuffer {
    /// Creates a new buffer with uninitialized data.
    pub fn new(gpu: &GpuCtx, size: u64, usage: wgpu::BufferUsages) -> Self {
        let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });
        Self {
            buffer: Rc::new(buffer),
        }
    }

    /// Creates a buffer with the given bytes.
    pub fn init(gpu: &GpuCtx, data: &[u8], usage: wgpu::BufferUsages) -> Self {
        let buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                //TODO: User config
                usage,
            });

        Self {
            buffer: Rc::new(buffer),
        }
    }

    /// Update buffer GPU data with bytes provided.
    pub fn write(&mut self, gpu: &GpuCtx, data: &[u8]) {
        gpu.queue.write_buffer(&self.buffer, 0, data);
    }

    pub fn get_handle(&self) -> Rc<Buffer> { Rc::clone(&self.buffer) }

    pub async fn map_buffer(&self, gpu: &GpuCtx) -> wgpu::BufferView {
        let buffer = self.buffer.as_ref().slice(..);
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        gpu.device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();
        buffer.get_mapped_range()
    }

    pub fn unmap(&self) { self.buffer.unmap(); }
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
