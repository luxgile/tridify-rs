use std::error::Error;

use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceTexture, TextureView,
};

use crate::core::Color;
use crate::GpuCtx;
use crate::Rect;
use crate::ShapeBuffer;

use super::Brush;

/// Rendering configuration on how to create and represent the given frame.
pub struct RenderOptions {
    pub clear_color: Color,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            clear_color: Color::BLACK,
        }
    }
}

pub struct RenderPassBuilder {
    gpu_commands: CommandEncoder,
    frame_view: TextureView,
    frame_texture: Option<SurfaceTexture>,
}
impl RenderPassBuilder {
    pub fn from_gpu(gpu: &GpuCtx) -> Result<Self, Box<dyn Error>> {
        let (frame_texture, frame_view) = gpu.get_output_frame();
        let gpu_commands = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        Ok(Self {
            gpu_commands,
            frame_view,
            frame_texture,
        })
    }

    pub fn build_render_pass(&mut self, options: RenderOptions) -> RenderPass {
        let pass = self.gpu_commands.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.frame_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(options.clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        RenderPass { pass }
    }

    pub fn finish_render(self, wnd: &GpuCtx) {
        wnd.queue.submit(Some(self.gpu_commands.finish()));
        if let Some(frame_texture) = self.frame_texture {
            frame_texture.present();
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

    ///Draw batch on the canvas.
    pub fn render_shapes(&mut self, gpu: &GpuCtx, brush: &'a mut Brush, buffer: &'a ShapeBuffer) {
        if brush.needs_update() {
            brush.update(gpu);
        }
        self.render_shapes_cached(brush, buffer);
    }

    /// Draw batch on canvas. Does not check if brush requires any changes.
    pub fn render_shapes_cached(&mut self, brush: &'a Brush, buffer: &'a ShapeBuffer) {
        let pipeline = brush.get_pipeline();
        self.pass.set_pipeline(pipeline);
        let bind_groups = brush.get_bind_groups();
        bind_groups
            .iter()
            .for_each(|(id, bg)| self.pass.set_bind_group(*id, bg, &[]));

        self.pass
            .set_vertex_buffer(0, buffer.vertex_buffer.slice(..));
        self.pass
            .set_index_buffer(buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.pass.draw_indexed(0..buffer.index_len, 0, 0..1);
    }

    pub fn finish(self) {
    }
}
