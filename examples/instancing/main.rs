use std::{error::Error, path::Path};

use glam::{Mat4, Quat, Vec3};
use tridify_rs::{
    input_layout::{InputLayout, InputType},
    *,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Tridify::new();
    let window = app.create_window()?;
    let gpu_ctx = window.ctx();

    //Load texture from path.
    let texture = Texture::from_path(gpu_ctx, Path::new(r#"examples/instancing/texture.png"#));

    //Sampler defines how the texture will be rendered in shapes.
    let sampler = Sampler::new_default(gpu_ctx);

    let camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 50.0 + Vec3::Y * 20.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );
    let camera_buf = camera.build_buffer(gpu_ctx);

    //Create brush to draw the shapes.
    let mut brush = Brush::from_source(
        BrushDesc::default(),
        gpu_ctx,
        include_str!("shader.wgsl").to_string(),
    )?;

    let mut input_layout = InputLayout::new_vertex_standard();
    input_layout
        .group_per_instance()
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4);
    brush.set_input_layout(input_layout);

    //Bind camera, sampler and texture to the brush. Make sure group_index and loc_index are the same as
    //in the shader.
    brush.bind(0, 0, camera_buf);
    brush.bind(1, 0, texture);
    brush.bind(1, 1, sampler);

    //Create and bake a shape batch with a cube in it.
    let vertex_buffer = ShapeBatch::new()
        .add_cube(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE * 5., Color::WHITE)
        .bake_buffers(gpu_ctx);

    let mut instance_buffer = InstanceBuffer::new();
    for x in -4..5 {
        for z in -4..5 {
            instance_buffer.push_instance(
                Transform::from_pos(Vec3::new((x * 10) as f32, 0.0, (z * 10) as f32))
                    .build_matrix()
                    .to_cols_array(),
            )
        }
    }
    let instance_buffer_baked = instance_buffer.bake(gpu_ctx);
    let instance_buffer_len = instance_buffer.len() as u32;

    //Setup the window render loop.
    window.set_render_loop(move |gpu, _| {
        //Render frame as usual.
        let mut pass_builder = gpu.create_gpu_cmds();
        let mut render_pass = pass_builder.start_render_pass(RenderOptions::default());
        render_pass.render_shapes(
            gpu,
            &mut brush,
            &vertex_buffer,
            Some((instance_buffer_len, &instance_buffer_baked)),
        );
        render_pass.finish();
        pass_builder.complete(gpu);
    });

    //Start program logic cycle.
    app.start(());
}
