mod core;
mod render;
pub use crate::core::*;
pub use render::*;

// pub fn start<F>(func: F)
// where
//     F: FnOnce(&mut Graphics, &mut Nucley, &EventLoopWindowTarget<()>),
// {
//     let mut app = Nucley::new();
//     let mut graphics = Graphics::new();
//     let event_loop = EventLoop::new();
//     func(&mut graphics, &mut app, &event_loop);

//     event_loop.run(move |event, eloop, flow| match event {
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } => match event {
//             WindowEvent::CloseRequested => {
//                 app.destroy_window(&window_id);
//                 if !app.has_windows() {
//                     *flow = ControlFlow::Exit;
//                 }
//             }
//             WindowEvent::Resized(size) => {
//                 let wnd = app.get_window_mut(&window_id).unwrap();
//                 wnd.wnd
//                     .resize(&mut graphics, UVec2::new(size.width, size.height))
//             }
//             _ => {}
//         },
//         Event::RedrawRequested(id) => {
//             let wnd = app.get_window_mut(&id).unwrap();
//             // wnd.update(&mut graphics);
//         }
//         _ => {}
//     })
// }
