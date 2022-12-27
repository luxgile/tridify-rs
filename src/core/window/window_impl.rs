use glium::Display;

use crate::{Canvas, Color};

pub trait Window {
    fn display(&self) -> &Display;
    fn start_frame(&mut self, color: Color) -> Canvas;

    /// Get the canvas's delta time.
    #[must_use]
    fn delta_time(&self) -> f64;

    /// Get the canvas's frame count.
    #[must_use]
    fn frame_count(&self) -> u64;
}
