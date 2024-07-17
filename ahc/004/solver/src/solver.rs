pub mod beam;

pub trait Solver {
    fn solve(&self) -> Vec<Vec<char>>;
}
