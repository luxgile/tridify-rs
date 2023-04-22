use std::{
    error::Error,
    time::{Duration, Instant},
};

use egui::Context;
use glam::UVec2;

use winit::dpi::LogicalSize;

use crate::{FrameContext, RenderOptions, RenderPass, RenderPassBuilder};

#[cfg(feature = "egui")]
use crate::EguiContext;

/// Desktop window representation. Stores it's own GPU context and render loop.
pub struct Window {
    pub(crate) ctx: GpuCtx,
    pub(crate) user_loop: Option<Box<dyn FnMut(&mut GpuCtx, &FrameContext)>>,
}
impl Window {
    /// Step through render loop once.
    pub fn render_step(&mut self, frame_ctx: &FrameContext) {
        if let Some(user_loop) = self.user_loop.as_mut() {
            user_loop.as_mut()(&mut self.ctx, frame_ctx);
        }
    }

    /// Define closure that will be called each time the window is rendered
    pub fn set_render_loop(&mut self, func: impl FnMut(&mut GpuCtx, &FrameContext) + 'static) {
        self.user_loop = Some(Box::new(func));
    }

    pub fn ctx(&self) -> &GpuCtx { &self.ctx }
    pub fn view_mut(&mut self) -> &mut GpuCtx { &mut self.ctx }

    #[cfg(feature = "egui")]
    pub fn init_egui(&mut self) {
        if self.ctx.egui.is_some() {
            eprintln!("Egui has already been initialized.");
            return;
        }
        self.ctx.egui = Some(EguiContext::new(&self.ctx));
    }
}

/// Holds GPU context, devices, surfaces, etc. for a window. Must be used on most GPU related
/// functions.
pub struct GpuCtx {
    pub(crate) created_time: Instant,
    pub(crate) last_draw_time: Instant,

    pub(crate) winit_wnd: winit::window::Window,

    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,

    #[cfg(feature = "egui")]
    pub(crate) egui: Option<EguiContext>,
}

impl GpuCtx {
    pub fn set_wnd_size(&mut self, size: UVec2) {
        self.winit_wnd
            .set_inner_size(LogicalSize::new(size.x, size.y));
        self.set_wnd_gpu_size(size);
    }

    pub fn get_wnd_size(&self) -> UVec2 {
        let size = self.winit_wnd.inner_size();
        UVec2::new(size.width, size.height)
    }

    /// Change window GPU surface dimension.
    pub fn set_wnd_gpu_size(&mut self, size: UVec2) {
        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.redraw();
    }

    /// Force the window to render again.
    pub fn redraw(&self) { self.winit_wnd.request_redraw(); }

    /// Create a new frame that will be drawn to.
    pub fn create_render_builder(&self) -> RenderPassBuilder {
        RenderPassBuilder::new(self).expect(
            "Issue creating render pass builder. Make sure rendering cycle is being done properly.",
        )
    }

    /// Time the window has been running since its creation.
    pub fn time_running(&self) -> Duration { self.created_time.elapsed() }

    #[cfg(feature = "egui")]
    pub fn egui_ctx(&mut self) -> Context {
        self.egui
            .as_ref()
            .expect("Egui context not initialized.")
            .ctx()
    }

    #[cfg(feature = "egui")]
    pub fn egui_start(&mut self, dt: f64) {
        self.egui
            .as_mut()
            .expect("Egui context not initialized.")
            .start(dt);
    }
}
