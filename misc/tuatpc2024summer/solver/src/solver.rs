use crate::io::Output;

pub mod anneal;
pub mod greedy;
pub mod construct;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
