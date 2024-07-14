use solver::{game::Game, parse_input};

extern crate solver;

fn main() {
    let input = parse_input();

    let game = Game::new(input);
}
