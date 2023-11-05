use glam::{Mat4, Quat, Vec3};
use tridify_rs::*;

use std::{error::Error, path::Path};

const ROTATION_SPEED: f32 = 180.0;

fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Tridify::new();
    let window = app.create_window()?;

    //Init egui for testing
    // window.init_egui();

    let gpu_ctx = window.ctx();

    //Load texture from path.
    let texture = Texture::from_path(gpu_ctx, Path::new("examples/res/texture.png"));

    //Sampler defines how the texture will be rendered in shapes.
    let sampler = Sampler::new_default(gpu_ctx);

    let mut camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 10.0 + Vec3::Y * 10.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );

    //Create and bake a shape batch with a cube in it.
    let mut model = PbrModel::from_path(gpu_ctx, Path::new("examples/res/sphere.obj"));
    model.transform.set_scale(Vec3::ONE * 3.0);
    model.get_main_palette().set_diffuse(texture.clone());
    model.check(gpu_ctx);

    let mut skybox = Skybox::new(gpu_ctx);
    skybox.palette.set_diffuse_texture(texture);
    skybox.palette.check(gpu_ctx);

    //Setup the window render loop.
    window.set_render_loop(move |gpu, frame_ctx| {
        //Render frame as usual.
        let mut pass_builder = gpu.create_gpu_cmds();
        let mut render_pass = pass_builder.start_render_pass(RenderOptions::default());

        //Render skybox
        skybox.palette.update_camera(gpu, &camera);
        render_pass.render(&skybox);

        //Render cube
        model.transform.rotate(Quat::from_rotation_z(
            ROTATION_SPEED * frame_ctx.delta_time as f32,
        ));
        model.update_camera(gpu, &camera);
        render_pass.render(&model);
        // let model = Mat4::from_rotation_y(frame_ctx.elapsed_time as f32 * 0.25);
        // let mvp = camera.build_camera_matrix() * model;
        // camera_buf.write(gpu, bytemuck::cast_slice(&mvp.to_cols_array()));
        // render_pass.render_raw(&mut cube_brush, &cube_shape_buffer, None);
        render_pass.finish();

        pass_builder.complete(gpu);
    });

    // Start program.
    app.start(())
}
