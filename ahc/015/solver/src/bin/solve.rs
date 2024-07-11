use std::io::{stdin, BufReader};

use proconio::{input, source::line::LineSource};

use solver::{
    game::Game,
    solver::{greedy::GreedySolver, random::RandomSolver, Solver},
};

extern crate solver;

const N: usize = 10;

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        a: [usize; N * N],
    }

    let mut game = Game::new(a, N);
    // let solver = RandomSolver::new(0);
    let solver = GreedySolver::new();
    while game.turn < N * N {
        input! {
            from &mut source,
            pos: usize,
        }
        game.place(pos);
        eprintln!("{}", game.board_str());
        eprintln!("{}", solver.raw_eval(&game));
        let op = solver.get_move(&game);
        game.slide(op.clone());
        println!("{}", op);
    }
}
