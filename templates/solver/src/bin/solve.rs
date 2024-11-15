use solver::{
    io::IO,
    solver::{greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = GreedySolver::new(&input);
    let output = solver.solve();
    io.write(&output);
}
