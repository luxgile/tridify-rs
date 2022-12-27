use std::path::Path;

use nucley::{vertex, Color, RenderOptions};

pub fn main() {
    nucley::start(|graphics, app, event_loop| async {
        let brush = Brush::default_lit();
        let batch = ShapeBatch::default();
        batch.add_triangle([
            vertex!(-0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.0, 0.5, 0.0, Color::SILVER),
            vertex!(0.5, -0.5, 0.0, Color::SILVER),
        ]);

        let wnd = app.create_window(graphics, event_loop, |wnd, graphics| {
            let frame = wnd
                .start_frame(&graphics, None)
                .expect("Issue creating frame.");
            frame.render(&brush, &batch);
            frame.finish(&graphics);
        });
    })
}
