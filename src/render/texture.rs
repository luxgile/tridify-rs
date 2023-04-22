use std::{num::NonZeroU32, path::Path, rc::Rc};

use egui::Vec2;
use glam::{UVec2, UVec3};
use wgpu::{
    ImageCopyTexture, ImageDataLayout, ShaderStages, TextureAspect, TextureDescriptor,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};

use crate::{GpuCtx, ToBinder};

bitflags::bitflags! {
    /// Specifies how the texture will be used for optimizations.
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

/// GPU texture handle.
#[derive(Debug)]
pub struct Texture {
    pub desc: TextureDesc,
    texture: Rc<wgpu::Texture>,
    view: wgpu::TextureView,
}

impl Texture {
    pub fn from_path(gpu: &GpuCtx, path: &Path) -> Self {
        let image =
            image::open(path).unwrap_or_else(|_| panic!("Error loading image at {:?}", path));
        let desc = TextureDesc {
            size: TextureSize::D2(UVec2::new(image.width(), image.height())),
            usage: TextureUsage::TEXTURE_BIND | TextureUsage::DESTINATION,
        };
        let texture = Self::new(gpu, desc, None);
        texture.write_pixels(gpu, &image.to_rgba8());
        texture
    }

    pub fn init(gpu: &GpuCtx, desc: TextureDesc, data: &[u8], label: Option<&str>) -> Self {
        let texture = Self::new(gpu, desc, label);
        texture.write_pixels(gpu, data);
        texture
    }

    pub fn new(gpu: &GpuCtx, desc: TextureDesc, label: Option<&str>) -> Self {
        let size = desc.size.get_size();
        let texture = gpu.device.create_texture(&TextureDescriptor {
            label,
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
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        Self {
            desc,
            texture: Rc::new(texture),
            view,
        }
    }

    ///Queues a write into the texture
    pub fn write_pixels(&self, gpu: &GpuCtx, data: &[u8]) {
        let size = self.desc.size.get_size();
        gpu.queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size.x * 4),
                rows_per_image: Some(size.y),
            },
            wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: size.z,
            },
        );
    }

    ///Queues a write into the texture updating only a subset of it
    pub fn write_region_pixels(&self, gpu: &GpuCtx, data: &[u8], origin: UVec3) {
        let size = self.desc.size.get_size();
        gpu.queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: origin.x,
                    y: origin.y,
                    z: origin.z,
                },
                aspect: TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size.x * 4),
                rows_per_image: Some(size.y),
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

    fn debug_name(&self) -> &'static str { "Texture" }
}
