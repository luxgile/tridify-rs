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

<<<<<<< HEAD
=======
use std::{error::Error, path::Path};

use nucley::*;
>>>>>>> 1e7ce10f09c6e40bc248f618e263f9a6a160a076

//         //Finish drawing the frame
//         canvas.finish_canvas()?;
//         Ok(())
//     }
// }
<<<<<<< HEAD


use std::{path::Path, error::Error, cell::RefCell};

use nucley::*;

=======
>>>>>>> 1e7ce10f09c6e40bc248f618e263f9a6a160a076
fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Nucley::new();
    let window = app.create_window()?;
    let window_view = window.view();
    let mut db = Database::<Texture>::new();

<<<<<<< HEAD
    //Load texture from path.
    let texture = Texture::from_path(window_view, Path::new("examples/draw_cube/UV_1k.jpg"));
    let texture_ref = db.add_asset(texture);

    //Binder holds textures, matrices and other uniforms.
    let mut binder = Binder::new();
    binder.set_bind(0, &BinderPart::Texture(texture_ref.clone()));
    binder.set_bind(1, &BinderPart::Sampler);
=======
    let texture = Texture::from_path(window.view(), Path::new("examples/draw_cube/UV_1k.jpg"));
    let binder = Binder::new(window.view());
>>>>>>> 1e7ce10f09c6e40bc248f618e263f9a6a160a076

    //Create brush to draw the shapes.
    let mut brush = Brush::from_path(
        window.view(),
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\basic.wgsl"#),
    )?;
    // Add binder information to the brush
    brush.set_binder(0, binder);

    //Create a shape batch and add a triangle to it.
    let mut batch = ShapeBatch::default();
    batch.add_triangle([
        vertex!(-0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.0, 0.5, 0.0, Color::SILVER),
    ]);

    //Bake batches into GPU buffers.
    let buffer = batch.bake_buffers(window.view())?;

    //Setup the window render loop.
    // TODO: Add app to closure
    window.run(move |wnd| {
        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(wnd, &mut brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });

    // Start program.
    app.start();
}
