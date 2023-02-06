// use std::path::Path;

// use glium::texture::SrgbTexture2d;

// use crate::Window;

// #[derive(Debug)]
// pub struct TextureSettings {}

use std::{
    cell::{Cell, RefCell},
    num::NonZeroU32,
    path::Path,
    rc::Rc,
};

use glam::{UVec2, UVec3};
use image::GenericImageView;
use wgpu::{
    ImageCopyTexture, ImageDataLayout, ShaderStages, TextureAspect, TextureDescriptor,
    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

use crate::{Asset, Color, Graphics, ToBinder};

bitflags::bitflags! {

    pub struct TextureUsage: u32 {
        const DESTINATION = 1 << 0;
        const SOURCE = 1 << 1;
        const TEXTURE_BIND = 1 << 2;
        const STORAGE_BIND = 1 << 3;
        const RENDER = 1 << 4;
    }
}

#[derive(Debug)]
pub enum TextureSize {
    D1(u32),
    D2(UVec2),
    D3(UVec3),
}
impl TextureSize {
    pub fn get_size(&self) -> UVec3 {
        match self {
            TextureSize::D1(x) => UVec3::new(*x, 1, 1),
            TextureSize::D2(size) => UVec3::new(size.x, size.y, 1),
            TextureSize::D3(size) => UVec3::new(size.x, size.y, size.z),
        }
    }
    pub fn get_wgpu_dimension(&self) -> wgpu::TextureDimension {
        match self {
            TextureSize::D1(_) => wgpu::TextureDimension::D1,
            TextureSize::D2(_) => wgpu::TextureDimension::D2,
            TextureSize::D3(_) => wgpu::TextureDimension::D3,
        }
    }
    pub fn get_wgpu_view_dimension(&self) -> wgpu::TextureViewDimension {
        match self {
            TextureSize::D1(_) => wgpu::TextureViewDimension::D1,
            TextureSize::D2(_) => wgpu::TextureViewDimension::D2,
            TextureSize::D3(_) => wgpu::TextureViewDimension::D3,
        }
    }
}

#[derive(Debug)]
pub struct TextureDesc {
    pub size: TextureSize,
    pub usage: TextureUsage,
}
impl TextureDesc {
    fn get_wgpu_usage(&self) -> TextureUsages {
        let mut usage = TextureUsages::empty();
        if self.usage.contains(TextureUsage::DESTINATION) {
            usage |= TextureUsages::COPY_DST;
        }
        if self.usage.contains(TextureUsage::SOURCE) {
            usage |= TextureUsages::COPY_SRC;
        }
        if self.usage.contains(TextureUsage::TEXTURE_BIND) {
            usage |= TextureUsages::TEXTURE_BINDING;
        }
        if self.usage.contains(TextureUsage::STORAGE_BIND) {
            usage |= TextureUsages::STORAGE_BINDING;
        }
        if self.usage.contains(TextureUsage::RENDER) {
            usage |= TextureUsages::RENDER_ATTACHMENT;
        }
        usage
    }
}

#[derive(Debug)]
pub struct Texture {
    pub desc: TextureDesc,
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}

impl Texture {
    pub fn from_path(graphics: &impl Graphics, path: &Path) -> Self {
        let image = image::open(path).expect("Error loading image.");
        let desc = TextureDesc {
            size: TextureSize::D2(UVec2::new(image.width(), image.height())),
            usage: TextureUsage::TEXTURE_BIND | TextureUsage::DESTINATION,
        };
        let mut texture = Self::new(graphics, desc);
        texture.lazy_write_data(graphics, &image.to_rgba8());
        texture
    }

    pub fn new(graphics: &impl Graphics, desc: TextureDesc) -> Self {
        let size = desc.size.get_size();
        let texture = graphics.get_device().create_texture(&TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: size.z,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: desc.size.get_wgpu_dimension(),
            format: TextureFormat::Rgba8UnormSrgb,
            usage: desc.get_wgpu_usage(),
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        Self {
            desc,
            texture,
            view,
        }
    }

    ///Writes data into the texture lazily, which means it won't be done until all GPU commands are sent.
    pub fn lazy_write_data(&mut self, graphics: &impl Graphics, data: &[u8]) {
        let size = self.desc.size.get_size();
        graphics.get_queue().write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(size.x * 4),
                rows_per_image: NonZeroU32::new(size.y),
            },
            wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: size.z,
            },
        );
    }
}

impl ToBinder for Texture {
    fn get_layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index,
            visibility: ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: self.desc.size.get_wgpu_view_dimension(),
                multisampled: false,
            },
            count: None,
        }
    }

    fn get_group(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::TextureView(&self.view),
        }
    }
}
 
impl Asset for Texture {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
