use solver::{
    io::Input,
    solver::{greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let input = Input::read();
    let mut solver = GreedySolver::new(input);
    let output = solver.solve();
    output.write();
}
