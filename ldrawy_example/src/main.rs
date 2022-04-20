use ldrawy::{
    self,
    drawy::{Brush, Color, ShapeBatch, Vertex, Window},
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
        batch.add_triangle(
            Vertex::from_viewport(-0.5, -0.5),
            Vertex::from_viewport(0.0, 0.5),
            Vertex::from_viewport(0.5, -0.5),
        );
        let vertex_src = r#"
            #version 330 core
            in vec2 pos;
            void main() {
                gl_Position = vec4(pos, 0.0, 1.0);
            }
            "#;
        let fragment_src = r#"
            #version 330 core
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
            "#;
        let brush = Brush::from_source(wnd, vertex_src, fragment_src, None);
        canvas.draw_batch(wnd, &brush, batch.bake_buffers(wnd));

        if let Err(e) = canvas.finish_canvas() {
            println!("{}", e)
        }
    });
}
