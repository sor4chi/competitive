use proconio::input;
use solver::{
    game::{Game, Input, N},
    policy::{beam::BeamPolicy, greedy::GreedyPolicy, greedy_sep::GreedySepPolicy, Policy},
};

extern crate solver;

fn main() {
    input! {
        row: [(usize,usize,usize,usize); N],
    }

    let input = Input::new(row);
    let game = Game::new(input);

    let policy = BeamPolicy::new(game);
    let (used_orders, ops) = policy.solve();
    print!("{} ", used_orders.len());
    for order in used_orders {
        print!("{} ", order + 1);
    }
    println!();

    print!("{} ", ops.len());
    for op in ops {
        print!("{} {} ", op.0, op.1);
    }
    println!();
}
