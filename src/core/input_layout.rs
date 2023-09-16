//Input builder
//1. Get all input groups per vertex or instance
//2. Bake it into VertexBufferLayout
//3. Use baked data into Brush

use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

#[derive(Default)]
pub struct InputLayout {
    groups: Vec<InputLayoutGroup>,
}
impl InputLayout {
    pub fn new() -> Self { Self { groups: Vec::new() } }
    /// Returns a InputLayout with only a Vec3 on loc 0 for vertex position
    pub fn new_vertex_minimal() -> InputLayout {
        let mut layout = InputLayout::new();
        layout.group_per_vertex().add_input(InputType::Vec3);
        layout
    }

    /// Returns a InputLayout with Vec3 for position, Vec4 for color and Vec2 for uvs.
    /// This is the default used when no InputLayout is specified.
    pub fn new_vertex_standard() -> InputLayout {
        let mut layout = InputLayout::new();
        layout
            .group_per_vertex()
            .add_input(InputType::Vec3)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec2);
        layout
    }

    pub fn group_per_vertex(&mut self) -> &mut InputLayoutGroup {
        let group = InputLayoutGroup {
            ty: InputLayoutGroupType::Vertex,
            inputs: Vec::new(),
            cached_attributes: Vec::new(),
        };
        self.groups.push(group);
        self.groups.last_mut().unwrap()
    }

    pub fn group_per_instance(&mut self) -> &mut InputLayoutGroup {
        let group = InputLayoutGroup {
            ty: InputLayoutGroupType::Instance,
            inputs: Vec::new(),
            cached_attributes: Vec::new(),
        };
        self.groups.push(group);
        self.groups.last_mut().unwrap()
    }

    pub fn bake(&mut self) -> Vec<VertexBufferLayout> {
        let mut baked_groups = Vec::new();
        let mut start_loc = 0;
        for group in &mut self.groups {
            baked_groups.push(group.bake_layout(&mut start_loc));
        }
        baked_groups
    }
}

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
    ty: InputLayoutGroupType,
    inputs: Vec<InputType>,
    cached_attributes: Vec<VertexAttribute>,
}

impl InputLayoutGroup {
    pub fn add_input(&mut self, input: InputType) -> &mut InputLayoutGroup {
        self.inputs.push(input);
        self
    }
    fn bake_layout(&mut self, start_loc: &mut u32) -> VertexBufferLayout<'_> {
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
            step_mode: self.ty.get_step_mode(),
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
