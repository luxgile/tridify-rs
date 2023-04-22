mod egui_backend;

use std::iter;

use egui::FontDefinitions;
use wgpu::Error;
use winit::event::Event;

use crate::GpuCtx;

use self::egui_backend::*;

pub struct EguiContext {
    pub(crate) platform: Platform,
    pub(crate) egui_rp: RenderPass,
}

impl EguiContext {
    pub fn new(gpu: &GpuCtx) -> Self {
        let size = gpu.get_wnd_size();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.x,
            physical_height: size.y,
            scale_factor: gpu.winit_wnd.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        let egui_rp = RenderPass::new(
            &gpu.device,
            gpu.surface.get_capabilities(&gpu.adapter).formats[0],
            1,
        );

        Self { platform, egui_rp }
    }

    pub fn event(&mut self, event: &Event<()>) { self.platform.handle_event(event); }

    pub fn start(&mut self) { self.platform.begin_frame(); }

    pub fn render(&mut self, gpu: &GpuCtx, dt: f64) {
        self.platform.update_time(dt);

        let output_frame = match gpu.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => {
                // This error occurs when the app is minimized on Windows.
                // Silently return here to prevent spamming the console with:
                // "The underlying surface has changed, and therefore the swap chain must be updated"
                return;
            }
            Err(e) => {
                eprintln!("Dropped frame with error: {}", e);
                return;
            }
        };
        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let full_output = self.platform.end_frame(Some(&gpu.winit_wnd));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        let size = gpu.get_wnd_size();
        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: size.x,
            physical_height: size.y,
            scale_factor: gpu.winit_wnd.scale_factor() as f32,
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
        output_frame.present();

        self.egui_rp
            .remove_textures(tdelta)
            .expect("Error removing textures");
    }

    pub fn ctx(&self) -> egui::Context { self.platform.context() }
}
