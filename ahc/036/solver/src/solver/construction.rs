use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    time::Instant,
};

use fixedbitset::FixedBitSet;
use rand::Rng;
use rand::{prelude::SliceRandom, SeedableRng};

use crate::{
    io::{Input, Operation, Output, SignalUpdate, IO},
    original_lib::id::IncrementalIDGenerator,
    util::{visualize_a, visualize_components},
};

use super::Solver;

pub struct ConstructionSolver {
    io: IO,
    input: Input,
}

impl ConstructionSolver {
    pub fn new(io: IO, input: Input) -> Self {
        ConstructionSolver { io, input }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    node: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.cost.partial_cmp(&other.cost).unwrap().reverse()
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Some(self.cmp(other))
        Some(other.cost.cmp(&self.cost))
    }
}

// costが小さい方が優先されるかを確認
#[test]
fn test_node_cmp() {
    let a = Node { cost: 1, node: 0 };
    let b = Node { cost: 2, node: 0 };
    let mut heap = BinaryHeap::from(vec![a, b]);
    let c = heap.pop().unwrap();
    assert_eq!(c.cost, 1);
}

struct ComponentEdges {
    edges: HashSet<(usize, usize)>,
}

impl ComponentEdges {
    fn new() -> Self {
        ComponentEdges {
            edges: HashSet::new(),
        }
    }

    fn insert(&mut self, a: usize, b: usize) {
        if a < b {
            self.edges.insert((a, b));
        } else {
            self.edges.insert((b, a));
        }
    }

    fn contains(&self, a: usize, b: usize) -> bool {
        if a < b {
            self.edges.contains(&(a, b))
        } else {
            self.edges.contains(&(b, a))
        }
    }
}

fn euclidean_distance(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    ((dx * dx + dy * dy) as f64).sqrt() as f64
}

#[derive(Debug)]
struct GraphNodeInfo {
    start: usize,
    last_node: usize,
    length: usize,
    reversed: bool,
    points: FixedBitSet,
}

impl Solver for ConstructionSolver {
    fn solve(&mut self) -> Output {
        // tsを見て使う都市を取得
        let mut travel_targets = FixedBitSet::with_capacity(self.input.n);
        for t in self.input.ts.iter() {
            travel_targets.insert(*t);
        }
        let mut ignorables = FixedBitSet::with_capacity(self.input.n);
        for i in 0..self.input.n {
            if !travel_targets[i] {
                ignorables.insert(i);
            }
        }
        let high_cost_operation_tl = 2500;
        let tl = 2900;
        let start = Instant::now();
        let mut best_a = vec![];
        let mut best_operations = vec![];

        let mut components: Vec<Vec<usize>> = vec![];
        let mut components_used = FixedBitSet::with_capacity(self.input.n);
        // component内の辺かどうかを高速に判定するためのHashSet
        let mut component_edges = ComponentEdges::new();
        // let mut rng = rand::thread_rng();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

        let mut raw_graph = HashMap::new();
        for (a, b) in self.input.edges.iter() {
            raw_graph.entry(*a).or_insert_with(Vec::new).push(*b);
            raw_graph.entry(*b).or_insert_with(Vec::new).push(*a);
        }

        let all_1_dist = {
            // ワーシャルフロイド法で全点間の最短距離を求める
            let mut dist = vec![vec![usize::MAX; self.input.n]; self.input.n];
            for i in 0..self.input.n {
                dist[i][i] = 0;
            }
            for (a, b) in self.input.edges.iter() {
                dist[*a][*b] = 1;
                dist[*b][*a] = 1;
            }
            for k in 0..self.input.n {
                for i in 0..self.input.n {
                    for j in 0..self.input.n {
                        if dist[i][k] == usize::MAX || dist[k][j] == usize::MAX {
                            continue;
                        }
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }

            dist
        };

        {
            // unreachable handling
            // 適当に1点選ぶ
            let start = 0;
            // BFSをしてstartからtravel_targetsだけを通って到達可能でないtravel_targetsの点を列挙する
            let mut queue = VecDeque::new();
            queue.push_back(start);
            let mut visited = FixedBitSet::with_capacity(self.input.n);
            visited.insert(start);
            while let Some(node) = queue.pop_front() {
                for next in raw_graph.get(&node).unwrap() {
                    if visited[*next] || !travel_targets[*next] {
                        continue;
                    }
                    visited.insert(*next);
                    queue.push_back(*next);
                }
            }
            let mut unreachable = vec![];
            // travel_targetsなのに到達不可能な点をunreachableに追加
            for i in 0..self.input.n {
                if travel_targets[i] && !visited[i] {
                    unreachable.push(i);
                }
            }

            for u in unreachable.iter() {
                // uからunreachableでないかつtravel_targetsな点へいく最短距離のパスをBFSで探す
                let mut queue = VecDeque::new();
                queue.push_back(vec![*u]);
                let mut visited = FixedBitSet::with_capacity(self.input.n);
                visited.insert(*u);
                let mut found_path = vec![];
                while let Some(path) = queue.pop_front() {
                    let node = *path.last().unwrap();
                    if travel_targets[node] && !unreachable.contains(&node) {
                        found_path.clone_from(&path);
                        break;
                    }
                    for next in raw_graph.get(&node).unwrap() {
                        if visited[*next] {
                            continue;
                        }
                        visited.insert(*next);
                        queue.push_back({
                            let mut new_path = path.clone();
                            new_path.push(*next);
                            new_path
                        });
                    }
                }

                for p in found_path {
                    ignorables.set(p, false);
                }
            }
        }

        // {
        //     let mut tmp_components = vec![];
        //     for i in 0..self.input.n {
        //         if !ignorables[i] {
        //             tmp_components.push(vec![i]);
        //         }
        //     }
        //     visualize_components(
        //         &tmp_components,
        //         &self.input.nodes,
        //         &raw_graph,
        //         "construction.png",
        //     );
        // }

        {
            // High Cost Operation
            let mut last_lap_duration = None;
            let mut lap = 1;
            let mut prev_components_size = usize::MAX;
            // もし前のラップでcomponentsのサイズが変わらなかったら終了
            while components.len() != prev_components_size {
                prev_components_size = components.len();
                if let Some(last_lap_duration) = last_lap_duration {
                    // もし残り時間が前のラップの時間を超えたら終了
                    if start.elapsed().as_millis() + last_lap_duration > high_cost_operation_tl {
                        eprintln!("\x1b[33m[WARNING]\x1b[0m high cost operation timeout");
                        break;
                    }
                }

                let lap_start = Instant::now();

                let mut high_cost_operations = vec![];
                for i in 0..self.input.t {
                    let from = if i > 0 { self.input.ts[i - 1] } else { 0 };
                    let to: usize = self.input.ts[i];

                    //ダイクストラ法で最短経路を求める
                    let mut dist = vec![usize::MAX; self.input.n];
                    dist[from] = 0;
                    // パスがcomponent_edgesに含まれていればコスト1、含まれていなければコストself.input.lb
                    let mut queue = BinaryHeap::new();
                    queue.push(Node {
                        cost: 0,
                        node: from,
                    });
                    while let Some(Node { cost, node }) = queue.pop() {
                        if cost > dist[node] {
                            continue;
                        }
                        if node == to {
                            break;
                        }
                        for next in raw_graph.get(&node).unwrap() {
                            let next_cost = if component_edges.contains(node, *next) {
                                cost + 1
                            } else {
                                cost + self.input.lb
                            };
                            if next_cost < dist[*next] {
                                dist[*next] = next_cost;
                                queue.push(Node {
                                    cost: next_cost,
                                    node: *next,
                                });
                            }
                        }
                    }
                    high_cost_operations.push((dist[to], from, to));
                }

                high_cost_operations.sort_by_key(|x| -(x.0 as i64));
                // コスト総和を出力
                eprintln!(
                    "high cost operations: {:?}",
                    high_cost_operations.iter().map(|x| x.0).sum::<usize>()
                );

                'high_cost_challenges: for i in 0..high_cost_operations.len() {
                    let most_expensive_operation = high_cost_operations[i];
                    let from = most_expensive_operation.1;
                    let to = most_expensive_operation.2;
                    // all_1_distを使って最短経路を復元
                    let mut path = vec![];
                    let mut cur = to;
                    while cur != from {
                        for next in raw_graph.get(&cur).unwrap() {
                            if all_1_dist[*next][from] + 1 == all_1_dist[cur][from] {
                                path.push(cur);
                                cur = *next;
                                break;
                            }
                        }
                    }
                    path.push(from);
                    let components_size = components.iter().map(|x| x.len()).sum::<usize>();
                    let this_path_used = path.iter().copied().collect::<FixedBitSet>();
                    let mut components_used_if_path_added = components_used.clone();
                    components_used_if_path_added.union_with(&this_path_used);
                    // 全て訪れないといけないわけではなく、ignorableでない都市を訪れればよい
                    // ignorableをnotしてcomponents_used_if_path_addedとの差集合を取る
                    let mut missing_bits = ignorables.clone();
                    missing_bits.toggle_range(..);
                    missing_bits.difference_with(&components_used_if_path_added);
                    let missing = missing_bits.count_ones(..);

                    // componentsにpathを追加した時のaの余白を数える
                    let a_padding = (self.input.la as i32 - (components_size + path.len()) as i32)
                        .max(0) as usize;

                    // a_paddingがmissingより大きい場合はpathを追加
                    if a_padding >= missing {
                        for i in 0..(path.len() - 1) {
                            component_edges.insert(path[i], path[i + 1]);
                        }
                        components.push(path);
                        // component_usedとhigh_cost_usedのORを取る
                        components_used.union_with(&this_path_used);
                        break 'high_cost_challenges;
                    } else {
                        // let prev_padding = self.input.la - components_size;
                        // if prev_padding <= missing {
                        //     continue 'high_cost_challenges;
                        // }
                        // let mut truncated_path = vec![];
                        // for i in 0..(prev_padding - missing).min(path.len()) {
                        //     truncated_path.push(path[i]);
                        // }
                        // let mut truncated_high_cost_used =
                        //     FixedBitSet::with_capacity(self.input.n);
                        // for node in truncated_path.iter() {
                        //     truncated_high_cost_used.insert(*node);
                        // }
                        // for i in 0..(truncated_path.len() - 1) {
                        //     component_edges
                        //         .insert(truncated_path[i], truncated_path[i + 1]);
                        // }
                        // components.push(truncated_path);
                        // components_used.union_with(&truncated_high_cost_used);
                        continue 'high_cost_challenges;
                    }
                }

                // visualize_components(
                //     &components,
                //     &self.input.nodes,
                //     &raw_graph,
                //     "construction.png",
                // );

                let lap_elapsed = lap_start.elapsed().as_millis();
                eprintln!("lap: {}, time: {}ms", lap, lap_elapsed);

                last_lap_duration = Some(lap_elapsed);

                lap += 1;
            }

            eprintln!(
                "components size: {}",
                components.iter().map(|x| x.len()).sum::<usize>()
            );

            eprintln!(
                "high cost operation time: {}ms",
                start.elapsed().as_millis()
            );

            // visualize_components(
            //     &components,
            //     &self.input.nodes,
            //     &raw_graph,
            //     "construction_after_high_cost.png",
            // );
        }

        let mut best_score = usize::MAX;
        let mut laps = 1;

        for i in 0..self.input.n {
            if !ignorables[i] && !components_used[i] {
                components.push(vec![i]);
            }
        }

        let components_size = components.iter().map(|x| x.len()).sum::<usize>();

        eprintln!("after construction components size: {}", components_size);

        assert!(self.input.la >= components_size);

        let mut left = self.input.la - components_size;

        'construction: loop {
            let lap_start = Instant::now();

            let start_connect = Instant::now();
            #[derive(Eq, Hash, PartialEq, Copy, Clone)]
            struct ComponentID(usize);
            impl From<usize> for ComponentID {
                fn from(x: usize) -> Self {
                    ComponentID(x)
                }
            }
            let mut idg = IncrementalIDGenerator::<ComponentID>::new();
            // 各componentにidを振る
            let mut component_map = HashMap::new();
            let mut component_ids = vec![];
            for component in components.iter() {
                let id = idg.generate();
                component_ids.push(id);
                component_map.insert(id, component.clone());
            }
            let mut pairs = vec![];
            for i_id in component_ids.iter() {
                for j_id in component_ids.iter() {
                    if i_id.0 < j_id.0 {
                        pairs.push((*i_id, *j_id));
                    }
                }
            }
            pairs.shuffle(&mut rng);
            'connect: loop {
                while let Some((i, j)) = pairs.pop() {
                    // components[i]とcomponents[j]が始始・終終・始終・終始のいずれかで繋がっているか確認
                    // もし繋がっているならcomponents[i]とcomponents[j]を結合
                    for bit in 0..(1 << 2) {
                        let mut a = component_map.get(&i).unwrap().clone();
                        let mut b = component_map.get(&j).unwrap().clone();
                        if bit & 1 == 1 {
                            a.reverse();
                        }
                        if bit & 2 == 2 {
                            b.reverse();
                        }
                        let a_last_nexts = raw_graph.get(a.last().unwrap()).unwrap();
                        let b_first_nexts = raw_graph.get(b.first().unwrap()).unwrap();
                        // 一つあいて隣り合ってるケース: a_last_nextsとb_first_nextsに共通要素がある
                        // a_last_nextsとb_first_nextsの積集合をとる
                        let intersection = a_last_nexts
                            .iter()
                            .copied()
                            .filter(|x| b_first_nexts.contains(x))
                            .collect::<HashSet<_>>();
                        if !intersection.is_empty() && left > 0 {
                            let bridge_node = *intersection.iter().next().unwrap();
                            // bridge_nodeをaとbの間に挿入
                            a.push(bridge_node);
                            a.append(&mut b);
                            // aとbは両方なくなるので削除
                            component_map.remove(&i);
                            component_map.remove(&j);
                            // 古いidを削除
                            component_ids.retain(|x| x != &i);
                            component_ids.retain(|x| x != &j);
                            // 新しいcomponentを追加
                            let new_id = idg.generate();
                            component_map.insert(new_id, a);
                            component_ids.push(new_id);
                            left -= 1;
                            // 古いidを使ったpairsを削除
                            pairs.retain(|(x, y)| x != &i && x != &j && y != &i && y != &j);
                            // 新たな組み合わせができるのでpairsに追加
                            for k in component_ids.iter() {
                                if k != &new_id {
                                    if new_id.0 < k.0 {
                                        pairs.push((new_id, *k));
                                    } else {
                                        pairs.push((*k, new_id));
                                    }
                                }
                            }
                            continue 'connect;
                        }
                        // 隣り合ってるケース: a_last_nextsにb.first()が含まれているかつb_first_nextsにa.last()が含まれている
                        if a_last_nexts.contains(b.first().unwrap())
                            && b_first_nexts.contains(a.last().unwrap())
                        {
                            a.append(&mut b);
                            // aとbは両方なくなるので削除
                            component_map.remove(&i);
                            component_map.remove(&j);
                            // 古いidを削除
                            component_ids.retain(|x| x != &i);
                            component_ids.retain(|x| x != &j);
                            // 新しいcomponentを追加
                            let new_id = idg.generate();
                            component_map.insert(new_id, a);
                            component_ids.push(new_id);
                            // 古いidを使ったpairsを削除
                            pairs.retain(|(x, y)| x != &i && x != &j && y != &i && y != &j);
                            // 新たな組み合わせができるのでpairsに追加
                            for k in component_ids.iter() {
                                if k != &new_id {
                                    if new_id.0 < k.0 {
                                        pairs.push((new_id, *k));
                                    } else {
                                        pairs.push((*k, new_id));
                                    }
                                }
                            }
                            continue 'connect;
                        }
                        // 重なっているケース: a_lastとb_firstが同じ
                        if a.last() == b.first() {
                            a.pop();
                            a.append(&mut b);
                            // aとbは両方なくなるので削除
                            component_map.remove(&i);
                            component_map.remove(&j);
                            // 古いidを削除
                            component_ids.retain(|x| x != &i);
                            component_ids.retain(|x| x != &j);
                            // 新しいcomponentを追加
                            let new_id = idg.generate();
                            component_map.insert(new_id, a);
                            component_ids.push(new_id);
                            // 古いidを使ったpairsを削除
                            pairs.retain(|(x, y)| x != &i && x != &j && y != &i && y != &j);
                            // 新たな組み合わせができるのでpairsに追加
                            for k in component_ids.iter() {
                                if k != &new_id {
                                    if new_id.0 < k.0 {
                                        pairs.push((new_id, *k));
                                    } else {
                                        pairs.push((*k, new_id));
                                    }
                                }
                            }
                            // leftを増やす
                            left += 1;
                            continue 'connect;
                        }
                    }
                }

                break;
            }

            let components = component_ids
                .iter()
                .map(|x| component_map.get(x))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().clone())
                .collect::<Vec<_>>();

            eprintln!(
                "components size: {}",
                components.iter().map(|x| x.len()).sum::<usize>()
            );

            eprintln!("connect time: {}ms", start_connect.elapsed().as_millis());

            // visualize_components(
            //     &components,
            //     &self.input.nodes,
            //     &raw_graph,
            //     &format!("construction_{}_after_connect.png", laps),
            // );

            let a = components
                .iter()
                .flat_map(|x| x.iter().copied())
                .collect::<Vec<_>>();

            eprintln!("a size: {}", a.len());
            eprintln!("a left: {}", self.input.la - a.len());

            let mut operations = Vec::new();

            let mut modified_graph: HashMap<usize, Vec<GraphNodeInfo>> = HashMap::new();
            (1..=self.input.lb).for_each(|width| {
                for start in 0..(a.len() - width + 1) {
                    // aのstart+1からstart+widthまでの部分列をmodifed_graphに追加、逆も追加
                    let mut path = vec![];
                    let mut path_used = FixedBitSet::with_capacity(self.input.n);
                    for ai in a.iter().skip(start).take(width) {
                        path.push(*ai);
                        path_used.insert(*ai);
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
                        if path_used[*next_start_node] {
                            continue;
                        }
                        modified_graph
                            .entry(*next_start_node)
                            .or_default()
                            .push(GraphNodeInfo {
                                start,
                                last_node: path[width - 1],
                                length: width,
                                reversed: false,
                                points: path_used.clone(),
                            });
                    }
                    if width == 1 {
                        continue;
                    }
                    // 逆も追加
                    path.reverse();
                    let next_start_nodes = raw_graph.get(&path[0]).unwrap();
                    for next_start_node in next_start_nodes {
                        if path_used[*next_start_node] {
                            continue;
                        }
                        modified_graph
                            .entry(*next_start_node)
                            .or_default()
                            .push(GraphNodeInfo {
                                start,
                                last_node: path[width - 1],
                                length: width,
                                reversed: true,
                                points: path_used.clone(),
                            });
                    }
                }
            });

            let mut cur_node = 0; // 都市0を最初とする
            let mut cur_b: Vec<i32> = vec![-1; self.input.lb];
            let mut score = 0;

            // 都市を順に訪れる
            let mut current_target_idx = 0;
            while current_target_idx < self.input.ts.len() {
                let t = self.input.ts.get(current_target_idx).unwrap();
                let next_t = self.input.ts.get(current_target_idx + 1);

                if start.elapsed().as_millis() > tl {
                    eprintln!("timeout");
                    break 'construction;
                }

                // dijkstraだとO(V+ElogV)かかるので、01-BFSで計算量をO(V+E)にする
                let mut dist = vec![usize::MAX; self.input.n];
                dist[cur_node] = 0;
                let mut best_paths = vec![];
                let mut cur_b_points = FixedBitSet::with_capacity(self.input.n);
                for bi in cur_b.iter() {
                    if *bi != -1 {
                        cur_b_points.insert(*bi as usize);
                    }
                }
                // そしてcur_bを辿っていけるところを全てqueueに入れる
                let mut pre_queue = vec![];
                pre_queue.push((cur_node, vec![]));
                {
                    let mut micro_bfs_queue = VecDeque::new();
                    micro_bfs_queue.push_back((cur_node, vec![]));
                    let mut visited = FixedBitSet::with_capacity(self.input.n);
                    while let Some((node, paths)) = micro_bfs_queue.pop_front() {
                        for next_path in raw_graph.get(&node).unwrap() {
                            if !cur_b_points[*next_path] {
                                continue;
                            }
                            if visited[*next_path] {
                                continue;
                            }
                            visited.insert(*next_path);
                            let mut next_paths = paths.clone();
                            let graph_node = modified_graph
                                .get(&node)
                                .unwrap()
                                .iter()
                                .find(|x| x.last_node == *next_path)
                                .unwrap();
                            next_paths.push(graph_node);
                            micro_bfs_queue.push_back((*next_path, next_paths.clone()));
                            pre_queue.push((*next_path, next_paths));
                            dist[*next_path] = 0;
                        }
                    }
                }
                // もしnext_tがあれば
                if let Some(next_t) = next_t {
                    // pre_queueをnext_tまでの距離が小さい順にソート
                    pre_queue.sort_by_key(|x| {
                        let node = x.0;
                        all_1_dist[node][*next_t]
                    });
                }

                // pre_queueを使ってqueueを作る
                let mut queue = VecDeque::new();
                for (node, paths) in pre_queue {
                    queue.push_back((node, paths));
                }

                let mut super_check_bit = FixedBitSet::with_capacity(self.input.n);
                super_check_bit.insert(*t);
                if let Some(next_t) = next_t {
                    super_check_bit.insert(*next_t);
                }
                let mut is_super_route = false;

                'dijkstra: while let Some((node, paths)) = queue.pop_front() {
                    if node == *t {
                        best_paths = paths;
                        // continue;
                        break;
                    }
                    let mut nexts = vec![];
                    for next_path in modified_graph.get(&node).unwrap() {
                        let next_node = next_path.last_node;
                        let d = dist[node] + 1;
                        if d < dist[next_node] {
                            dist[next_node] = d;
                            let mut next_paths = paths.clone();
                            next_paths.push(next_path);
                            let goal_dist = all_1_dist[next_node][*t];
                            nexts.push((goal_dist, next_node, next_paths.clone()));
                            // tとnext_tを両方カバーしているパスを発見したらそれを使う(super_check_bitがnext_path.pointsの部分集合であるか)
                            if super_check_bit.is_subset(&next_path.points) {
                                operations.push(Operation::Comment("SUPER ROUTE".to_string()));
                                best_paths = next_paths;
                                is_super_route = true;
                                break 'dijkstra;
                            }
                        }
                    }

                    // nextsは距離が小さいものからpush_backする
                    nexts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    for (_, next_node, next_paths) in nexts {
                        queue.push_back((next_node, next_paths));
                    }
                }

                assert!(!best_paths.is_empty());

                // 目的地に到着するまで
                for (i, next_path) in best_paths.iter().enumerate() {
                    let is_end_path = i == best_paths.len() - 1;
                    let mut path = vec![];
                    if next_path.reversed {
                        for i in (0..next_path.length).rev() {
                            path.push(a[next_path.start + i]);
                        }
                    } else {
                        for i in 0..next_path.length {
                            path.push(a[next_path.start + i]);
                        }
                    }
                    // next_pathの要素が全てcur_bに含まれているか確認
                    let mut is_all_in_cur_b = true;
                    for next in path.iter() {
                        if !cur_b.contains(&(*next as i32)) {
                            is_all_in_cur_b = false;
                            break;
                        }
                    }
                    if !is_all_in_cur_b {
                        // 最もいい位置でbを更新したい
                        // cur_bから長さpath.len()の部分列を取り出し
                        let mut best_bi = 0;
                        let mut best_score = 0;
                        // スコアが最大になるようなbiを探す
                        for bi in 0..(self.input.lb - path.len() + 1) {
                            // スコアは部分列のそれぞれの都市と次の都市の距離の和
                            let mut score = 0;
                            for i in 0..path.len() {
                                if cur_b[bi + i] == -1 {
                                    // usize::MAXを足すとオーバーフローするのである程度でかくオーバーフローしない値を足す
                                    score += 100000;
                                } else {
                                    score += all_1_dist[cur_b[bi + i] as usize][*t];
                                }
                            }
                            if score > best_score {
                                best_score = score;
                                best_bi = bi;
                            }
                        }
                        // next_pathを青にする
                        operations.push(Operation::SignalUpdate(SignalUpdate {
                            len: path.len(),
                            ai: next_path.start,
                            bi: best_bi,
                        }));
                        score += 1;

                        let mut next_path_for_b = path.clone();
                        if next_path.reversed {
                            next_path_for_b.reverse();
                        }
                        for next_node in next_path_for_b.iter() {
                            cur_b[best_bi] = *next_node as i32;
                            best_bi += 1;
                        }
                    }
                    for next in path.iter() {
                        operations.push(Operation::Move(*next));
                        if current_target_idx < self.input.ts.len()
                            && *next == self.input.ts[current_target_idx]
                        {
                            current_target_idx += 1;
                        }
                    }
                    if is_end_path && is_super_route {
                        for next in path.iter().rev().skip(1) {
                            operations.push(Operation::Move(*next));
                            if current_target_idx < self.input.ts.len()
                                && *next == self.input.ts[current_target_idx]
                            {
                                current_target_idx += 1;
                            }
                        }
                        for next in path.iter().skip(1) {
                            operations.push(Operation::Move(*next));
                            if current_target_idx < self.input.ts.len()
                                && *next == self.input.ts[current_target_idx]
                            {
                                current_target_idx += 1;
                            }
                        }
                    }
                }

                cur_node = best_paths.last().unwrap().last_node;
            }

            if best_score > score {
                best_score = score;
                best_a.clone_from(&a);
                best_operations.clone_from(&operations);
            }

            let lap_time = lap_start.elapsed().as_millis();

            eprintln!(
                "lap: {}, score: {}, best_score: {}, lap_time: {}ms",
                laps, score, best_score, lap_time
            );
            laps += 1;
        }

        // // TODO: 最終提出には消す
        // // もしoperationsが空の場合、それは時間内に終わらなかったことを意味するので意図的にRuntime Errorを発生させる
        // if best_operations.is_empty() {
        //     panic!("Runtime Error");
        // }

        // つじつま合わせでもしaが足りなかったら適当に追加
        if best_a.len() < self.input.la {
            eprintln!(
                "\x1b[33m[WARNING]\x1b[0m a is not enough, {} < {}",
                best_a.len(),
                self.input.la
            );
            while best_a.len() < self.input.la {
                best_a.push(0);
            }
        }

        eprintln!("\x1b[32m[INFO]\x1b[0m best_score: {}", best_score);

        // visualize_a(&best_a, &self.input.nodes, &raw_graph, "construction.png");

        Output {
            operations: best_operations,
            a: best_a,
        }
    }
}
