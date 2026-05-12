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

#[derive(Clone, Debug)]
pub struct Cubie {
    pub grid_pos: [i32; 3],
    pub face_colors: [FaceColor; 6],
}

impl Cubie {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            grid_pos: [x, y, z],
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

pub struct RubiksCube {
    pub cubies: Vec<Cubie>,
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

        Self { cubies }
    }
}
