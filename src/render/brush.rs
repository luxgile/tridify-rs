use std::path::Path;

use glium::Program;

use crate::core::Window;

use super::{Uniform, UniformBuffer};

///Used to tell the GPU how to draw the shapes provided.
pub struct Brush {
    program: Program,
    uniform_buffer: UniformBuffer,
}

impl Brush {
    pub fn from_path(
        wnd: &Window, vertex: &Path, fragment: &Path, geometry: Option<&Path>,
    ) -> Self {
        let geometry_source = if let Some(path) = geometry {
            let source = std::fs::read_to_string(path);
            Some(source.unwrap())
        } else {
            None
        };
        Self::from_source(
            wnd,
            std::fs::read_to_string(vertex).unwrap(),
            std::fs::read_to_string(fragment).unwrap(),
            geometry_source,
        )
    }
    pub fn from_source<'a>(
        wnd: &Window, vertex: String, fragment: String, geometry: Option<String>,
    ) -> Self {
        let program = glium::Program::from_source(
            wnd.display(),
            vertex.as_str(),
            fragment.as_str(),
            geometry.as_deref(),
        )
        .unwrap();
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
