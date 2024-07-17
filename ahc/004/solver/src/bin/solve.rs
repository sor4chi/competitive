use proconio::input;
use solver::game::Game;

extern crate solver;

fn main() {
    input! {
        n: usize,
    }

    let game = Game::new(n);
}
