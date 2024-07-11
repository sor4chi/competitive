pub mod beam;
pub mod greedy;
pub mod greedy_sep;

pub trait Policy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>);
}
