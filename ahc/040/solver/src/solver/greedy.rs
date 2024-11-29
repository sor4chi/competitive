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
        let mut measurements = vec![];
        let mut operations = vec![];
        for i in 0..self.input.N.min(self.input.T) {
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
            let res = self.io.measure(&Query {
                operations: operations.clone(),
            });
            measurements.push(res);
            eprintln!("{} {}", res.0, res.1);
        }
        for _ in self.input.N..self.input.T {
            self.io.measure(&Query {
                operations: operations.clone(),
            });
        }
    }
}
