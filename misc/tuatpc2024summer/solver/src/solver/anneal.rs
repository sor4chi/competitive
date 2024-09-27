use std::{
    collections::{HashSet, VecDeque},
    f32::consts::E,
    time::Instant,
};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{
    board::Board,
    io::{Input, Operation, Output, IO},
    util::visualize_score_transition,
};

use super::Solver;

pub struct AnnealSolver {
    io: IO,
    input: Input,
}

impl AnnealSolver {
    pub fn new(io: IO, input: Input) -> Self {
        AnnealSolver { io, input }
    }
}

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

enum AnnealNeighbor {
    Swap,
    BigSwap,
    Shuffle,
}

fn choose_neighbor(rng: &mut impl Rng) -> AnnealNeighbor {
    // let p = rng.gen::<f64>();
    // if p < 0.9 {
    //     AnnealNeighbor::Swap
    // } else if p < 0.99 {
    //     AnnealNeighbor::BigSwap
    // } else {
    //     AnnealNeighbor::Shuffle
    // }

    AnnealNeighbor::Swap
}

impl Solver for AnnealSolver {
    fn solve(&mut self) -> Output {
        let mut operations = vec![];
        let mut board = Board::new(self.input.h, self.input.w);
        let mut score = 0;
        let mut t = 0;
        let mut rng = rand::thread_rng();
        let total_tl = 3000;
        while t < self.input.n {
            eprintln!("=== t = {} ===", t);
            let empty_size = board.empty_size();
            let fill_size = empty_size.min(self.input.n - t);
            eprintln!("fill_size = {}", fill_size);
            if fill_size == 0 {
                break;
            }
            let mut swappable_pos = vec![];
            for r in 0..self.input.h {
                for c in 0..self.input.w {
                    if board.get(r, c).is_none() {
                        swappable_pos.push((r, c));
                    }
                }
            }
            // 最もスコアが高くなる盤面を焼きなましで探す。self.input.aのt~t+fill_size個を次に置く対象の宝石とする
            let mut skip_idxes = HashSet::new();
            let mut place_jewels = if self.input.h > 4 || self.input.w > 4 {
                let mut place_jewels = vec![];
                for i in 0..fill_size {
                    place_jewels.push(self.input.a[t + i]);
                }
                place_jewels
            } else {
                // 詰みやすいのはself.input.h<=4 && self.input.w<=4のとき
                let mut distribution = [0; 4];
                let mut left = fill_size;
                let mut required = 0;
                let mut idx = 0;
                let mut place_jewels = vec![];
                while left > 0 && t + idx < self.input.n {
                    let jewel = self.input.a[t + idx];
                    let jewel_required = 3 - distribution[jewel - 1] % 3;
                    // もしjewel_requiredが3ならば、left-requiredが3以上ないといけない
                    if jewel_required == 3 {
                        if left - required >= 3 {
                            place_jewels.push(jewel);
                            distribution[jewel - 1] += 1;
                            distribution[jewel - 1] %= 3;
                            left -= 1;
                            required += 2;
                        } else {
                            skip_idxes.insert(t + idx);
                        }
                    } else {
                        place_jewels.push(jewel);
                        distribution[jewel - 1] += 1;
                        distribution[jewel - 1] %= 3;
                        left -= 1;
                        required -= 1;
                    }
                    idx += 1;
                }
                place_jewels
            };

            let mut best_board = board.clone();
            // ランダムにplace_jewelsを置いていく
            place_jewels.shuffle(&mut rng);
            for r in 0..self.input.h {
                for c in 0..self.input.w {
                    if !best_board.is_placable(r, c) {
                        continue;
                    }
                    if let Some(jewel) = place_jewels.pop() {
                        best_board.place(r, c, jewel);
                    } else {
                        break;
                    }
                }
            }
            let mut best_score = {
                let mut cur_board = best_board.clone();
                cur_board.organize()
            };
            eprintln!("first_score = {}", best_score);
            let start_anneal = Instant::now();
            let tl = (total_tl as f64 * (fill_size as f64 / self.input.n as f64) * 0.9) as u128;
            let start_temp = 1e4;
            let end_temp = 1e-3;
            let mut temp = start_temp;
            let mut cur_board = best_board.clone();
            let mut cur_score = best_score;

            let mut iter = 0;
            let mut scores = vec![];
            let mut last_best_updates = 0;
            while start_anneal.elapsed().as_millis() < tl {
                // 一定回数bestが更新されなかったらbestからもう一度スタートする
                if iter - last_best_updates > 1000 {
                    cur_board = best_board.clone();
                    cur_score = best_score;
                    last_best_updates = iter;
                }
                let neighbor = choose_neighbor(&mut rng);
                let mut next_board = cur_board.clone();

                match neighbor {
                    AnnealNeighbor::Swap => {
                        let (r, c) = *swappable_pos.iter().choose(&mut rng).unwrap();
                        let (r2, c2) = *swappable_pos.iter().choose(&mut rng).unwrap();
                        if r == r2 && c == c2 {
                            continue;
                        }
                        if next_board.get(r, c) == next_board.get(r2, c2) {
                            continue;
                        }
                        next_board.swap(r, c, r2, c2);
                    }
                    AnnealNeighbor::BigSwap => {
                        // shuffleして、最初の3<=k<=10.min(fill_size)個をswapする
                        let k = rng.gen_range(3..=10).min(fill_size);
                        swappable_pos.shuffle(&mut rng);
                        for i in 0..k {
                            let (r, c) = swappable_pos[i];
                            let (r2, c2) = swappable_pos[(i + 1) % k];
                            if r == r2 && c == c2 {
                                continue;
                            }
                            if next_board.get(r, c) == next_board.get(r2, c2) {
                                continue;
                            }
                            next_board.swap(r, c, r2, c2);
                        }
                    }
                    AnnealNeighbor::Shuffle => {
                        // すべての宝石をshuffleする
                        swappable_pos.shuffle(&mut rng);
                        for i in 0..swappable_pos.len() {
                            let (r, c) = swappable_pos[i];
                            let (r2, c2) = swappable_pos[(i + 1) % swappable_pos.len()];
                            if r == r2 && c == c2 {
                                continue;
                            }
                            if next_board.get(r, c) == next_board.get(r2, c2) {
                                continue;
                            }
                            next_board.swap(r, c, r2, c2);
                        }
                    }
                }

                let next_score = {
                    let mut next_board = next_board.clone();
                    next_board.organize()
                };

                let diff = next_score as i64 - cur_score as i64;
                if diff > 0 || rng.gen::<f64>() < (diff as f64 / temp).exp() {
                    cur_board = next_board;
                    cur_score = next_score;
                    if cur_score > best_score {
                        last_best_updates = iter;
                        best_board = cur_board.clone();
                        best_score = cur_score;
                    }
                }

                scores.push(cur_score);
                temp = start_temp
                    + (end_temp - start_temp) * start_anneal.elapsed().as_millis() as f64
                        / tl as f64;
                iter += 1;
            }

            // visualize_score_transition(&scores, format!("figure/anneal_{}.png", t).as_str());

            eprintln!("iter = {}", iter);
            eprintln!("best_score = {}", best_score);

            let mut fill_cnt = 0;
            let mut ti = t;
            'place_jewels: while fill_cnt < fill_size && ti < self.input.n {
                let jewel = self.input.a[ti];
                if skip_idxes.contains(&ti) {
                    operations.push(Operation {
                        place: None,
                        organize: false,
                    });
                    ti += 1;
                    continue;
                }
                for r in 0..self.input.h {
                    for c in 0..self.input.w {
                        if board.is_placable(r, c)
                            && best_board.get(r, c).is_some_and(|x| x == jewel)
                        {
                            board.place(r, c, jewel);
                            operations.push(Operation {
                                place: Some((r + 1, c + 1)),
                                organize: false,
                            });
                            fill_cnt += 1;
                            ti += 1;
                            continue 'place_jewels;
                        }
                    }
                }
            }

            // best_boardとboardが同じことを確認
            for r in 0..self.input.h {
                for c in 0..self.input.w {
                    assert_eq!(best_board.get(r, c), board.get(r, c));
                }
            }

            operations.last_mut().unwrap().organize = true;

            score += board.organize();
            t = ti;
        }

        if t < self.input.n {
            eprintln!("WARNING: {} jewels are not placed", self.input.n - t);
        }
        // 残りの宝石をNone, falseで埋める
        while t < self.input.n {
            operations.push(Operation {
                place: None,
                organize: false,
            });
            t += 1;
        }

        eprintln!("anneal = {}", score);

        Output { operations, score }
    }
}
