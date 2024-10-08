use crate::io::Output;

pub mod bulk_arm;
pub mod multi_arm_tree;
pub mod multi_op;
pub mod one_arm_tree;
pub mod one_op;

pub trait Solver {
    fn solve(&mut self) -> Output;
}
