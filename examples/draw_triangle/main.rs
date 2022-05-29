use std::path::Path;

use ldrawy::UserWindowHandler;

use ldrawy::*;

//Start up window.
pub fn main() -> Result<(), LErr> {
    Window::create_and_run(WindowSettings::new(60), UserWindow::default())
}

//User defined window with data neccesary between frames. In this case we cache the brush and batch
//to save performance.
#[derive(Default)]
pub struct UserWindow {
    brush: Option<Brush>,
    batch: Option<ShapeBatch>,
}

impl UserWindowHandler for UserWindow {
    //Called once when window is created.
    fn startup(&mut self, wnd: &Window) -> Result<(), LErr> {
        //Brush define how shapes are drawn. Here we are creating one based in the shaders found below.
        self.brush = Some(
            Brush::from_path(
                wnd,
                Path::new("examples/shared_assets/basic.vert"),
                Path::new("examples/shared_assets/basic.frag"),
                None,
            )
            .expect("Failed to create brush"),
        );
        //Batches are a collection of shapes that will be drawn. These need to be baked once the frame will be drawn.
        let mut batch = ShapeBatch::default();

        //Add triangle passing vertices and color.
        batch.add_triangle([
            vertex!(-0.5, -0.5, 0.0, Color::SILVER),
            vertex!(0.0, 0.5, 0.0, Color::SILVER),
            vertex!(0.5, -0.5, 0.0, Color::SILVER),
        ]);
        self.batch = Some(batch);
        Ok(())
    }

    //Called all frames
    fn process_render(&mut self, wnd: &Window) -> Result<(), LErr> {
        //Obtain canvas, where the frame will be drawn.
        let mut canvas = wnd.start_frame(Color::BLUE_TEAL);

        //Draw the batch cached into the canvas.
        canvas.draw_batch(
            wnd,
            self.brush.as_ref().unwrap(),
            self.batch.as_ref().unwrap().bake_buffers(wnd),
            &DrawParams::default(),
        );

        //Make sure you finish the canvas at the end of rendering.
        canvas.finish_canvas()?;
        Ok(())
    }
}
