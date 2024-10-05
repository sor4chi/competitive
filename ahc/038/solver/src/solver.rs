use crate::io::Output;

pub mod one_op;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
