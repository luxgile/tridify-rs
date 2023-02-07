// use std::path::Path;

// use glium::Program;

// use crate::{core::Window, LErr};

// use super::{Uniform, UniformBuffer};

use std::{
    borrow::Cow, cell::RefCell, collections::HashMap, error::Error, fs::File, io::Read, path::Path,
    rc::Rc,
};

use wgpu::{
    BindGroup, FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, VertexState,
};

use crate::{AssetRef, Binder, ToBinder, Vertex, WindowView};

use super::Graphics;

// //TODO: PIPELINE GOES HERE
// ///Used to tell the GPU how to draw the shapes provided.
// #[derive(Debug)]
pub struct Brush {
    pub shader: ShaderModule,
    pub cached_pipeline: Option<RenderPipeline>,
    assets_to_bind: HashMap<u32, HashMap<u32, AssetRef<dyn ToBinder>>>,
    needs_update: bool,
}

impl Brush {
    pub fn from_path(graphics: &impl Graphics, shader_path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut source = String::new();
        File::open(shader_path)?.read_to_string(&mut source)?;
        Self::from_source(graphics, source)
    }

    pub fn from_source(
        graphics: &impl Graphics, shader_source: String,
    ) -> Result<Self, Box<dyn Error>> {
        let device = graphics.get_device();
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source.as_str())),
        });
        Ok(Self {
            shader,
            assets_to_bind: HashMap::new(),
            cached_pipeline: None,
            needs_update: true,
        })
    }

    pub fn bind(&mut self, group_index: u32, loc_index: u32, asset: AssetRef<dyn ToBinder>) {
        if let Some(map) = self.assets_to_bind.get_mut(&group_index) {
            map.insert(loc_index, asset);
        }
        self.needs_update = true;
    }

    pub fn needs_update(&self) -> bool { self.needs_update }

    pub fn update(&mut self, graphics: &impl Graphics) {
        let device = graphics.get_device();
        let mut bind_layouts = Vec::new();
        // for binder in self.binders.iter_mut() {
        //     if binder.1.needs_update() {
        //         binder
        //             .1
        //             .update(graphics)
        //             .expect("Error while updating binder in brush");
        //     }
        //     if let Some(layout) = binder.1.bind_layout.as_ref() {
        //         bind_layouts.push(layout);
        //     }
        // }
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_layouts,
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &self.shader,
                entry_point: "vs_main",
                buffers: &[Vertex::DESC],
            },
            fragment: Some(FragmentState {
                module: &self.shader,
                entry_point: "fs_main",
                targets: &[Some(
                    graphics
                        .get_surface()
                        .get_supported_formats(graphics.get_adapter())[0]
                        .into(),
                )],
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
    }

    // ///Adds a uniform to the brush. Returns error if the uniform already exists.
    // pub fn add_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
    //     self.uniform_buffer.add_uniform(uniform)
    // }
    // ///Changes or adds a uniform to the brush.
    // pub fn update_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
    //     if self.uniform_buffer.has_uniform(&uniform) {
    //         return self.uniform_buffer.change_uniform(uniform);
    //     } else {
    //         return self.uniform_buffer.add_uniform(uniform);
    //     }
    // }
    // ///Changes or adds a uniform to the brush. Returns error if the uniform does not exist.
    // pub fn change_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
    //     self.uniform_buffer.change_uniform(uniform)
    // }
    // ///Removes all uniforms from the brush.
    // pub fn clear_uniforms(&mut self) { self.uniform_buffer.clear(); }

    // /// Get a reference to the brush's program.
    // #[must_use]
    // pub fn program(&self) -> &Program { &self.program }

    // /// Get a reference to the brush's uniform buffer.
    // #[must_use]
    // pub fn uniform_buffer(&self) -> &UniformBuffer { &self.uniform_buffer }
}

// mod brush_templates {
//     pub const UNLIT_FRAG: &str = r#"
//         #version 330 core
//         in vec4 frag_color;
//         in vec2 frag_uv;

//         uniform sampler2D main_tex;

//         out vec4 out_color;

//         void main(){
//             out_color=vec4(frag_color)*texture(main_tex,frag_uv);
//         }"#;

//     pub const UNLIT_VERT: &str = r#"
//         #version 330 core
//         in vec3 pos;
//         in vec4 color;
//         in vec2 uv;

//         uniform mat4 mvp;

//         out vec4 frag_color;
//         out vec2 frag_uv;

//         void main(){
//             frag_uv=uv;
//             frag_color=color;
//             gl_Position=mvp*vec4(pos,1.);
//         }"#;
// }
