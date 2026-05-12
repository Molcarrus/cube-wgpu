use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::{
    camera::CameraUniform, cube::RubiksCube, mesh::{Vertex, generate_cubie_mesh}
};

struct CubieGpu {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    model_bind_group: wgpu::BindGroup,
    _model_buffer: wgpu::Buffer,
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    model_bgl: wgpu::BindGroupLayout,
    cubies: Vec<CubieGpu>,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, aspect: f32, cube: &RubiksCube) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/cube.wgsl").into()),
        });

        let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let camera_uniform = CameraUniform::new_hardcoded(aspect);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&camera_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let model_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Model BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[Some(&camera_bgl), Some(&model_bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format.add_srgb_suffix(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        let cubies = cube
            .cubies
            .iter()
            .map(|cubie| {
                Self::build_cubie_gpu(device, &model_bgl, cubie)
            })
            .collect::<Vec<_>>();

        Self {
            pipeline,
            camera_buffer,
            camera_bind_group,
            model_bgl,
            cubies,
        }
    }

    fn build_cubie_gpu(
        device: &wgpu::Device,
        model_bgl: &wgpu::BindGroupLayout,
        cubie: &crate::cube::Cubie,
    ) -> CubieGpu {
        let (vertices, indices) = generate_cubie_mesh(cubie);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Cubie VB"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Cubie IB"), contents: bytemuck::cast_slice(&indices), usage: wgpu::BufferUsages::INDEX });

        let [x, y, z] = cubie.grid_pos;
        let spacing = 1.05_f32;
        let model = Mat4::from_translation(Vec3::new(x as f32 * spacing, y as f32 * spacing, z as f32 * spacing));

        let model_data = model.to_cols_array_2d();

        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Cubie Model Buffer"), contents: bytemuck::cast_slice(&model_data), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("Cubie Model BG"), layout: model_bgl, entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: model_buffer.as_entire_binding(),
        }] });

        CubieGpu { vertex_buffer, index_buffer, num_indices: indices.len() as u32, model_bind_group, _model_buffer: model_buffer }
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.camera_bind_group, &[]);

        for cubie in &self.cubies {
            pass.set_bind_group(1, &cubie.model_bind_group, &[]);
            pass.set_vertex_buffer(0, cubie.vertex_buffer.slice(..));
            pass.set_index_buffer(cubie.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..cubie.num_indices, 0, 0..1);
        }
    }
}
