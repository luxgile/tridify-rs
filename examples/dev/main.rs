use ldrawy::{
    self,
    drawy::{Brush, Color, ShapeBatch, Vertex, Window},
    vertex,
};

fn main() {
    Window::create_and_run(|wnd| {
        println!(
            "Frame:{} - Delta:{:.4}s ({:.2} ms)",
            wnd.frame_count(),
            wnd.delta_time(),
            wnd.delta_time() * 1000.0
        );
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);
        let mut batch = ShapeBatch::default();
        batch.add_square(vertex!(0.0, 0.0), 1.0, 1.0);
        let brush = Brush::new_basic(wnd);
        canvas.draw_batch(wnd, &brush, batch.bake_buffers(wnd));

        if let Err(e) = canvas.finish_canvas() {
            println!("{}", e)
        }
    });
}
