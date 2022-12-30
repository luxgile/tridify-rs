use std::{error::Error, path::Path};

use nucley::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Nucley::new();
    let window = app.create_window()?;

    //Create brush to draw the shapes.
    let brush = Brush::from_path(
        window.view(),
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\basic.wgsl"#),
    )?;

    //Create a shape batch and add a triangle to it.
    let mut batch = ShapeBatch::default();
    batch.add_triangle([
        vertex!(-0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.5, -0.5, 0.0, Color::SILVER),
        vertex!(0.0, 0.5, 0.0, Color::SILVER),
    ]);

    //Bake batches into GPU buffers.
    let buffer = batch.bake_buffers(window.view())?;

    //Setup the window render loop.
    window.run(move |wnd| {
        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(&brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });

    //Start program.
    app.start();
}
