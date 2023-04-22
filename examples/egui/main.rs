use std::error::Error;

use tridify_rs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let wnd = app.create_window()?;

    wnd.init_egui();
    let mut egui_demo = egui_demo_lib::DemoWindows::default();

    wnd.set_render_loop(move |gpu, frame_ctx| {
        // Start egui frame.
        gpu.egui().start();

        egui_demo.ui(&gpu.egui().ctx());

        //Finish rendering UI and draw into the screen.
        gpu.egui().render(gpu, frame_ctx.delta_time);
    });

    app.start(());
}
