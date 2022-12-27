use std::{
    collections::HashMap,
    error::Error,
    future::Future,
    time::{Duration, Instant},
};

use glam::UVec2;
use wgpu::{
    DeviceDescriptor, Features, Limits, RenderPipeline, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::{
    event_loop::{self, EventLoopWindowTarget},
    window::WindowId,
};

use crate::{Color, Frame, Window, WindowSurface};

pub struct App {
    windows: HashMap<WindowId, Window>,
}
impl App {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn has_windows(&self) -> bool { !self.windows.is_empty() }

    pub fn destroy_window(&mut self, wnd_id: &WindowId) { self.windows.remove(wnd_id); }

    pub async fn create_window(
        &mut self, graphics: &mut Graphics, event_loop: &EventLoopWindowTarget<()>,
        user_loop: impl Fn(&mut WindowSurface, &mut Graphics) + 'static,
    ) -> Result<&Window, Box<dyn Error>> {
        let wnd = winit::window::Window::new(&event_loop)?;
        let wnd_id = wnd.id();
        let surface = unsafe { graphics.wgpu.create_surface(&wnd) };
        if let None = graphics.device {
            graphics.init_internals(&surface).await;
        }
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(graphics.get_adapter()?)[0],
            width: wnd.inner_size().width,
            height: wnd.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(graphics.get_device()?, &surface_config);
        let window = Window {
            user_loop: Box::new(user_loop),
            wnd: WindowSurface {
                wnd,
                surface_config,
                surface,
            },
        };
        self.windows.insert(wnd_id, window);
        Ok(&self.windows[&wnd_id])
    }

    pub fn get_window(&self, id: &WindowId) -> Result<&Window, &str> {
        self.windows.get(id).ok_or("No window found.")
    }
    pub fn get_window_mut(&mut self, id: &WindowId) -> Result<&mut Window, &str> {
        self.windows.get_mut(id).ok_or("No window found.")
    }
}
pub struct Graphics {
    wgpu: wgpu::Instance,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    pipeline: Option<RenderPipeline>,
}
impl Graphics {
    pub fn new() -> Self {
        env_logger::init();
        let wgpu = wgpu::Instance::new(wgpu::Backends::all());
        Self {
            wgpu,
            adapter: None,
            device: None,
            queue: None,
            pipeline: None,
        }
    }

    async fn init_internals(&mut self, surface: &Surface) -> Result<(), Box<dyn Error>> {
        let adapter = self
            .wgpu
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or("Not able to request an adapter.")?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await?;

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        Ok(())
    }

    fn get_wait_time(&self, start: &Instant) -> Instant {
        let max_fps = 60;
        let elapsed_time = Instant::now().duration_since(*start).as_millis() as u64;
        let wait_time = if 1000 / max_fps as u64 >= elapsed_time {
            (1000 / max_fps as u64) - elapsed_time
        } else {
            0 //If this happens, it means the window is already running slower that it should.
        };
        *start + Duration::from_millis(wait_time)
    }

    pub fn get_queue(&self) -> Result<&wgpu::Queue, &'static str> {
        self.queue
            .as_ref()
            .ok_or("No queue found. Have you started the graphics properly?")
    }
    pub fn get_adapter(&self) -> Result<&wgpu::Adapter, &'static str> {
        self.adapter
            .as_ref()
            .ok_or("No adapter found. Have you started the graphics properly?")
    }
    pub fn get_device(&self) -> Result<&wgpu::Device, &'static str> {
        self.device
            .as_ref()
            .ok_or("No device found. Have you started the graphics properly?")
    }
}
