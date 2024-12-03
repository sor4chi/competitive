pub mod estimation;
pub mod greedy;
pub mod row_packing;

pub trait Solver {
    fn solve(&mut self);
}
