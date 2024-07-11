use std::collections::HashSet;

use super::super::{
    game::{Game, N},
    util::{manhattan, tsp},
};

use super::Policy;

pub struct GreedySepPolicy {
    game: Game,
}

impl GreedySepPolicy {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    fn picking_minimum(&self) -> (Vec<usize>, Vec<(usize, usize)>, usize) {
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
                let dist = manhattan(cur, restaurant);
                if dist < min && !used_orders.contains(&i) {
                    min = dist;
                    min_idx = i;
                }
            }
            assert!(min_idx != std::usize::MAX);
            used_orders.insert(min_idx);
            ops.push((self.game.input.a[min_idx], self.game.input.b[min_idx]));
            cur = (self.game.input.a[min_idx], self.game.input.b[min_idx]);
        }
        let v: Vec<(usize, usize)> = used_orders
            .iter()
            .map(|&i| (self.game.input.c[i], self.game.input.d[i]))
            .collect();
        let path = tsp(v.clone(), 900);
        // curに一番近いindexをスタートとする
        let mut min = std::usize::MAX;
        let mut min_idx = std::usize::MAX;
        for i in 0..path.len() {
            let dist = manhattan(cur, v[path[i]]);
            if dist < min {
                min = dist;
                min_idx = i;
            }
        }
        for i in 0..path.len() {
            ops.push(v[path[(min_idx + i) % path.len()]]);
        }
        ops.push((400, 400));
        let mut total_cost = 0;
        for i in 0..ops.len() - 1 {
            total_cost += manhattan(ops[i], ops[i + 1]);
        }
        (used_orders.into_iter().collect(), ops, total_cost)
    }

    fn releaseing_minimum(&self) -> (Vec<usize>, Vec<(usize, usize)>, usize) {
        let mut cur = (400, 400);
        let mut used_orders = HashSet::new();
        let mut ops = vec![];
        ops.push(cur);
        for _ in 0..50 {
            // curレストランから配達までの距離が最小のものを選ぶ
            let mut min = std::usize::MAX;
            let mut min_idx = std::usize::MAX;
            for i in 0..N {
                let release = (self.game.input.c[i], self.game.input.d[i]);
                let dist = manhattan(cur, release);
                if dist < min && !used_orders.contains(&i) {
                    min = dist;
                    min_idx = i;
                }
            }
            assert!(min_idx != std::usize::MAX);
            used_orders.insert(min_idx);
            ops.push((self.game.input.c[min_idx], self.game.input.d[min_idx]));
            cur = (self.game.input.c[min_idx], self.game.input.d[min_idx]);
        }
        let v: Vec<(usize, usize)> = used_orders
            .iter()
            .map(|&i| (self.game.input.a[i], self.game.input.b[i]))
            .collect();
        let path = tsp(v.clone(), 900);
        // curに一番近いindexをスタートとする
        let mut min = std::usize::MAX;
        let mut min_idx = std::usize::MAX;
        for i in 0..path.len() {
            let dist = manhattan(cur, v[path[i]]);
            if dist < min {
                min = dist;
                min_idx = i;
            }
        }
        for i in 0..path.len() {
            ops.push(v[path[(min_idx + i) % path.len()]]);
        }
        ops.push((400, 400));
        ops.reverse();
        let mut total_cost = 0;
        for i in 0..ops.len() - 1 {
            total_cost += manhattan(ops[i], ops[i + 1]);
        }
        (used_orders.into_iter().collect(), ops, total_cost)
    }
}

impl Policy for GreedySepPolicy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>) {
        // releasing_minimumとpicking_minimumの結果を比較して最小のものを選ぶ
        let (used_orders_picking, ops_picking, total_cost_picking) = self.picking_minimum();
        let (used_orders_releasing, ops_releasing, total_cost_releasing) =
            self.releaseing_minimum();
        eprintln!("picking: {}", total_cost_picking);
        eprintln!("releasing: {}", total_cost_releasing);
        if total_cost_picking < total_cost_releasing {
            (used_orders_picking, ops_picking)
        } else {
            (used_orders_releasing, ops_releasing)
        }
    }
}
