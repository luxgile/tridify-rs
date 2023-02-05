use std::{
    error::Error,
    time::{Duration, Instant},
};

use glam::UVec2;
use wgpu::{Adapter, Device, Queue};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use crate::{Graphics, Nucley};
use crate::{Color, Frame, RenderOptions};

pub struct Window {
    pub(crate) wnd: WindowView,
    pub(crate) user_loop: Option<Box<dyn FnMut(&mut WindowView)>>,
}
impl Window {
    pub fn update(&mut self) {
        if let Some(user_loop) = self.user_loop.as_mut() {
            user_loop.as_mut()(&mut self.wnd);
        }
    }
    pub fn run(&mut self, func: impl FnMut( &mut WindowView) + 'static) {
        self.user_loop = Some(Box::new(func));
    }

    pub fn view(&self) -> &WindowView { &self.wnd }
    pub fn view_mut(&mut self) -> &mut WindowView { &mut self.wnd }
}

pub struct WindowView {
    pub(crate) wnd: winit::window::Window,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}
impl Graphics for WindowView {
    fn get_adapter(&self) -> &Adapter { &self.adapter }
    fn get_device(&self) -> &Device { &self.device }
    fn get_queue(&self) -> &Queue { &self.queue }
    fn get_surface(&self) -> &wgpu::Surface { &self.surface }
}
impl WindowView {
    // pub fn update(&mut self, graphics: &mut Graphics) { self.user_loop.as_mut()(self, graphics); }

    pub fn resize(&mut self, size: UVec2) {
        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.wnd.request_redraw();
    }

    pub fn start_frame(&self, options: Option<RenderOptions>) -> Result<Frame, Box<dyn Error>> {
        Frame::new(self, options)
    }
}
