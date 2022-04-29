use glium::Program;

use crate::core::Window;

use super::{Uniform, UniformBuffer};

///Used to configurate how to draw shapes in the GPU
pub struct Brush {
    program: Program,
    uniform_buffer: UniformBuffer,
}

impl Brush {
    pub fn new_basic(wnd: &Window) -> Self {
        let program = glium::Program::from_source(
            wnd.display(),
            r#"
            #version 330 core
            in vec3 pos;
            in vec4 color;
            in vec2 uv;

            out vec4 frag_color;
            out vec2 frag_uv;

            void main() {
                frag_uv = uv;
                frag_color = color;
                gl_Position = vec4(pos, 1.0);
            }
            "#,
            r#"
            #version 330 core
            in vec4 frag_color;
            in vec2 frag_uv;

            uniform sampler2D main_tex;

            out vec4 out_color;

            void main() {
                out_color = vec4(frag_color) + texture(main_tex, frag_uv);
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
            glium::Program::from_source(wnd.display(), vertex, fragment, geometry).unwrap();
        Self {
            program,
            uniform_buffer: UniformBuffer::new(Vec::new()),
        }
    }
    pub fn add_uniform(&mut self, uniform: Uniform) -> &Self {
        self.uniform_buffer.add_uniform(uniform);
        self
    }
    pub fn change_uniform(&mut self, uniform: Uniform) -> &Self {
        self.uniform_buffer.change_uniform(uniform);
        self
    }
    pub fn clear_uniforms(&mut self) { self.uniform_buffer.clear(); }

    /// Get a reference to the brush's program.
    #[must_use]
    pub fn program(&self) -> &Program { &self.program }

    /// Get a reference to the brush's uniform buffer.
    #[must_use]
    pub fn uniform_buffer(&self) -> &UniformBuffer { &self.uniform_buffer }
}
