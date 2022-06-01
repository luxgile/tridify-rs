use glium::uniforms::{self, SamplerBehavior};

use crate::LErr;

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

impl PartialEq for Uniform {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
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
    Texture1D(glium::texture::SrgbTexture1d, Option<TextureSettings>),
    Texture2D(glium::texture::SrgbTexture2d, Option<TextureSettings>),
    Texture3D(glium::texture::SrgbTexture3d, Option<TextureSettings>),
    DepthTexture2D(glium::texture::DepthTexture2d, Option<TextureSettings>),
}

#[derive(Debug)]
pub struct UniformBuffer {
    uniforms: Vec<Uniform>,
}

impl UniformBuffer {
    pub fn new(uniforms: Vec<Uniform>) -> Self { Self { uniforms } }

    ///Add uniform to the buffer, if uniform already exists, returns error.
    pub fn add_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
        if self.has_uniform(&uniform) {
            return Err(LErr::new(format!(
                "Cannot add uniform '{}' as it's already in the buffer",
                uniform.name
            )));
        }
        self.uniforms.push(uniform);
        Ok(())
    }
    pub fn has_uniform(&self, uniform: &Uniform) -> bool { self.uniforms.contains(uniform) }

    /// Get a reference to the uniform buffer's uniforms.
    #[must_use]
    pub fn uniforms(&self) -> &[Uniform] { self.uniforms.as_ref() }

    /// Changes a uniform, returns error if the uniform does not exist.
    pub fn change_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
        let pos = self
            .uniforms()
            .iter()
            .position(|x| x.name() == uniform.name());
        if let None = pos {
            return Result::Err(LErr::new(format!(
                "No uniform named '{}' was found",
                uniform.name
            )));
        }
        self.uniforms[pos.unwrap()].value = uniform.value;
        return Result::Ok(());
    }
    ///Clears all uniforms.
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
                UniformValue::UInt(value) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::UnsignedInt(*value),
                ),
                UniformValue::Vec2(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Vec2(*value))
                }
                UniformValue::Vec3(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Vec3(*value))
                }
                UniformValue::Vec4(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Vec4(*value))
                }
                UniformValue::Matrix2(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Mat2(*value))
                }
                UniformValue::Matrix3(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Mat3(*value))
                }
                UniformValue::Matrix4(value) => {
                    output(uniform.name.as_str(), uniforms::UniformValue::Mat4(*value))
                }
                UniformValue::Texture1D(value, _) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::SrgbTexture1d(value, Some(SamplerBehavior::default())),
                ),
                UniformValue::Texture2D(value, _) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::SrgbTexture2d(&value, Some(SamplerBehavior::default())),
                ),
                UniformValue::Texture3D(value, _) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::SrgbTexture3d(&value, Some(SamplerBehavior::default())),
                ),
                UniformValue::DepthTexture2D(value, _) => output(
                    uniform.name.as_str(),
                    uniforms::UniformValue::DepthTexture2d(
                        &value,
                        Some(SamplerBehavior::default()),
                    ),
                ),
                _ => panic!("{:?} not yet implemented", uniform),
            }
        }
    }
}
