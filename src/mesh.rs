use bytemuck::{Pod, Zeroable};

use crate::cube::Cubie;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub fn generate_cubie_mesh(cubie: &Cubie) -> (Vec<Vertex>, Vec<u16>) {
    let s = 0.48_f32;

    let faces = &[
        (
            [1.0, 0.0, 0.0],
            [[s, -s, -s], [s, -s, s], [s, s, s], [s, s, -s]],
            0,
        ),
        // -X  Blue
        (
            [-1.0, 0.0, 0.0],
            [[-s, -s, s], [-s, -s, -s], [-s, s, -s], [-s, s, s]],
            1,
        ),
        // +Y  White
        (
            [0.0, 1.0, 0.0],
            [[-s, s, -s], [s, s, -s], [s, s, s], [-s, s, s]],
            2,
        ),
        // -Y  Yellow
        (
            [0.0, -1.0, 0.0],
            [[-s, -s, s], [s, -s, s], [s, -s, -s], [-s, -s, -s]],
            3,
        ),
        // +Z  Red
        (
            [0.0, 0.0, 1.0],
            [[-s, -s, s], [-s, s, s], [s, s, s], [s, -s, s]],
            4,
        ),
        // -Z  Orange
        (
            [0.0, 0.0, -1.0],
            [[s, -s, -s], [s, s, -s], [-s, s, -s], [-s, -s, -s]],
            5,
        ),
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (normal, corners, color_idx) in faces {
        let base = vertices.len() as u16;
        let color = cubie.face_colors[*color_idx].to_rgb();

        for pos in corners {
            vertices.push(Vertex {
                position: *pos,
                normal: *normal,
                color,
            });
        }

        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    (vertices, indices)
}
