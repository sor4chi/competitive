use crate::io::{Input, Output, IO};

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
    fn solve(&mut self) -> Output {
        unimplemented!()
    }
}
