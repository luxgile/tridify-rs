use std::{
    collections::HashMap,
    error::Error,
    future::Future,
    time::{Duration, Instant},
};

use glam::UVec2;
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Features, Limits, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{
    event::{self, Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

use crate::{Color, Frame, Window, WindowView};

pub struct Nucley {
    windows: HashMap<WindowId, Window>,
    wb: Option<EventLoop<()>>,
    wgpu: wgpu::Instance,
}
impl Nucley {
    pub fn new() -> Self {
        Self {
            wgpu: wgpu::Instance::new(Backends::all()),
            wb: Some(EventLoop::new()),
            windows: HashMap::new(),
        }
    }

    pub fn has_windows(&self) -> bool { !self.windows.is_empty() }

    pub fn destroy_window(&mut self, wnd_id: &WindowId) { self.windows.remove(wnd_id); }

    pub fn create_window(&mut self) -> Result<&mut Window, Box<dyn Error>> {
        let wnd = winit::window::Window::new(self.wb.as_ref().unwrap())?;
        let wnd_id = wnd.id();
        let surface = unsafe { self.wgpu.create_surface(&wnd) };
        let adapter = pollster::block_on(self.wgpu.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .ok_or("Error requesting adapter.")?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_webgl2_defaults(),
            },
            None,
        ))?;
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: wnd.inner_size().width,
            height: wnd.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);
        let window = Window {
            user_loop: None,
            wnd: WindowView {
                wnd,
                adapter,
                device,
                queue,
                surface_config,
                surface,
            },
        };
        self.windows.insert(wnd_id, window);
        let window = self.windows.get_mut(&wnd_id).unwrap();
        Ok(window)
    }
    pub fn start(mut self) -> ! {
        let event_loop = self.wb.take().unwrap();
        event_loop.run(move |event, eloop, flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => match event {
                WindowEvent::CloseRequested => {
                    self.destroy_window(&window_id);
                    if !self.has_windows() {
                        *flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::Resized(size) => {
                    let wnd = self.get_window_mut(&window_id).unwrap();
                    wnd.wnd.resize(UVec2::new(size.width, size.height))
                }
                _ => {}
            },
            Event::RedrawRequested(id) => {
                let wnd = self.get_window_mut(&id).unwrap();
                wnd.update();
            }
            _ => {}
        });
    }

    pub fn get_window(&self, id: &WindowId) -> Result<&Window, &str> {
        self.windows.get(id).ok_or("No window found.")
    }
    pub fn get_window_mut(&mut self, id: &WindowId) -> Result<&mut Window, &str> {
        self.windows.get_mut(id).ok_or("No window found.")
    }
}
pub trait Graphics {
    fn get_adapter(&self) -> &Adapter;
    fn get_device(&self) -> &Device;
    fn get_queue(&self) -> &Queue;
    fn get_surface(&self) -> &Surface;
}
// impl Graphics {
//     pub fn new() -> Self {
//         env_logger::init();
//         let wgpu = wgpu::Instance::new(wgpu::Backends::all());
//         let surface = unsafe { wgpu.create_surface(&wnd) };
//         let surface
//         Self {
//             wgpu,
//             cached_surface: None,
//             adapter: None,
//             device: None,
//             queue: None,
//             format: None,
//         }
//     }

//     async fn init_internals(&mut self, surface: &Surface) -> Result<(), Box<dyn Error>> {
//         let adapter = self
//             .wgpu
//             .request_adapter(&RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::default(),
//                 force_fallback_adapter: false,
//                 compatible_surface: Some(&surface),
//             })
//             .await
//             .ok_or("Not able to request an adapter.")?;

//         let (device, queue) = adapter
//             .request_device(
//                 &DeviceDescriptor {
//                     label: None,
//                     features: Features::empty(),
//                     limits: Limits::downlevel_webgl2_defaults(),
//                 },
//                 None,
//             )
//             .await?;

//         self.format = Some(surface.get_supported_formats(&adapter)[0]);
//         self.adapter = Some(adapter);
//         self.device = Some(device);
//         self.queue = Some(queue);
//         Ok(())
//     }
// }
