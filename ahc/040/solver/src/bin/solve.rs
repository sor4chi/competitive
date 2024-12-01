use solver::{
    io::IO,
    solver::{row_packing::RowPackingSolver, Solver},
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let mut solver = RowPackingSolver::new(&input, &io);
    solver.solve()
}
