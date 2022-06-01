use std::path::Path;

use ldrawy::{
    self, Brush, Color, DrawParams, LErr, ShapeBatch, Texture2D, Uniform, UserWindowHandler, Vec3,
    Window, WindowSettings,
};

struct MainWindow {
    brush: Option<Brush>,
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
            Path::new("examples/shared_assets/basic.vert"),
            Path::new("examples/shared_assets/basic.frag"),
            None,
        )
        .expect("Failed to create brush");

        let texture = Texture2D::new(wnd, Path::new("examples/draw_rects/assets/wood.png"));

        brush.add_uniform(Uniform::new(
            String::from("main_tex"),
            ldrawy::UniformValue::Texture2D(texture.texture, None),
        ))?;

        self.brush = Option::Some(brush);
        Ok(())
    }
    fn cleanup(&mut self, _wnd: &Window) { println!("Cleaned process") }
    fn process_render(&mut self, wnd: &Window) -> Result<(), LErr> {
        self.debug_framerate(wnd);

        //Start frame
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

        //Create batch and add shapes that will be drawn
        let mut batch = ShapeBatch::default();
        batch.add_2d_square(Vec3::new(-0.5, -0.5, 0.0), 0.5, 0.5, Color::YELLOW);
        batch.add_2d_square(Vec3::new(0.5, -0.5, 0.0), 0.5, 0.5, Color::BLUE);
        batch.add_2d_square(Vec3::new(0.5, 0.5, 0.0), 0.5, 0.5, Color::GREEN);
        batch.add_2d_square(Vec3::new(-0.5, 0.5, 0.0), 0.5, 0.5, Color::RED);

        //Draw batch using the brush created
        canvas.draw_batch(
            wnd,
            self.brush.as_ref().unwrap(),
            batch.bake_buffers(wnd),
            &DrawParams::default(),
        );

        //Finish drawing the frame
        canvas.finish_canvas()?;
        Ok(())
    }
}

fn main() -> Result<(), LErr> {
    Window::create_and_run(WindowSettings::new(60), MainWindow { brush: None })
}
