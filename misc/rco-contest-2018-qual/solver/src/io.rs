use std::fmt::{write, Display};

use proconio::{input, marker::Chars};

pub struct Input {
    /// 100
    pub n: usize,
    /// 8
    pub k: usize,
    /// 50
    pub h: usize,
    /// 50
    pub w: usize,
    /// 2500
    pub t: usize,
    /// 50 x 50
    pub maps: Vec<Vec<Vec<char>>>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
            k: usize,
            h: usize,
            w: usize,
            t: usize,
            rows: [[Chars; h]; n],
        }

        let maps = rows;

        Input {
            n,
            k,
            h,
            w,
            t,
            maps,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_char(&self) -> char {
        match self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.to_char();
        write!(f, "{}", c)
    }
}

pub const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

pub struct Output {
    pub m: Vec<usize>,
    pub commands: Vec<Dir>,
}

impl Output {
    pub fn write(&self) {
        let s = self
            .m
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", s);
        let s = self
            .commands
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("{}", s);
    }
}
