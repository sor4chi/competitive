use solver::{
    io::IO,
    solver::{beam::BeamSolver, greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    // let mut solver = GreedySolver::new(io, input);
    let mut solver = BeamSolver::new(io, input);
    let output = solver.solve();
    output.write();
}
