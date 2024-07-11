use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};
use solver::game::{Game, M, N};

extern crate solver;

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let game = Game::new(n);
}
