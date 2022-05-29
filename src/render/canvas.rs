use glium::{BackfaceCullingMode, Surface};

use crate::{
    core::{Color, Window},
    LErr,
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

    pub fn backface_culling(&mut self, value: BackfaceCullingMode) -> &mut Self {
        self.params.backface_culling = value;
        self
    }
    pub fn depth_test(&mut self, value: glium::Depth) -> &mut Self {
        self.params.depth = value;
        self
    }
}

/// Manages the current frame being drawn.
pub struct Canvas {
    frame: glium::Frame,
}

impl Canvas {
    pub fn new(frame: glium::Frame) -> Self { Self { frame } }

    pub fn clear_color(&mut self, color: Color) {
        self.frame.clear_color(color.r, color.g, color.b, color.a);
    }
    pub fn finish_canvas(self) -> Result<(), LErr> {
        let res = self.frame.finish();
        if let Err(e) = res {
            return Err(LErr::new(format!("{:?}", e)));
        }
        Ok(())
    }
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
}
