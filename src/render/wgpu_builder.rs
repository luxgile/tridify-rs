use std::error::Error;

use wgpu::{DeviceDescriptor, Features, Limits, RequestAdapterOptions};

pub struct WgpuBuilder;
impl WgpuBuilder {
    pub fn build_context(
        wgpu: &wgpu::Instance, surface: Option<&wgpu::Surface>,
    ) -> Result<(wgpu::Adapter, wgpu::Device, wgpu::Queue), Box<dyn Error>> {
        let adapter = pollster::block_on(wgpu.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: surface,
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
        Ok((adapter, device, queue))
    }
}
