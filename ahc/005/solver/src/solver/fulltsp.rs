use crate::rand_xorshift;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

use super::super::{game::Game, graph::Point};
use super::{Direction, Solver};

pub struct FullTSPSolver<'a> {
    game: &'a Game,
}

enum Neighbor {
    OPT_2,
    DELETE,
    INSERT,
}

const NEIGHBORS: [Neighbor; 5] = [
    Neighbor::OPT_2,
    Neighbor::OPT_2,
    Neighbor::OPT_2,
    Neighbor::DELETE,
    Neighbor::INSERT,
];

const MONITOR: bool = false;

impl FullTSPSolver<'_> {
    pub fn new(game: &Game) -> FullTSPSolver {
        FullTSPSolver { game }
    }

    fn get_path(
        &self,
        all_dist_map: &HashMap<Point, HashMap<Point, usize>>,
        v: &[Point],
        best_order: Vec<usize>,
    ) -> (Vec<Direction>, usize) {
        let mut best_operation = vec![];
        for i in 0..best_order.len() {
            let point = v[best_order[i] as usize];
            best_operation.push(point);
        }
        let mut path = vec![];
        let mut actual_score = 0;

        for i in 0..best_operation.len() - 1 {
            let start = best_operation[i];
            let goal = best_operation[i + 1];
            let dist = all_dist_map.get(&start).unwrap();
            let to_the_next_path = self.game.graph.get_path(start, goal, dist);
            actual_score += dist[&goal];
            for i in 0..to_the_next_path.len() - 1 {
                let mut current = to_the_next_path[i];
                let next = to_the_next_path[i + 1];
                while current != next {
                    let dir = current.direction_to(next);
                    path.push(dir);
                    current.apply_dir(dir);
                }
            }
        }

        (path, actual_score)
    }

    fn inner_solve(
        &self,
        all_points: HashMap<Point, HashSet<usize>>,
        all_dist_map: HashMap<Point, HashMap<Point, usize>>,
        inner_limit: usize,
    ) -> (Vec<Direction>, usize) {
        let start = Point::from(self.game.input.s);

        let mut best_score = 0;
        let mut v = vec![start];
        v.extend(all_points.keys().copied().collect::<Vec<_>>());
        v.push(start);
        let mut v_rev = HashMap::new();
        for (i, vi) in v.iter().enumerate() {
            v_rev.insert(vi, i);
        }

        let best_order = {
            // === FROM: 焼きなましのための前準備 ===
            let mut best_order: Vec<u16> = (0..v.len()).map(|i| i as u16).collect::<Vec<_>>();
            // n手目にbest_score_cache[n]の距離がかかるということをキャッシュしておく
            let mut best_score_cache = vec![];
            for i in 0..v.len() - 1 {
                let dij = all_dist_map.get(&v[best_order[i] as usize]).unwrap();
                let score = dij.get(&v[best_order[i + 1] as usize]).unwrap();
                best_score += *score;
                best_score_cache.push(*score);
            }
            // ある辺に属している頂点集合
            let mut best_resolve_map = self.game.resolve_map.clone();
            // ある頂点が属してる辺
            let mut best_resolve_map_rev = self.game.resolve_map_rev.clone();
            // 使われなくなった頂点集合
            let mut best_unused_points = HashSet::new();
            // === END: 焼きなましのための前準備 ===

            let timer = Instant::now();
            let start_temp = 100.0;
            let end_temp = 0.001;
            let mut rng = rand_xorshift::XorShiftRng::from_seed([0; 16]);
            let mut temp = start_temp;
            let mut iter = 0;

            while (timer.elapsed().as_millis() as usize) < inner_limit {
                iter += 1;

                let selected_neighbor = NEIGHBORS.iter().choose(&mut rng).unwrap();

                match selected_neighbor {
                    Neighbor::OPT_2 => {
                        // 2-opt
                        let i = rng.gen_range(1..best_order.len() - 2);
                        let j = rng.gen_range(i + 1..best_order.len() - 1);
                        let mut new_order: Vec<u16> = best_order.clone();
                        new_order[i..=j].reverse();
                        // スコアを差分計算する
                        let mut new_score = best_score;
                        new_score -= best_score_cache[i - 1];
                        new_score -= best_score_cache[j];
                        let new_dist_i = *all_dist_map
                            .get(&v[new_order[i - 1] as usize])
                            .unwrap()
                            .get(&v[new_order[i] as usize])
                            .unwrap();
                        let new_dist_j = *all_dist_map
                            .get(&v[new_order[j] as usize])
                            .unwrap()
                            .get(&v[new_order[j + 1] as usize])
                            .unwrap();
                        new_score += new_dist_i + new_dist_j;

                        let diff = new_score as f64 - best_score as f64;
                        if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
                            if MONITOR {
                                eprintln!("score: {}, temp: {}", new_score, temp);
                            }
                            best_order = new_order;
                            best_score = new_score;
                            best_score_cache[i - 1] = new_dist_i;
                            best_score_cache[j] = new_dist_j;
                            best_score_cache[i..j].reverse();
                        }
                    }
                    Neighbor::DELETE => {
                        // 削除する頂点をbest_resolve_map_revのkeyから選ぶ
                        let delete = best_resolve_map_rev.keys().choose(&mut rng).unwrap();
                        // スタートの頂点は削除しない
                        if *delete == start {
                            continue;
                        }
                        // 削除する頂点が属している辺が他の頂点に属していない場合はスキップ
                        let mut ok = true;
                        let edges = best_resolve_map_rev.get(delete).unwrap();
                        for &edge in edges {
                            if best_resolve_map.get(&edge).unwrap().len() == 1 {
                                ok = false;
                                break;
                            }
                        }
                        if !ok {
                            continue;
                        }
                        let mut best_resolve_map_clone = best_resolve_map.clone();
                        // 削除する頂点が属している辺を取得
                        for &edge in edges {
                            // 辺から頂点を削除
                            best_resolve_map_clone
                                .get_mut(&edge)
                                .unwrap()
                                .remove(delete);
                        }
                        let mut best_resolve_map_rev_clone = best_resolve_map_rev.clone();
                        // 頂点を削除
                        best_resolve_map_rev_clone.remove(delete);
                        // 削除する頂点をbest_orderから削除
                        let mut new_order = best_order.clone();
                        // 削除する頂点のIdを取得
                        let delete_node_id = *v_rev.get(delete).unwrap() as u16;
                        let delete_node_id_idx =
                            new_order.iter().position(|&x| x == delete_node_id).unwrap();
                        new_order.remove(delete_node_id_idx);
                        // スコアを差分計算する
                        let mut new_score = best_score;
                        new_score -= best_score_cache[delete_node_id_idx - 1];
                        new_score -= best_score_cache[delete_node_id_idx];
                        let new_dist = *all_dist_map
                            .get(&v[new_order[delete_node_id_idx - 1] as usize])
                            .unwrap()
                            .get(&v[new_order[delete_node_id_idx] as usize])
                            .unwrap();
                        new_score += new_dist;
                        let diff = new_score as f64 - best_score as f64;
                        if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
                            if MONITOR {
                                eprintln!("score: {}, temp: {}", new_score, temp);
                            }
                            best_order = new_order;
                            best_score = new_score;
                            best_score_cache.remove(delete_node_id_idx);
                            best_score_cache[delete_node_id_idx - 1] = new_dist;
                            best_unused_points.insert(*delete);
                            best_resolve_map = best_resolve_map_clone;
                            best_resolve_map_rev = best_resolve_map_rev_clone;
                        }
                    }
                    Neighbor::INSERT => {
                        // best_unused_pointsが空の場合はスキップ
                        if best_unused_points.is_empty() {
                            continue;
                        }
                        // 挿入する頂点をbest_unused_pointsから選ぶ
                        let insert = best_unused_points.iter().choose(&mut rng).unwrap();
                        // 挿入する頂点をbest_orderに挿入する位置を選ぶ
                        let insert_idx = rng.gen_range(1..best_order.len() - 1);
                        // 挿入する頂点をbest_orderに挿入
                        let mut new_order = best_order.clone();
                        // 頂点のIdを取得
                        let insert_node_id = *v_rev.get(insert).unwrap() as u16;
                        new_order.insert(insert_idx, insert_node_id);
                        // スコアを差分計算する
                        let mut new_score = best_score;
                        new_score -= best_score_cache[insert_idx - 1];
                        let new_dist_to_new = *all_dist_map
                            .get(&v[new_order[insert_idx - 1] as usize])
                            .unwrap()
                            .get(&v[new_order[insert_idx] as usize])
                            .unwrap();
                        let new_dist_from_new = *all_dist_map
                            .get(&v[new_order[insert_idx] as usize])
                            .unwrap()
                            .get(&v[new_order[insert_idx + 1] as usize])
                            .unwrap();
                        new_score += new_dist_to_new + new_dist_from_new;
                        let diff = new_score as f64 - best_score as f64;
                        if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
                            if MONITOR {
                                eprintln!("score: {}, temp: {}", new_score, temp);
                            }
                            best_order = new_order;
                            best_score = new_score;
                            best_score_cache[insert_idx - 1] = new_dist_to_new;
                            best_score_cache.insert(insert_idx, new_dist_from_new);
                            // 挿入する頂点が属している辺を取得
                            let edges = all_points.get(&insert.clone()).unwrap();
                            for &edge in edges {
                                // 辺に頂点を挿入
                                best_resolve_map
                                    .get_mut(&edge)
                                    .unwrap()
                                    .insert(insert.clone());
                            }
                            // 頂点を挿入
                            best_resolve_map_rev.insert(insert.clone(), edges.clone());
                            best_unused_points.remove(&insert.clone());
                        }
                    }
                }

                temp = start_temp
                    + (end_temp - start_temp) * timer.elapsed().as_millis() as f64
                        / inner_limit as f64;
            }

            eprintln!("iter: {}", iter);

            best_order
        };

        // self.get_path(&all_dist_map, &v, best_order)

        let mut path = vec![];
        let mut best_operation = vec![];
        for i in 0..best_order.len() {
            let point = v[best_order[i] as usize];
            best_operation.push(point);
        }

        // それぞれの点を巡回する
        let mut cleared_lines = HashSet::new();
        let mut actual_score = 0;
        let mut current = best_operation[0];
        for i in 0..best_operation.len() - 1 {
            let next = best_operation[i + 1];
            let mut is_already_all_covered = true;
            let mut needs_to_clear = HashSet::new();
            if let Some(cover_next_lines) = self.game.resolve_map_rev.get(&next) {
                for line_id in cover_next_lines.iter() {
                    if !cleared_lines.contains(line_id) {
                        is_already_all_covered = false;
                        needs_to_clear.insert(*line_id);
                    }
                }
            } else {
                is_already_all_covered = false;
            }
            if is_already_all_covered {
                continue;
            }
            let dij = all_dist_map.get(&current).unwrap();
            let to_the_next_path = self.game.graph.get_path(current, next, dij);
            for p in to_the_next_path.iter().skip(1) {
                if needs_to_clear.is_empty() {
                    break;
                }
                actual_score += dij[p] - dij[&current];
                while current.x < p.x {
                    path.push(Direction::Down);
                    current.x += 1;
                }
                while current.x > p.x {
                    path.push(Direction::Up);
                    current.x -= 1;
                }
                while current.y < p.y {
                    path.push(Direction::Right);
                    current.y += 1;
                }
                while current.y > p.y {
                    path.push(Direction::Left);
                    current.y -= 1;
                }
                if let Some(crossing_lines) = self.game.resolve_map_rev.get(p) {
                    for line_id in crossing_lines.iter() {
                        cleared_lines.insert(*line_id);
                        needs_to_clear.remove(line_id);
                    }
                }
            }
        }

        if current != best_operation[best_operation.len() - 1] {
            let dij = all_dist_map.get(&current).unwrap();
            let to_the_next_path =
                self.game
                    .graph
                    .get_path(current, best_operation[best_operation.len() - 1], dij);
            for p in to_the_next_path.iter().skip(1) {
                actual_score += dij[p] - dij[&current];
                while current.x < p.x {
                    path.push(Direction::Down);
                    current.x += 1;
                }
                while current.x > p.x {
                    path.push(Direction::Up);
                    current.x -= 1;
                }
                while current.y < p.y {
                    path.push(Direction::Right);
                    current.y += 1;
                }
                while current.y > p.y {
                    path.push(Direction::Left);
                    current.y -= 1;
                }
            }
        }

        (path, actual_score)
    }
}

impl Solver for FullTSPSolver<'_> {
    fn solve(&self) -> Vec<Direction> {
        let all_points = self
            .game
            .resolve_map_rev
            .clone()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashMap<_, _>>();
        let mut all_dist_map = self
            .game
            .resolve_map_rev
            .clone()
            .keys()
            .map(|k| (*k, self.game.graph.dijkstra(*k)))
            .collect::<HashMap<_, _>>();
        let center = Point::from(self.game.input.s);
        all_dist_map.insert(center, self.game.graph.dijkstra(center));
        const TRIAL: usize = 1;
        let mut best_path = vec![];
        let mut best_score = usize::MAX;
        const TL: usize = 2900;
        for _ in 0..TRIAL {
            let (path, score) =
                self.inner_solve(all_points.clone(), all_dist_map.clone(), TL / TRIAL);
            eprintln!("score: {}", score);
            if score < best_score {
                best_path = path;
                best_score = score;
            }
        }
        eprintln!("best score: {}", best_score);
        best_path
    }
}
