use std::error::Error;

use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Extent3d, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceTexture, TextureView, TextureViewDescriptor,
};
use wgpu::{ImageCopyBuffer, ImageCopyTexture, Origin3d, TextureAspect};

use crate::core::Color;
use crate::Texture;
use crate::VertexBuffer;
use crate::{GpuBuffer, InstanceBufferBuilder};
use crate::{GpuCtx, InstanceBuffer};
use crate::{OutputSurface, Painter, Shape};
use crate::{Rect, Renderable};

use super::Brush;

/// Rendering configuration on how to create and represent the given frame.
pub struct RenderOptions {
    pub clear_color: Option<Color>,
}

impl RenderOptions {
    fn wgpu_load(&self) -> wgpu::LoadOp<wgpu::Color> {
        if let Some(color) = self.clear_color {
            return wgpu::LoadOp::Clear(color.into());
        } else {
            return wgpu::LoadOp::Load;
        }
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            clear_color: Some(Color::BLACK),
        }
    }
}

pub struct GpuCommands {
    gpu_commands: CommandEncoder,
    frame_view: TextureView,
    surface: Option<SurfaceTexture>,
}
impl GpuCommands {
    pub fn from_gpu(gpu: &GpuCtx) -> Result<Self, Box<dyn Error>> {
        let gpu_commands = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        //TODO: Surface needs to store both surface texture and surface texture view!
        let (surface, frame_view) = match gpu.get_output() {
            OutputSurface::Window(wnd) => {
                let frame_texture = wnd.surface.get_current_texture().unwrap();
                let frame_view = frame_texture
                    .texture
                    .create_view(&TextureViewDescriptor::default());
                (Some(frame_texture), frame_view)
            }
            OutputSurface::Headless(tex) => (None, tex.create_wgpu_view()),
        };

        Ok(Self {
            gpu_commands,
            surface,
            frame_view,
        })
    }

    pub fn start_render_pass(&mut self, options: RenderOptions) -> RenderPass {
        let pass = self.gpu_commands.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.frame_view,
                resolve_target: None,
                ops: Operations {
                    load: options.wgpu_load(),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        RenderPass { pass }
    }

    pub fn texture_to_buffer(&mut self, source: &Texture, dest: &GpuBuffer) {
        let texture_size = source.desc.size.get_size();
        let source_copy = ImageCopyTexture {
            texture: source.get_wgpu_handle(),
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        };
        let dest_copy = ImageCopyBuffer {
            buffer: &dest.get_handle(),
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(texture_size.x * 4),
                rows_per_image: Some(texture_size.y),
            },
        };
        let copy_size = Extent3d {
            width: texture_size.x,
            height: texture_size.y,
            depth_or_array_layers: texture_size.z,
        };
        self.gpu_commands
            .copy_texture_to_buffer(source_copy, dest_copy, copy_size);
    }

    pub fn complete(self, gpu: &GpuCtx) {
        gpu.queue.submit(Some(self.gpu_commands.finish()));
        if let Some(surface) = self.surface {
            surface.present();
        }
    }
}

/// Manages the current frame being drawn.
pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    /// Rect provided needs to be in pixels.
    pub fn set_scissor(&mut self, rect: &Rect) {
        self.pass.set_scissor_rect(
            rect.pos.x as u32,
            rect.pos.y as u32,
            rect.size.x as u32,
            rect.size.y as u32,
        );
    }

    ///Simplified render function for high-level inputs.
    pub fn render(&mut self, renderable: &'a impl Renderable) {
        for (shape, painter) in renderable.iter_pairs() {
            self.render_raw(painter.get_brush(), shape.get_vbuffer(), None);
        }
    }

    ///Render using raw buffers as inputs.
    pub fn render_raw(
        &mut self, brush: &'a Brush, vertex: &'a VertexBuffer, instance: Option<&'a InstanceBuffer>,
    ) {
        if brush.needs_update() {
            println!("WARNING: Brush with changes is not being saved before being used. Make sure to call 'update' beforehand.");
        }

        let pipeline = brush.get_pipeline();
        self.pass.set_pipeline(pipeline);
        let bind_groups = brush.get_bind_groups();
        bind_groups
            .iter()
            .for_each(|(id, bg)| self.pass.set_bind_group(*id, bg, &[]));

        //Vertex input
        self.pass
            .set_vertex_buffer(0, vertex.vertex_buffer.buffer.slice(..));
        self.pass.set_index_buffer(
            vertex.index_buffer.buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        //Instance input
        let mut instance_count = 1;
        if let Some(buffer) = instance {
            instance_count = buffer.length;
            self.pass
                .set_vertex_buffer(1, buffer.buffer.buffer.slice(..));
        }

        self.pass
            .draw_indexed(0..vertex.index_len, 0, 0..instance_count);
    }

    pub fn finish(self) {
    }
}
