use std::time::{Duration, Instant};

use glium::{
    glutin::{
        self,
        event::{Event, VirtualKeyCode},
        event_loop::ControlFlow,
    },
    implement_vertex, texture,
    uniforms::{self, SamplerBehavior},
    vertex::Attribute,
    Display, Program, Surface,
};
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }

    pub const CLEAR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const SILVER: Color = Color::new(0.75, 0.75, 0.75, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const MAROON: Color = Color::new(0.5, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const PURPLE: Color = Color::new(0.5, 0.0, 0.5, 1.0);
    pub const GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);
    pub const LIME: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const BLUE_NAVY: Color = Color::new(0.0, 0.0, 0.5, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const BLUE_TEAL: Color = Color::new(0.0, 0.5, 0.5, 1.0);
    pub const BLUE_AQUA: Color = Color::new(0.0, 1.0, 1.0, 1.0);
}

unsafe impl Attribute for Color {
    fn get_type() -> glium::vertex::AttributeType { glium::vertex::AttributeType::F32F32F32F32 }
}

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

pub struct WindowSettings {
    max_fps: u64,
}

pub struct Window {
    settings: WindowSettings,
    display: Display,
    delta_time: f64,
    frame_count: u64,
}

impl Window {
    pub fn create_and_run(mut user: impl UserWindowHandler + 'static) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut window = Window {
            settings: WindowSettings { max_fps: 60 },
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
        let mut canvas = Canvas {
            frame: self.display.draw(),
        };
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

implement_vertex!(Vertex, pos, color);
#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 2],
    color: Color,
}

impl Vertex {
    /// Center       [ 0,  0],
    /// Top Right    [ 1,  1],
    /// Bottom Left  [-1, -1],
    pub fn from_viewport(x: f32, y: f32, c: Option<Color>) -> Self {
        Self {
            pos: [x, y],
            color: c.unwrap_or(Color::WHITE),
        }
    }
    #[must_use]
    #[inline]
    pub fn x(&self) -> f32 { self.pos[0] }
    #[must_use]
    #[inline]
    pub fn y(&self) -> f32 { self.pos[1] }
}

#[macro_export]
macro_rules! vertex {
    ($a:expr, $b:expr) => {
        Vertex::from_viewport($a, $b, None)
    };
    ($a:expr, $b:expr, $c:expr) => {
        Vertex::from_viewport($a, $b, Some($c))
    };
}

#[derive(Debug)]
pub struct TextureSettings {}

#[derive(Debug)]
pub struct Uniform {
    name: String,
    value: UniformValue,
}

impl Uniform {
    pub fn new(name: String, value: UniformValue) -> Self { Self { name, value } }
    pub fn from_str(name: &str, value: UniformValue) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}
#[derive(Debug)]
pub enum UniformValue {
    Float(f32),
    Int(i32),
    UInt(u32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Matrix2([[f32; 2]; 2]),
    Matrix3([[f32; 3]; 3]),
    Matrix4([[f32; 4]; 4]),
    Texture1D(texture::Texture1d, Option<TextureSettings>),
    Texture2D(texture::Texture2d, Option<TextureSettings>),
    Texture3D(texture::Texture3d, Option<TextureSettings>),
    DepthTexture2D(texture::DepthTexture2d, Option<TextureSettings>),
}

#[derive(Debug)]
pub struct UniformBuffer {
    uniforms: Vec<Uniform>,
}

impl UniformBuffer {
    pub fn new(uniforms: Vec<Uniform>) -> Self { Self { uniforms } }
}

impl uniforms::Uniforms for UniformBuffer {
    fn visit_values<'a, F: FnMut(&str, uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        for uniform in self.uniforms.iter() {
            match &uniform.value {
                UniformValue::Float(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Float(*value))
                }
                UniformValue::Int(value) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::SignedInt(*value),
                ),
                UniformValue::Texture2D(value, _) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::Texture2d(&value, Some(SamplerBehavior::default())),
                ),
                UniformValue::Vec3(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Vec3(*value))
                }
                _ => panic!("{:?} not yet implemented", uniform),
            }
        }
    }
}

///Queue of shapes to be drawn. All shapes added to the same batch will be drawn at the same time using the same brush.
#[derive(Default)]
pub struct ShapeBatch {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    index_count: u32,
}

impl ShapeBatch {
    ///Add a triangle to the batch specifying its 3 vertices
    pub fn add_triangle(&mut self, v: [Vertex; 3]) {
        let index = self.index_count;
        self.vertices.push(v[0]);
        self.indices.push(index);
        self.vertices.push(v[1]);
        self.indices.push(index + 1);
        self.vertices.push(v[2]);
        self.indices.push(index + 2);
        self.index_count += 3;
    }

    ///Add a square to the batch specifying the center, width and height
    pub fn add_square(&mut self, c: Vertex, w: f32, h: f32) {
        //Adding vertices
        let hw = w / 2.0;
        let hh = h / 2.0;
        self.vertices.push(vertex!(c.x() - hw, c.y() - hh, c.color));
        self.vertices.push(vertex!(c.x() + hw, c.y() - hh, c.color));
        self.vertices.push(vertex!(c.x() - hw, c.y() + hh, c.color));
        self.vertices.push(vertex!(c.x() + hw, c.y() + hh, c.color));

        //Adding indices
        let index = self.index_count;
        self.indices.push(index);
        self.indices.push(index + 1);
        self.indices.push(index + 2);
        self.indices.push(index + 2);
        self.indices.push(index + 1);
        self.indices.push(index + 3);

        self.index_count += 4;
    }

    pub fn bake_buffers(&self, wnd: &Window) -> ShapeBuffer {
        let vertex_buffer = glium::VertexBuffer::new(wnd.display(), &self.vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            wnd.display(),
            glium::index::PrimitiveType::TrianglesList,
            &self.indices,
        )
        .unwrap();
        ShapeBuffer {
            vertex_buffer,
            index_buffer,
        }
    }
}

///Buffers created from the batch and prepared to be sent directly to the GPU
pub struct ShapeBuffer {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
}

///Used to configurate how to draw shapes in the GPU
pub struct Brush {
    program: Program,
    uniform_buffer: UniformBuffer,
}

impl Brush {
    pub fn new_basic(wnd: &Window) -> Self {
        let program = glium::Program::from_source(
            &wnd.display,
            r#"
            #version 330 core
            in vec2 pos;
            in vec4 color;

            out vec4 frag_color;

            void main() {
                frag_color = color;
                float y_pos = pos.y;
                gl_Position = vec4(pos.x, y_pos, 0.0, 1.0);
            }
            "#,
            r#"
            #version 330 core
            in vec4 frag_color;
            out vec4 out_color;

            void main() {
                out_color = vec4(frag_color);
            }
            "#,
            None,
        )
        .unwrap();
        Self {
            program,
            uniform_buffer: UniformBuffer::new(Vec::new()),
        }
    }
    pub fn from_source<'a>(
        wnd: &Window, vertex: &'a str, fragment: &'a str, geometry: Option<&'a str>,
    ) -> Self {
        let program =
            glium::Program::from_source(&wnd.display, vertex, fragment, geometry).unwrap();
        Self {
            program,
            uniform_buffer: UniformBuffer::new(Vec::new()),
        }
    }
    pub fn add_uniform(mut self, uniform: Uniform) -> Self {
        self.uniform_buffer.uniforms.push(uniform);
        self
    }
    pub fn change_uniform(&mut self, uniform: Uniform) -> &Self {
        let pos = self
            .uniform_buffer
            .uniforms
            .iter()
            .position(|x| x.name == uniform.name)
            .unwrap();
        self.uniform_buffer.uniforms[pos].value = uniform.value;
        self
    }
    pub fn clear_uniforms(&mut self) { self.uniform_buffer.uniforms.clear(); }
}

pub struct Canvas {
    frame: glium::Frame,
}

impl Canvas {
    pub fn clear_color(&mut self, color: Color) {
        self.frame.clear_color(color.r, color.g, color.b, color.a);
    }
    pub fn finish_canvas(self) -> Result<(), glium::SwapBuffersError> { self.frame.finish() }
    pub fn draw_batch(&mut self, _wnd: &Window, brush: &Brush, buffers: ShapeBuffer) {
        self.frame
            .draw(
                &buffers.vertex_buffer,
                &buffers.index_buffer,
                &brush.program,
                &brush.uniform_buffer,
                &Default::default(),
            )
            .unwrap();
    }
}
