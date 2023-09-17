use std::error::Error;

use tridify_rs::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Tridify::new();
    let window = app.create_window()?;

    //Stores WGPU context and devices. Must be used in most GPU functions.
    let gpu_ctx = window.ctx();

    //Create brush to draw the shapes.
    let mut brush = Brush::from_source(
        BrushDesc::default(),
        gpu_ctx,
        include_str!("shader.wgsl").to_string(),
    )?;

    //Create a shape batch, add a triangle to it and create a GPU buffer with mesh data.
    let buffer = VertexBufferBuilder::new()
        .add_triangle([
            vertex!(-0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.0, 0.5, 0.0, Color::SILVER),
        ])
        .build_buffers(gpu_ctx);

    window.set_render_loop(move |gpu, _| {
        //Create a render pass builder which we will use to define multiple render passes (In this case, only one).
        let mut pass_builder = gpu.create_gpu_cmds();

        //Build a render pass which will take care of the brush and shapes to draw them and binding it with the GPU.
        let mut render_pass = pass_builder.start_render_pass(RenderOptions::default());
        render_pass.render_shapes(gpu, &mut brush, &buffer);
        render_pass.finish();

        //Execute all drawing commands from all render passes and render into screen.
        pass_builder.complete(gpu);
    });

    //Start program logic cycle.
    app.start(());
}
