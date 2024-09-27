use solver::{
    io::IO,
    solver::{anneal::AnnealSolver, construct::ConstructSolver, greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    // let mut solver = GreedySolver::new(io.clone(), input.clone());
    // let output1 = solver.solve();
    // let mut solver = AnnealSolver::new(io.clone(), input.clone());
    // let output2 = solver.solve();
    // if output1.score > output2.score {
    //     output1.write();
    //     eprintln!("Score = {}", output1.score);
    // } else {
    //     output2.write();
    //     eprintln!("Score = {}", output2.score);
    // }
    let mut solver = ConstructSolver::new(io, input);
    let output3 = solver.solve();
    output3.write();
    eprintln!("Score = {}", output3.score);
}
