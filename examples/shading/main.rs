use glam::{Mat4, Quat, Vec3};
use tridify_rs::*;

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Tridify::new();
    let window = app.create_window()?;

    //Init egui for testing
    window.init_egui();

    let gpu_ctx = window.ctx();

    let mut egui_pass = EguiPass::new(gpu_ctx);

    //Load texture from path.
    let texture = Texture::from_path(gpu_ctx, Path::new(r#"examples/texture_cube/texture.png"#));

    //Sampler defines how the texture will be rendered in shapes.
    let sampler = Sampler::new_default(gpu_ctx);

    let mut camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 10.0 + Vec3::Y * 10.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );
    let mut camera_buf = camera.build_buffer(gpu_ctx);

    //Create brush to draw the shapes.
    let mut cube_brush = Brush::from_source(
        BrushDesc::default(),
        gpu_ctx,
        include_str!("shader.wgsl").to_string(),
    )?;
    //Bind camera, sampler and texture to the brush. Make sure group_index and loc_index are the same as
    //in the shader.
    cube_brush.bind(0, 0, camera_buf.clone());
    cube_brush.bind(1, 0, texture.clone());
    cube_brush.bind(1, 1, sampler.clone());

    //Create and bake a shape batch with a cube in it.
    let cube_shape_buffer = VertexBufferBuilder::new()
        .add_cube(
            Vec3::ZERO,
            Quat::from_rotation_x(35.) * Quat::from_rotation_y(35.),
            Vec3::ONE * 5.,
            Color::WHITE,
        )
        .build_buffers(gpu_ctx);

    let mut skybox_brush = Brush::from_source(
        BrushDesc::default(),
        gpu_ctx,
        include_str!("skybox.wgsl").to_string(),
    )?;
    skybox_brush.bind(0, 0, camera_buf.clone());
    skybox_brush.bind(1, 0, texture);
    skybox_brush.bind(1, 1, sampler);
    let skybox_shape_buffer = VertexBufferBuilder::new()
        .add_inv_cube(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE, Color::WHITE)
        .build_buffers(gpu_ctx);

    //Setup the window render loop.
    window.set_render_loop(move |gpu, frame_ctx| {
        let model = Mat4::from_rotation_y(frame_ctx.elapsed_time as f32);
        let cached_pos = camera.view.get_pos();

        //Render frame as usual.
        let mut pass_builder = gpu.create_gpu_cmds();
        let mut render_pass = pass_builder.start_render_pass(RenderOptions::default());

        //Render skybox
        camera.view.set_pos(Vec3::ZERO);
        let mvp = camera.build_camera_matrix() * model;
        camera_buf.write(gpu, bytemuck::cast_slice(&mvp.to_cols_array()));
        render_pass.render_shapes(gpu, &mut skybox_brush, &skybox_shape_buffer, None);

        camera.view.set_pos(cached_pos);
        let mvp = camera.build_camera_matrix() * model;

        //Updating the gpu buffer will update all brushes binded as well.
        camera_buf.write(gpu, bytemuck::cast_slice(&mvp.to_cols_array()));
        render_pass.render_shapes(gpu, &mut cube_brush, &cube_shape_buffer, None);

        gpu.egui_start(frame_ctx.delta_time);

        egui::CentralPanel::default().show(&gpu.egui_ctx(), |ui| {
            ui.heading("Testing");
        });

        egui_pass.render(gpu);

        render_pass.finish();
        pass_builder.complete(gpu);
    });

    // Start program.
    app.start(())
}
