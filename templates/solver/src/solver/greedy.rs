use crate::io::{Input, Output};

use super::Solver;

pub struct GreedySolver {
    input: Input,
}

impl GreedySolver {
    pub fn new(input: Input) -> Self {
        GreedySolver { input }
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        unimplemented!()
    }
}
