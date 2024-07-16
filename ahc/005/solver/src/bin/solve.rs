use solver::{
    game::Game,
    parse_input,
    solver::{greedy::GreedySolver, tsp::TSPSolver, Solver},
};

extern crate solver;

fn main() {
    let input = parse_input();

    let game = Game::new(input);
    let solver = TSPSolver::new(&game);
    let directions = solver.solve();
    for direction in directions {
        print!("{}", direction);
    }
}
