use std::{error::Error, path::Path};

use nucley::*;
pub fn main() -> Result<(), Box<dyn Error>> {
    //Create app and main window.
    let mut app = Tridify::new();
    let window = app.create_window()?;
    //Window view stores WGPU context and devices. Must be used in most GPU functions.
    let window_view = window.view();

    //Create brush to draw the shapes.
    let mut brush = Brush::from_path(
        window_view,
        Path::new(r#"D:\Development\Rust Crates\LDrawy\examples\shared_assets\basic.wgsl"#),
    )?;

    //Create a shape batch, add a triangle to it and create a GPU buffer with mesh data.
    let buffer = ShapeBatch::new()
        .add_triangle([
            vertex!(-0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.0, 0.5, 0.0, Color::SILVER),
        ])
        .bake_buffers(window_view)?;

    //Setup the window render loop.
    window.set_render_loop(move |wnd, _| {
        //Create a frame, render shapes with brush and finish it to draw into the screen.
        let mut frame = wnd.start_frame(None).expect("Issue creating frame.");
        frame.render(wnd, &mut brush, &buffer);
        frame.finish(wnd).expect("Error finishing frame.");
    });

    //Start program.
    app.start(());
}
