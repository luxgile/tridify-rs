use std::time::{Duration, Instant};

use glam::UVec2;
use wgpu::{
    PresentMode, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureUsages,
    TextureViewDescriptor,
};
use winit::dpi::LogicalSize;

use crate::{GpuCommands, Texture, TextureDesc, TextureSize, TextureUsage, WgpuBuilder};

#[cfg(feature = "egui")]
use crate::EguiContext;
#[cfg(feature = "egui")]
use egui::Context;

pub struct WindowSurface {
    pub(crate) winit_wnd: winit::window::Window,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface,
}

pub enum OutputSurface {
    Window(WindowSurface),
    Headless(Texture),
}

impl OutputSurface {
    pub fn get_texture_view(&self) -> wgpu::TextureView {
        match &self {
            crate::OutputSurface::Window(wnd) => {
                let frame_texture = wnd
                    .surface
                    .get_current_texture()
                    .expect("Error getting frame.");

                frame_texture
                    .texture
                    .create_view(&TextureViewDescriptor::default())
            }
            crate::OutputSurface::Headless(tex) => tex.create_wgpu_view(),
        }
    }
}

/// Holds GPU context, devices, surfaces, etc. for a window. Must be used on most GPU related
/// functions.
pub struct GpuCtx {
    pub(crate) created_time: Instant,
    pub(crate) last_draw_time: Instant,
    pub(crate) output: OutputSurface,
    pub(crate) adapter: wgpu::Adapter,
    //TODO: Make private
    pub device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,

    #[cfg(feature = "egui")]
    pub(crate) egui: Option<EguiContext>,
}

const DEFAULT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

impl GpuCtx {
    pub fn from_wnd(wgpu: &wgpu::Instance, wnd: winit::window::Window) -> Self {
        let surface = unsafe {
            wgpu.create_surface(&wnd)
                .expect("Error creating window surface")
        };
        let (adapter, device, queue) = WgpuBuilder::build_context(wgpu, Some(&surface))
            .expect("Error creating WGPU context for window.");

        let surface_config = SurfaceConfiguration {
            view_formats: vec![surface.get_capabilities(&adapter).formats[0]],
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: wnd.inner_size().width,
            height: wnd.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        let output = OutputSurface::Window(crate::WindowSurface {
            winit_wnd: wnd,
            surface_config,
            surface,
        });

        Self {
            created_time: Instant::now(),
            last_draw_time: Instant::now(),
            output,
            adapter,
            device,
            queue,
            #[cfg(feature = "egui")]
            egui: None,
        }
    }

    pub fn from_texture(wgpu: &wgpu::Instance, size: TextureSize) -> Self {
        let (adapter, device, queue) = WgpuBuilder::build_context(wgpu, None)
            .expect("Error creating WGPU context for window.");
        let texture_desc = TextureDesc {
            size,
            usage: TextureUsage::RENDER | TextureUsage::SOURCE,
            format: DEFAULT_FORMAT,
        };
        let texture = Texture::new_internal(&device, texture_desc, None);
        let output = OutputSurface::Headless(texture);

        Self {
            created_time: Instant::now(),
            last_draw_time: Instant::now(),
            output,
            adapter,
            device,
            queue,
            #[cfg(feature = "egui")]
            egui: None,
        }
    }

    pub fn set_output_size(&mut self, size: UVec2) {
        match &mut self.output {
            OutputSurface::Window(wnd) => {
                wnd.winit_wnd
                    .set_inner_size(LogicalSize::new(size.x, size.y));
                self.set_output_surface_size(size);
                self.redraw();
            }
            OutputSurface::Headless(_) => {
                panic!("Cannot change size for headless output.");
            }
        }
    }
    pub(crate) fn set_output_surface_size(&mut self, size: UVec2) {
        match &mut self.output {
            OutputSurface::Window(wnd) => {
                wnd.surface_config.width = size.x.max(1);
                wnd.surface_config.height = size.y.max(1);
                wnd.surface.configure(&self.device, &wnd.surface_config);
                self.redraw();
            }
            OutputSurface::Headless(_) => {
                panic!("Cannot change size for headless output.");
            }
        };
    }

    pub fn get_output_size(&self) -> UVec2 {
        match &self.output {
            OutputSurface::Window(wnd) => {
                let size = wnd.winit_wnd.inner_size();
                UVec2::new(size.width, size.height)
            }
            OutputSurface::Headless(tex) => tex.desc.size.get_size().truncate(),
        }
    }

    pub fn get_output(&self) -> &OutputSurface { &self.output }

    /// Force the window to render again. Does nothing if headless.
    pub fn redraw(&self) {
        match &self.output {
            OutputSurface::Window(wnd) => wnd.winit_wnd.request_redraw(),
            OutputSurface::Headless(_) => {}
        }
    }

    pub fn get_capabilities(&self) -> SurfaceCapabilities {
        match &self.output {
            OutputSurface::Window(wnd) => wnd.surface.get_capabilities(&self.adapter),
            OutputSurface::Headless(tex) => SurfaceCapabilities {
                usages: tex.desc.usage.get_wgpu_usages(),
                present_modes: vec![PresentMode::AutoVsync],
                formats: vec![DEFAULT_FORMAT],
                alpha_modes: vec![wgpu::CompositeAlphaMode::Auto],
            },
        }
    }

    /// Create a new frame that will be drawn to.
    pub fn create_gpu_cmds(&self) -> GpuCommands {
        GpuCommands::from_gpu(self).expect(
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
