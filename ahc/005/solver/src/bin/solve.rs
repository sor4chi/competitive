use solver::{
    game::Game,
    parse_input,
    solver::{greedy::GreedySolver, Solver},
};

extern crate solver;

fn main() {
    let input = parse_input();

    let game = Game::new(input);
    let solver = GreedySolver::new(&game);
    let directions = solver.solve();
    for direction in directions {
        print!("{}", direction);
    }
}
