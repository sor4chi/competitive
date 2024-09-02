use crate::{
    io::{Input, IO},
    util::visualize_a,
};

use fixedbitset::FixedBitSet;
use rand::{prelude::SliceRandom, Rng};

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use crate::io::{Operation, Output, SignalUpdate};

use super::Solver;

pub struct OptimizeASolver {
    io: IO,
    input: Input,
}

impl OptimizeASolver {
    pub fn new(io: IO, input: Input) -> Self {
        OptimizeASolver { io, input }
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    ((dx * dx + dy * dy) as f64).sqrt()
}

impl Solver for OptimizeASolver {
    fn solve(&mut self) -> Output {
        let start = Instant::now();
        let tl = 2900;
        let mut raw_graph = HashMap::new();
        for (a, b) in self.input.edges.iter() {
            raw_graph.entry(*a).or_insert_with(Vec::new).push(*b);
            raw_graph.entry(*b).or_insert_with(Vec::new).push(*a);
        }

        let mut best_operations = vec![];
        let mut best_a = vec![];
        let mut best_score = usize::MAX;

        let mut lap_times = vec![];
        let mut avg_lap_time = 0;
        let mut laps = 0;
        let mut rng = rand::thread_rng();
        let mut perm = (0..self.input.n).collect::<Vec<_>>();
        perm.shuffle(&mut rng);
        loop {
            // 開始からavg_lap_time*1.1倍の残り時間がなければ終了
            if start.elapsed().as_millis() + avg_lap_time * 11 / 10 > tl {
                break;
            }
            laps += 1;

            let lap_start = Instant::now();
            let mut high_cost_operation: Vec<(usize, usize, usize)> = vec![];

            let mut a = {
                // DFSで200ms以内に最も長いパスを探す
                let start_dfs = Instant::now();
                let mut max_path = vec![];
                let mut max_path_len = 0;
                // permから一つ取り出してstart_nodeとする
                let start_node = perm.pop().unwrap();
                let mut stack = vec![(
                    start_node,
                    vec![start_node],
                    FixedBitSet::with_capacity(self.input.n),
                )];
                while let Some((cur, path, visited)) = stack.pop() {
                    if path.len() > max_path_len {
                        max_path.clone_from(&path);
                        max_path_len = path.len();
                    }
                    if start_dfs.elapsed().as_millis() > 50 {
                        break;
                    }

                    let mut nexts = vec![];
                    for &next in raw_graph.get(&cur).unwrap() {
                        if !path.contains(&next) {
                            // 始点と終点の絶対距離が遠いほど評価値が高くなるようにする
                            let from = (500, 500);
                            let score = distance(from, self.input.nodes[next]);
                            nexts.push((next, score));
                        }
                    }

                    nexts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    for (next, _) in nexts {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        let mut new_visited = visited.clone();
                        new_visited.insert(next);
                        stack.push((next, new_path, new_visited));
                    }
                }
                let mut used = vec![false; self.input.n];
                for &node in &max_path {
                    used[node] = true;
                }

                let mut a = vec![];
                a.extend_from_slice(&max_path);

                let mut perm = (0..self.input.n).collect::<Vec<_>>();
                perm.shuffle(&mut rng);
                while used.iter().any(|&x| !x) {
                    // 未使用の都市をランダムに選ぶ
                    let mut start = 0;
                    while used[perm[start]] {
                        start += 1;
                    }
                    let cur = perm[start];
                    let mut path = VecDeque::new();
                    path.push_back(cur);
                    used[cur] = true;
                    let mut prev_stacked = false;
                    let mut next_stacked = false;
                    while path.len() < self.input.lb && (!prev_stacked || !next_stacked) {
                        let start = path.front().unwrap();
                        let mut prevs = raw_graph.get(start).unwrap().clone();
                        let mut nexts = raw_graph.get(start).unwrap().clone();
                        prevs.shuffle(&mut rng);
                        nexts.shuffle(&mut rng);
                        if !prev_stacked {
                            prev_stacked = true;
                            while let Some(prev) = prevs.pop() {
                                if !used[prev] {
                                    path.push_front(prev);
                                    used[prev] = true;
                                    prev_stacked = false;
                                    break;
                                }
                            }
                        }
                        if !next_stacked {
                            next_stacked = true;
                            while let Some(next) = nexts.pop() {
                                if !used[next] {
                                    path.push_back(next);
                                    used[next] = true;
                                    next_stacked = false;
                                    break;
                                }
                            }
                        }
                    }
                    path.iter().for_each(|&x| a.push(x));
                }

                a
            };

            {
                let mut operations = Vec::new();

                struct GraphNodeInfo {
                    path: Vec<usize>,
                    start: usize,
                    reversed: bool,
                }
                let mut modified_graph: HashMap<usize, Vec<GraphNodeInfo>> = HashMap::new();
                let take_graph_construction = Instant::now();
                (1..=self.input.lb).for_each(|width| {
                    for start in 0..(a.len() - width + 1) {
                        // aのstart+1からstart+widthまでの部分列をmodifed_graphに追加、逆も追加
                        let mut path = vec![];
                        for ai in a.iter().skip(start).take(width) {
                            path.push(*ai);
                        }
                        // もしpathがgraph上で繋がっていなければスキップ
                        let mut is_connected = true;
                        for i in 0..(path.len() - 1) {
                            if !raw_graph.get(&path[i]).unwrap().contains(&path[i + 1]) {
                                is_connected = false;
                                break;
                            }
                        }
                        if !is_connected {
                            continue;
                        }
                        // スタートはpathのstartに隣接する都市
                        let next_start_nodes = raw_graph.get(&path[0]).unwrap();
                        for next_start_node in next_start_nodes {
                            modified_graph.entry(*next_start_node).or_default().push(
                                GraphNodeInfo {
                                    path: path.clone(),
                                    start,
                                    reversed: false,
                                },
                            );
                        }
                        // 逆も追加
                        path.reverse();
                        let next_start_nodes = raw_graph.get(&path[0]).unwrap();
                        for next_start_node in next_start_nodes {
                            modified_graph.entry(*next_start_node).or_default().push(
                                GraphNodeInfo {
                                    path: path.clone(),
                                    start,
                                    reversed: true,
                                },
                            );
                        }
                    }
                });
                eprintln!(
                    "graph construction: {}ms",
                    take_graph_construction.elapsed().as_millis()
                );

                let mut cur_node = 0; // 都市0を最初とする
                let mut cur_b = vec![-1; self.input.lb];
                let mut cur_bi = 0;

                // 都市を順に訪れる
                let dijkstra_1st_start = Instant::now();
                for t in &self.input.ts {
                    // targetからBFSをして最短距離をキャッシュ
                    let mut dist_from_target = vec![usize::MAX; self.input.n];
                    dist_from_target[*t] = 0;
                    let mut queue = VecDeque::new();
                    queue.push_back((*t, 0));
                    while let Some((node, d)) = queue.pop_front() {
                        for next in raw_graph.get(&node).unwrap() {
                            if d + 1 < dist_from_target[*next] {
                                dist_from_target[*next] = d + 1;
                                queue.push_back((*next, d + 1));
                            }
                        }
                    }

                    // dijkstraだとO(V+ElogV)かかるので、01-BFSで計算量をO(V+E)にする
                    let mut dist = vec![usize::MAX; self.input.n];
                    dist[cur_node] = 0;
                    let mut best_paths = vec![];
                    let mut best_cost = usize::MAX;
                    let mut queue = VecDeque::new();
                    queue.push_back((cur_node, vec![]));
                    while let Some((node, paths)) = queue.pop_front() {
                        if node == *t {
                            if paths.len() < best_cost {
                                best_cost = paths.len();
                                best_paths = paths;
                            }
                            // continue;
                            break;
                        }
                        let mut front_nexts = vec![];
                        let mut back_nexts = vec![];
                        for next_path in modified_graph.get(&node).unwrap() {
                            let next_node = next_path.path.last().unwrap();
                            let mut is_all_in_cur_b = true;
                            for next in next_path.path.iter() {
                                if !cur_b.contains(&(*next as i32)) {
                                    is_all_in_cur_b = false;
                                    break;
                                }
                            }
                            let cost = if is_all_in_cur_b { 0 } else { 1 };
                            let d = dist[node] + cost;
                            if d < dist[*next_node] {
                                dist[*next_node] = d;
                                let mut next_paths = paths.clone();
                                next_paths.push(next_path);
                                let goal_dist = dist_from_target[*next_node];
                                if cost == 0 {
                                    front_nexts.push((goal_dist, *next_node, next_paths));
                                } else {
                                    back_nexts.push((goal_dist, *next_node, next_paths));
                                }
                            }
                        }

                        // front_nextsは距離が大きいものからpush_frontする
                        front_nexts.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                        for (_, next_node, next_paths) in front_nexts {
                            queue.push_front((next_node, next_paths));
                        }
                        // back_nextsは距離が小さいものからpush_backする
                        back_nexts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                        for (_, next_node, next_paths) in back_nexts {
                            queue.push_back((next_node, next_paths));
                        }
                    }

                    assert!(!best_paths.is_empty());

                    high_cost_operation.push((best_cost, cur_node, *t));

                    // 目的地に到着するまで
                    for next_path in best_paths {
                        // next_pathの要素が全てcur_bに含まれているか確認
                        let mut is_all_in_cur_b = true;
                        for next in next_path.path.iter() {
                            if !cur_b.contains(&(*next as i32)) {
                                is_all_in_cur_b = false;
                                break;
                            }
                        }
                        if !is_all_in_cur_b {
                            if cur_bi + next_path.path.len() > self.input.lb {
                                cur_bi = 0;
                            }
                            // next_pathを青にする
                            operations.push(Operation::SignalUpdate(SignalUpdate {
                                len: next_path.path.len(),
                                ai: next_path.start,
                                bi: cur_bi,
                            }));

                            let mut next_path_for_b = next_path.path.clone();
                            if next_path.reversed {
                                next_path_for_b.reverse();
                            }
                            for next_node in next_path_for_b.iter() {
                                cur_b[cur_bi] = *next_node as i32;
                                cur_bi += 1;
                            }
                        }
                        for next in next_path.path.iter() {
                            operations.push(Operation::Move(*next));
                        }

                        cur_node = *t;
                    }
                }
                eprintln!(
                    "dijkstra 1st: {}ms",
                    dijkstra_1st_start.elapsed().as_millis()
                );
            }

            // 高いコストのものから順に、都市間の最短経路をaへ追加
            high_cost_operation.sort_by_key(|x| -(x.0 as i32));
            let mut high_cost_operation_queue = high_cost_operation
                .iter()
                .map(|(cost, start, end)| (*cost, *start, *end, false))
                .collect::<VecDeque<_>>();
            let mut managed_high_cost_operation_node = HashSet::new();
            while let Some((cost, start, end, skipped)) = high_cost_operation_queue.pop_front() {
                if (managed_high_cost_operation_node.contains(&start)
                    || managed_high_cost_operation_node.contains(&end))
                    && !skipped
                {
                    high_cost_operation_queue.push_back((cost, start, end, true));
                    continue;
                }
                managed_high_cost_operation_node.insert(start);
                managed_high_cost_operation_node.insert(end);
                // 最短経路はBFSで探す
                let mut visited = FixedBitSet::with_capacity(self.input.n);
                let mut queue = VecDeque::new();
                queue.push_back((start, vec![]));
                while let Some((node, path)) = queue.pop_front() {
                    if a.len() == self.input.la {
                        break;
                    }
                    if node == end {
                        // eprintln!(
                        //     "reached, from {} to {}, cost: {}, new_cost: {}",
                        //     start,
                        //     end,
                        //     cost,
                        //     path.len() / self.input.lb + 1
                        // );
                        for next in path.iter() {
                            a.push(*next);
                            if a.len() == self.input.la {
                                break;
                            }
                        }
                        break;
                    }
                    if visited.contains(node) {
                        continue;
                    }
                    visited.insert(node);
                    for next in raw_graph.get(&node).unwrap() {
                        let mut next_path = path.clone();
                        next_path.push(*next);
                        queue.push_back((*next, next_path));
                    }
                }
            }

            {
                let mut operations = Vec::new();

                struct GraphNodeInfo {
                    path: Vec<usize>,
                    start: usize,
                    reversed: bool,
                }
                let mut modified_graph: HashMap<usize, Vec<GraphNodeInfo>> = HashMap::new();
                (1..=self.input.lb).for_each(|width| {
                    for start in 0..(a.len() - width + 1) {
                        // aのstart+1からstart+widthまでの部分列をmodifed_graphに追加、逆も追加
                        let mut path = vec![];
                        for ai in a.iter().skip(start).take(width) {
                            path.push(*ai);
                        }
                        // もしpathがgraph上で繋がっていなければスキップ
                        let mut is_connected = true;
                        for i in 0..(path.len() - 1) {
                            if !raw_graph.get(&path[i]).unwrap().contains(&path[i + 1]) {
                                is_connected = false;
                                break;
                            }
                        }
                        if !is_connected {
                            continue;
                        }
                        // スタートはpathのstartに隣接する都市
                        let next_start_nodes = raw_graph.get(&path[0]).unwrap();
                        for next_start_node in next_start_nodes {
                            modified_graph.entry(*next_start_node).or_default().push(
                                GraphNodeInfo {
                                    path: path.clone(),
                                    start,
                                    reversed: false,
                                },
                            );
                        }
                        // 逆も追加
                        path.reverse();
                        let next_start_nodes = raw_graph.get(&path[0]).unwrap();
                        for next_start_node in next_start_nodes {
                            modified_graph.entry(*next_start_node).or_default().push(
                                GraphNodeInfo {
                                    path: path.clone(),
                                    start,
                                    reversed: true,
                                },
                            );
                        }
                    }
                });

                let mut cur_node = 0; // 都市0を最初とする
                let mut cur_b = vec![-1; self.input.lb];
                let mut cur_bi = 0;
                let mut score = 0;

                // 都市を順に訪れる
                let dijkstra_2nd_start = Instant::now();
                for t in &self.input.ts {
                    // targetからBFSをして最短距離をキャッシュ
                    let mut dist_from_target = vec![usize::MAX; self.input.n];
                    dist_from_target[*t] = 0;
                    let mut queue = VecDeque::new();
                    queue.push_back((*t, 0));
                    while let Some((node, d)) = queue.pop_front() {
                        for next in raw_graph.get(&node).unwrap() {
                            if d + 1 < dist_from_target[*next] {
                                dist_from_target[*next] = d + 1;
                                queue.push_back((*next, d + 1));
                            }
                        }
                    }

                    // dijkstraだとO(V+ElogV)かかるので、01-BFSで計算量をO(V+E)にする
                    let mut dist = vec![usize::MAX; self.input.n];
                    dist[cur_node] = 0;
                    let mut best_paths = vec![];
                    let mut best_cost = usize::MAX;
                    let mut queue = VecDeque::new();
                    queue.push_back((cur_node, vec![]));
                    while let Some((node, paths)) = queue.pop_front() {
                        if node == *t {
                            if paths.len() < best_cost {
                                best_cost = paths.len();
                                best_paths = paths;
                            }
                            // continue;
                            break;
                        }
                        let mut front_nexts = vec![];
                        let mut back_nexts = vec![];
                        for next_path in modified_graph.get(&node).unwrap() {
                            let next_node = next_path.path.last().unwrap();
                            let mut is_all_in_cur_b = true;
                            for next in next_path.path.iter() {
                                if !cur_b.contains(&(*next as i32)) {
                                    is_all_in_cur_b = false;
                                    break;
                                }
                            }
                            let cost = if is_all_in_cur_b { 0 } else { 1 };
                            let d = dist[node] + cost;
                            if d < dist[*next_node] {
                                dist[*next_node] = d;
                                let mut next_paths = paths.clone();
                                next_paths.push(next_path);
                                let goal_dist = dist_from_target[*next_node];
                                if cost == 0 {
                                    front_nexts.push((goal_dist, *next_node, next_paths));
                                } else {
                                    back_nexts.push((goal_dist, *next_node, next_paths));
                                }
                            }
                        }

                        // front_nextsは距離が大きいものからpush_frontする
                        front_nexts.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                        for (_, next_node, next_paths) in front_nexts {
                            queue.push_front((next_node, next_paths));
                        }
                        // back_nextsは距離が小さいものからpush_backする
                        back_nexts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                        for (_, next_node, next_paths) in back_nexts {
                            queue.push_back((next_node, next_paths));
                        }
                    }

                    assert!(!best_paths.is_empty());

                    // 目的地に到着するまで
                    for next_path in best_paths {
                        // next_pathの要素が全てcur_bに含まれているか確認
                        let mut is_all_in_cur_b = true;
                        for next in next_path.path.iter() {
                            if !cur_b.contains(&(*next as i32)) {
                                is_all_in_cur_b = false;
                                break;
                            }
                        }
                        if !is_all_in_cur_b {
                            if cur_bi + next_path.path.len() > self.input.lb {
                                cur_bi = 0;
                            }
                            // next_pathを青にする
                            operations.push(Operation::SignalUpdate(SignalUpdate {
                                len: next_path.path.len(),
                                ai: next_path.start,
                                bi: cur_bi,
                            }));
                            score += 1;

                            let mut next_path_for_b = next_path.path.clone();
                            if next_path.reversed {
                                next_path_for_b.reverse();
                            }
                            for next_node in next_path_for_b.iter() {
                                cur_b[cur_bi] = *next_node as i32;
                                cur_bi += 1;
                            }
                        }
                        for next in next_path.path.iter() {
                            operations.push(Operation::Move(*next));
                        }

                        cur_node = *t;
                    }
                }
                eprintln!(
                    "dijkstra 2nd: {}ms",
                    dijkstra_2nd_start.elapsed().as_millis()
                );

                if score < best_score {
                    best_score = score;
                    best_operations = operations;
                    best_a.clone_from(&a);
                }

                eprintln!("lap: {}, score: {}", laps, score);
            }

            let lap_time = lap_start.elapsed().as_millis();
            lap_times.push(lap_time);
            avg_lap_time = lap_times.iter().sum::<u128>() / lap_times.len() as u128;

            eprintln!(
                "best_score: {}, time: {}ms, avg: {}ms",
                best_score, lap_time, avg_lap_time
            );
        }

        eprintln!("total laps: {}", laps);
        eprintln!("best score: {}", best_score);
        eprintln!("elapsed: {}ms", start.elapsed().as_millis());

        visualize_a(&best_a, &self.input.nodes, &raw_graph, "optimize_a_371.png");

        Output {
            a: best_a,
            operations: best_operations,
        }
    }
}
