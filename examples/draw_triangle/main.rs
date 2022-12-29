use std::{error::Error, path::Path};

use nucley::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Nucley::new();
    let mut window = app.create_window()?;
    let brush = Brush::from_path(
        window.view(),
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\basic.wgsl"#),
    )?;
    let mut batch = ShapeBatch::default();
    batch.add_triangle([
        vertex!(-0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.0, 0.5, 0.0, Color::SILVER),
    ]);
    let buffer = batch.bake_buffers(window.view())?;

    window.run(move |wnd| {
        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(&brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });
    app.start();
}
