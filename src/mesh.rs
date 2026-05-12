use bytemuck::{Pod, Zeroable};

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

pub fn single_cubie() -> (Vec<Vertex>, Vec<u16>) {
    let s = 0.48_f32;

    let faces: &[([f32; 3], [[f32; 3]; 4], [f32; 3])] = &[
        (
            [1.0, 0.0, 0.0],
            [[s, -s, -s], [s, -s, s], [s, s, s], [s, s, -s]],
            [0.0, 0.8, 0.0],
        ),
        // -X  Blue
        (
            [-1.0, 0.0, 0.0],
            [[-s, -s, s], [-s, -s, -s], [-s, s, -s], [-s, s, s]],
            [0.0, 0.0, 0.9],
        ),
        // +Y  White
        (
            [0.0, 1.0, 0.0],
            [[-s, s, -s], [s, s, -s], [s, s, s], [-s, s, s]],
            [1.0, 1.0, 1.0],
        ),
        // -Y  Yellow
        (
            [0.0, -1.0, 0.0],
            [[-s, -s, s], [s, -s, s], [s, -s, -s], [-s, -s, -s]],
            [1.0, 1.0, 0.0],
        ),
        // +Z  Red
        (
            [0.0, 0.0, 1.0],
            [[-s, -s, s], [-s, s, s], [s, s, s], [s, -s, s]],
            [0.9, 0.0, 0.0],
        ),
        // -Z  Orange
        (
            [0.0, 0.0, -1.0],
            [[s, -s, -s], [s, s, -s], [-s, s, -s], [-s, -s, -s]],
            [1.0, 0.5, 0.0],
        ),
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for (normal, corners, color) in faces {
        let base = vertices.len() as u16;

        for pos in corners {
            vertices.push(Vertex {
                position: *pos,
                normal: *normal,
                color: *color,
            });
        }

        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    (vertices, indices)
}
