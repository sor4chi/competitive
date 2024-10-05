use solver::{
    io::IO,
    solver::{multi_op::MultiOPSolver, one_op::OneOPSolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = MultiOPSolver::new(io, input);
    let output = solver.solve();
    output.write();
}
