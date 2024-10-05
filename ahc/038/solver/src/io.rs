use std::fmt::Display;

use proconio::input;

use crate::game::{ArmNodeID, ArmTree, Direction};

pub struct Input {
    pub n: usize,
    pub m: usize,
    pub v: usize,
    pub s: Vec<Vec<bool>>,
    pub t: Vec<Vec<bool>>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
            m: usize,
            v: usize,
            // s: [[usize; m]; n],
            s: [String; n],
            t: [String; n],
        }

        Input {
            n,
            m,
            v,
            s: s.iter()
                .map(|s| s.chars().map(|c| c == '1').collect())
                .collect(),
            t: t.iter()
                .map(|t| t.chars().map(|c| c == '1').collect())
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rotate {
    Stay,
    Left,
    Right,
}

impl Display for Rotate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotate::Left => write!(f, "L"),
            Rotate::Right => write!(f, "R"),
            Rotate::Stay => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Shift(Direction),
    Stay,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Shift(Direction::Up) => write!(f, "U"),
            Move::Shift(Direction::Down) => write!(f, "D"),
            Move::Shift(Direction::Left) => write!(f, "L"),
            Move::Shift(Direction::Right) => write!(f, "R"),
            Move::Stay => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    PickOrRelease,
    Stay,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::PickOrRelease => write!(f, "P"),
            Action::Stay => write!(f, "."),
        }
    }
}

pub struct Operation {
    pub move_to: Move,
    pub rotates: Vec<Rotate>,
    pub actions: Vec<Action>,
}

pub struct Output {
    pub flatten_tree: Vec<(ArmNodeID, usize)>,
    pub initial_pos: (usize, usize),
    pub operations: Vec<Operation>,
}

impl Output {
    pub fn write(&self) {
        println!("{}", self.flatten_tree.len() + 1);
        for (parent, len) in &self.flatten_tree {
            println!("{} {}", parent.0, *len);
        }
        println!("{} {}", self.initial_pos.0, self.initial_pos.1);

        for op in &self.operations {
            println!(
                "{}{}{}",
                op.move_to,
                op.rotates.iter().map(|r| r.to_string()).collect::<String>(),
                op.actions.iter().map(|a| a.to_string()).collect::<String>()
            );
        }
    }
}
