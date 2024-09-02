use solver::{
    io::IO,
    solver::{
        construction::ConstructionSolver, greedy::GreedySolver, optimize_a::OptimizeASolver, Solver,
    },
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = ConstructionSolver::new(io, input);
    let output = solver.solve();
    output.write();
}
