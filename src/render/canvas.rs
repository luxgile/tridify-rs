use glium::Surface;

use crate::core::{Color, Window};

use super::{Brush, ShapeBuffer};

/// Manages the current frame being drawn.
pub struct Canvas {
    frame: glium::Frame,
}

impl Canvas {
    pub fn new(frame: glium::Frame) -> Self { Self { frame } }

    pub fn clear_color(&mut self, color: Color) {
        self.frame.clear_color(color.r, color.g, color.b, color.a);
    }
    pub fn finish_canvas(self) -> Result<(), glium::SwapBuffersError> { self.frame.finish() }
    pub fn draw_batch(&mut self, _wnd: &Window, brush: &Brush, buffers: ShapeBuffer) {
        self.frame
            .draw(
                buffers.vertex_buffer(),
                buffers.index_buffer(),
                brush.program(),
                brush.uniform_buffer(),
                &Default::default(),
            )
            .unwrap();
    }
}
