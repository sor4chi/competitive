use solver::{game::Game, parse_input, strategy::{greedy::GreedyStrategy, Strategy}};

extern crate solver;

fn main() {
    let input = parse_input();

    let game = Game::new(input);
    let strategy = GreedyStrategy::new(&game);
    let directions = strategy.solve();
    for direction in directions {
        print!("{}", direction);
    }
}
