use glam::{Mat4, Quat, Vec3};
use nucley::*;

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Nucley::new();
    let window = app.create_window()?;
    let window_view = window.view();

    //Load texture from path.
    let texture = Texture::from_path(
        window_view,
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\draw_cube\texture.png"#),
    );

    //Sampler defines how the texture will be rendered in shapes.
    let sampler = Sampler::new_default(window_view);

    let camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 10.0 + Vec3::Y * 10.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );
    let mut camera_buf = camera.build_buffer(window_view);

    //Create brush to draw the shapes.
    let mut brush = Brush::from_path(
        window.view(),
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\3d_basic.wgsl"#),
    )?;
    // Bind camera, sampler and texture to the brush. Make sure group_index and loc_index are the same as
    // in the shader.
    brush.bind(0, 0, camera_buf.clone());
    brush.bind(1, 0, texture);
    brush.bind(1, 1, sampler);

    //Create a shape batch and add a triangle to it.
    let mut batch = ShapeBatch::default();
    batch.add_cube(
        Vec3::ZERO,
        Quat::from_rotation_x(35.) * Quat::from_rotation_y(35.),
        Vec3::ONE * 5.,
        Color::WHITE,
    );
    //Bake batches into GPU buffers.
    let buffer = batch.bake_buffers(window.view())?;

    let mut model = Mat4::IDENTITY;
    let mut time_running = 0.;
    //Setup the window render loop.
    window.run(move |wnd| {
        //TODO: Implement actual delta time.
        //TODO: Simplify below code further.
        time_running += 0.16;
        model = Mat4::from_rotation_y(time_running);
        let mvp = camera.build_camera_matrix() * model;
        camera_buf.write(wnd, bytemuck::cast_slice(&mvp.to_cols_array()));
        brush.bind(0, 0, camera_buf.clone());

        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(wnd, &mut brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });

    // Start program.
    app.start(());
}
