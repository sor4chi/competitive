use rand::{seq::SliceRandom, Rng};

use crate::io::{Direction, Input, Operation, Query, Rotation, IO};

use super::Solver;

pub struct GreedySolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl GreedySolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> GreedySolver<'a> {
        GreedySolver { input, io }
    }
}

impl Solver for GreedySolver<'_> {
    fn solve(&mut self) {
        let mut operations = vec![];
        for i in 0..self.input.N {
            let mut rotation = Rotation::Stay;
            if self.input.rects[i].0 < self.input.rects[i].1 {
                rotation = Rotation::Rotate;
            }
            operations.push(Operation {
                p: i,
                r: rotation,
                d: Direction::Up,
                b: -1,
            });
        }
        let measure = self.io.measure(&Query {
            operations: operations.clone(),
        });
        let mut best_score = measure.0 + measure.1;
        let mut best_operations = operations.clone();
        let mut perm = (0..self.input.N).collect::<Vec<_>>();
        let mut seed = [0; 32];
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
        for _ in 0..self.input.T - 1 {
            perm.shuffle(&mut rng);
            let mut new_operations = best_operations.clone();
            for i in 0..self.input.N / 4 {
                let p = perm[i];
                let rotation = if rng.gen::<bool>() {
                    Rotation::Rotate
                } else {
                    Rotation::Stay
                };
                new_operations[p].r = rotation;
                let b = rng.gen_range(-1..p as isize);
                new_operations[p].b = b;
                let d = if rng.gen::<bool>() {
                    Direction::Up
                } else {
                    Direction::Left
                };
                new_operations[p].d = d;
            }
            let measure = self.io.measure(&Query {
                operations: new_operations.clone(),
            });
            let score = measure.0 + measure.1;
            if score < best_score {
                best_score = score;
                best_operations = new_operations.clone();
            }
        }
    }
}
