pub mod window_impl;
pub use window_impl::*;

pub mod default_window;
pub use default_window::*;

pub mod user_impl;
pub use user_impl::*;

///Basic settings to create a window.
#[derive(Debug, Clone)]
pub struct WindowSettings {
    max_fps: u64,
}

impl WindowSettings {
    pub fn new(max_fps: u64) -> Self { Self { max_fps } }
}
