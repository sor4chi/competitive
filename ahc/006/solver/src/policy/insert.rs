use std::collections::HashSet;

use crate::util::{output, tsp_with_validator};

use super::super::{game::Game, game::N, util::manhattan};

use super::Policy;

pub struct InsertPolicy {
    game: Game,
}

impl InsertPolicy {
    pub fn new(game: Game) -> Self {
        Self { game }
    }
}

fn calc_total_dist(ops: &Vec<(usize, usize)>) -> usize {
    let mut dist = 0;
    for i in 0..ops.len() - 1 {
        dist += manhattan(ops[i], ops[i + 1]);
    }
    dist
}

impl Policy for InsertPolicy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let center = (400, 400);
        let mut min_dist = std::usize::MAX;
        let mut min_idx = std::usize::MAX;
        for i in 0..N {
            let restaurant = (self.game.input.a[i], self.game.input.b[i]);
            let house = (self.game.input.c[i], self.game.input.d[i]);
            let dist = manhattan(center, restaurant) + manhattan(center, house);
            if dist < min_dist {
                min_dist = dist;
                min_idx = i;
            }
        }
        let mut used_orders = HashSet::new();
        used_orders.insert(min_idx);
        let mut ops = vec![
            center,
            (self.game.input.a[min_idx], self.game.input.b[min_idx]),
            (self.game.input.c[min_idx], self.game.input.d[min_idx]),
            center,
        ];
        for _ in 0..49 {
            let mut min_cost = std::usize::MAX;
            let mut min_idx = std::usize::MAX;
            let mut min_ops = vec![];
            for i in 0..N {
                // i は入れようとする配達のインデックス
                if used_orders.contains(&i) {
                    continue;
                }
                let restaurant = (self.game.input.a[i], self.game.input.b[i]);
                let house = (self.game.input.c[i], self.game.input.d[i]);
                let mut min_restaurant_cost = std::usize::MAX;
                let mut min_restaurant_idx = std::usize::MAX;
                let mut min_house_cost = std::usize::MAX;
                let mut min_house_idx = std::usize::MAX;
                for j in 0..ops.len() {
                    // レストランに一番近いopを探す
                    let cost = manhattan(ops[j], restaurant);
                    if cost < min_restaurant_cost {
                        min_restaurant_cost = cost;
                        min_restaurant_idx = j;
                    }
                }
                for j in 0..ops.len() {
                    let cost = manhattan(ops[j], house);
                    if cost < min_house_cost {
                        min_house_cost = cost;
                        min_house_idx = j;
                    }
                }
                if min_restaurant_idx > min_house_idx {
                    continue;
                }
                let mut ops_cand1 = ops.clone();
                ops_cand1.insert(min_house_idx + 1, house);
                ops_cand1.insert(min_restaurant_idx + 1, restaurant);
                let cost1 = calc_total_dist(&ops_cand1);
                if cost1 < min_cost
                    && ops_cand1.first().unwrap() == &center
                    && ops_cand1.last().unwrap() == &center
                {
                    min_cost = cost1;
                    min_idx = i;
                    min_ops = ops_cand1;
                }
                let mut ops_cand2 = ops.clone();
                ops_cand2.insert(min_house_idx, house);
                ops_cand2.insert(min_restaurant_idx, restaurant);
                let cost2 = calc_total_dist(&ops_cand2);
                if cost2 < min_cost
                    && ops_cand2.first().unwrap() == &center
                    && ops_cand2.last().unwrap() == &center
                {
                    min_cost = cost2;
                    min_idx = i;
                    min_ops = ops_cand2;
                }
                let mut ops_cand3 = ops.clone();
                ops_cand3.insert(min_house_idx + 1, house);
                ops_cand3.insert(min_restaurant_idx, restaurant);
                let cost3 = calc_total_dist(&ops_cand3);
                if cost3 < min_cost
                    && min_restaurant_idx != min_house_idx
                    && ops_cand3.first().unwrap() == &center
                    && ops_cand3.last().unwrap() == &center
                {
                    min_cost = cost3;
                    min_idx = i;
                    min_ops = ops_cand3;
                }
                let mut ops_cand4 = ops.clone();
                ops_cand4.insert(min_house_idx, house);
                ops_cand4.insert(min_restaurant_idx + 1, restaurant);
                let cost4 = calc_total_dist(&ops_cand4);
                if cost4 < min_cost
                    && min_restaurant_idx != min_house_idx
                    && ops_cand4.first().unwrap() == &center
                    && ops_cand4.last().unwrap() == &center
                {
                    min_cost = cost4;
                    min_idx = i;
                    min_ops = ops_cand4;
                }
            }
            used_orders.insert(min_idx);
            ops = min_ops;
            // output(used_orders.clone().into_iter().collect(), ops.clone());
        }

        let best_order = tsp_with_validator(ops.clone(), 1900, &|order| {
            // convert order to pos
            let mut pos = vec![];
            for i in 0..order.len() {
                pos.push(ops[order[i]]);
            }
            // output(used_orders.clone().into_iter().collect(), pos.clone());
            self.game.validate(&pos)
        });

        let mut best_ops = vec![];
        for i in 0..best_order.len() {
            best_ops.push(ops[best_order[i]]);
        }

        (used_orders.into_iter().collect(), best_ops)
    }
}
