pub mod beam;
pub mod greedy;
pub mod greedy_sep;
pub mod insert;

pub trait Policy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>);
}
