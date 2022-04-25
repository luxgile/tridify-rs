use ldrawy::{
    self,
    drawy::{Brush, Color, ShapeBatch, Uniform, UniformValue, UserWindowHandler, Vertex, Window},
    vertex,
};

struct MainWindow {
    brush: Option<Brush>,
    offset: f32,
}
impl UserWindowHandler for MainWindow {
    fn startup(&mut self, wnd: &Window) {
        let brush = Brush::new_basic(wnd)
            .add_uniform(Uniform::from_str("Offset", UniformValue::Float(0.0)));
        self.brush = Option::Some(brush);
    }
    fn cleanup(&mut self, _wnd: &Window) { println!("Cleaned process") }
    fn process_render(&mut self, wnd: &Window) {
        /*println!(
            "Frame:{} - Delta:{:.4}s ({:.2} ms)",
            wnd.frame_count(),
            wnd.delta_time(),
            wnd.delta_time() * 1000.0
        );
        */
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);
        let mut batch = ShapeBatch::default();
        let brush = self.brush.as_mut().unwrap();
        batch.add_square(vertex!(0.0, 0.0), 1.0, 1.0);
        self.offset += (wnd.delta_time() * 0.01) as f32;
        brush.change_uniform(Uniform::from_str(
            "Offset",
            UniformValue::Float(self.offset),
        ));
        canvas.draw_batch(wnd, self.brush.as_ref().unwrap(), batch.bake_buffers(wnd));

        if let Err(e) = canvas.finish_canvas() {
            println!("{}", e)
        }
    }
}

//TODO: For some reason framerate goes faster if you move the cursor
fn main() {
    Window::create_and_run(MainWindow {
        brush: None,
        offset: 0.0,
    });
}
