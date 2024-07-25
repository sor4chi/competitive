pub mod bayesian;
pub mod greedy;
pub mod greedy_2;

pub trait Solver {
    fn solve(&mut self);
}
