use crate::cube::{Face, FaceColor, RubiksCube};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Yellow = 1,
    Red = 2,
    Orange = 3,
    Green = 4,
    Blue = 5,
}

impl Color {
    pub fn from_face_color(fc: FaceColor) -> Self {
        match fc {
            FaceColor::White => Color::White,
            FaceColor::Yellow => Color::Yellow,
            FaceColor::Red => Color::Red,
            FaceColor::Orange => Color::Orange,
            FaceColor::Green => Color::Green,
            FaceColor::Blue => Color::Blue,
            FaceColor::Black => panic!("Black is an interior face"),
        }
    }
}

pub const U: usize = 0;
pub const D: usize = 1;
pub const F: usize = 2;
pub const B: usize = 3;
pub const R: usize = 4;
pub const L: usize = 5;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CubeState {
    pub faces: [[Color; 9]; 6],
}

impl CubeState {
    pub fn solved() -> Self {
        Self {
            faces: [
                [Color::White; 9],
                [Color::Yellow; 9],
                [Color::Red; 9],
                [Color::Orange; 9],
                [Color::Green; 9],
                [Color::Blue; 9],
            ],
        }
    }

    pub fn from_rubiks_cube(cube: &RubiksCube) -> Self {
        let mut faces = [[Color::White; 9]; 6];

        for cubie in &cube.cubies {
            let [x, y, z] = cubie.grid_pos;

            if y == 1 {
                let color = Color::from_face_color(cubie.face_colors[2]);
                let col = (x + 1) as usize;
                let row = (1 - z) as usize;
                faces[U][row * 3 + col] = color;
            }
            if y == -1 {
                let color = Color::from_face_color(cubie.face_colors[3]);
                let col = (x + 1) as usize;
                let row = (z + 1) as usize;
                faces[D][row * 3 + col] = color;
            }
            if z == 1 {
                let color = Color::from_face_color(cubie.face_colors[4]);
                let col = (x + 1) as usize;
                let row = (1 - y) as usize;
                faces[F][row * 3 + col] = color;
            }
            if z == -1 {
                let color = Color::from_face_color(cubie.face_colors[5]);
                let col = (1 - x) as usize;
                let row = (1 - y) as usize;
                faces[B][row * 3 + col] = color;
            }
            if x == 1 {
                let color = Color::from_face_color(cubie.face_colors[0]);
                let col = (1 - z) as usize;
                let row = (1 - y) as usize;
                faces[R][row * 3 + col] = color;
            }
            if x == -1 {
                let color = Color::from_face_color(cubie.face_colors[1]);
                let col = (z + 1) as usize;
                let row = (1 - y) as usize;
                faces[L][row * 3 + col] = color;
            }
        }

        Self { faces }
    }

    pub fn is_solved(&self) -> bool {
        self.faces.iter().all(|face| {
            let center = face[4];
            face.iter().all(|&s| s == center)
        })
    }

    pub fn apply_move(&self, face: Face, clockwise: bool) -> Self {
        let mut next = self.clone();
        next.apply_move_mut(face, clockwise);
        next
    }

    pub fn apply_move_mut(&mut self, face: Face, clockwise: bool) {
        match face {
            Face::Up => self.rotate_u(clockwise),
            Face::Down => self.rotate_d(clockwise),
            Face::Front => self.rotate_f(clockwise),
            Face::Back => self.rotate_b(clockwise),
            Face::Right => self.rotate_r(clockwise),
            Face::Left => self.rotate_l(clockwise),
        }
    }

    fn rotate_face_stickers(&mut self, f: usize, clockwise: bool) {
        let face = &mut self.faces[f];
        if clockwise {
            // 0→2→8→6, 1→5→7→3
            let tmp = face[0];
            face[0] = face[6];
            face[6] = face[8];
            face[8] = face[2];
            face[2] = tmp;

            let tmp = face[1];
            face[1] = face[3];
            face[3] = face[7];
            face[7] = face[5];
            face[5] = tmp;
        } else {
            // reverse
            let tmp = face[0];
            face[0] = face[2];
            face[2] = face[8];
            face[8] = face[6];
            face[6] = tmp;

            let tmp = face[1];
            face[1] = face[5];
            face[5] = face[7];
            face[7] = face[3];
            face[3] = tmp;
        }
    }

    fn rotate_u(&mut self, clockwise: bool) {
        self.rotate_face_stickers(U, clockwise);

        // Top row of F, R, B, L cycles
        // CW:  F-top → L-top → B-top → R-top → F-top
        // (each "top row" is stickers 0,1,2)
        let f = self.faces[F];
        let r = self.faces[R];
        let b = self.faces[B];
        let l = self.faces[L];

        if clockwise {
            // R-top ← F-top
            self.faces[R][0] = f[0];
            self.faces[R][1] = f[1];
            self.faces[R][2] = f[2];
            // B-top ← R-top
            self.faces[B][0] = r[0];
            self.faces[B][1] = r[1];
            self.faces[B][2] = r[2];
            // L-top ← B-top
            self.faces[L][0] = b[0];
            self.faces[L][1] = b[1];
            self.faces[L][2] = b[2];
            // F-top ← L-top
            self.faces[F][0] = l[0];
            self.faces[F][1] = l[1];
            self.faces[F][2] = l[2];
        } else {
            // L-top ← F-top
            self.faces[L][0] = f[0];
            self.faces[L][1] = f[1];
            self.faces[L][2] = f[2];
            // B-top ← L-top
            self.faces[B][0] = l[0];
            self.faces[B][1] = l[1];
            self.faces[B][2] = l[2];
            // R-top ← B-top
            self.faces[R][0] = b[0];
            self.faces[R][1] = b[1];
            self.faces[R][2] = b[2];
            // F-top ← R-top
            self.faces[F][0] = r[0];
            self.faces[F][1] = r[1];
            self.faces[F][2] = r[2];
        }
    }

    fn rotate_d(&mut self, clockwise: bool) {
        self.rotate_face_stickers(D, clockwise);

        // Bottom row (stickers 6,7,8) of F, L, B, R cycles
        let f = self.faces[F];
        let r = self.faces[R];
        let b = self.faces[B];
        let l = self.faces[L];

        if clockwise {
            // L-bot ← F-bot
            self.faces[L][6] = f[6];
            self.faces[L][7] = f[7];
            self.faces[L][8] = f[8];
            // B-bot ← L-bot
            self.faces[B][6] = l[6];
            self.faces[B][7] = l[7];
            self.faces[B][8] = l[8];
            // R-bot ← B-bot
            self.faces[R][6] = b[6];
            self.faces[R][7] = b[7];
            self.faces[R][8] = b[8];
            // F-bot ← R-bot
            self.faces[F][6] = r[6];
            self.faces[F][7] = r[7];
            self.faces[F][8] = r[8];
        } else {
            // R-bot ← F-bot
            self.faces[R][6] = f[6];
            self.faces[R][7] = f[7];
            self.faces[R][8] = f[8];
            // B-bot ← R-bot
            self.faces[B][6] = r[6];
            self.faces[B][7] = r[7];
            self.faces[B][8] = r[8];
            // L-bot ← B-bot
            self.faces[L][6] = b[6];
            self.faces[L][7] = b[7];
            self.faces[L][8] = b[8];
            // F-bot ← L-bot
            self.faces[F][6] = l[6];
            self.faces[F][7] = l[7];
            self.faces[F][8] = l[8];
        }
    }

    fn rotate_f(&mut self, clockwise: bool) {
        self.rotate_face_stickers(F, clockwise);

        // U-bottom, R-left, D-top, L-right cycle
        // U bottom = [6,7,8]
        // R left   = [0,3,6]
        // D top    = [2,1,0] (reversed)
        // L right  = [8,5,2] (reversed)
        let u = self.faces[U];
        let r = self.faces[R];
        let d = self.faces[D];
        let l = self.faces[L];

        if clockwise {
            // R-left ← U-bottom
            self.faces[R][0] = u[6];
            self.faces[R][3] = u[7];
            self.faces[R][6] = u[8];
            // D-top (reversed) ← R-left
            self.faces[D][2] = r[0];
            self.faces[D][1] = r[3];
            self.faces[D][0] = r[6];
            // L-right (reversed) ← D-top
            self.faces[L][8] = d[2];
            self.faces[L][5] = d[1];
            self.faces[L][2] = d[0];
            // U-bottom ← L-right
            self.faces[U][6] = l[8];
            self.faces[U][7] = l[5];
            self.faces[U][8] = l[2];
        } else {
            // L-right ← U-bottom
            self.faces[L][8] = u[6];
            self.faces[L][5] = u[7];
            self.faces[L][2] = u[8];
            // D-top ← L-right (reversed)
            self.faces[D][0] = l[8];
            self.faces[D][1] = l[5];
            self.faces[D][2] = l[2];
            // R-left (reversed) ← D-top
            self.faces[R][0] = d[2];
            self.faces[R][3] = d[1];
            self.faces[R][6] = d[0];
            // U-bottom ← R-left
            self.faces[U][6] = r[0];
            self.faces[U][7] = r[3];
            self.faces[U][8] = r[6];
        }
    }

    fn rotate_b(&mut self, clockwise: bool) {
        self.rotate_face_stickers(B, clockwise);

        // U-top, L-left, D-bottom, R-right cycle
        // U top    = [0,1,2]
        // L left   = [0,3,6]
        // D bottom = [8,7,6] (reversed)
        // R right  = [2,5,8]
        let u = self.faces[U];
        let r = self.faces[R];
        let d = self.faces[D];
        let l = self.faces[L];

        if clockwise {
            // L-left ← U-top (reversed)
            self.faces[L][0] = u[2];
            self.faces[L][3] = u[1];
            self.faces[L][6] = u[0];
            // D-bottom ← L-left
            self.faces[D][8] = l[0];
            self.faces[D][7] = l[3];
            self.faces[D][6] = l[6];
            // R-right (reversed) ← D-bottom
            self.faces[R][2] = d[8];
            self.faces[R][5] = d[7];
            self.faces[R][8] = d[6];
            // U-top ← R-right
            self.faces[U][0] = r[2];
            self.faces[U][1] = r[5];
            self.faces[U][2] = r[8];
        } else {
            // R-right ← U-top
            self.faces[R][2] = u[0];
            self.faces[R][5] = u[1];
            self.faces[R][8] = u[2];
            // D-bottom (reversed) ← R-right
            self.faces[D][6] = r[2];
            self.faces[D][7] = r[5];
            self.faces[D][8] = r[8];
            // L-left ← D-bottom
            self.faces[L][0] = d[6];
            self.faces[L][3] = d[7];
            self.faces[L][6] = d[8];
            // U-top (reversed) ← L-left
            self.faces[U][0] = l[6];
            self.faces[U][1] = l[3];
            self.faces[U][2] = l[0];
        }
    }

    fn rotate_r(&mut self, clockwise: bool) {
        self.rotate_face_stickers(R, clockwise);

        // U-right, B-left(reversed), D-right, F-right cycle
        // U right = [2,5,8]
        // F right = [2,5,8]
        // D right = [2,5,8]
        // B left  = [6,3,0] (reversed because B is mirrored)
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];
        let b = self.faces[B];

        if clockwise {
            // F-right ← U-right
            self.faces[F][2] = u[2];
            self.faces[F][5] = u[5];
            self.faces[F][8] = u[8];
            // D-right ← F-right
            self.faces[D][2] = f[2];
            self.faces[D][5] = f[5];
            self.faces[D][8] = f[8];
            // B-left (reversed) ← D-right
            self.faces[B][6] = d[2];
            self.faces[B][3] = d[5];
            self.faces[B][0] = d[8];
            // U-right ← B-left (reversed)
            self.faces[U][2] = b[6];
            self.faces[U][5] = b[3];
            self.faces[U][8] = b[0];
        } else {
            // B-left (reversed) ← U-right
            self.faces[B][6] = u[2];
            self.faces[B][3] = u[5];
            self.faces[B][0] = u[8];
            // D-right ← B-left (reversed)
            self.faces[D][2] = b[6];
            self.faces[D][5] = b[3];
            self.faces[D][8] = b[0];
            // F-right ← D-right
            self.faces[F][2] = d[2];
            self.faces[F][5] = d[5];
            self.faces[F][8] = d[8];
            // U-right ← F-right
            self.faces[U][2] = f[2];
            self.faces[U][5] = f[5];
            self.faces[U][8] = f[8];
        }
    }

    fn rotate_l(&mut self, clockwise: bool) {
        self.rotate_face_stickers(L, clockwise);

        // U-left, F-left, D-left, B-right(reversed) cycle
        // U left  = [0,3,6]
        // F left  = [0,3,6]
        // D left  = [0,3,6]
        // B right = [2,5,8] reversed → [8,5,2]
        let u = self.faces[U];
        let f = self.faces[F];
        let d = self.faces[D];
        let b = self.faces[B];

        if clockwise {
            // B-right (reversed) ← U-left
            self.faces[B][8] = u[0];
            self.faces[B][5] = u[3];
            self.faces[B][2] = u[6];
            // D-left ← B-right (reversed)
            self.faces[D][0] = b[8];
            self.faces[D][3] = b[5];
            self.faces[D][6] = b[2];
            // F-left ← D-left
            self.faces[F][0] = d[0];
            self.faces[F][3] = d[3];
            self.faces[F][6] = d[6];
            // U-left ← F-left
            self.faces[U][0] = f[0];
            self.faces[U][3] = f[3];
            self.faces[U][6] = f[6];
        } else {
            // F-left ← U-left
            self.faces[F][0] = u[0];
            self.faces[F][3] = u[3];
            self.faces[F][6] = u[6];
            // D-left ← F-left
            self.faces[D][0] = f[0];
            self.faces[D][3] = f[3];
            self.faces[D][6] = f[6];
            // B-right (reversed) ← D-left
            self.faces[B][8] = d[0];
            self.faces[B][5] = d[3];
            self.faces[B][2] = d[6];
            // U-left ← B-right (reversed)
            self.faces[U][0] = b[8];
            self.faces[U][3] = b[5];
            self.faces[U][6] = b[2];
        }
    }
}
