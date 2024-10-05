use solver::{
    io::IO,
    solver::{one_op::OneOPSolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = OneOPSolver::new(io, input);
    let output = solver.solve();
    output.write();
}
