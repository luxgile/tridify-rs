use std::error::Error;

use glam::UVec2;
use tridify_rs::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let app = Tridify::new();
    let gpu_ctx = app.create_headless(TextureDesc {
        size: TextureSize::D2(UVec2::new(1920, 1080)),
        usage: TextureUsage::RENDER,
    });

    //Create brush to draw the shapes.
    let mut brush = Brush::from_source(
        BrushDesc::default(),
        &gpu_ctx,
        include_str!("shader.wgsl").to_string(),
    )?;

    //Create a shape batch, add a triangle to it and create a GPU buffer with mesh data.
    let buffer = ShapeBatch::new()
        .add_triangle([
            vertex!(-0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.0, 0.5, 0.0, Color::SILVER),
        ])
        .bake_buffers(&gpu_ctx);

    let mut pass_builder = gpu_ctx.create_render_builder();
    let mut render_pass = pass_builder.build_render_pass(RenderOptions::default());
    render_pass.render_shapes(&gpu_ctx, &mut brush, &buffer);
    //TODO: Paste gpu output into buffer
    render_pass.finish();
    pass_builder.finish_render(&gpu_ctx);

    Ok(())
}
