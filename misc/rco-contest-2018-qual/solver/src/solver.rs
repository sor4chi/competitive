use crate::io::Output;

pub mod beam;
pub mod greedy;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
