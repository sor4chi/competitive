use solver::{
    io::IO,
    solver::{estimation::EstimationSolver, row_packing::RowPackingSolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = EstimationSolver::new(&input, &io);
    solver.solve()
}
