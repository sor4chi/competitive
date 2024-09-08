use crate::io::Operations;

pub const DIRECTIONS: &[(i32, i32, Operations); 4] = &[
    (0, -1, Operations::Left),
    (0, 1, Operations::Right),
    (-1, 0, Operations::Up),
    (1, 0, Operations::Down),
];

pub struct Board {
    pub size: usize,
    pub wall_h: Vec<Vec<bool>>,
    pub wall_v: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(size: usize, h: Vec<Vec<u8>>, v: Vec<Vec<u8>>) -> Self {
        let mut wall_h = vec![vec![false; size]; size];
        for i in 0..size {
            for j in 0..size - 1 {
                wall_h[i][j] = h[i][j] == b'1';
            }
        }

        let mut wall_v = vec![vec![false; size]; size];
        for i in 0..size - 1 {
            for j in 0..size {
                wall_v[i][j] = v[i][j] == b'1';
            }
        }

        Board {
            size,
            wall_h,
            wall_v,
        }
    }

    pub fn can_move(&self, i: usize, j: usize, dir: Operations) -> bool {
        match dir {
            Operations::Left => j > 0 && !self.wall_h[i][j - 1],
            Operations::Right => j < self.size - 1 && !self.wall_h[i][j],
            Operations::Up => i > 0 && !self.wall_v[i - 1][j],
            Operations::Down => i < self.size - 1 && !self.wall_v[i][j],
        }
    }

    pub fn nexts(&self, i: usize, j: usize) -> Vec<(usize, usize, Operations)> {
        let mut nexts = vec![];
        for &(di, dj, dir) in DIRECTIONS {
            if self.can_move(i, j, dir) {
                nexts.push((i + di as usize, j + dj as usize, dir));
            }
        }
        nexts
    }
}
