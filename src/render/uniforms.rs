use glium::uniforms::{self, SamplerBehavior};

use super::TextureSettings;

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

    /// Get a reference to the uniform's name.
    #[must_use]
    pub fn name(&self) -> &str { self.name.as_ref() }

    /// Get a reference to the uniform's value.
    #[must_use]
    pub fn value(&self) -> &UniformValue { &self.value }
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
    Texture1D(glium::texture::SrgbTexture2d, Option<TextureSettings>),
    Texture2D(glium::texture::SrgbTexture2d, Option<TextureSettings>),
    Texture3D(glium::texture::SrgbTexture2d, Option<TextureSettings>),
    DepthTexture2D(glium::texture::DepthTexture2d, Option<TextureSettings>),
}

#[derive(Debug)]
pub struct UniformBuffer {
    uniforms: Vec<Uniform>,
}

impl UniformBuffer {
    pub fn new(uniforms: Vec<Uniform>) -> Self { Self { uniforms } }

    #[must_use]
    pub fn add_uniform(&mut self, uniform: Uniform) { self.uniforms.push(uniform); }

    /// Get a reference to the uniform buffer's uniforms.
    #[must_use]
    pub fn uniforms(&self) -> &[Uniform] { self.uniforms.as_ref() }
    pub fn change_uniform(&mut self, uniform: Uniform) {
        let pos = self
            .uniforms()
            .iter()
            .position(|x| x.name() == uniform.name())
            .unwrap();
        self.uniforms[pos].value = uniform.value;
    }
    pub fn clear(&mut self) { self.uniforms.clear() }
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
                    uniforms::UniformValue::SrgbTexture2d(&value, Some(SamplerBehavior::default())),
                ),
                UniformValue::Vec3(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Vec3(*value))
                }
                _ => panic!("{:?} not yet implemented", uniform),
            }
        }
    }
}
