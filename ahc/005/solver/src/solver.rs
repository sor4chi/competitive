use std::fmt::{self, Display};

pub mod greedy;
pub mod tsp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,    // U
    Down,  // D
    Left,  // L
    Right, // R
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Left => "L",
                Direction::Right => "R",
            }
        )
    }
}

pub trait Solver {
    fn solve(&self) -> Vec<Direction>;
}
