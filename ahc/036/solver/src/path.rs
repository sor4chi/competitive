use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    mem::swap,
};

use crate::{
    io::{Operation, SignalUpdate},
    original_lib::id::IncrementalIDGenerator,
};

#[derive(Clone)]
pub struct Path {
    pub nodes: VecDeque<usize>,
    // 先頭が隣接するパスのID
    pub first_next_paths_ids: HashSet<PathID>,
    // 末尾が隣接するパスのID
    pub last_next_paths_ids: HashSet<PathID>,
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "    nodes: {:?},\n    first_next_paths_ids: {:?},\n    last_next_paths_ids: {:?}",
            self.nodes, self.first_next_paths_ids, self.last_next_paths_ids
        )
    }
}

impl Path {
    fn flip(&mut self) {
        self.nodes = self.nodes.iter().copied().rev().collect();
        swap(
            &mut self.first_next_paths_ids,
            &mut self.last_next_paths_ids,
        );
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct PathID(pub usize);

impl From<usize> for PathID {
    fn from(id: usize) -> Self {
        PathID(id)
    }
}

#[derive(Clone)]
pub struct PathGroup {
    // a
    a: Vec<usize>,
    // id生成用
    id_generator: IncrementalIDGenerator<PathID>,
    // 入力の辺
    raw_edges: Vec<(usize, usize)>,
    // 入力の点
    raw_nodes: Vec<(usize, usize)>,
    // グラフ
    graph: HashMap<usize, Vec<usize>>,
    // id -> パス
    pub path_map: HashMap<PathID, Path>,
    // 点 -> id
    pub point_map: HashMap<usize, PathID>,
    // lb
    lb: usize,
}

impl Debug for PathGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // path_mapを見やすく
        let mut path_map_str = String::new();
        // sort by id
        let mut path_map_vec = self.path_map.iter().collect::<Vec<_>>();
        path_map_vec.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
        for (id, path) in path_map_vec {
            path_map_str.push_str(&format!("  {:?}:\n{:?},\n", id, path));
        }
        write!(
            f,
            "PathGroup {{\n  a: {:?},\n  path_map: {{\n{}  }},\n  point_map: {:?},\n  lb: {}\n}}",
            self.a, path_map_str, self.point_map, self.lb
        )
    }
}

impl PathGroup {
    pub fn new(raw_edges: Vec<(usize, usize)>, raw_nodes: Vec<(usize, usize)>, lb: usize) -> Self {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for (a, b) in &raw_edges {
            graph.entry(*a).or_default().push(*b);
            graph.entry(*b).or_default().push(*a);
        }
        PathGroup {
            a: vec![],
            id_generator: IncrementalIDGenerator::<PathID>::with_start(1000),
            raw_edges,
            raw_nodes,
            graph,
            path_map: HashMap::new(),
            point_map: HashMap::new(),
            lb,
        }
    }

    pub fn initialize_with_separated_nodes(&mut self) {
        for i in 0..self.raw_nodes.len() {
            let id = PathID::from(i);
            self.point_map.insert(i, id);
            self.path_map.insert(
                id,
                Path {
                    nodes: VecDeque::from(vec![i]),
                    first_next_paths_ids: HashSet::new(),
                    last_next_paths_ids: HashSet::new(),
                },
            );
        }
        // 最初は1つの点なのでgraph[i]を使って隣接点を取得しfirst_next_paths_ids, last_next_paths_idsを更新
        for (a, b) in &self.raw_edges {
            let a_id = self.point_map[a];
            let b_id = self.point_map[b];
            let a_path = self.path_map.get_mut(&a_id).unwrap();
            a_path.first_next_paths_ids.insert(b_id);
            a_path.last_next_paths_ids.insert(b_id);
            let b_path = self.path_map.get_mut(&b_id).unwrap();
            b_path.first_next_paths_ids.insert(a_id);
            b_path.last_next_paths_ids.insert(a_id);
        }
    }

    pub fn reset(&mut self) {
        self.a.clear();
        self.path_map.clear();
        self.point_map.clear();
        self.initialize_with_separated_nodes();
    }

    pub fn evaluate(&self) -> f64 {
        // 全てのpath平均が高いほど良い
        let mut sum = 0;
        for path in self.path_map.values() {
            sum += path.nodes.len();
        }
        sum as f64 / self.path_map.len() as f64
    }

    fn shortest_path(&mut self, start: usize, target: usize) -> (Vec<Operation>, usize) {
        // 01-bfs
        let mut queue = VecDeque::new();
        let mut dist_map = HashMap::new();
        let a = self.get_a();
        struct BFSState {
            current: usize,
            dist: usize,
            depth: usize,
            operations: Vec<Operation>,
        }
        let initial_state = BFSState {
            current: start,
            dist: 0,
            depth: 0,
            operations: vec![],
        };
        dist_map.insert(start, 0);
        queue.push_back(initial_state);

        let mut best_operations = vec![];
        let mut best_cost = std::usize::MAX;
        while let Some(BFSState {
            current,
            dist,
            depth,
            operations,
        }) = queue.pop_front()
        {
            // 現在地がtargetなら,もし最短経路なら更新
            if current == target && dist < best_cost {
                best_operations = operations.clone();
                best_cost = dist;
            }
            // 現在地の隣接点をgraphから取得
            let nexts = &self.graph[&current];
            let current_path_id = self.point_map[&current];
            for &next in nexts {
                // 隣接点にいくためのコストを計算する。パスを跨いだり、パス内をlb以上動くたびにコストが1かかる、それ以外は0
                let next_path_id = self.point_map[&next];
                let c =
                    if current_path_id == next_path_id && depth < self.lb && !operations.is_empty()
                    {
                        0
                    } else {
                        1
                    };
                let d = dist + c;
                if !dist_map.contains_key(&next) || d < dist_map[&next] {
                    dist_map.insert(next, d);
                    if c == 0 {
                        let mut new_operations = operations.clone();
                        new_operations.push(Operation::Move(next));
                        queue.push_front(BFSState {
                            current: next,
                            dist: d,
                            depth: depth + 1,
                            operations: new_operations,
                        });
                    } else {
                        let mut new_operations = operations.clone();
                        // aにあるnextのindexを取得
                        let a_idx = a.iter().position(|&x| x == next).unwrap();
                        new_operations.push(Operation::SignalUpdate(SignalUpdate {
                            len: self.lb,
                            ai: a_idx,
                            bi: 0,
                        }));
                        queue.push_back(BFSState {
                            current: next,
                            dist: d,
                            depth: 0,
                            operations: new_operations,
                        });
                    }
                }
            }
        }

        // let mut stack = vec![(target, vec![target])];
        // let mut founded_path = vec![];
        // let mut visited = HashSet::new();
        // while let Some((current, path)) = stack.pop() {
        //     if current == start {
        //         founded_path = path;
        //         break;
        //     }
        //     let mut nexts = vec![];
        //     for &next in &self.graph[&current] {
        //         if visited.contains(&next) {
        //             continue;
        //         }
        //         nexts.push((next, dist_map[&next]));
        //     }
        //     nexts.sort_by_key(|x| -(x.1 as i64));
        //     for (next, _) in nexts {
        //         let mut new_path = path.clone();
        //         new_path.push(next);
        //         stack.push((next, new_path));
        //         visited.insert(next);
        //     }
        // }

        // founded_path.pop();
        // founded_path.reverse();

        // (founded_path, dist_map[&target])
        (best_operations, dist_map[&target])
    }

    pub fn connect_paths(&mut self, a: PathID, b: PathID) -> Result<PathID, ()> {
        // aとbが隣接パスであることを確認
        // aの隣接点にbがあるか
        let a_path = &self.path_map[&a];
        let a_next_paths = vec![
            a_path.first_next_paths_ids.iter(),
            a_path.last_next_paths_ids.iter(),
        ]
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>();
        if !a_next_paths.contains(&b) {
            return Err(());
        }

        // aとbをつなぐ
        let a_path = &self.path_map[&a];
        let b_path = &self.path_map[&b];
        let mut new_path = Path {
            nodes: VecDeque::new(),
            first_next_paths_ids: HashSet::new(),
            last_next_paths_ids: HashSet::new(),
        };

        // bit全探索、2桁で表表、表裏、裏表、裏裏の4通り
        for bit in 0..(1 << 2) {
            let mut a_path = a_path.clone();
            let mut b_path = b_path.clone();
            if bit & 1 == 1 {
                a_path.flip();
            }
            if bit & 2 == 2 {
                b_path.flip();
            }
            // a_pathのlast_next_paths_idsにbかつb_pathのfist_next_paths_idsにaがあるか
            if !a_path.last_next_paths_ids.contains(&b) || !b_path.first_next_paths_ids.contains(&a)
            {
                continue;
            }

            // グラフ上で隣接かどうかを確認
            let a_last = a_path.nodes.back().unwrap();
            let b_first = b_path.nodes.front().unwrap();
            if !self.graph[a_last].contains(b_first) {
                continue;
            }

            new_path.nodes.extend(a_path.nodes.iter().copied());
            new_path.nodes.extend(b_path.nodes.iter().copied());

            new_path
                .first_next_paths_ids
                .clone_from(&a_path.first_next_paths_ids);
            // パスが1点の時の対策
            new_path.first_next_paths_ids.remove(&b);

            new_path
                .last_next_paths_ids
                .clone_from(&b_path.last_next_paths_ids);
            // パスが1点の時の対策
            new_path.last_next_paths_ids.remove(&a);

            break;
        }

        // 新たなパスのidを生成
        let new_id = self.id_generator.generate();

        // first_next_paths_idsの隣接関係を更新
        let all_next_paths_ids = new_path
            .first_next_paths_ids
            .union(&new_path.last_next_paths_ids)
            .cloned()
            .collect::<HashSet<_>>();

        // a,bのnext_pathsの全体集合
        let past_all_next_paths_ids = a_path
            .first_next_paths_ids
            .union(&a_path.last_next_paths_ids)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&b_path.first_next_paths_ids)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&b_path.last_next_paths_ids)
            .cloned()
            .collect::<HashSet<_>>();

        for next_path_id in all_next_paths_ids {
            let next_path = self.path_map.get_mut(&next_path_id).unwrap();
            // もしfirst_next_paths_idsにaかbがあれば、new_idを追加
            if next_path.first_next_paths_ids.contains(&a)
                || next_path.first_next_paths_ids.contains(&b)
            {
                next_path.first_next_paths_ids.insert(new_id);
            }
            // もしlast_next_paths_idsにaかbがあれば、new_idを追加
            if next_path.last_next_paths_ids.contains(&a)
                || next_path.last_next_paths_ids.contains(&b)
            {
                next_path.last_next_paths_ids.insert(new_id);
            }
        }

        // 過去の隣接関係を全て削除
        for next_path_id in past_all_next_paths_ids {
            let next_path = self.path_map.get_mut(&next_path_id).unwrap();
            next_path.first_next_paths_ids.remove(&a);
            next_path.first_next_paths_ids.remove(&b);
            next_path.last_next_paths_ids.remove(&a);
            next_path.last_next_paths_ids.remove(&b);
        }

        // 新たなパスを登録
        self.path_map.insert(new_id, new_path);
        // 古いパスを削除
        self.path_map.remove(&a);
        self.path_map.remove(&b);
        // 点 -> idの更新
        let old_points_map = self.point_map.clone();
        for point in old_points_map.keys() {
            if self.point_map[point] == a || self.point_map[point] == b {
                self.point_map.insert(*point, new_id);
            }
        }
        Ok(new_id)
    }

    pub fn get_a(&mut self) -> Vec<usize> {
        if !self.a.is_empty() {
            return self.a.clone();
        }
        for path in self.path_map.values() {
            self.a.extend(path.nodes.iter().copied());
        }
        self.a.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_group() {
        let raw_edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let raw_nodes = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        let mut path_group = PathGroup::new(raw_edges.clone(), raw_nodes.clone(), 0);
        path_group.initialize_with_separated_nodes();

        assert_eq!(path_group.path_map.len(), 4);
        assert_eq!(path_group.point_map.len(), 4);
        // 0を含むパスのIDを取得
        let zero_contained_path_id = path_group.point_map[&0];
        // 0を含むパスの点を取得
        let zero_contained_path_nodes = path_group.path_map[&zero_contained_path_id]
            .nodes
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        assert_eq!(zero_contained_path_nodes, HashSet::from_iter(vec![0]));
        // 0を含むパスの隣接する全てのパスのIDを取得
        let zero_contained_path_next_ids = vec![
            path_group.path_map[&zero_contained_path_id]
                .first_next_paths_ids
                .iter(),
            path_group.path_map[&zero_contained_path_id]
                .last_next_paths_ids
                .iter(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
        // 0を含むパスの隣接する全てのパスに含まれる点を取得
        let next_paths = zero_contained_path_next_ids
            .iter()
            .flat_map(|id| path_group.path_map[id].nodes.clone())
            .collect::<HashSet<_>>();
        assert_eq!(next_paths, HashSet::from_iter(vec![1, 3]));
        let a_id = path_group.point_map[&0];
        let b_id = path_group.point_map[&1];
        path_group.connect_paths(a_id, b_id).unwrap();
        assert_eq!(path_group.path_map.len(), 3);
        assert_eq!(path_group.point_map.len(), 4);
        // 0を含むパスのIDを取得
        let updated_zero_contained_path_id = path_group.point_map[&0];
        assert_ne!(zero_contained_path_id, updated_zero_contained_path_id);
        // 0を含むパスの点を取得
        let updated_zero_contained_path_nodes = path_group.path_map
            [&updated_zero_contained_path_id]
            .nodes
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        assert_eq!(
            updated_zero_contained_path_nodes,
            HashSet::from_iter(vec![0, 1])
        );
        // 0を含むパスの隣接する全てのパスのIDを取得
        let updated_zero_contained_path_next_ids = vec![
            path_group.path_map[&updated_zero_contained_path_id]
                .first_next_paths_ids
                .iter(),
            path_group.path_map[&updated_zero_contained_path_id]
                .last_next_paths_ids
                .iter(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();

        // 0を含むパスの隣接する全てのパスに含まれる点を取得
        let next_paths = updated_zero_contained_path_next_ids
            .iter()
            .flat_map(|id| path_group.path_map[id].nodes.clone())
            .collect::<HashSet<_>>();
        assert_eq!(next_paths, HashSet::from_iter(vec![2, 3]));
    }

    #[test]
    fn test_path_group_vis() {
        let raw_edges = vec![
            (0, 5),
            (1, 2),
            (1, 3),
            (2, 4),
            (3, 4),
            (3, 6),
            (3, 7),
            (5, 7),
            (5, 8),
        ];
        let raw_nodes = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        let mut path_group = PathGroup::new(raw_edges.clone(), raw_nodes.clone(), 0);
        path_group.initialize_with_separated_nodes();
        eprintln!("{:?}", path_group);
        let a_id = path_group.point_map[&3];
        let b_id = path_group.point_map[&4];
        let id_generated_1 = path_group.connect_paths(a_id, b_id).unwrap();
        eprintln!("{:?}", path_group);
        let a_id = path_group.point_map[&5];
        let b_id = path_group.point_map[&7];
        let id_generated_2 = path_group.connect_paths(a_id, b_id).unwrap();
        eprintln!("{:?}", path_group);
        let b_id = path_group.point_map[&0];
        let id_generated_3 = path_group.connect_paths(id_generated_2, b_id).unwrap();
        eprintln!("{:?}", path_group);
        path_group.connect_paths(id_generated_1, id_generated_3);
        eprintln!("{:?}", path_group);
    }
}
