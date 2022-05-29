use std::path::Path;

use glium::{BackfaceCullingMode, Depth};
use ldrawy::{
    self, Brush, Color, DrawParams, LErr, Mat4, Quat, ShapeBatch, Texture2D, Uniform, UniformValue,
    UserWindowHandler, Vec3, Window, WindowSettings,
};

struct MainWindow {
    brush: Option<Brush>,
    camera: Mat4,
    projection: Mat4,
    rotation: f64,
}
impl MainWindow {
    fn debug_framerate(&self, wnd: &Window) {
        println!(
            "Frame:{} - Delta:{:.4}s ({:.2} ms)",
            wnd.frame_count(),
            wnd.delta_time(),
            wnd.delta_time() * 1000.0
        );
    }
}
impl UserWindowHandler for MainWindow {
    fn startup(&mut self, wnd: &Window) -> Result<(), LErr> {
        //Create a brush and add a uniform main texture
        let mut brush = Brush::from_path(
            wnd,
            Path::new("examples/shared_assets/basic3d.vert"),
            Path::new("examples/shared_assets/basic3d.frag"),
            None,
        )?;

        let texture = Texture2D::new(wnd, Path::new("examples/draw_cube/UV_1k.jpg"));

        brush
            .add_uniform(Uniform::new(
                String::from("main_tex"),
                ldrawy::UniformValue::Texture2D(texture.texture, None),
            ))
            .expect("Failed to add uniform");

        self.projection = Mat4::perspective_lh(f32::to_radians(90.), 16.0 / 9.0, 0.5, 100.);
        self.camera = Mat4::look_at_lh(Vec3::Z * 10.0, Vec3::ZERO, Vec3::Y);

        self.brush = Option::Some(brush);
        Ok(())
    }
    fn cleanup(&mut self, _wnd: &Window) { println!("Cleaned process") }
    fn process_render(&mut self, wnd: &Window) -> Result<(), LErr> {
        self.debug_framerate(wnd);

        //Start frame
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

        let mut batch = ShapeBatch::default();
        let mut brush = self.brush.as_mut().unwrap();

        //Adding a cube to the batch
        batch.add_cube(
            Vec3::ZERO,
            Quat::from_rotation_y(self.rotation as f32),
            Vec3::ONE * 5.,
            Color::WHITE,
        );

        self.rotation += wnd.delta_time();

        let mvp = self.projection * self.camera;

        //Update the camera transforms in the shader
        brush
            .update_uniform(Uniform::new(
                String::from("mvp"),
                UniformValue::Matrix4(mvp.to_cols_array_2d()),
            ))
            .expect("Failed to change uniform");

        //Draw the batch
        let mut draw_params = DrawParams::new();
        draw_params
            .backface_culling(BackfaceCullingMode::CullCounterClockwise)
            .depth_test(Depth {
                test: glium::DepthTest::IfMoreOrEqual,
                ..Default::default()
            });
        canvas.draw_batch(wnd, brush, batch.bake_buffers(wnd), &draw_params);

        //Finish drawing the frame
        canvas.finish_canvas()?;
        Ok(())
    }
}

fn main() {
    let res = Window::create_and_run(
        WindowSettings::new(60),
        MainWindow {
            brush: None,
            camera: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            rotation: 0.0,
        },
    );
    if let Err(e) = res {
        println!("{:?}", e);
    }
}
