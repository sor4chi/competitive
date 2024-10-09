use std::fmt::Display;

use proconio::input;

use crate::game::{ArmNodeID, ArmTree, Direction};

#[derive(Clone)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub v: usize,
    pub s: Vec<Vec<bool>>,
    pub t: Vec<Vec<bool>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Rotate {
    pub fn reverse(&self) -> Rotate {
        match self {
            Rotate::Left => Rotate::Right,
            Rotate::Right => Rotate::Left,
            Rotate::Stay => Rotate::Stay,
        }
    }

    fn idx(&self) -> usize {
        match self {
            Rotate::Left => 0,
            Rotate::Stay => 1,
            Rotate::Right => 2,
        }
    }

    pub fn diff(&self, other: Rotate) -> usize {
        let diff = (self.idx() as i32 - other.idx() as i32).abs();
        diff as usize
    }

    // 目標にあわせるための回転操作列を返す
    pub fn align(&self, other: Rotate) -> Vec<Rotate> {
        let real_diff = other.idx() as i32 - self.idx() as i32;
        match real_diff {
            0 => vec![],
            1 => vec![Rotate::Right],
            -1 => vec![Rotate::Left],
            2 => vec![Rotate::Right, Rotate::Right],
            -2 => vec![Rotate::Left, Rotate::Left],
            _ => panic!("Invalid diff: {}", real_diff),
        }
    }
}

#[test]
fn test_rotate_diff() {
    assert_eq!(Rotate::Left.diff(Rotate::Right), 2);
    assert_eq!(Rotate::Left.diff(Rotate::Left), 0);
    assert_eq!(Rotate::Left.diff(Rotate::Stay), 1);
    assert_eq!(Rotate::Right.diff(Rotate::Stay), 1);
    assert_eq!(Rotate::Stay.diff(Rotate::Stay), 0);
    assert_eq!(Rotate::Stay.diff(Rotate::Right), 1);
}

#[test]
fn test_rotate_align() {
    assert_eq!(
        Rotate::Left.align(Rotate::Right),
        vec![Rotate::Right, Rotate::Right]
    );
    assert_eq!(Rotate::Left.align(Rotate::Left), vec![]);
    assert_eq!(Rotate::Left.align(Rotate::Stay), vec![Rotate::Right]);
    assert_eq!(Rotate::Right.align(Rotate::Stay), vec![Rotate::Left]);
    assert_eq!(Rotate::Stay.align(Rotate::Stay), vec![]);
    assert_eq!(Rotate::Stay.align(Rotate::Right), vec![Rotate::Right]);
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

#[derive(Clone)]
pub struct Operation {
    pub move_to: Move,
    pub rotates: Vec<Rotate>,
    pub actions: Vec<Action>,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.move_to,
            self.rotates
                .iter()
                .map(|r| r.to_string())
                .collect::<String>(),
            self.actions
                .iter()
                .map(|a| a.to_string())
                .collect::<String>()
        )
    }
}

pub struct Output {
    pub flatten_tree: Vec<(ArmNodeID, usize)>,
    pub initial_pos: (usize, usize),
    pub operations: Vec<Operation>,
}

impl Output {}

#[derive(Default, Clone)]
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
    pub fn write(&self, output: &Output) {
        println!("{}", output.flatten_tree.len() + 1);
        for (parent, len) in &output.flatten_tree {
            println!("{} {}", parent.0, *len);
        }
        println!("{} {}", output.initial_pos.0, output.initial_pos.1);

        for op in &output.operations {
            println!("{}", op);
        }
    }
}
