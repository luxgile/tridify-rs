use std::{error::Error, path::Path};

use bytemuck::bytes_of;
use egui::{
    epaint::{
        ahash::{HashMap, HashMapExt, HashSet},
        ClippedShape, ImageDelta,
    },
    plot::Text,
    FullOutput, RawInput, TextureId, TexturesDelta,
};

use glam::{UVec2, Vec2, Vec4};
use tridify_rs::*;

pub struct EguiContext {
    ctx: egui::Context,
    input: egui::RawInput,
    brush: Brush,
    draw_batches: Vec<(Rect, ShapeBuffer)>,
    textures: HashMap<TextureId, Texture>,
    screen_size_buffer: GpuBuffer,
}

impl EguiContext {
    pub fn new(gpu: &GpuCtx) -> Self {
        let brush_desc = BrushDesc {
            blend: wgpu::BlendState {
                alpha: AlphaBlend::Premultiplied.into(),
                color: AlphaBlend::Default.into(),
            },
        };
        let mut brush =
            Brush::from_source(brush_desc, gpu, include_str!("shader.wgsl").to_string())
                .expect("Error compiling egui brush");
        let binder = Sampler::new_default(gpu);
        //Create texture with only one white pixel
        let texture_desc = TextureDesc {
            usage: TextureUsage::TEXTURE_BIND | TextureUsage::DESTINATION,
            size: TextureSize::D2(UVec2::new(1, 1)),
        };
        let texture = Texture::init(gpu, texture_desc, bytes_of(&Color::WHITE.to_rgba8()), None);

        let screen_size = gpu.get_wnd_size().as_vec2().extend(0.).extend(0.);
        let screen_size_buffer = GpuBuffer::init(gpu, bytes_of(&screen_size.to_array()));
        brush.bind(0, 0, screen_size_buffer.clone());
        brush.bind(1, 0, texture);
        brush.bind(1, 1, binder);

        Self {
            brush,
            screen_size_buffer,
            draw_batches: Vec::new(),
            textures: HashMap::new(),
            ctx: egui::Context::default(),
            input: egui::RawInput::default(),
        }
    }

    pub fn ctx(&self) -> &egui::Context { &self.ctx }

    pub fn start(&mut self, gpu: &GpuCtx) {
        let wnd_size = gpu.get_wnd_size();
        self.screen_size_buffer.write(
            gpu,
            bytes_of(&wnd_size.as_vec2().extend(0.).extend(0.).to_array()),
        );
        self.input.screen_rect = Some(egui::Rect {
            min: egui::Pos2::ZERO,
            max: egui::pos2(wnd_size.x as f32, wnd_size.y as f32),
        });
        self.ctx.begin_frame(self.input.take());
        self.draw_batches.clear();
    }

    pub fn render(&mut self, gpu: &GpuCtx) {
        let output = self.ctx.end_frame();
        self.handle_shapes(gpu, output.shapes);
        self.set_textures(gpu, output.textures_delta.set);

        let mut pass_builder = gpu.create_render_builder();
        let mut pass = pass_builder.build_render_pass(RenderOptions {
            clear_color: Color::CLEAR,
        });

        if self.brush.needs_update() {
            self.brush.update(gpu);
        }

        for (scissor, buffer) in &self.draw_batches {
            pass.set_scissor(scissor);
            pass.render_shapes_cached(&self.brush, buffer);
        }

        pass.finish();
        pass_builder.finish_render(gpu);
        self.free_textures(output.textures_delta.free);
    }

    fn set_textures(&mut self, gpu: &GpuCtx, set_textures: Vec<(TextureId, ImageDelta)>) {
        for (id, image_delta) in set_textures {
            let origin = match image_delta.pos {
                Some(pos) => UVec2::new(pos[0] as u32, pos[1] as u32),
                None => UVec2::ZERO,
            }
            .extend(0);
            let image_size = image_delta.image.size();
            let texture = self.textures.entry(id).or_insert_with(|| {
                println!("Creating texture: {:?}", id);
                Texture::new(
                    gpu,
                    TextureDesc {
                        size: TextureSize::D2(UVec2::new(
                            image_size[0] as u32,
                            image_size[1] as u32,
                        )),
                        usage: TextureUsage::TEXTURE_BIND | TextureUsage::DESTINATION,
                    },
                    Some(format!("EGui texture [{:?}]", id).as_str()),
                )
            });
            match image_delta.image {
                egui::ImageData::Color(image) => {
                    let data: Vec<Color> = image.pixels.iter().map(|x| Color::from(*x)).collect();
                    texture.write_region_pixels(gpu, bytemuck::cast_slice(&data), origin);
                }
                egui::ImageData::Font(image) => {
                    let data: Vec<Color> = image.srgba_pixels(None).map(Color::from).collect();
                    texture.write_region_pixels(gpu, bytemuck::cast_slice(&data), origin);
                }
            };
        }
    }

    fn handle_shapes(&mut self, gpu: &GpuCtx, shapes: Vec<ClippedShape>) {
        let primitives = self.ctx.tessellate(shapes);
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
                .push((rect, shape_batch.bake_buffers(gpu)));
        }
    }

    fn free_textures(&mut self, textures: Vec<TextureId>) {
        for id in textures {
            self.textures.remove(&id);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let wnd = app.create_window()?;
    let mut egui = EguiContext::new(wnd.ctx());

    let mut egui_demo = egui_demo_lib::DemoWindows::default();

    wnd.set_render_loop(move |wnd, _| {
        //Start egui frame.
        egui.start(wnd);

        egui_demo.ui(&egui.ctx);

        //Finish rendering UI and draw into the screen.
        egui.render(wnd);
    });

    app.start(());
}
