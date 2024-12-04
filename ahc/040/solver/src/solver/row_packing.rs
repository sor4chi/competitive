use std::{collections::HashSet, mem::swap, time::Instant};

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use rand_pcg::Pcg64Mcg;

use crate::{
    io::{Direction, Input, Operation, Query, Rotation, IO},
    state::State,
};

use nalgebra as na;

use super::Solver;

pub struct RowPackingSolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl RowPackingSolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> RowPackingSolver<'a> {
        RowPackingSolver { input, io }
    }
}

const SIZE_LOWER_BOUND: usize = 10000;
const SIZE_UPPER_BOUND: usize = 100000;

fn search(row_width: usize, rects: &[(usize, usize)], inv: bool) -> (usize, Vec<usize>) {
    let mut width = 0;
    let mut max_width = 0;
    let mut height = 0;
    let mut max_height_in_row = 0;
    let mut row_counts = vec![];
    let mut row_count = 0;
    for (w, h) in rects {
        let (mut w, mut h) = if w < h { (*w, *h) } else { (*h, *w) };
        if inv {
            swap(&mut w, &mut h);
        }
        if width + w > row_width {
            max_width = max_width.max(width);
            width = 0;
            height += max_height_in_row;
            max_height_in_row = 0;
            row_counts.push(row_count);
            row_count = 0;
        }
        width += w;
        max_height_in_row = max_height_in_row.max(h);
        row_count += 1;
    }
    row_counts.push(row_count);
    (max_width + height + max_height_in_row, row_counts)
}

const BASE_TRIAL: usize = 30;

impl Solver for RowPackingSolver<'_> {
    fn solve(&mut self) {
        let start = Instant::now();
        let mut measurement_width_indicies = vec![];
        let mut measurement_width_values = vec![];
        let mut measurement_height_indicies = vec![];
        let mut measurement_height_values = vec![];
        for i in 0..self.input.N {
            measurement_width_indicies.push(vec![i]);
            measurement_width_values.push(self.input.rects[i].0);
            measurement_height_indicies.push(vec![i]);
            measurement_height_values.push(self.input.rects[i].1);
        }
        let mut rng = Pcg64Mcg::new(42);
        let trial = BASE_TRIAL.min(self.input.T);
        // 最初の3つを見て最も大きい(つまりmax(width, height)が最も大きい)rectを探す
        let mut max_rect = 0;
        let mut max_rect_index = 0;
        for i in 0..3 {
            let (w, h) = self.input.rects[i];
            if w.max(h) > max_rect {
                max_rect = w.max(h);
                max_rect_index = i;
            }
        }
        for _ in 0..self.input.T - trial {
            // split in 2
            let mut width_measure_group = vec![];
            let mut height_measure_group = vec![];
            for i in max_rect_index + 1..self.input.N {
                if rng.gen_bool(0.5) {
                    width_measure_group.push(i);
                } else {
                    height_measure_group.push(i);
                }
            }
            let mut operations = vec![Operation {
                p: max_rect_index,
                r: Rotation::Stay,
                d: Direction::Up,
                b: -1,
            }];
            for i in 0..width_measure_group.len() {
                operations.push(Operation {
                    p: width_measure_group[i],
                    r: Rotation::Stay,
                    d: Direction::Left,
                    b: -1,
                });
            }
            for i in 0..height_measure_group.len() {
                operations.push(Operation {
                    p: height_measure_group[i],
                    r: Rotation::Stay,
                    d: Direction::Up,
                    b: -1,
                });
            }
            operations.sort_by_key(|op| op.p);
            let query = Query { operations };
            let (width, height) = self.io.measure(&query);
            width_measure_group.insert(0, max_rect_index);
            height_measure_group.insert(0, max_rect_index);
            measurement_width_indicies.push(width_measure_group.clone());
            measurement_width_values.push(width);
            measurement_height_indicies.push(height_measure_group.clone());
            measurement_height_values.push(height);
        }

        // 観測行列Aと観測ベクトルyを作成
        assert!(measurement_width_indicies.len() == measurement_width_values.len());
        let measurement_count = measurement_width_indicies.len();
        let mut A_width = na::DMatrix::<f64>::zeros(measurement_count, self.input.N);
        let mut A_height = na::DMatrix::<f64>::zeros(measurement_count, self.input.N);

        for (i, group) in measurement_width_indicies.iter().enumerate() {
            for &j in group {
                A_width[(i, j)] = 1.0;
            }
        }
        for (i, group) in measurement_height_indicies.iter().enumerate() {
            for &j in group {
                A_height[(i, j)] = 1.0;
            }
        }
        let y_width = na::DVector::<f64>::from_vec(
            measurement_width_values
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        );
        let y_height = na::DVector::<f64>::from_vec(
            measurement_height_values
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        );

        // 最小二乗法で推定
        // 事前分布の平均と分散を設定
        // 事前分布となるサイズはLOWER_BOUNDからUPPER_BOUNDの間の一様分布
        let prior_mean = (SIZE_LOWER_BOUND + SIZE_UPPER_BOUND) as f64 / 2.0;
        let prior_var = ((SIZE_UPPER_BOUND - SIZE_LOWER_BOUND) as f64).powi(2) / 12.0;

        // 事前分布から正則化項の係数を計算
        let lambda_reg = (self.input.sigma as f64).powi(2) / prior_var;

        let AtA_width = A_width.transpose() * &A_width
            + lambda_reg * na::DMatrix::<f64>::identity(self.input.N, self.input.N);
        let AtY_width = A_width.transpose() * &y_width
            + lambda_reg * prior_mean * na::DVector::<f64>::repeat(self.input.N, 1.0);
        let AtA_height = A_height.transpose() * &A_height
            + lambda_reg * na::DMatrix::<f64>::identity(self.input.N, self.input.N);
        let AtY_height = A_height.transpose() * &y_height
            + lambda_reg * prior_mean * na::DVector::<f64>::repeat(self.input.N, 1.0);

        let estimated_width = na::linalg::Cholesky::new(AtA_width)
            .unwrap()
            .solve(&AtY_width);

        let estimated_height = na::linalg::Cholesky::new(AtA_height)
            .unwrap()
            .solve(&AtY_height);

        let estimated_input = Input {
            N: self.input.N,
            T: self.input.T,
            sigma: self.input.sigma,
            rects: {
                let mut r = vec![];
                for i in 0..self.input.N {
                    r.push((estimated_width[i] as usize, estimated_height[i] as usize));
                }
                r
            },
        };

        // searchが最も小さくなるような場所を探す
        let row_widths = {
            let mut visited = HashSet::new();
            let mut score_widths = vec![];
            for inv in &[false, true] {
                for width in (0..=1000000).step_by(1000) {
                    let (score, row_counts) = search(width, &estimated_input.rects, *inv);
                    if !visited.insert((row_counts, *inv)) {
                        continue;
                    }
                    score_widths.push((score, width, *inv));
                }
            }
            score_widths.sort_by_key(|x| x.0);
            score_widths
        };
        let each_tl = 2900 / trial as u128;
        for t in 0..trial {
            if t >= row_widths.len() {
                eprintln!("t={} is out of range", t);
                self.io.measure(&Query { operations: vec![] });
                continue;
            }
            let mut best_operations = vec![];
            let mut cur_width = 0;
            for i in 0..estimated_input.N {
                let mut rotate = if estimated_input.rects[i].0 < estimated_input.rects[i].1 {
                    Rotation::Stay
                } else {
                    Rotation::Rotate
                };
                if row_widths[t].2 {
                    rotate.flip();
                }
                let w = if rotate == Rotation::Stay {
                    estimated_input.rects[i].0
                } else {
                    estimated_input.rects[i].1
                };
                best_operations.push(Operation {
                    p: i,
                    r: rotate,
                    d: Direction::Up,
                    b: if cur_width + w <= row_widths[t].1 {
                        (i - 1) as isize
                    } else {
                        -1
                    },
                });
                cur_width += w;
                if cur_width > row_widths[t].1 {
                    cur_width = 0;
                }
            }
            // 焼きなまし
            let start_temp = 1000.0;
            let end_temp = 1.0;
            let mut best_operations = best_operations.clone();
            let mut best_score = {
                let mut state = State::new(&estimated_input);
                let _ = state.query(&estimated_input, &best_operations);
                state.score_t as usize
            };
            let mut best_deleted = vec![];
            let mut cur_operations = best_operations.clone();
            let mut cur_score = best_score;
            let mut cur_deleted = best_deleted.clone();
            let mut temp = start_temp;
            let start_annealing = Instant::now();
            while start_annealing.elapsed().as_millis() < each_tl {
                let mut operations = cur_operations.clone();
                let mut deleted = cur_deleted.clone();
                let prob = rng.gen_range(0.0..1.0);
                if prob < 0.9 {
                    let selected = rng.gen_range(0..operations.len() - 1);
                    operations[selected].r = match operations[selected].r {
                        Rotation::Stay => Rotation::Rotate,
                        Rotation::Rotate => Rotation::Stay,
                    };
                } else {
                    let selected = rng.gen_range(0..operations.len() - 1);
                    if operations[selected + 1].b != selected as isize {
                        operations.remove(selected);
                        deleted.push(selected);
                    }
                }
                let score = {
                    let mut state = State::new(&estimated_input);
                    let _ = state.query(&estimated_input, &operations);
                    state.score_t as usize
                };
                if score < cur_score {
                    cur_score = score;
                    cur_operations.clone_from(&operations);
                    cur_deleted.clone_from(&deleted);
                    if cur_score < best_score {
                        best_score = cur_score;
                        best_operations.clone_from(&cur_operations);
                        best_deleted.clone_from(&cur_deleted);
                    }
                } else {
                    let diff = (cur_score as isize - score as isize) as f64;
                    if rng.gen_bool((diff / temp).exp()) {
                        cur_score = score;
                        cur_operations.clone_from(&operations);
                        cur_deleted.clone_from(&deleted);
                    }
                }
                let elapsed = start_annealing.elapsed().as_millis() as f64;
                temp = start_temp + (end_temp - start_temp) * elapsed / each_tl as f64;
            }

            self.io.measure(&Query {
                operations: best_operations,
            });
        }
    }
}
