use std::{fmt::Display, io::Write};

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use crate::io::{Direction, Input, Operation, Query, Rotation, IO};

use std::fs::File;

use super::Solver;

pub struct RowPackingSolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl RowPackingSolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> RowPackingSolver<'a> {
        RowPackingSolver { input, io }
    }
}

impl Solver for RowPackingSolver<'_> {
    fn solve(&mut self) {
        // rectsを一列に並べてpackingする
        let mut operations = vec![];
        for i in 0..self.input.N {
            operations.push(Operation {
                p: i,
                r: Rotation::Stay,
                d: Direction::Up,
                b: if i % ((self.input.N as f32).sqrt().ceil() as usize) == 0 {
                    -1
                } else {
                    (i - 1) as isize
                },
            });
        }
        let seed = 42;
        let mut rng = Pcg64Mcg::new(seed);
        for i in 0..self.input.T {
            self.io.measure(&Query {
                operations: operations.clone(),
            });
            let mut current_opeartions = operations.clone();
            let mut perm = (0..self.input.N).collect::<Vec<_>>();
            perm.shuffle(&mut rng);
            let selects = rng.gen_range(1..=self.input.N);
            for i in 0..selects {
                let p = perm[i];
                let mut op = current_opeartions[p].clone();
                op.r = match op.r {
                    Rotation::Stay => Rotation::Rotate,
                    Rotation::Rotate => Rotation::Stay,
                };
                current_opeartions[p] = op;
            }
            operations = current_opeartions;
        }
    }
}
