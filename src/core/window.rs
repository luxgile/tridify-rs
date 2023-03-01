use std::{
    error::Error,
    time::{Duration, Instant},
};

use glam::UVec2;
use wgpu::{Adapter, Device, Queue};

use crate::Graphics;
use crate::{Frame, FrameContext, RenderOptions};

/// Desktop window representation. Stores it's own GPU context and render loop.
pub struct Window {
    pub(crate) created_time: Instant,
    pub(crate) last_draw_time: Instant,
    pub(crate) wnd: WindowView,
    pub(crate) user_loop: Option<Box<dyn FnMut(&mut WindowView, &FrameContext)>>,
}
impl Window {
    /// Step through render loop once.
    pub fn render_step(&mut self, frame_ctx: &FrameContext) {
        if let Some(user_loop) = self.user_loop.as_mut() {
            user_loop.as_mut()(&mut self.wnd, frame_ctx);
        }
    }

    /// Define closure that will be called each time the window is rendered
    pub fn set_render_loop(&mut self, func: impl FnMut(&mut WindowView, &FrameContext) + 'static) {
        self.user_loop = Some(Box::new(func));
    }

    /// Force the window to render again.
    pub fn redraw(&self) { self.wnd.redraw(); }

    pub fn view(&self) -> &WindowView { &self.wnd }
    pub fn view_mut(&mut self) -> &mut WindowView { &mut self.wnd }

    /// Time the window has been running since its creation.
    pub fn time_running(&self) -> Duration { self.created_time.elapsed() }
}

/// Holds GPU context, devices, surfaces, etc. for a window. Must be used on most GPU related
/// functions.
pub struct WindowView {
    pub(crate) winit_wnd: winit::window::Window,
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
    /// Change window GPU surface size.
    pub fn resize(&mut self, size: UVec2) {
        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.redraw();
    }

    /// Force the window to render again.
    pub fn redraw(&self) { self.winit_wnd.request_redraw(); }

    /// Create a new frame that will be drawn to.
    pub fn start_frame(&self, options: Option<RenderOptions>) -> Result<Frame, Box<dyn Error>> {
        Frame::new(self, options)
    }
}
