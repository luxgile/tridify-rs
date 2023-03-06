use std::error::Error;

use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceTexture, TextureView, TextureViewDescriptor,
};

use crate::core::Color;
use crate::Graphics;
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

/// Manages the current frame being drawn.
pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
    encoder: CommandEncoder,
    frame_view: TextureView,
    frame_texture: SurfaceTexture,
}

impl<'a> RenderPass<'a> {
    pub fn new(graphics: &impl Graphics, options: RenderOptions) -> Result<Self, Box<dyn Error>> {
        let frame_texture = graphics.get_surface().get_current_texture()?;
        let frame_view = frame_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = graphics
            .get_device()
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &frame_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(options.clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        Ok(Self {
            pass,
            encoder,
            frame_view,
            frame_texture,
        })
    }

    ///Draw batch on the canvas.
    pub fn render(&mut self, graphics: &impl Graphics, brush: &mut Brush, buffer: &ShapeBuffer) {
        if brush.needs_update() {
            brush.update(graphics);
        }

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

    ///Finishes frame drawing.
    pub fn finish(mut self, graphics: &impl Graphics) -> Result<(), &'static str> {
        let queue = graphics.get_queue();
        queue.submit(Some(self.encoder.finish()));
        self.frame_texture.present();
        Ok(())
    }
}
