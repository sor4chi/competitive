use proconio::input;

pub mod game;
pub mod graph;
pub mod util;

pub struct Input {
    pub n: usize,
    pub s: (usize, usize),
    pub c: Vec<Vec<char>>,
}

pub fn parse_input() -> Input {
    input! {
        n: usize,
        s: (usize, usize),
        c: [proconio::marker::Chars; n],
    }

    Input { n, s, c }
}
