use glam::{UVec2, Vec2};
use glium::{BackfaceCullingMode, Surface};

use crate::{
    core::{Color, Window},
    LErr, Rect,
};

use super::{Brush, ShapeBuffer};

#[derive(Debug, Default)]
pub struct DrawParams<'a> {
    params: glium::DrawParameters<'a>,
}
impl<'a> DrawParams<'a> {
    pub fn new() -> Self {
        Self {
            params: glium::DrawParameters::default(),
        }
    }

    ///Whenever faces are drawn only the front, back or both.
    pub fn backface_culling(&mut self, value: BackfaceCullingMode) -> &mut Self {
        self.params.backface_culling = value;
        self
    }

    ///How to calculate depth testing and render priority.
    pub fn depth_test(&mut self, value: glium::Depth) -> &mut Self {
        self.params.depth = value;
        self
    }

    pub fn as_ref(&mut self) -> &Self { self }
}

/// Manages the current frame being drawn.
pub struct Canvas {
    frame: glium::Frame,
}

impl Canvas {
    pub fn new(frame: glium::Frame) -> Self { Self { frame } }

    ///Clears the color in the canvas and resets to the specified color.
    pub fn clear_color(&mut self, color: Color) {
        self.frame.clear_color(color.r, color.g, color.b, color.a);
    }

    ///Finishes frame drawing.
    pub fn finish_canvas(self) -> Result<(), LErr> {
        let res = self.frame.finish();
        if let Err(e) = res {
            return Err(LErr::new(format!("{:?}", e)));
        }
        Ok(())
    }

    ///Draw batch on the canvas.
    pub fn draw_batch(
        &mut self, _wnd: &Window, brush: &Brush, buffers: ShapeBuffer, params: &DrawParams,
    ) {
        self.frame
            .draw(
                buffers.vertex_buffer(),
                buffers.index_buffer(),
                brush.program(),
                brush.uniform_buffer(),
                &params.params,
            )
            .unwrap();
    }
    pub fn get_size(&self) -> UVec2 {
        let dim = self.frame.get_dimensions();
        UVec2::new(dim.0, dim.1)
    }
    pub fn get_rect(&self) -> Rect {
        let dim = self.frame.get_dimensions();
        Rect::new(Vec2::new(0., 0.), Vec2::new(dim.0 as f32, dim.1 as f32))
    }
}
