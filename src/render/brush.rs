use std::{borrow::Cow, collections::HashMap, error::Error, fs::File, io::Read, path::Path};

use wgpu::{
    BindGroup, BlendState, ColorTargetState, FragmentState, MultisampleState,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, ShaderModuleDescriptor, VertexState,
};

use crate::{
    input_layout::{self, InputLayout, InputLayoutGroup},
    Binder, GpuCtx, ToBinder, Vertex,
};

pub enum ColorBlend {
    Default,
    Premultiplied,
    Additive,
    SoftAdditive,
    Multiplied,
}
impl From<ColorBlend> for wgpu::BlendComponent {
    fn from(val: ColorBlend) -> Self {
        match val {
            ColorBlend::Default => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            ColorBlend::Premultiplied => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrc,
                operation: wgpu::BlendOperation::Add,
            },
            ColorBlend::Additive => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            ColorBlend::SoftAdditive => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::OneMinusDst,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            ColorBlend::Multiplied => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Dst,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
        }
    }
}
pub enum AlphaBlend {
    Default,
    Premultiplied,
    Additive,
    SoftAdditive,
    Multiplied,
}
impl From<AlphaBlend> for wgpu::BlendComponent {
    fn from(val: AlphaBlend) -> Self {
        match val {
            AlphaBlend::Default => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
            AlphaBlend::Premultiplied => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            AlphaBlend::Additive => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            AlphaBlend::SoftAdditive => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::OneMinusDst,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            AlphaBlend::Multiplied => wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Dst,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
        }
    }
}

pub struct BrushDesc {
    pub blend: wgpu::BlendState,
}
impl BrushDesc {
    pub fn new(color: ColorBlend, alpha: AlphaBlend) -> Self {
        Self {
            blend: BlendState {
                color: color.into(),
                alpha: alpha.into(),
            },
        }
    }
}
impl Default for BrushDesc {
    fn default() -> Self {
        Self {
            blend: BlendState {
                color: ColorBlend::Default.into(),
                alpha: ColorBlend::Default.into(),
            },
        }
    }
}

///Used to tell the GPU how to draw the shapes provided.
pub struct Brush {
    desc: BrushDesc,
    compiled_shader: ShaderModule,
    cached_pipeline: Option<RenderPipeline>,
    cached_bindings: Vec<(u32, BindGroup)>,
    assets_to_bind: HashMap<u32, Binder>,
    input_layout: InputLayout,
    needs_update: bool,
}

impl Brush {
    /// Create brush from shader path.
    pub fn from_path(
        desc: BrushDesc, wnd: &GpuCtx, shader_path: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        let mut source = String::new();
        File::open(shader_path)?.read_to_string(&mut source)?;
        Self::from_source(desc, wnd, source)
    }

    /// Create brush directly providing the shader source.
    pub fn from_source(
        desc: BrushDesc, wnd: &GpuCtx, shader_source: String,
    ) -> Result<Self, Box<dyn Error>> {
        let device = &wnd.device;
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source.as_str())),
        });
        let mut input_layout = InputLayout::new();
        input_layout.set_vertex_input(InputLayoutGroup::new_vertex_standard());
        Ok(Self {
            desc,
            compiled_shader: shader,
            assets_to_bind: HashMap::new(),
            cached_bindings: Vec::new(),
            cached_pipeline: None,
            input_layout,
            needs_update: true,
        })
    }

    pub fn set_input_layout(&mut self, layout: InputLayout) {
        self.input_layout = layout;
        self.needs_update = true;
    }

    /// Bind asset given a group and location index. Both indices need to match with shader's or it
    /// will panic when baking and linking with rendering pipeline.
    pub fn bind(&mut self, group_index: u32, loc_index: u32, asset: impl ToBinder + 'static) {
        let asset = Box::new(asset);
        if let Some(binder) = self.assets_to_bind.get_mut(&group_index) {
            binder.bind(loc_index, asset);
        } else {
            let mut binder = Binder::new();
            binder.bind(loc_index, asset);
            self.assets_to_bind.insert(group_index, binder);
        }
        self.needs_update = true;
    }

    /// Returns if brush has been modified and needs to update the GPU with new data.
    pub fn needs_update(&self) -> bool {
        self.needs_update
    }

    /// Update GPU bindings and pipelines with current brush data.
    pub fn update(&mut self, gpu: &GpuCtx) {
        let device = &gpu.device;
        self.cached_bindings.clear();
        let mut bgls: Vec<(u32, wgpu::BindGroupLayout)> = Vec::new();
        for (i, binder) in self.assets_to_bind.iter() {
            //Bake group
            let (bgl, bg) = binder.bake(gpu);
            bgls.push((*i, bgl));
            self.cached_bindings.push((*i, bg));
        }
        bgls.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bgls.iter().map(|x| &x.1).collect::<Vec<_>>(),
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &self.compiled_shader,
                entry_point: "vs_main",
                // buffers: &[Vertex::DESC], //TODO: Get InputLayout into here!
                buffers: &self.input_layout.bake(),
            },
            fragment: Some(FragmentState {
                module: &self.compiled_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    write_mask: wgpu::ColorWrites::ALL,
                    format: gpu.get_capabilities().formats[0],
                    blend: Some(self.desc.blend),
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });
        self.cached_pipeline = Some(pipeline);
        self.needs_update = false;
    }

    pub fn get_pipeline(&self) -> &RenderPipeline {
        self.cached_pipeline.as_ref().unwrap()
    }

    pub fn get_bind_groups(&self) -> &Vec<(u32, BindGroup)> {
        &self.cached_bindings
    }
}
