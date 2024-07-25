use crate::io::{Input, IO};
use rand::{prelude::SliceRandom, Rng};
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    f64::consts::PI,
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

pub struct BayesianSolver {
    input: Input,
    io: IO,
}

// 正規分布の累積分布関数
fn normal_cdf(x: f64, mean: f64, sigma: f64) -> f64 {
    0.5 * (1.0 + libm::erf((x - mean) / (sigma * 2.0f64.sqrt())))
}

fn probability_in_range(mean: f64, sigma: f64, l: f64, r: f64) -> f64 {
    if mean < l {
        return probability_in_range(mean, sigma, 2.0 * mean - r, 2.0 * mean - l);
    }

    let p_l = normal_cdf(l, mean, sigma);
    let p_r = normal_cdf(r, mean, sigma);
    p_r - p_l
}

impl BayesianSolver {
    pub fn new(io: IO, input: Input) -> Self {
        BayesianSolver { input, io }
    }

    fn likelihood(&self, board: &Board, divination: &Vec<(usize, usize)>, res: usize) -> f64 {
        // divination配列で占ったところ、resが出た。
        // boardの値の総和をv(divination)、k=divination.len()とすると、
        // 占った時の値は平均が(k-v(divination)) * eps + v(divination)(1-eps)、分散がk * eps * (1-eps)の正規分布からサンプルされた値xに対してmax(0, round(x))がresとなる
        // この結果からboardである尤度を計算する
        let mut sum = 0;
        for (x, y) in divination {
            sum += board.cells[*x][*y];
        }
        let k = divination.len();
        let eps = self.input.eps;
        let mu = (k - sum) as f64 * eps + sum as f64 * (1.0 - eps);
        let sigma = (k as f64 * eps * (1.0 - eps)).sqrt();
        // 正規分布の累積分布関数を求める
        // 0の場合は-infから0.5の確率を求める
        if res == 0 {
            return probability_in_range(mu, sigma, -1e9, 0.5);
        }
        probability_in_range(mu, sigma, res as f64 - 0.5, res as f64 + 0.5)
    }
}

impl Solver for BayesianSolver {
    fn solve(&mut self) {
        assert!(self.input.m == 2 || (self.input.m == 3 && self.input.n <= 10));

        // ありうる配置を全て作る
        let mut q = VecDeque::new();
        let board = Board::new(&self.input);
        q.push_back(board);
        for mino in &self.input.minos {
            let mut next_q = VecDeque::new();
            while let Some(b) = q.pop_front() {
                for x in 0..self.input.n - mino.height + 1 {
                    for y in 0..self.input.n - mino.width + 1 {
                        let mut new_board = b.clone();
                        for i in 0..mino.height {
                            for j in 0..mino.width {
                                if mino.shape[i][j] {
                                    new_board.cells[x + i][y + j] += 1;
                                }
                            }
                        }
                        next_q.push_back(new_board);
                    }
                }
            }
            q = next_q;
        }

        let mut board_cands = Vec::new();
        while let Some(b) = q.pop_front() {
            board_cands.push(b);
        }

        // ベイズ推定を行う
        let mut probs = vec![1.0 / board_cands.len() as f64; board_cands.len()];
        let mut rng = rand::thread_rng();

        // 相互情報量を計算しておく
        let mut mis = Vec::new();

        for x in 0..self.input.n {
            for y in 0..self.input.n {
                let mut cnts = HashMap::new();
                for b in &board_cands {
                    *cnts.entry(b.cells[x][y]).or_insert(0) += 1;
                }
                let mut mi = 0.0;
                for (_, cnt) in cnts {
                    let p = cnt as f64 / board_cands.len() as f64;
                    mi -= p * p.log2();
                }
                mis.push((mi, (x, y)));
            }
        }

        mis.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        loop {
            // let mut perm = (0..self.input.n * self.input.n).collect::<Vec<_>>();
            // perm.shuffle(&mut rng);
            // let mut divination = Vec::new();
            // for i in 0..self.input.n * self.input.n / 4 {
            //     let x = perm[i] / self.input.n;
            //     let y = perm[i] % self.input.n;
            //     divination.push((x, y));
            // }

            // MISの上位n^2/8個を占いとして使う
            let mut divination = Vec::new();
            // for i in 0..self.input.n * self.input.n / 8 {
            for i in self.input.n * self.input.n / 4..self.input.n * self.input.n {
                divination.push(mis[i].1);
            }
            // // 他self.input.n * self.input.n / 8個をランダムに選ぶ
            // for _ in 0..self.input.n * self.input.n / 8 {
            //     let mut x = rng.gen_range(0..self.input.n);
            //     let mut y = rng.gen_range(0..self.input.n);
            //     while divination.contains(&(x, y)) {
            //         x = rng.gen_range(0..self.input.n);
            //         y = rng.gen_range(0..self.input.n);
            //     }
            //     divination.push((x, y));
            // }
            // for mi in mis.iter() {
            //     // 相互情報量が1.0以上のものを占いとして使う
            //     if mi.0 >= 1.0 {
            //         divination.push(mi.1);
            // }
            // }

            let res = self.io.query_divination(divination.clone());
            for i in 0..board_cands.len() {
                probs[i] *= self.likelihood(&board_cands[i], &divination, res);
            }

            let sum = probs.iter().sum::<f64>();
            for i in 0..board_cands.len() {
                probs[i] /= sum;
            }

            let mut max_prob = 0.0;
            let mut max_prob_idx = 0;

            for i in 0..board_cands.len() {
                if probs[i] > max_prob {
                    max_prob = probs[i];
                    max_prob_idx = i;
                }
            }

            eprintln!("max_prob: {}", max_prob);
            if max_prob >= 0.8 {
                let mut board_info = Vec::new();
                for x in 0..self.input.n {
                    for y in 0..self.input.n {
                        if board_cands[max_prob_idx].cells[x][y] != 0 {
                            board_info.push((x, y));
                        }
                    }
                }

                let res = self.io.answer(board_info);
                if res {
                    break;
                } else {
                    probs[max_prob_idx] = 0.0;
                }
            }

            // misを更新する、board_candsの中で確率が高い100個を選ぶ
            let mut new_mis = Vec::new();
            let mut high_probs = probs.iter().enumerate().collect::<Vec<_>>();
            high_probs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            for x in 0..self.input.n {
                for y in 0..self.input.n {
                    let mut cnts = HashMap::new();
                    for (idx, _) in &high_probs {
                        *cnts.entry(board_cands[*idx].cells[x][y]).or_insert(0.0) +=
                            1.0 * probs[*idx].powf(20.0);
                    }
                    let mut mi = 0.0;
                    for (_, cnt) in cnts {
                        let p = cnt as f64 / high_probs.len() as f64;
                        mi -= p * p.log2();
                    }
                    new_mis.push((mi, (x, y)));
                }
            }
            new_mis.sort_by(|a, b| {
                if a.0 < b.0 {
                    std::cmp::Ordering::Greater
                } else if a.0 > b.0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            mis = new_mis;
        }
    }
}
