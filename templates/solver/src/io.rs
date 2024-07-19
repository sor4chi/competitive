use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

pub struct Input {
    pub n: usize,
}

impl Input {
    pub fn read() -> Self {
        let stdin = stdin();
        let mut source = LineSource::new(BufReader::new(stdin.lock()));

        input! {
            from &mut source,
            n: usize,
        }

        Self { n }
    }
}

pub struct Output {}

impl Output {
    pub fn write(&self) {}
}
