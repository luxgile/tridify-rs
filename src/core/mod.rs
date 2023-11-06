mod camera;
mod color;
mod input;
mod input_layout;
mod math;
mod transform;
mod tridify;
mod window;

pub use camera::*;
pub use color::*;
pub use input::*;
pub use input_layout::*;
pub use math::*;
pub use transform::*;
pub use tridify::*;
pub use window::*;

use crate::GpuCtx;

pub trait Asset {
    fn needs_update(&self) -> bool { true }
    fn update(&mut self, gpu: &GpuCtx);
    fn check(&mut self, gpu: &GpuCtx) {
        if self.needs_update() {
            self.update(gpu);
        }
    }
}
