use std::collections::{HashMap, VecDeque};

use crate::io::{Input, Operation, Output, SignalUpdate, IO};

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

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        let mut cur_b = vec![-1; self.input.lb];
        let mut cur_bi = 0;
        let mut graph = HashMap::new();
        for (a, b) in self.input.edges.iter() {
            graph.entry(*a).or_insert_with(Vec::new).push(*b);
            graph.entry(*b).or_insert_with(Vec::new).push(*a);
        }
        let mut cur_node = 0; // 都市0を最初とする
        let mut a = (0..self.input.n).collect::<Vec<_>>();
        (self.input.n..self.input.la).for_each(|_| a.push(0));
        let mut operations = Vec::new();

        // 都市を順に訪れる
        for t in &self.input.ts {
            // currentからtまでの最短経路を探す
            let mut found_path = Vec::new();
            let mut visited = vec![false; self.input.n];
            let mut queue = VecDeque::new();
            queue.push_back((cur_node, vec![]));
            while let Some((node, path)) = queue.pop_front() {
                if node == *t {
                    found_path = path;
                    break;
                }
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                for &next in graph.get(&node).unwrap() {
                    let mut next_path = path.clone();
                    next_path.push(next);
                    queue.push_back((next, next_path));
                }
            }

            // 目的地に到着するまで
            for next in found_path {
                // もしcur_bにnextが含まれていなければ現在地の次の都市の信号を青にする
                if !cur_b.contains(&(next as i32)) {
                    operations.push(Operation::SignalUpdate(SignalUpdate {
                        len: 1,
                        ai: next,
                        bi: cur_bi,
                    }));
                    cur_b[cur_bi] = next as i32;
                    cur_bi = (cur_bi + 1) % self.input.lb;
                }
                // 都市tに移動
                operations.push(Operation::Move(next));
                cur_node = *t;
            }
        }

        eprintln!("{:?}", cur_b);

        Output { a, operations }
    }
}
