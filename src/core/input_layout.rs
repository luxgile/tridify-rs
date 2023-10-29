//Input builder
//1. Get all input groups per vertex or instance
//2. Bake it into VertexBufferLayout
//3. Use baked data into Brush

use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub trait GpuDataLayout {
    fn get_layout() -> InputLayoutGroup;
}

pub struct InputLayout {
    pub vertex: Option<InputLayoutGroup>,
    pub instance: Option<InputLayoutGroup>,
}
impl InputLayout {
    pub fn new() -> Self {
        Self {
            vertex: None,
            instance: None,
        }
    }

    pub fn set_vertex_input(&mut self, input: InputLayoutGroup) { self.vertex = Some(input); }

    pub fn set_instance_input(&mut self, input: InputLayoutGroup) { self.instance = Some(input); }

    pub fn bake(&mut self) -> Vec<VertexBufferLayout> {
        let mut baked_groups = Vec::new();
        let mut start_loc = 0;
        if let Some(input) = &mut self.vertex {
            baked_groups.push(input.bake_layout(InputLayoutGroupType::Vertex, &mut start_loc));
        }
        if let Some(input) = &mut self.instance {
            baked_groups.push(input.bake_layout(InputLayoutGroupType::Instance, &mut start_loc));
        }
        baked_groups
    }
}

#[derive(PartialEq)]
enum InputLayoutGroupType {
    Vertex,
    Instance,
}
impl InputLayoutGroupType {
    fn get_step_mode(&self) -> VertexStepMode {
        match self {
            InputLayoutGroupType::Vertex => VertexStepMode::Vertex,
            InputLayoutGroupType::Instance => VertexStepMode::Instance,
        }
    }
}

pub struct InputLayoutGroup {
    inputs: Vec<InputType>,
    cached_attributes: Vec<VertexAttribute>,
}

impl InputLayoutGroup {
    pub fn new_vertex() -> Self {
        Self {
            inputs: Vec::new(),
            cached_attributes: Vec::new(),
        }
    }

    pub fn new_instance() -> Self {
        Self {
            inputs: Vec::new(),
            cached_attributes: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: InputType) -> &mut InputLayoutGroup {
        self.inputs.push(input);
        self
    }
    fn bake_layout(
        &mut self, ty: InputLayoutGroupType, start_loc: &mut u32,
    ) -> VertexBufferLayout<'_> {
        let index = start_loc;
        let mut offset = 0;
        self.cached_attributes.clear();
        for input in &self.inputs {
            self.cached_attributes.push(VertexAttribute {
                offset,
                shader_location: *index,
                format: input.get_format(),
            });

            offset += input.get_size();
            *index += 1;
        }

        VertexBufferLayout {
            array_stride: offset,
            step_mode: ty.get_step_mode(),
            attributes: &self.cached_attributes,
        }
    }
}

pub enum InputType {
    F32,
    Vec2,
    Vec3,
    Vec4,
}

impl InputType {
    fn get_size(&self) -> u64 {
        match self {
            InputType::F32 => std::mem::size_of::<f32>() as u64,
            InputType::Vec2 => std::mem::size_of::<[f32; 2]>() as u64,
            InputType::Vec3 => std::mem::size_of::<[f32; 3]>() as u64,
            InputType::Vec4 => std::mem::size_of::<[f32; 4]>() as u64,
        }
    }

    fn get_format(&self) -> VertexFormat {
        match self {
            InputType::F32 => VertexFormat::Float32,
            InputType::Vec2 => VertexFormat::Float32x2,
            InputType::Vec3 => VertexFormat::Float32x3,
            InputType::Vec4 => VertexFormat::Float32x4,
        }
    }
}
