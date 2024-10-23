use crate::io::{Input, Output, IO};

use super::Solver;

pub struct GreedySolver<'a> {
    input: &'a Input,
}

impl GreedySolver<'_> {
    pub fn new(input: &Input) -> GreedySolver {
        GreedySolver { input }
    }
}

impl Solver for GreedySolver<'_> {
    fn solve(&mut self) -> Output {
        unimplemented!()
    }
}
