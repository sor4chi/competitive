use rand::{rngs::StdRng, Rng};

use crate::game::{Dir, Game};

use super::Solver;

pub struct RandomSolver {
    rng: StdRng,
}

impl RandomSolver {
    pub fn new(seed: u8) -> Self {
        Self {
            rng: rand::SeedableRng::from_seed([seed; 32]),
        }
    }
}

impl Solver for RandomSolver {
    fn get_move(&self, _game: &Game) -> Dir {
        let mut rng = self.rng.clone();
        match rng.gen_range(0..4) {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!(),
        }
    }
}
