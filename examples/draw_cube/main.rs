// use std::path::Path;

// use glium::{BackfaceCullingMode, Depth};
// use ldrawy::*;

// fn main() -> Result<(), LErr> {
//     DefaultWindow::create_and_run(WindowSettings::new(60), AppHandle::default())
// }

// // User defined window with data neccesary between frames.
// // In this case we cache the brush, batch, camera matrices and cube rotation.
// #[derive(Default)]
// struct AppHandle {
//     brush: Option<Brush>,
//     batch: Option<ShapeBatch>,
//     camera: Mat4,
//     projection: Mat4,
//     rotation: f32,
// }

// impl UserHandle<DefaultWindow> for AppHandle {
//     fn startup(&mut self, wnd: &mut DefaultWindow) -> Result<(), LErr> {
//         //Create a brush and add a uniform main texture
//         let mut brush = Brush::from_base_unlit(wnd);

//         let texture = Texture2D::new(wnd, Path::new("examples/draw_cube/UV_1k.jpg"));

//         brush
//             .add_uniform(Uniform::new(
//                 String::from("main_tex"),
//                 ldrawy::UniformValue::Texture2D(texture.texture, None),
//             ))
//             .expect("Failed to add uniform");

//         self.brush = Some(brush);

//         //Creating a batch and adding a cube to it.
//         let mut batch = ShapeBatch::default();
//         batch.add_cube(
//             Vec3::ZERO,
//             Quat::from_rotation_x(35.) * Quat::from_rotation_y(35.),
//             Vec3::ONE * 5.,
//             Color::WHITE,
//         );
//         self.batch = Some(batch);

//         //Add camera matrices to our window.
//         self.projection = Mat4::perspective_lh(f32::to_radians(90.), 16.0 / 9.0, 0.5, 100.);
//         self.camera = Mat4::look_at_lh(Vec3::Z * 10.0, Vec3::ZERO, Vec3::Y);

//         Ok(())
//     }

//     fn process_render(&mut self, wnd: &mut DefaultWindow) -> Result<(), LErr> {
//         //Start frame
//         let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

//         //Update rotation and calculate camera mvp.
//         self.rotation += wnd.delta_time() as f32;
//         let model = Mat4::from_rotation_y(self.rotation);
//         let mvp = self.projection * self.camera * model;

//         //Update the camera transforms in the shader
//         let brush = self.brush.as_mut().unwrap();
//         brush
//             .update_uniform(Uniform::new(
//                 String::from("mvp"),
//                 UniformValue::Matrix4(mvp.to_cols_array_2d()),
//             ))
//             .expect("Failed to change uniform");

//         //Draw the batch with specified drawing parameters.
//         let mut draw_params = DrawParams::new();
//         draw_params
//             .backface_culling(BackfaceCullingMode::CullCounterClockwise)
//             .depth_test(Depth {
//                 test: glium::DepthTest::IfMoreOrEqual,
//                 ..Default::default()
//             });
//         canvas.draw_batch(
//             wnd,
//             brush,
//             self.batch.as_ref().unwrap().bake_buffers(wnd),
//             &draw_params,
//         );

use glam::{Quat, Vec3};
use nucley::*;

//         //Finish drawing the frame
//         canvas.finish_canvas()?;
//         Ok(())
//     }
// }

use std::{cell::RefCell, error::Error, path::Path, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Nucley::new();
    let window = app.create_window()?;
    let window_view = window.view();

    //Load texture from path.
    let texture = AssetRef::new(Texture::from_path(
        window_view,
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\draw_cube\UV_1k.jpg"#),
    ));

    //Sampler defines how the texture will be rendered in shapes.
    let sampler = AssetRef::new(Sampler::new_default(window_view));

    let camera = Camera::new(
        Transform::from_look_at(Vec3::NEG_Z * 10.0, Vec3::ZERO, Vec3::Y),
        Projection::default(),
    );
    let camera_buf = AssetRef::new(camera.build_buffer(window_view));

    //Create brush to draw the shapes.
    let mut brush = Brush::from_path(
        window.view(),
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\basic.wgsl"#),
    )?;
    // Bind camera, sampler and texture to the brush. Make sure group_index and loc_index are the same as
    // in the shader.
    brush.bind(0, 0, camera_buf);
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

    //Setup the window render loop.
    window.run(move |wnd| {
        //TODO: Add binder into brush and fill binder every frame needed.
        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(wnd, &mut brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });

    // Start program.
    app.start(());
}
