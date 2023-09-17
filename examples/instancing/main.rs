use std::{error::Error, path::Path};

use glam::{Mat4, Quat, Vec3};
use tridify_rs::{
    input_layout::{InputLayout, InputType},
    *,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let window = app.create_window()?;
    let gpu_ctx = window.ctx();

    let texture = Texture::from_path(gpu_ctx, Path::new(r#"examples/instancing/texture.png"#));
    let sampler = Sampler::new_default(gpu_ctx);

    let camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 50.0 + Vec3::Y * 20.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );

    let mut brush = Brush::from_source(
        BrushDesc::default(),
        gpu_ctx,
        include_str!("shader.wgsl").to_string(),
    )?;
    brush.bind(0, 0, camera.build_buffer(gpu_ctx));
    brush.bind(1, 0, texture);
    brush.bind(1, 1, sampler);

    //We need to modify the standard input layout to include our instance data.
    //In this case to include a Matrix4x4 we need to add four times a Vec4.
    let mut input_layout = InputLayout::new_vertex_standard();
    input_layout
        .group_per_instance()
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4)
        .add_input(InputType::Vec4);
    brush.set_input_layout(input_layout);

    //Create and bake a shape batch with a cube in it.
    let vertex_buffer = VertexBufferBuilder::new()
        .add_cube(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE * 5., Color::WHITE)
        .build_buffers(gpu_ctx);

    //Create instance buffer. In this case we loop through a 10x10 grid to create 100 cubes.
    let mut instance_buffer_builder = InstanceBufferBuilder::new();
    for x in -4..5 {
        for z in -4..5 {
            let transform = Transform::from_pos(Vec3::new((x * 10) as f32, 0.0, (z * 10) as f32));

            //Instance buffer only accepts POD structs, so we need to convert the Transform into a byte array.
            instance_buffer_builder.push_instance(transform.build_matrix().to_cols_array())
        }
    }
    let instance_buffer = instance_buffer_builder.bake(gpu_ctx);

    //Setup the window render loop.
    window.set_render_loop(move |gpu, _| {
        //Render frame as usual.
        let mut pass_builder = gpu.create_gpu_cmds();
        let mut render_pass = pass_builder.start_render_pass(RenderOptions::default());
        render_pass.render_shapes(gpu, &mut brush, &vertex_buffer, Some(&instance_buffer));
        render_pass.finish();
        pass_builder.complete(gpu);
    });

    //Start program logic cycle.
    app.start(());
}
