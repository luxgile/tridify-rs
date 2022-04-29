use ldrawy::{self, vertex, Brush, Color, ShapeBatch, UserWindowHandler, Vertex, Window};

struct MainWindow {
    brush: Option<Brush>,
    offset: f32,
}
impl UserWindowHandler for MainWindow {
    fn startup(&mut self, wnd: &Window) {
        let brush = Brush::new_basic(wnd);
        self.brush = Option::Some(brush);
    }
    fn cleanup(&mut self, _wnd: &Window) { println!("Cleaned process") }
    fn process_render(&mut self, wnd: &Window) {
        println!(
            "Frame:{} - Delta:{:.4}s ({:.2} ms)",
            wnd.frame_count(),
            wnd.delta_time(),
            wnd.delta_time() * 1000.0
        );

        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);
        let mut batch = ShapeBatch::default();
        batch.add_square(vertex!(-0.5, -0.5, Color::YELLOW), 0.5, 0.5);
        batch.add_square(vertex!(0.5, -0.5, Color::BLUE), 0.5, 0.5);
        batch.add_square(vertex!(0.5, 0.5, Color::GREEN), 0.5, 0.5);
        batch.add_square(vertex!(-0.5, 0.5, Color::RED), 0.5, 0.5);
        self.offset += (wnd.delta_time() * 0.1) as f32;
        canvas.draw_batch(wnd, self.brush.as_ref().unwrap(), batch.bake_buffers(wnd));

        if let Err(e) = canvas.finish_canvas() {
            println!("{}", e)
        }
    }
}

fn main() {
    Window::create_and_run(MainWindow {
        brush: None,
        offset: 0.0,
    });
}
