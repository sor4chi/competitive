use crate::io::{Input, Output, IO};
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::Solver;

pub struct GreedySolver {
    io: IO,
    input: Input,
}

impl GreedySolver {
    pub fn new(io: IO, input: Input) -> Self {
        GreedySolver { io, input }
    }
}

const DISTANCE_THRESHOLD: usize = 50_000_000; // これ以下のマンハッタン距離のドリンクはグループ化する

fn chop_to_spans(
    drinks: &[(usize, usize)],
    from: (usize, usize),
    to: (usize, usize),
) -> Vec<((usize, usize), (usize, usize))> {
    let mut operations = vec![];
    let mut cur = from;
    //cur.0 <x<to.0の中でcur.1<y<cur.1+100_000_000の範囲の点を取得する
    const CONNECT_AREA: usize = 100_000_000;
    let mut points = vec![];
    for &(x, y) in drinks {
        if cur.0 < x && x <= to.0 && cur.1 <= y && y < cur.1 + CONNECT_AREA {
            points.push((x, y));
        }
    }
    points.sort_by_key(|&(x, y)| (x, y));
    for i in 0..points.len() {
        let next = (points[i].0, cur.1);
        operations.push((cur, next));
        cur = next;
    }
    if cur.0 != to.0 {
        let next = (to.0, cur.1);
        operations.push((cur, next));
        cur = next;
    }
    // cur.1 <y<to.1の中でcur.0<x<cur.0+100_000_000の範囲の点を取得する
    let mut points = vec![];
    for &(x, y) in drinks {
        if cur.1 < y && y <= to.1 && cur.0 <= x && x < cur.0 + CONNECT_AREA {
            points.push((x, y));
        }
    }
    points.sort_by_key(|&(x, y)| (y, x));
    for i in 0..points.len() {
        let next = (cur.0, points[i].1);
        operations.push((cur, next));
        cur = next;
    }
    if cur.1 != to.1 {
        let next = (cur.0, to.1);
        operations.push((cur, next));
        cur = next;
    }
    operations
}

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        let mut cost = 0;
        let mut generated_drinks = vec![(0, 0)];
        let mut operations: Vec<((usize, usize), (usize, usize))> = vec![];
        let max_x = self.input.drinks.iter().map(|&(a, _)| a).max().unwrap();
        let max_y = self.input.drinks.iter().map(|&(_, b)| b).max().unwrap();
        chop_to_spans(&self.input.drinks, (0, 0), (max_x, 0))
            .iter()
            .for_each(|&(from, to)| {
                let distance = (to.0 - from.0) as i64 + (to.1 - from.1) as i64;
                cost += distance;
                operations.push((from, to));
                generated_drinks.push(to);
            });
        chop_to_spans(&self.input.drinks, (0, 0), (0, max_y))
            .iter()
            .for_each(|&(from, to)| {
                let distance = (to.0 - from.0) as i64 + (to.1 - from.1) as i64;
                cost += distance;
                operations.push((from, to));
                generated_drinks.push(to);
            });

        // 最短距離がえぐいランキングを作る
        // self.input.drinks間のマンハッタン距離が大きいものを優先的に処理したい
        // (距離, 点1, 点2) の形式でヒープに突っ込む
        #[derive(PartialEq, Eq, Debug)]
        struct State(usize, (usize, usize), (usize, usize));
        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0).reverse()
            }
        }
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        let mut heap = BinaryHeap::new();
        for &(a, b) in &self.input.drinks {
            let mut closest_distance = usize::MAX;
            let mut closest_point = (0, 0);
            for &(x, y) in &self.input.drinks {
                if (a, b) == (x, y) {
                    continue;
                }
                if a >= x && b >= y {
                    let distance = (a - x) + (b - y);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_point = (x, y);
                    }
                }
            }
            {
                // 原点から
                let distance = a + b;
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_point = (0, 0);
                }
                // 軸上から
                let distance = a;
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_point = (a, 0);
                }
                let distance = b;
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_point = (0, b);
                }
            }
            assert_ne!(closest_distance, usize::MAX);
            heap.push(State(closest_distance, (a, b), closest_point));
        }

        let mut force_connect: HashMap<(usize, usize), ((usize, usize), usize)> = HashMap::new();
        for State(distance, (a, b), (x, y)) in heap.into_sorted_vec() {
            // 同じ(x,y)なら距離の遠いものを優先する
            // // 閾値を下回る距離のものは無視する
            // if distance < DISTANCE_THRESHOLD {
            //     continue;
            // }
            if let Some((_, d)) = force_connect.get(&(x, y)) {
                if distance > *d {
                    force_connect.insert((x, y), ((a, b), distance));
                }
            } else {
                force_connect.insert((x, y), ((a, b), distance));
            }
        }

        let mut visited = HashSet::new();
        while visited.len() < self.input.drinks.len() {
            let next_connect = {
                // もしforce_connectの中でvisitedに含まれるものがあれば、それを使う
                let mut next_connect = None;
                for (might_gen_drink, (target_drink, _)) in &force_connect {
                    if visited.contains(might_gen_drink) && !visited.contains(target_drink) {
                        next_connect = Some((*might_gen_drink, *target_drink));
                        break;
                    }
                }
                if next_connect.is_some() {
                    let mut closest_drink_operation = None;
                    let mut closest_distance = usize::MAX;
                    let (a, b) = next_connect.unwrap().1;
                    for &(x, y) in &generated_drinks {
                        if a >= x && b >= y {
                            let distance = (a - x) + (b - y); // マンハッタン距離
                            if distance < closest_distance {
                                closest_distance = distance;
                                closest_drink_operation = Some(((x, y), (a, b)));
                            }
                        }
                    }
                    closest_drink_operation
                } else {
                    let mut closest_drink_operation = None;
                    let mut closest_distance = usize::MAX;
                    for &(a, b) in &self.input.drinks {
                        if visited.contains(&(a, b)) {
                            continue;
                        }
                        for &(x, y) in &generated_drinks {
                            if a >= x && b >= y {
                                let distance = (a - x) + (b - y); // マンハッタン距離
                                if distance < closest_distance {
                                    closest_distance = distance;
                                    closest_drink_operation = Some(((x, y), (a, b)));
                                }
                            }
                        }
                    }
                    closest_drink_operation
                }
            };
            if let Some((gen_drink, target_drink)) = next_connect {
                for operation in chop_to_spans(&self.input.drinks, gen_drink, target_drink) {
                    let distance = (operation.1 .0 - operation.0 .0) as i64
                        + (operation.1 .1 - operation.0 .1) as i64;
                    cost += distance;
                    operations.push(operation);
                    generated_drinks.push(operation.1);
                }
                visited.insert(target_drink);
            }
        }
        eprintln!("cost: {}", cost);
        Output { operations }
    }
}
