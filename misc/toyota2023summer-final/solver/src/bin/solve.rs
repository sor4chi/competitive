use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};
use solver::{solver::greedy::GreedySolver, Solver};

extern crate solver;

fn main() {
    let mut solver = GreedySolver::new();
    solver.solve();
}
