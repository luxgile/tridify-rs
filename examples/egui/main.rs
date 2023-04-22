use std::error::Error;

use tridify_rs::*;

fn main() -> Result<(), Box<dyn Error>> {
    //Create app and window
    let mut app = Tridify::new();
    let wnd = app.create_window()?;

    //Opt egui in for this window
    wnd.init_egui();

    //Egui pass will take care of processing egui outputs and drawing them
    let mut egui_pass = EguiPass::new(wnd.ctx());

    //Egui official demo for testing purposes
    let mut egui_demo = egui_demo_lib::DemoWindows::default();

    wnd.set_render_loop(move |gpu, frame_ctx| {
        //Start egui frame
        gpu.egui_start(frame_ctx.delta_time);

        //Here start to draw egui however you'd like
        egui_demo.ui(&gpu.egui_ctx());

        //Process egui outputs and draw them into the screen
        egui_pass.render(gpu);
    });

    app.start(());
}
