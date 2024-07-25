use crate::io::{Input, IO};
use rand::{prelude::SliceRandom, Rng};
use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
};

use super::Solver;

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<usize>>,
}

impl Board {
    fn new(input: &Input) -> Self {
        Board {
            cells: vec![vec![0; input.n]; input.n],
        }
    }
}

pub struct GreedySolver {
    input: Input,
    io: IO,
}

impl GreedySolver {
    pub fn new(io: IO, input: Input) -> Self {
        GreedySolver { input, io }
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self) {
        let mut known_board = vec![vec![-1; self.input.n]; self.input.n];
        let mut rng = rand::thread_rng();
        let mut sorted_minos = self.input.minos.clone();
        sorted_minos.sort_by_key(|mino| -(mino.d as i32));
        // 雑にランダム配置で100000個盤面生成
        let mut boards = Vec::new();
        while boards.len() < 100000 {
            let mut board = Board::new(&self.input);
            for mino in &self.input.minos {
                let x = rng.gen_range(0..self.input.n - mino.height + 1);
                let y = rng.gen_range(0..self.input.n - mino.width + 1);
                for i in 0..mino.shape.len() {
                    for j in 0..mino.shape[i].len() {
                        if mino.shape[i][j] {
                            board.cells[x + i][y + j] += 1;
                        }
                    }
                }
            }
            boards.push(board);
        }

        loop {
            // 情報量が多いセルからdigする
            let mut mis = Vec::new();
            for x in 0..self.input.n {
                for y in 0..self.input.n {
                    if known_board[x][y] != -1 {
                        continue;
                    }
                    let expect_max_dup_cnt = 10;
                    let mut cnt = vec![0; expect_max_dup_cnt];
                    for board in &boards {
                        cnt[board.cells[x][y]] += 1;
                    }
                    let mut mi = 0.0;
                    for c in cnt.iter().take(expect_max_dup_cnt) {
                        if *c == 0 {
                            continue;
                        }
                        let p = *c as f64 / boards.len() as f64;
                        mi += -p * p.log2();
                    }
                    mis.push((mi, x, y));
                }
            }

            mis.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            // 10個ほる
            for (_, x, y) in mis.iter().rev().take(30) {
                let res = self.io.query_dig(*x, *y);
                known_board[*x][*y] = res as i32;
            }

            // // 一番大きいミノから順に配置していく
            // let mut board_candidates = Vec::new();
            // let mut q = VecDeque::new();
            // let board = Board::new(&self.input);
            // q.push_back(board);
            // for mino in &sorted_minos {
            //     let mut next_q = VecDeque::new();
            //     while let Some(board) = q.pop_front() {
            //         for x in 0..self.input.n - mino.height + 1 {
            //             for y in 0..self.input.n - mino.width + 1 {
            //                 let mut next_board = board.clone();
            //                 let mut is_ok = true;
            //                 'outer: for i in 0..mino.shape.len() {
            //                     for j in 0..mino.shape[i].len() {
            //                         if mino.shape[i][j] {
            //                             if known_board[x + i][y + j] == 0 {
            //                                 is_ok = false;
            //                                 break 'outer;
            //                             }
            //                             next_board.cells[x + i][y + j] += 1;
            //                             if known_board[x + i][y + j] != -1
            //                                 && known_board[x + i][y + j]
            //                                     < next_board.cells[x + i][y + j] as i32
            //                             {
            //                                 is_ok = false;
            //                                 break 'outer;
            //                             }
            //                         }
            //                     }
            //                 }
            //                 if is_ok {
            //                     next_q.push_back(next_board);
            //                 }
            //             }
            //         }
            //     }

            //     eprintln!("next_q.len() = {}", next_q.len());
            //     // シャッフルして上位30個だけ残す
            //     let mut next_q = next_q.into_iter().collect::<Vec<_>>();
            //     next_q.shuffle(&mut rng);
            //     q = next_q.into_iter().take(30).collect::<VecDeque<_>>();
            // }

            // known_boardの中でもっとも多い値を持つセル周りから決めていく
            let mut board_candidates = Vec::new();
            let mut q = VecDeque::new();
            let board = Board::new(&self.input);
            q.push_back((board, HashSet::new()));
            // known_boardを優先順位に並べる
            let mut known_board_order = Vec::new();
            for x in 0..self.input.n {
                for y in 0..self.input.n {
                    if known_board[x][y] != -1 {
                        known_board_order.push((known_board[x][y], x, y));
                    }
                }
            }
            known_board_order.sort_by_key(|a| -a.0);
            for (v, cx, cy) in known_board_order.iter().take(self.input.m) {
                let mut next_q = VecDeque::new();
                while let Some(board) = q.pop_front() {
                    let mut minos = self.input.minos.clone();
                    minos.shuffle(&mut rng);
                    for mino in &minos {
                        if board.1.contains(&mino.id) {
                            continue;
                        }
                        // (cx,cy)が含まれないといけないので、必然的に左上は(max(cx - mino.height + 1, 0), max(cy - mino.width + 1, 0))、右下は(min(cx + mino.height - 1, self.input.n - mino.height), min(cy + mino.width - 1, self.input.n - mino.width))の範囲になる
                        // overflowに注意
                        let from_x = max(*cx as i32 - mino.height as i32 + 1, 0) as usize;
                        let from_y = max(*cy as i32 - mino.width as i32 + 1, 0) as usize;
                        let to_x = min(*cx + mino.height - 1, self.input.n - mino.height);
                        let to_y = min(*cy + mino.width - 1, self.input.n - mino.width);
                        for x in from_x..=to_x {
                            for y in from_y..=to_y {
                                let mut next_board = board.clone();
                                let mut is_ok = true;
                                'outer: for i in 0..mino.height {
                                    for j in 0..mino.width {
                                        if mino.shape[i][j] {
                                            if known_board[x + i][y + j] == 0 {
                                                is_ok = false;
                                                break 'outer;
                                            }
                                            next_board.0.cells[x + i][y + j] += 1;
                                        }
                                    }
                                }
                                // next_boardのcx,cyがv以下であることを確認
                                if next_board.0.cells[*cx][*cy] > *v as usize {
                                    is_ok = false;
                                }
                                if is_ok {
                                    let mut next_set = board.1.clone();
                                    next_set.insert(mino.id);
                                    next_q.push_back((next_board.0, next_set));
                                }
                            }
                        }
                    }
                }

                eprintln!("next_q.len() = {}", next_q.len());
                // シャッフルして上位30個だけ残す
                let mut next_q = next_q.into_iter().collect::<Vec<_>>();
                next_q.shuffle(&mut rng);
                q = next_q.into_iter().take(30).collect::<VecDeque<_>>();
            }

            while let Some(board) = q.pop_front() {
                board_candidates.push(board);
            }

            self.io.debug_clear(&self.input);
            // board_candidates.len()が5以下であれば回答を試す
            if board_candidates.len() <= 5 {
                for board in &board_candidates {
                    let mut is_valid = true;
                    let mut board_info = Vec::new();
                    for x in 0..self.input.n {
                        for y in 0..self.input.n {
                            if known_board[x][y] != -1
                                && known_board[x][y] != board.0.cells[x][y] as i32
                            {
                                is_valid = false;
                                break;
                            }
                            if board.0.cells[x][y] > 0 {
                                board_info.push((x, y));
                            }
                        }
                    }

                    if !is_valid {
                        continue;
                    }

                    let res = self.io.answer(board_info);
                    eprintln!("res = {}", res);
                    if res {
                        return;
                    }
                }
            } else {
                // color debug if board_candidates[0]
                for x in 0..self.input.n {
                    for y in 0..self.input.n {
                        if board_candidates[0].0.cells[x][y] > 0 {
                            self.io.debug_colorize(x, y, "#ff8888");
                        }
                    }
                }
            }

            boards = board_candidates.into_iter().map(|b| b.0).collect();
        }
    }
}
