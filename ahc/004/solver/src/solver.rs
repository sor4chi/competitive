pub mod beam;

pub trait Solver {
    fn solve(&mut self) -> Vec<Vec<char>>;
}
