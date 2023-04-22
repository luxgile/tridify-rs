use std::{error::Error, path::Path};

use bytemuck::bytes_of;
use egui::{
    epaint::{
        ahash::{HashMap, HashMapExt, HashSet},
        ClippedShape, ImageDelta,
    },
    plot::Text,
    FullOutput, RawInput, TextureId, TexturesDelta,
};

use glam::{UVec2, Vec2, Vec4};
use tridify_rs::*;

pub struct EguiContext {}

impl EguiContext {
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Tridify::new();
    let wnd = app.create_window()?;
    let mut egui = EguiContext::new(wnd.ctx());

    let mut egui_demo = egui_demo_lib::DemoWindows::default();

    wnd.set_render_loop(move |wnd, _| {
        //Start egui frame.
        egui.start(wnd);

        egui_demo.ui(&egui.ctx);

        //Finish rendering UI and draw into the screen.
        egui.render(wnd);
    });

    app.start(());
}
