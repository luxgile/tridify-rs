use std::{
    error::Error,
    time::{Duration, Instant},
};

use egui::Context;
use glam::UVec2;

use crate::{FrameContext, GpuCtx, RenderOptions, RenderPass, RenderPassBuilder};

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
