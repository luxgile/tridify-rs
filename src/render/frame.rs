use std::error::Error;

use glam::{UVec2, Vec2};
use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, SurfaceTexture, TextureView, TextureViewDescriptor,
};

use crate::core::WindowView;
use crate::Graphics;
use crate::ShapeBuffer;
use crate::{core::Color, Rect};

use super::Brush;

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

/// Manages the current frame being drawn.
pub struct Frame {
    options: Option<RenderOptions>,
    frame_texture: SurfaceTexture,
    frame_view: TextureView,
    encoder: CommandEncoder,
}

impl Frame {
    pub fn new(
        graphics: &impl Graphics, options: Option<RenderOptions>,
    ) -> Result<Self, Box<dyn Error>> {
        let frame_texture = graphics.get_surface().get_current_texture()?;
        let frame_view = frame_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let encoder = graphics
            .get_device()
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        Ok(Self {
            options,
            frame_texture,
            frame_view,
            encoder,
        })
    }

    ///Draw batch on the canvas.
    pub fn render(&mut self, brush: &Brush, buffer: &ShapeBuffer) {
        let mut pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.frame_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(
                        self.options
                            .get_or_insert(RenderOptions::default())
                            .clear_color
                            .into(),
                    ),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&brush.pipeline);
        //TODO: pass.set_bind_group(0, &, offsets)
        pass.set_vertex_buffer(0, buffer.vertex_buffer.slice(..));
        pass.set_index_buffer(buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..buffer.index_len, 0, 0..1);
    }
    ///Finishes frame drawing.
    pub fn finish(mut self, graphics: &impl Graphics) -> Result<(), &'static str> {
        let queue = graphics.get_queue();
        queue.submit(Some(self.encoder.finish()));
        self.frame_texture.present();
        Ok(())
    }
}
