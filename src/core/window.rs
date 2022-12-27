use std::{
    error::Error,
    time::{Duration, Instant},
};

use glam::UVec2;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::{graphics::Graphics, Color, Frame, RenderOptions};

pub struct Window {
    pub(crate) wnd: WindowSurface,
    pub(crate) user_loop: Box<dyn Fn(&mut WindowSurface, &mut Graphics)>,
}
impl Window {
    pub fn update(&mut self, graphics: &mut Graphics) {
        self.user_loop.as_ref()(&mut self.wnd, graphics);
    }
}

pub struct WindowSurface {
    pub(crate) wnd: winit::window::Window,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface,
}
impl WindowSurface {
    // pub fn update(&mut self, graphics: &mut Graphics) { self.user_loop.as_mut()(self, graphics); }

    pub fn resize(&mut self, graphics: &Graphics, size: UVec2) {
        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);
        self.surface
            .configure(&graphics.get_device().unwrap(), &self.surface_config);
        self.wnd.request_redraw();
    }

    pub fn start_frame(
        &self, graphics: &Graphics, options: Option<RenderOptions>,
    ) -> Result<Frame, Box<dyn Error>> {
        Frame::new(self, graphics, options)
    }
}
