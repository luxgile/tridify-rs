use std::{
    collections::HashMap,
    error::Error,
    future::Future,
    time::{Duration, Instant}, cell::RefCell,
};

use env_logger::fmt::*;
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

use crate::{Color, Frame, Window, WindowView, Texture};

pub struct AppCtx<'a, T> {
    //event loop
    user_ctx: &'a T,
}
pub struct Nucley {
    windows: HashMap<WindowId, Window>,
    wb: Option<EventLoop<()>>,
    wgpu: wgpu::Instance,
    pub test: Option<RefCell<Texture>>,
}
impl Nucley {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook))
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        Self {
            wgpu: wgpu::Instance::new(Backends::all()),
            wb: Some(EventLoop::new()),
            windows: HashMap::new(),
            test: None
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

        #[cfg(target_arch = "wasm32")]
        {
            use winit::dpi::PhysicalSize;
            wnd.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(wnd.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

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
    pub fn start<T: 'static>(mut self, user_ctx: T) -> ! {
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
                let app_ctx = AppCtx {
                    user_ctx: &user_ctx,
                };
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
