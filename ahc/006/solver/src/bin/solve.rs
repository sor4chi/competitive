use proconio::input;
use solver::{
    game::{Game, Input, N},
    policy::{
        beam::BeamPolicy, greedy::GreedyPolicy, greedy_sep::GreedySepPolicy, insert::InsertPolicy,
        Policy,
    },
    util::output,
};

extern crate solver;

fn main() {
    input! {
        row: [(usize,usize,usize,usize); N],
    }

    let input = Input::new(row);
    let game = Game::new(input);

    let policy = InsertPolicy::new(game);
    let (used_orders, ops) = policy.solve();
    output(used_orders, ops);
}
