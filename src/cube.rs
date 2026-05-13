use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, Debug)]
pub enum FaceColor {
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
    Black,
}

impl FaceColor {
    pub fn to_rgb(self) -> [f32; 3] {
        match self {
            FaceColor::White => [1.00, 1.00, 1.00],
            FaceColor::Yellow => [1.00, 1.00, 0.00],
            FaceColor::Red => [0.90, 0.00, 0.00],
            FaceColor::Orange => [1.00, 0.45, 0.00],
            FaceColor::Green => [0.00, 0.75, 0.00],
            FaceColor::Blue => [0.00, 0.00, 0.90],
            FaceColor::Black => [0.08, 0.08, 0.08],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Face {
    Right,
    Left,
    Up,
    Down,
    Front,
    Back,
}

impl Face {
    pub fn axis(self) -> Vec3 {
        match self {
            Face::Right => Vec3::X,
            Face::Left => Vec3::NEG_X,
            Face::Up => Vec3::Y,
            Face::Down => Vec3::NEG_Y,
            Face::Front => Vec3::Z,
            Face::Back => Vec3::NEG_Z,
        }
    }

    pub fn layer(self) -> (usize, i32) {
        match self {
            Face::Right => (0, 1),
            Face::Left => (0, -1),
            Face::Up => (1, 1),
            Face::Down => (1, -1),
            Face::Front => (2, 1),
            Face::Back => (2, -1),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cubie {
    pub grid_pos: [i32; 3],
    pub transform: Mat4,
    pub face_colors: [FaceColor; 6],
}

impl Cubie {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let spacing = 1.05_f32;
        Self {
            grid_pos: [x, y, z],
            transform: Mat4::from_translation(Vec3::new(
                x as f32 * spacing,
                y as f32 * spacing,
                z as f32 * spacing,
            )),
            face_colors: [
                if x == 1 {
                    FaceColor::Green
                } else {
                    FaceColor::Black
                },
                if x == -1 {
                    FaceColor::Blue
                } else {
                    FaceColor::Black
                },
                if y == 1 {
                    FaceColor::White
                } else {
                    FaceColor::Black
                },
                if y == -1 {
                    FaceColor::Yellow
                } else {
                    FaceColor::Black
                },
                if z == 1 {
                    FaceColor::Red
                } else {
                    FaceColor::Black
                },
                if z == -1 {
                    FaceColor::Orange
                } else {
                    FaceColor::Black
                },
            ],
        }
    }
}

struct Animation {
    face: Face,
    clockwise: bool,
    angle: f32,
}

pub struct RubiksCube {
    pub cubies: Vec<Cubie>,
    anim: Option<Animation>,
    anim_speed: f32,
}

impl RubiksCube {
    pub fn new() -> Self {
        let mut cubies = Vec::with_capacity(26);

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    cubies.push(Cubie::new(x, y, z));
                }
            }
        }

        Self {
            cubies,
            anim: None,
            anim_speed: std::f32::consts::PI * 2.5,
        }
    }

    pub fn is_animating(&self) -> bool {
        self.anim.is_some()
    }

    pub fn rotate_face(&mut self, face: Face, clockwise: bool) {
        if self.anim.is_some() {
            return;
        }
        self.anim = Some(Animation {
            face,
            clockwise,
            angle: 0.0,
        })
    }

    pub fn update(&mut self, dt: f32) {
        let Some(anim) = self.anim.as_mut() else {
            return;
        };

        let target = std::f32::consts::FRAC_PI_2;
        anim.angle += self.anim_speed * dt;

        if anim.angle >= target {
            let face = anim.face;
            let clockwise = anim.clockwise;
            self.anim = None;
            self.finalize_rotation(face, clockwise);
        }
    }

    pub fn anim_rotation(&self) -> Option<(Mat4, Face)> {
        let anim = self.anim.as_ref()?;

        let angle = if anim.clockwise {
            -anim.angle
        } else {
            anim.angle
        };
        let mat = Mat4::from_axis_angle(anim.face.axis(), angle);

        Some((mat, anim.face))
    }

    pub fn is_in_anim_face(&self, cubie_index: usize) -> bool {
        let Some(anim) = &self.anim else { return false };
        self.cubie_in_face(cubie_index, anim.face)
    }

    fn cubie_in_face(&self, index: usize, face: Face) -> bool {
        let (axis, val) = face.layer();
        self.cubies[index].grid_pos[axis] == val
    }

    fn indices_in_face(&self, face: Face) -> Vec<usize> {
        let (axis, val) = face.layer();
        self.cubies
            .iter()
            .enumerate()
            .filter(|(_, c)| c.grid_pos[axis] == val)
            .map(|(i, _)| i)
            .collect()
    }

    fn finalize_rotation(&mut self, face: Face, clockwise: bool) {
        let angle = if clockwise {
            -std::f32::consts::FRAC_PI_2
        } else {
            std::f32::consts::FRAC_PI_2
        };
        let rot_mat = Mat4::from_axis_angle(face.axis(), angle);
        let rot_quat = Quat::from_axis_angle(face.axis(), angle);

        for i in self.indices_in_face(face) {
            let cubie = &mut self.cubies[i];

            cubie.transform = rot_mat * cubie.transform;

            let old_pos = Vec3::new(
                cubie.grid_pos[0] as f32,
                cubie.grid_pos[1] as f32,
                cubie.grid_pos[2] as f32,
            );
            let new_pos = rot_quat * old_pos;
            cubie.grid_pos = [
                new_pos.x.round() as i32,
                new_pos.y.round() as i32,
                new_pos.z.round() as i32,
            ];

            cubie.face_colors = permute_colors(cubie.face_colors, face, clockwise);
        }
    }
}

fn permute_colors(c: [FaceColor; 6], face: Face, clockwise: bool) -> [FaceColor; 6] {
    // Index reference:
    // 0=+X  1=-X  2=+Y  3=-Y  4=+Z  5=-Z

    let [px, nx, py, ny, pz, nz] = c;

    match (face, clockwise) {
        // CW looking from +X: +Y goes to +Z, +Z goes to -Y, -Y goes to -Z, -Z goes to +Y
        // new[+X,-X, +Y, -Y, +Z, -Z]
        (Face::Right, true)  => [px, nx,  nz, pz,  py, ny],
        (Face::Right, false) => [px, nx,  pz, nz,  ny, py],

        // Left is -X axis, CW from outside = CCW around +X
        (Face::Left,  true)  => [px, nx,  pz, nz,  ny, py],
        (Face::Left,  false) => [px, nx,  nz, pz,  py, ny],

        // CW looking from +Y: +Z goes to +X, +X goes to -Z, -Z goes to -X, -X goes to +Z
        (Face::Up,   true)  => [ pz, nz, py, ny,  nx, px],
        (Face::Up,   false) => [ nz, pz, py, ny,  px, nx],

        // Down is -Y axis, CW from outside = CCW around +Y
        (Face::Down, true)  => [ nz, pz, py, ny,  px, nx],
        (Face::Down, false) => [ pz, nz, py, ny,  nx, px],

        // CW looking from +Z: +X goes to +Y, +Y goes to -X, -X goes to -Y, -Y goes to +X
        (Face::Front, true)  => [ py, ny,  nx, px,  pz, nz],
        (Face::Front, false) => [ ny, py,  px, nx,  pz, nz],

        // Back is -Z axis, CW from outside = CCW around +Z
        (Face::Back,  true)  => [ ny, py,  px, nx,  pz, nz],
        (Face::Back,  false) => [ py, ny,  nx, px,  pz, nz],
    }
}
