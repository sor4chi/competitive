use std::{
    fmt::Display,
    io::{stdin, BufReader},
};

use proconio::{input, source::line::LineSource};

pub const BOARD_SIZE: usize = 50;

pub struct Input {
    pub si: usize,
    pub sj: usize,
    pub t: Vec<Vec<usize>>,
    pub p: Vec<Vec<usize>>,
}

impl Input {
    pub fn read() -> Self {
        let stdin = stdin();
        let mut source = LineSource::new(BufReader::new(stdin.lock()));

        input! {
            from &mut source,
            si: usize,
            sj: usize,
            t: [[usize; BOARD_SIZE]; BOARD_SIZE],
            p: [[usize; BOARD_SIZE]; BOARD_SIZE],
        }

        Self { si, sj, t, p }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{}", c)
    }
}

pub struct Output {
    pub directions: Vec<Direction>,
}

impl Output {
    pub fn write(&self) {
        for d in &self.directions {
            print!("{}", d);
        }
        println!();
    }
}
