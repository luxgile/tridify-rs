use std::path::Path;

use ldrawy::*;

fn main() -> Result<(), LErr> {
    Window::create_and_run(WindowSettings::new(60), MainWindow::default())
}

#[derive(Default)]
struct MainWindow {
    brush: Option<Brush>,
    batch: Option<ShapeBatch>,
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

        let texture = Texture2D::new(wnd, Path::new("examples/draw_rects/wood.png"));

        brush.add_uniform(Uniform::new(
            String::from("main_tex"),
            ldrawy::UniformValue::Texture2D(texture.texture, None),
        ))?;

        self.brush = Some(brush);

        //Create a batch, add 4 rectangles and add them to our window.
        let mut batch = ShapeBatch::default();
        batch.add_2d_square(Vec3::new(-0.5, -0.5, 0.0), 0.5, 0.5, Color::YELLOW);
        batch.add_2d_square(Vec3::new(0.5, -0.5, 0.0), 0.5, 0.5, Color::BLUE);
        batch.add_2d_square(Vec3::new(0.5, 0.5, 0.0), 0.5, 0.5, Color::GREEN);
        batch.add_2d_square(Vec3::new(-0.5, 0.5, 0.0), 0.5, 0.5, Color::RED);
        self.batch = Some(batch);

        Ok(())
    }
    fn process_render(&mut self, wnd: &Window) -> Result<(), LErr> {
        //Start frame
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

        //Draw batch using the brush created
        canvas.draw_batch(
            wnd,
            self.brush.as_ref().unwrap(),
            self.batch.as_ref().unwrap().bake_buffers(wnd),
            &DrawParams::default(),
        );

        //Finish drawing the frame
        canvas.finish_canvas()?;
        Ok(())
    }
}
