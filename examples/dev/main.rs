use std::path::Path;

use ldrawy::{
    self, vertex, Brush, Color, ShapeBatch, Texture2D, Uniform, UserWindowHandler, Window,
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
    fn startup(&mut self, wnd: &Window) {
        //Create a brush and add a uniform main texture
        let mut brush = Brush::new_basic(wnd);
        let texture = Texture2D::new(wnd, Path::new("examples/dev/assets/wood.png"));
        brush.add_uniform(Uniform::new(
            String::from("main_tex"),
            ldrawy::UniformValue::Texture2D(texture.texture, None),
        ));
        self.brush = Option::Some(brush);
    }
    fn cleanup(&mut self, _wnd: &Window) { println!("Cleaned process") }
    fn process_render(&mut self, wnd: &Window) {
        self.debug_framerate(wnd);

        //Start frame
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

        //Create batch and add shapes that will be drawn
        let mut batch = ShapeBatch::default();
        batch.add_square(vertex!(-0.5, -0.5, Color::YELLOW), 0.5, 0.5);
        batch.add_square(vertex!(0.5, -0.5, Color::BLUE), 0.5, 0.5);
        batch.add_square(vertex!(0.5, 0.5, Color::GREEN), 0.5, 0.5);
        batch.add_square(vertex!(-0.5, 0.5, Color::RED), 0.5, 0.5);

        //Draw batch using the brush created
        canvas.draw_batch(wnd, self.brush.as_ref().unwrap(), batch.bake_buffers(wnd));

        //Finish drawing the frame
        if let Err(e) = canvas.finish_canvas() {
            println!("{}", e)
        }
    }
}

fn main() { Window::create_and_run(MainWindow { brush: None }); }
