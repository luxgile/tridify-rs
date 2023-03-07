use std::error::Error;

use egui::RawInput;

use glam::{UVec2, Vec2};
use tridify_rs::*;

pub struct EguiContext {
    ctx: egui::Context,
    input: egui::RawInput,
    brush: Brush,
    draw_batches: Vec<(Rect, ShapeBuffer)>,
    texture: Option<Texture>,
}

impl EguiContext {
    pub fn new(wnd: &WindowCtx) -> Self {
        let brush_desc = BrushDesc {
            blend: wgpu::BlendState {
                alpha: AlphaBlend::Premultiplied.into(),
                color: AlphaBlend::Default.into(),
            },
        };
        let mut brush =
            Brush::from_source(brush_desc, wnd, include_str!("shader.wgsl").to_string())
                .expect("Error compiling egui brush");

        Self {
            brush,
            draw_batches: Vec::new(),
            texture: None,
            ctx: egui::Context::default(),
            input: egui::RawInput::default(),
        }
    }

    pub fn ctx(&self) -> &egui::Context { &self.ctx }

    pub fn start(&mut self, wnd: &WindowCtx) {
        let wnd_size = wnd.get_wnd_size();
        self.input.screen_rect = Some(egui::Rect {
            min: egui::pos2(0., 0.),
            max: egui::pos2(wnd_size.x as f32, wnd_size.y as f32),
        });
        self.ctx.begin_frame(self.input.take());
        self.draw_batches.clear();

        //TODO: Bind textures somehow??
        // if self.texture.is_none() {
        //     self.texture = Some()
        // }
    }

    pub fn render(&mut self, wnd: &WindowCtx) {
        let output = self.ctx.end_frame();
        let primitives = self.ctx.tessellate(output.shapes);
        for primitive in primitives {
            let rect = primitive.clip_rect;
            let rect = Rect::from_min_max(
                Vec2::new(rect.min.x, rect.min.y),
                Vec2::new(rect.max.x, rect.max.y),
            );

            let mut shape_batch = ShapeBatch::new();
            match primitive.primitive {
                egui::epaint::Primitive::Mesh(egui_mesh) => {
                    let vertices = egui_mesh
                        .vertices
                        .iter()
                        .map(|x| {
                            Vertex::new(
                                x.pos.x,
                                x.pos.y,
                                0.,
                                Some(Color::from(x.color)),
                                Some([x.uv.x, x.uv.y]),
                            )
                        })
                        .collect::<Vec<_>>();
                    let mesh = Mesh::new(vertices, egui_mesh.indices);
                    shape_batch.add_mesh(mesh);
                    //TODO: Setup textures
                }
                egui::epaint::Primitive::Callback(callback) => todo!(),
            }
            self.draw_batches
                .push((rect, shape_batch.bake_buffers(wnd)));
        }

        let mut pass_builder = wnd.create_render_builder();
        let mut pass = pass_builder.build_render_pass(RenderOptions {
            clear_color: Color::CLEAR,
        });

        if self.brush.needs_update() {
            self.brush.update(wnd);
        }

        for (scissor, buffer) in &self.draw_batches {
            pass.set_scissor(scissor);
            pass.render_shapes_cached(&self.brush, buffer);
        }

        pass.finish();
        pass_builder.finish_render(wnd);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let wnd = app.create_window()?;
    let mut egui = EguiContext::new(wnd.ctx());

    let mut name = "Arthur";
    let mut age = 42;

    wnd.set_render_loop(move |wnd, _| {
        //Start egui frame.
        egui.start(wnd);

        //Render UI using egui as usual.
        egui::CentralPanel::default().show(egui.ctx(), |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{}', age {}", name, age));
        });

        //Finish rendering UI and draw into the screen.
        egui.render(wnd);
    });

    app.start(());
}
