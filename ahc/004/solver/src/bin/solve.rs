use solver::{
    solver::{beam::BeamSolver, Solver},
    Input,
};

extern crate solver;

fn main() {
    let input = Input::default();
    let mut solver = BeamSolver::new(input);
    let result = solver.solve();
    for row in result {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
