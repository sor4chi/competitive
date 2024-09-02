use std::fmt::Display;

use proconio::input;

pub struct Input {
    pub n: usize,
    pub m: usize,
    pub t: usize,
    pub la: usize,
    pub lb: usize,
    pub edges: Vec<(usize, usize)>,
    pub ts: Vec<usize>,
    pub nodes: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
            m: usize,
            t: usize,
            la: usize,
            lb: usize,
            edges: [(usize, usize); m],
            ts: [usize; t],
            nodes: [(usize, usize); n],
        }

        Input {
            n,
            m,
            t,
            la,
            lb,
            edges,
            ts,
            nodes,
        }
    }
}

#[derive(Clone)]
pub struct SignalUpdate {
    pub len: usize,
    pub ai: usize,
    pub bi: usize,
}

#[derive(Clone)]
pub enum Operation {
    Comment(String),
    SignalUpdate(SignalUpdate),
    Move(usize),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::SignalUpdate(SignalUpdate { len, ai, bi }) => {
                write!(f, "s {} {} {}", len, ai, bi)
            }
            Operation::Move(node) => write!(f, "m {}", node),
            Operation::Comment(comment) => write!(f, "# {}", comment),
        }
    }
}

pub struct Output {
    pub a: Vec<usize>,
    pub operations: Vec<Operation>,
}

impl Output {
    pub fn write(&self) {
        println!(
            "{}",
            self.a
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        for op in &self.operations {
            println!("{}", op);
        }
    }
}
