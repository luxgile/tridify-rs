use std::error::Error;

use egui::RawInput;

use glam::Vec2;
use tridify_rs::*;

pub struct EguiContext {
    pub ctx: egui::Context,
    pub input: egui::RawInput,
    pub brush: Brush,
    pub shapes: ShapeBatch,
}

impl EguiContext {
    pub fn new() -> Self {
        let brush_desc = BrushDesc {
            blend: wgpu::BlendState {
                alpha: AlphaBlend::Premultiplied.into(),
                color: AlphaBlend::Default.into(),
            },
        };
        Self {
            shapes: ShapeBatch::new(),
            brush: todo!(),
            ctx: egui::Context::default(),
            input: egui::RawInput::default(),
        }
    }

    pub fn start(&mut self, graphics: &impl Graphics) {
        let wnd_size = graphics.get_screen_size();
        self.input.screen_rect = Some(egui::Rect {
            min: egui::pos2(0., 0.),
            max: egui::pos2(wnd_size.x as f32, wnd_size.y as f32),
        });
        self.ctx.begin_frame(self.input.take());
    }

    pub fn render(&mut self, graphics: &impl Graphics) {
        let output = self.ctx.end_frame();
        let meshes = self.ctx.tessellate(output.shapes);
        for mesh in meshes {
            let rect = mesh.clip_rect;
            let rect = Rect::from_min_max(
                Vec2::new(rect.min.x, rect.min.y),
                Vec2::new(rect.max.x, rect.max.y),
            );
            self.shapes.add_rect(&rect, Color::WHITE);
        }

        let pass = graphics
            .start_render_pass(RenderOptions {
                clear_color: Color::CLEAR,
            })
            .expect("Error creating Egui render pass.");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let wnd = app.create_window()?;

    let mut egui = EguiContext::new();
    egui.start(wnd.view());
    Ok(())
}
