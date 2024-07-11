use std::collections::HashSet;

use super::super::{game::Game, game::N, util::manhattan};

use super::Policy;

pub struct GreedyPolicy {
    game: Game,
}

impl GreedyPolicy {
    pub fn new(game: Game) -> Self {
        Self { game }
    }
}

impl Policy for GreedyPolicy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut cur = (400, 400);
        let mut used_orders = HashSet::new();
        let mut ops = vec![];
        ops.push(cur);
        for _ in 0..50 {
            // curレストランから配達までの距離が最小のものを選ぶ
            let mut min = std::usize::MAX;
            let mut min_idx = std::usize::MAX;
            for i in 0..N {
                let restaurant = (self.game.input.a[i], self.game.input.b[i]);
                let release = (self.game.input.c[i], self.game.input.d[i]);
                let dist = manhattan(cur, restaurant) + manhattan(restaurant, release);
                if dist < min && !used_orders.contains(&i) {
                    min = dist;
                    min_idx = i;
                }
            }
            assert!(min_idx != std::usize::MAX);
            used_orders.insert(min_idx);
            eprintln!(
                "cur: ({}, {}), order: ({}, {}) -> ({}, {})",
                cur.0,
                cur.1,
                self.game.input.a[min_idx],
                self.game.input.b[min_idx],
                self.game.input.c[min_idx],
                self.game.input.d[min_idx]
            );
            ops.push((self.game.input.a[min_idx], self.game.input.b[min_idx]));
            ops.push((self.game.input.c[min_idx], self.game.input.d[min_idx]));
            cur = (self.game.input.c[min_idx], self.game.input.d[min_idx]);
        }
        ops.push((400, 400));
        (used_orders.into_iter().collect(), ops)
    }
}
