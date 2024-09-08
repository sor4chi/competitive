use crate::io::Output;

pub mod greedy;
pub mod beam;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
