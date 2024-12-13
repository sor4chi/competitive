#![allow(non_snake_case)]

use std::fmt::Display;

use proconio::input_interactive;

pub struct Input {
    pub N: usize,
    pub T: usize,
    pub sigma: usize,
    pub rects: Vec<(usize, usize)>,
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub enum Rotation {
    Stay,
    Rotate,
}

impl Rotation {
    pub fn flip(&mut self) {
        *self = match self {
            Rotation::Stay => Rotation::Rotate,
            Rotation::Rotate => Rotation::Stay,
        };
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Stay => write!(f, "0"),
            Rotation::Rotate => write!(f, "1"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub enum Direction {
    Up,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "U"),
            Direction::Left => write!(f, "L"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub struct Operation {
    pub p: usize,
    pub r: Rotation,
    pub d: Direction,
    pub b: isize,
}

pub struct Query {
    pub operations: Vec<Operation>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input_interactive! {
            N: usize,
            T: usize,
            sigma: usize,
            rects: [(usize, usize); N],
        }

        Input { N, T, sigma, rects }
    }

    pub fn measure(&self, query: &Query) -> (usize, usize) {
        println!("{}", query.operations.len());
        for op in &query.operations {
            println!("{} {} {} {}", op.p, op.r, op.d, op.b);
        }

        input_interactive! {
            w: usize,
            h: usize,
        }

        (w, h)
    }

    pub fn comment(&self, comment: &str) {
        println!("#{}", comment);
    }
}
