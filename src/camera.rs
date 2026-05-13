use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: 0.4,
            distance: 8.0,
            aspect,
        }
    }

    fn eye(&self) -> Vec3 {
        Vec3::new(
            self.distance * self.pitch.cos() * self.yaw.sin(),
            self.distance * self.pitch.sin(),
            self.distance * self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn build_uniform(&self) -> CameraUniform {
        let view = Mat4::look_at_rh(self.eye(), Vec3::ZERO, Vec3::Y);
        let proj = Mat4::perspective_rh(45_f32.to_radians(), self.aspect, 0.1, 100.0);

        CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}
