use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use super::super::{game::Game, graph::Point};
use super::{Direction, Solver};

pub struct TSPSolver<'a> {
    game: &'a Game,
}

impl TSPSolver<'_> {
    pub fn new(game: &Game) -> TSPSolver {
        TSPSolver { game }
    }
    fn inner_solve(
        &self,
        all_points: HashMap<Point, HashSet<usize>>,
        all_dist_map: HashMap<Point, HashMap<Point, usize>>,
        inner_limit: usize,
    ) -> (Vec<Direction>, usize) {
        let game = self.game.clone();
        // とりあえずgraphを頼りに全交差点を通るような経路を作る
        let mut path = vec![];
        let graph = game.graph.clone();
        let center = Point::from(game.input.s);

        // 必要なポイントを満たす
        let timer = Instant::now();
        let first_tl = inner_limit / 4;
        let mut best_points = all_points.clone();
        let mut best_score = best_points.len();
        let all_lines = game
            .resolve_map
            .clone()
            .keys()
            .copied()
            .collect::<HashSet<_>>();

        let is_points_cover_all_lines = |points: &HashMap<Point, HashSet<usize>>| {
            let mut needs_line = all_lines.clone();
            for (_, lines) in points.iter() {
                for line in lines.iter() {
                    needs_line.remove(line);
                }
            }
            needs_line.is_empty()
        };

        assert!(is_points_cover_all_lines(&best_points));
        let first_start_temp = 1.0;
        let first_end_temp = 0.01;
        let mut first_temp = first_start_temp;

        while (timer.elapsed().as_millis() as usize) < first_tl {
            // best_pointsからランダムに一つ削除して、再度最適な点を求める
            let mut points = best_points.clone();
            let mut rng = rand::thread_rng();
            let mode = rng.gen_range(0..2);
            if mode == 0 {
                let keys = points.keys().copied().collect::<Vec<_>>();
                let selected = keys.choose(&mut rng).unwrap();
                points.remove(selected);
            } else {
                let selected = all_points.keys().choose(&mut rng).unwrap();
                points.insert(*selected, all_points[selected].clone());
            }
            if !is_points_cover_all_lines(&points) {
                continue;
            }

            let score = points.len();
            let diff = score as f64 - best_score as f64;

            if diff < 0.0 || rng.gen_bool((-diff / first_temp).exp()) {
                // eprintln!("score: {}, temp: {}", score, first_temp);
                best_points = points;
                best_score = score;
            }

            first_temp = first_start_temp
                + (first_end_temp - first_start_temp) * timer.elapsed().as_millis() as f64
                    / first_tl as f64;
        }

        let dist_map = all_dist_map;

        let mut best_score = 0;
        let best_operation = {
            let mut v = vec![center];
            v.extend(best_points.keys().copied().collect::<Vec<_>>());
            v.push(center);

            let mut best_order = (0..v.len()).map(|i| i as u8).collect::<Vec<u8>>();
            // n手目にbest_score_cache[n]の距離がかかるということをキャッシュしておく
            let mut best_score_cache = vec![];
            for i in 0..v.len() - 1 {
                let dij = dist_map.get(&v[best_order[i] as usize]).unwrap();
                let score = dij.get(&v[best_order[i + 1] as usize]).unwrap();
                best_score += *score;
                best_score_cache.push(*score);
            }
            let start = Instant::now();
            let start_temp = 30.0;
            let end_temp = 0.01;
            let mut rng = rand::thread_rng();
            let mut temp = start_temp;
            let limit = inner_limit / 4 * 3;
            let mut iter = 0;

            loop {
                iter += 1;
                let elapsed = start.elapsed().as_millis();
                if elapsed as usize > limit {
                    break;
                }

                // 2-opt
                let i = rng.gen_range(1..v.len() - 2);
                let j = rng.gen_range(i + 1..v.len() - 1);
                let mut new_order = best_order.clone();
                new_order[i..=j].reverse();
                // スコアを差分計算する
                let mut new_score = best_score;
                new_score -= best_score_cache[i - 1];
                new_score -= best_score_cache[j];
                let new_dist_i = *dist_map
                    .get(&v[new_order[i - 1] as usize])
                    .unwrap()
                    .get(&v[new_order[i] as usize])
                    .unwrap();
                let new_dist_j = *dist_map
                    .get(&v[new_order[j] as usize])
                    .unwrap()
                    .get(&v[new_order[j + 1] as usize])
                    .unwrap();
                new_score += new_dist_i + new_dist_j;

                // let new_score = score(&v, &new_order);

                let diff = new_score as f64 - best_score as f64;
                if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
                    // eprintln!("score: {}, temp: {}", new_score, temp);
                    best_order = new_order;
                    best_score = new_score;
                    best_score_cache[i - 1] = new_dist_i;
                    best_score_cache[j] = new_dist_j;
                    best_score_cache[i..j].reverse();
                }

                temp = start_temp + (end_temp - start_temp) * elapsed as f64 / limit as f64;
            }

            eprintln!("iter: {}", iter);

            let mut best_ops = vec![];
            for i in 0..best_order.len() {
                let point = v[best_order[i] as usize];
                best_ops.push(point);
            }
            best_ops
        };

        // それぞれの点を巡回する
        let mut cleared_lines = HashSet::new();
        let mut actual_score = 0;
        let mut current = best_operation[0];
        for i in 0..best_operation.len() - 1 {
            let next = best_operation[i + 1];
            let mut is_already_all_covered = true;
            let mut needs_to_clear = HashSet::new();
            if let Some(cover_next_lines) = game.resolve_map_rev.get(&next) {
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
            let dij = dist_map.get(&current).unwrap();
            let to_the_next_path = graph.get_path(current, next, dij);
            actual_score += dij.get(&next).unwrap();
            for p in to_the_next_path.iter().skip(1) {
                if needs_to_clear.is_empty() {
                    break;
                }
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
                if let Some(crossing_lines) = game.resolve_map_rev.get(p) {
                    for line_id in crossing_lines.iter() {
                        cleared_lines.insert(*line_id);
                        needs_to_clear.remove(line_id);
                    }
                }
            }
            // current = next;
        }

        if current != best_operation[best_operation.len() - 1] {
            eprintln!(
                "current: {:?}, last: {:?}",
                current,
                best_operation[best_operation.len() - 1]
            );
            let dij = dist_map.get(&current).unwrap();
            let to_the_next_path =
                graph.get_path(current, best_operation[best_operation.len() - 1], dij);
            actual_score += dij.get(&best_operation[best_operation.len() - 1]).unwrap();
            for p in to_the_next_path.iter().skip(1) {
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

impl Solver for TSPSolver<'_> {
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
            .iter()
            .map(|(k, _)| (*k, self.game.graph.dijkstra(*k)))
            .collect::<HashMap<_, _>>();
        let center = Point::from(self.game.input.s);
        all_dist_map.insert(center, self.game.graph.dijkstra(center));
        let TRIAL = 15;
        let mut best_path = vec![];
        let mut best_score = std::usize::MAX;
        let TL = 2850;
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
