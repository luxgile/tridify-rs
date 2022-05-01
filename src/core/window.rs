use std::time::{Duration, Instant};

use glium::{
    glutin::{
        self,
        event::{Event, VirtualKeyCode},
        event_loop::ControlFlow,
    },
    Display,
};

use crate::render::Canvas;

use super::Color;

/// Manages window lifetime events from the user side.
/// Needs to be implemented in user defined struct and sent to LDrawy to start drawing a window.
pub trait UserWindowHandler {
    fn startup(&mut self, _wnd: &Window) {
    }

    fn process_logic(&mut self) {
    }

    fn process_render(&mut self, _wnd: &Window) {
    }

    fn cleanup(&mut self, _wnd: &Window) {
    }
}

///Basic settings to create a window.
pub struct WindowSettings {
    max_fps: u64,
}

impl WindowSettings {
    pub fn new(max_fps: u64) -> Self { Self { max_fps } }
}

/// Internal representation of a window with direct access to OpenGL context. Needs to be used in most drawing and GPU
/// related functions.
pub struct Window {
    settings: WindowSettings,
    display: Display,
    delta_time: f64,
    frame_count: u64,
}

impl Window {
    /// Create and start the window run loop, using the settings and user handler provided.
    pub fn create_and_run(settings: WindowSettings, mut user: impl UserWindowHandler + 'static) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut window = Window {
            settings,
            display,
            delta_time: 0.0,
            frame_count: 0,
        };

        user.startup(&window);

        event_loop.run(move |ev, _, flow| {
            let start_time = Instant::now();

            if Self::manage_events(&ev, flow) {
                if let ControlFlow::Exit = flow {
                    user.cleanup(&window);
                }
                return;
            }

            window.frame_count += 1;
            user.process_render(&window);

            //Limit framerate
            let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;
            let wait_time = match window.settings.max_fps > 0
                && 1000 / window.settings.max_fps >= elapsed_time
            {
                true => 1000 / window.settings.max_fps - elapsed_time,
                false => 0,
            };
            window.delta_time = wait_time as f64 / 1000.0;

            let wait_instant = start_time + Duration::from_millis(wait_time);
            *flow = glutin::event_loop::ControlFlow::WaitUntil(wait_instant);
        });
    }

    ///Get Canvas to start drawing in the back buffer.
    pub fn start_frame(&self, color: Color) -> Canvas {
        let mut canvas = Canvas::new(self.display.draw());
        canvas.clear_color(color);
        canvas
    }

    ///Manages the current event and changes the flow if needed. Returns if a frame needs to be waited.
    fn manage_events(ev: &Event<()>, flow: &mut ControlFlow) -> bool {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *flow = ControlFlow::Exit;
                    return true;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. }
                    if input.virtual_keycode.unwrap() == VirtualKeyCode::Escape =>
                {
                    *flow = ControlFlow::Exit;
                    return true;
                }
                _ => return true,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => return false,
                glutin::event::StartCause::Init => return false,
                _ => return true,
            },
            _ => return true,
        }
    }

    /// Get the canvas's display.
    #[must_use]
    #[inline]
    pub fn display(&self) -> &Display { &self.display }

    /// Get the canvas's delta time.
    #[must_use]
    #[inline]
    pub fn delta_time(&self) -> f64 { self.delta_time }

    /// Get the canvas's frame count.
    #[must_use]
    #[inline]
    pub fn frame_count(&self) -> u64 { self.frame_count }
}
