mod egui_backend;

use std::iter;

use egui::FontDefinitions;
use wgpu::Error;
use winit::event::Event;

use crate::GpuCtx;

use self::egui_backend::*;

pub struct EguiContext {
    pub(crate) platform: Platform,
}

impl EguiContext {
    pub fn new(gpu: &GpuCtx) -> Self {
        let size = gpu.get_output_size();
        let scale_factor = match &gpu.output {
            crate::OutputSurface::Window(wnd) => wnd.winit_wnd.scale_factor(),
            crate::OutputSurface::Headless(_) => panic!("Egui not supported for headless mode."),
        };
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.x,
            physical_height: size.y,
            scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        Self { platform }
    }

    pub fn event(&mut self, event: &Event<()>) { self.platform.handle_event(event); }

    pub fn start(&mut self, dt: f64) {
        self.platform.begin_frame();
        self.platform.update_time(dt);
    }

    pub fn ctx(&self) -> egui::Context { self.platform.context() }
}

pub struct EguiPass {
    pub(crate) egui_rp: RenderPass,
}
impl EguiPass {
    pub fn new(gpu: &GpuCtx) -> Self {
        let egui_rp = RenderPass::new(&gpu.device, gpu.get_capabilities().formats[0], 1);
        Self { egui_rp }
    }

    pub fn render(&mut self, gpu: &mut GpuCtx) {
        let (output_frame, output_view) = gpu.get_output_frame();
        let mut egui = gpu.egui.as_mut().unwrap();
        let winit_wnd = match &gpu.output {
            crate::OutputSurface::Window(wnd) => &wnd.winit_wnd,
            crate::OutputSurface::Headless(_) => panic!("Egui not supported for headless mode."),
        };
        let full_output = egui.platform.end_frame(Some(winit_wnd));
        let paint_jobs = egui.platform.context().tessellate(full_output.shapes);

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        let size = gpu.get_output_size();
        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: size.x,
            physical_height: size.y,
            scale_factor: winit_wnd.scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        self.egui_rp
            .add_textures(&gpu.device, &gpu.queue, &tdelta)
            .expect("Error adding textures");
        self.egui_rp
            .update_buffers(&gpu.device, &gpu.queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        self.egui_rp
            .execute(
                &mut encoder,
                &output_view,
                &paint_jobs,
                &screen_descriptor,
                Some(wgpu::Color::BLACK),
            )
            .unwrap();
        // Submit the commands.
        gpu.queue.submit(iter::once(encoder.finish()));
        if let Some(output_frame) = output_frame {
            output_frame.present();
        }

        self.egui_rp
            .remove_textures(tdelta)
            .expect("Error removing textures");
    }
}
