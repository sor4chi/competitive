use solver::{
    io::IO,
    solver::{bayesian::BayesianSolver, greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::new();
    let input = io.read();
    if input.m == 2 || (input.m == 3 && input.n <= 10) {
        let mut solver = BayesianSolver::new(io, input);
        solver.solve();
        return;
    }
    let mut solver = GreedySolver::new(io, input);
    solver.solve();
}
