use std::{
    collections::HashMap,
    error::Error,
    time::{Duration, Instant},
};

use glam::UVec2;
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Features, InstanceDescriptor, Limits, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

use crate::{GpuCtx, RenderOptions, RenderPass, Texture, Window};

/// Represents basic information for a given windows rendering frame.
pub struct FrameContext<'a> {
    //event loop
    // pub user_ctx: &'a T,
    pub delta_time: f64,
    pub elapsed_time: f64,
    pub winit_event: &'a Event<'a, ()>,
    eloop: &'a EventLoopWindowTarget<()>,
}

/// Root struct which initializes WGPU, starts window management and handles application loop.
pub struct Tridify {
    windows: HashMap<WindowId, Window>,
    wb: Option<EventLoop<()>>,
    wgpu: wgpu::Instance,
}
impl Tridify {
    pub fn new() -> Self {
        // cfg_if::cfg_if! {
        //     if #[cfg(target_arch = "wasm32")] {
        //         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        //         console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        //     } else {
        //         env_logger::init();
        //     }
        // }

        Self {
            wgpu: wgpu::Instance::new(InstanceDescriptor::default()),
            wb: Some(EventLoop::new()),
            windows: HashMap::new(),
        }
    }

    pub fn has_windows(&self) -> bool { !self.windows.is_empty() }

    pub fn destroy_window(&mut self, wnd_id: &WindowId) { self.windows.remove(wnd_id); }

    pub fn create_window(&mut self) -> Result<&mut Window, Box<dyn Error>> {
        let wnd = winit::window::Window::new(self.wb.as_ref().unwrap())?;
        let wnd_id = wnd.id();
        let surface = unsafe {
            self.wgpu
                .create_surface(&wnd)
                .expect("Error creating window surface")
        };
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
            view_formats: vec![surface.get_capabilities(&adapter).formats[0]],
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: wnd.inner_size().width,
            height: wnd.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);

        // #[cfg(target_arch = "wasm32")]
        // {
        //     use winit::dpi::PhysicalSize;
        //     wnd.set_inner_size(PhysicalSize::new(450, 400));

        //     use winit::platform::web::WindowExtWebSys;
        //     web_sys::window()
        //         .and_then(|win| win.document())
        //         .and_then(|doc| {
        //             let dst = doc.get_element_by_id("wasm-example")?;
        //             let canvas = web_sys::Element::from(wnd.canvas());
        //             dst.append_child(&canvas).ok()?;
        //             Some(())
        //         })
        //         .expect("Couldn't append canvas to document body.");
        // }

        let window = Window {
            user_loop: None,
            ctx: GpuCtx {
                created_time: Instant::now(),
                last_draw_time: Instant::now(),
                winit_wnd: wnd,
                adapter,
                device,
                queue,
                surface_config,
                surface,

                #[cfg(feature = "egui")]
                egui: None,
            },
        };

        self.windows.insert(wnd_id, window);
        let window = self.windows.get_mut(&wnd_id).unwrap();
        Ok(window)
    }

    /// Begin application logic loop. Should be called last when initializing since this function
    /// can't never return.
    pub fn start<T: 'static>(mut self, user_ctx: T) -> ! {
        let event_loop = self.wb.take().unwrap();
        event_loop.run(move |event, eloop, flow| match event {
            Event::WindowEvent {
                event: ref wnd_event,
                window_id,
            } => {
                //Update egui if initilaized
                #[cfg(feature = "egui")]
                if let Ok(wnd) = self.get_window_mut(&window_id) {
                    if let Some(egui) = wnd.ctx.egui.as_mut() {
                        egui.event(&event);
                    }
                }

                match wnd_event {
                    WindowEvent::CloseRequested => {
                        self.destroy_window(&window_id);
                        if !self.has_windows() {
                            *flow = ControlFlow::Exit;
                        }
                    }
                    WindowEvent::Resized(size) => {
                        let wnd = self.get_window_mut(&window_id).unwrap();
                        wnd.ctx
                            .set_wnd_gpu_size(UVec2::new(size.width, size.height))
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                for (id, wnd) in self.windows.iter_mut() {
                    //TODO: User configurable
                    if wnd.ctx().last_draw_time.elapsed() >= Duration::from_millis(16.6 as u64) {
                        wnd.view_mut().redraw();
                        wnd.view_mut().last_draw_time = Instant::now();
                    }
                }
            }
            Event::RedrawRequested(id) => {
                let wnd = self.get_window_mut(&id).unwrap();
                let frame_ctx = FrameContext {
                    delta_time: wnd.ctx().last_draw_time.elapsed().as_secs_f64(),
                    elapsed_time: wnd.ctx().time_running().as_secs_f64(),
                    winit_event: &event,
                    // user_ctx: &user_ctx,
                    eloop,
                };
                wnd.render_step(&frame_ctx);
                wnd.view_mut().last_draw_time = Instant::now();
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
