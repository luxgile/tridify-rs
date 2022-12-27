mod core;
mod render;
pub use crate::core::*;
pub use render::*;

use std::future::Future;

use glam::UVec2;
use render::graphics::{App, Graphics};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};
pub fn start<F: Future>(
    func: impl FnOnce(&mut Graphics, &mut App, &EventLoopWindowTarget<()>) -> F,
) {
    pollster::block_on(internal_run(func));
}
async fn internal_run<F: Future>(
    func: impl FnOnce(&mut Graphics, &mut App, &EventLoopWindowTarget<()>) -> F,
) -> ! {
    let mut app = App::new();
    let mut graphics = Graphics::new();
    let event_loop = EventLoop::new();
    func(&mut graphics, &mut app, &event_loop).await;

    event_loop.run(move |event, eloop, flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => match event {
            WindowEvent::CloseRequested => {
                app.destroy_window(&window_id);
                if !app.has_windows() {
                    *flow = ControlFlow::Exit;
                }
            }
            WindowEvent::Resized(size) => {
                let wnd = app.get_window_mut(&window_id).unwrap();
                wnd.wnd
                    .resize(&mut graphics, UVec2::new(size.width, size.height))
            }
            _ => {}
        },
        Event::RedrawRequested(id) => {
            let wnd = app.get_window_mut(&id).unwrap();
            // wnd.update(&mut graphics);
        }
        _ => {}
    })
}
