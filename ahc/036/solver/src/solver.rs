use crate::io::Output;

pub mod construction;
pub mod greedy;
pub mod optimize_a;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
