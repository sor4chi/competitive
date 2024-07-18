pub mod original_lib;
pub mod solver;

pub trait Solver {
    fn solve(&mut self);
}

pub struct IDGenerator {
    count: usize,
}

impl IDGenerator {
    pub fn new() -> Self {
        IDGenerator { count: 0 }
    }

    pub fn generate(&mut self) -> usize {
        let id = self.count;
        self.count += 1;
        id
    }
}
