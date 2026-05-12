use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new_hardcoded(aspect: f32) -> Self {
        let eye = Vec3::new(4.0, 4.0, 4.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;

        let view = Mat4::look_at_rh(eye, target, up);
        let proj = Mat4::perspective_rh(45_f32.to_radians(), aspect, 0.1, 100.0);

        Self {
            view_proj: (proj * view).to_cols_array_2d(),
        }
    }
}
