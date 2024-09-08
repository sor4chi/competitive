use std::fmt::Display;

use proconio::{
    input,
    marker::{Bytes, Chars},
};

pub struct Input {
    pub si: usize,
    pub sj: usize,
    pub ti: usize,
    pub tj: usize,
    pub p: f64,
    pub h: Vec<Vec<u8>>,
    pub v: Vec<Vec<u8>>,
}

pub const BOARD_SIZE: usize = 20;
pub const MAX_OPERATIONS: usize = 200;

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            si: usize,
            sj: usize,
            ti: usize,
            tj: usize,
            p: f64,
            h: [Bytes; BOARD_SIZE],
            v: [Bytes; BOARD_SIZE - 1],
        }

        Input {
            si,
            sj,
            ti,
            tj,
            p,
            h,
            v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operations {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Left => write!(f, "L"),
            Operations::Right => write!(f, "R"),
            Operations::Up => write!(f, "U"),
            Operations::Down => write!(f, "D"),
        }
    }
}

pub struct Output {
    pub operations: Vec<Operations>,
}

impl Output {
    pub fn write(&self) {
        for operation in &self.operations {
            print!("{}", operation);
        }
        println!();
    }
}
