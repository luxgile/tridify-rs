use std::error::Error;

use glam::{UVec2, Vec2};
use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, SurfaceTexture, TextureView, TextureViewDescriptor,
};

use crate::core::WindowSurface;
use crate::graphics::Graphics;
use crate::{core::Color, Rect};

use super::Brush;

pub struct RenderOptions {
    pub clear_color: Color,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            clear_color: Color::WHITE,
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
        wnd: &WindowSurface, graphics: &Graphics, options: Option<RenderOptions>,
    ) -> Result<Self, Box<dyn Error>> {
        let frame_texture = wnd.surface.get_current_texture()?;
        let frame_view = frame_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let encoder = graphics
            .get_device()?
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        Ok(Self {
            options,
            frame_texture,
            frame_view,
            encoder,
        })
    }

    ///Draw batch on the canvas.
    pub fn render(&mut self, brush: Brush) {
        let pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
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

        //TODO: pass.set_pipeline(brush.pipeline);
        //TODO: pass.set_bind_group(0, &, offsets)
        //TODO: Vertex buffer
        //TODO: Index buffer
        //TODO: pass.draw_indexed(indices, 0, 0..1);
    }
    ///Finishes frame drawing.
    pub fn finish(mut self, graphics: &Graphics) -> Result<(), &'static str> {
        let queue = graphics.get_queue()?;
        queue.submit(Some(self.encoder.finish()));
        self.frame_texture.present();
        Ok(())
    }
}
