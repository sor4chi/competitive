use std::collections::VecDeque;

use crate::{
    board::{Board, HashTable},
    io::{Input, Operation, Output, IO},
};

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

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        let mut operations = vec![];
        let mut board = Board::new(self.input.h, self.input.w);
        let mut score = 0;
        let hash_table = HashTable::new(self.input.h, self.input.w);
        for t in 0..self.input.n {
            let mut best_pos = None;
            let jewel = self.input.a[t];
            // jewelを置く場所を探す
            let mut q = VecDeque::new();
            let first = match jewel {
                1 => (0, 0),
                2 => (0, self.input.w - 1),
                3 => (self.input.h - 1, 0),
                4 => (self.input.h - 1, self.input.w - 1),
                _ => unreachable!(),
            };
            let mut visited = vec![vec![false; self.input.w]; self.input.h];
            q.push_back(first);
            visited[first.0][first.1] = true;
            while let Some((r, c)) = q.pop_front() {
                if board.is_placable(r, c) {
                    best_pos = Some((r, c));
                    break;
                }
                for (dr, dc) in DIR.iter() {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= self.input.h as i32 || nc < 0 || nc >= self.input.w as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if visited[nr][nc] {
                        continue;
                    }
                    visited[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }

            let mut place = None;
            if let Some(best_pos) = best_pos {
                place = Some((best_pos.0 + 1, best_pos.1 + 1));

                board.place(best_pos.0, best_pos.1, jewel, &hash_table);
            } else {
                score += 100; // 捨てる
            }

            // すべて盤面が埋まっているか、もし最後のターンなら整理する
            let organize = board.is_all_filled() || t == self.input.n - 1;
            if organize {
                score += board.organize(&hash_table);
            }

            operations.push(Operation { place, organize });
        }

        eprintln!("greedy = {}", score);

        Output { operations, score }
    }
}
