use std::{env, io::BufRead};

use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;

use crate::{
    io::{Direction, Input, Operation, Query, Rotation, IO},
    state::State,
};
use nalgebra as na;

use std::fs::File;

use super::Solver;

pub struct EstimationSolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl EstimationSolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> EstimationSolver<'a> {
        EstimationSolver { input, io }
    }
}

const SIZE_LOWER_BOUND: usize = 10000;
const SIZE_UPPER_BOUND: usize = 100000;

fn load_source_value(n: usize, seed: usize) -> Vec<(i64, i64)> {
    let path = format!("../tools/in/{:04}.txt", seed);
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();
    for _ in 0..n + 1 {
        lines.next().unwrap().unwrap();
    }

    let mut source_rects = vec![];
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse::<i64>().unwrap();
        let y = iter.next().unwrap().parse::<i64>().unwrap();
        source_rects.push((x, y));
    }
    source_rects
}

const ROTATION_SLOTS: [Rotation; 2] = [Rotation::Stay, Rotation::Rotate];

impl Solver for EstimationSolver<'_> {
    fn solve(&mut self) {
        let mut measurement_indecies = vec![]; // even: width, odd: height
        let mut measurement_values = vec![];
        for i in 0..self.input.N {
            measurement_indecies.push(vec![i * 2]);
            measurement_values.push(self.input.rects[i].0);
            measurement_indecies.push(vec![i * 2 + 1]);
            measurement_values.push(self.input.rects[i].1);
        }

        let mut rng = Pcg64Mcg::new(42);
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
        for _ in 0..self.input.T - 1 {
            let mut width_measure_group = vec![];
            let mut height_measure_group = vec![];
            for i in max_rect_index + 1..self.input.N {
                if rng.gen_bool(0.5) {
                    width_measure_group.push((i, ROTATION_SLOTS[rng.gen_range(0..2)]));
                } else {
                    height_measure_group.push((i, ROTATION_SLOTS[rng.gen_range(0..2)]));
                }
            }
            let center_operation = Operation {
                p: max_rect_index,
                r: ROTATION_SLOTS[rng.gen_range(0..2)],
                d: Direction::Up,
                b: -1,
            };
            let mut operations = vec![center_operation];
            for i in 0..width_measure_group.len() {
                operations.push(Operation {
                    p: width_measure_group[i].0,
                    r: width_measure_group[i].1,
                    d: Direction::Left,
                    b: -1,
                });
            }
            for i in 0..height_measure_group.len() {
                operations.push(Operation {
                    p: height_measure_group[i].0,
                    r: height_measure_group[i].1,
                    d: Direction::Up,
                    b: -1,
                });
            }
            operations.sort_by_key(|op| op.p);
            let query = Query { operations };
            let (width, height) = self.io.measure(&query);
            width_measure_group.insert(0, (max_rect_index, center_operation.r));
            height_measure_group.insert(0, (max_rect_index, center_operation.r));
            let mut measurement_width_indicies = vec![];
            for (i, (p, r)) in width_measure_group.into_iter().enumerate() {
                measurement_width_indicies.push(p * 2 + (r == Rotation::Rotate) as usize);
            }
            let mut measurement_height_indicies = vec![];
            for (i, (p, r)) in height_measure_group.into_iter().enumerate() {
                measurement_height_indicies.push(p * 2 + (r == Rotation::Stay) as usize);
            }
            measurement_indecies.push(measurement_width_indicies);
            measurement_values.push(width);
            measurement_indecies.push(measurement_height_indicies);
            measurement_values.push(height);
        }

        // widthとheightを同時に推定
        assert!(measurement_indecies.len() == measurement_values.len());
        let measurement_count = measurement_indecies.len();
        let mut A = na::DMatrix::<f64>::zeros(measurement_count, self.input.N * 2);

        for (i, group) in measurement_indecies.iter().enumerate() {
            for &j in group {
                A[(i, j)] = 1.0;
            }
        }

        let y = na::DVector::<f64>::from_vec(
            measurement_values.into_iter().map(|x| x as f64).collect(),
        );

        // 事前分布の平均と分散を設定
        // 事前分布となるサイズはLOWER_BOUNDからUPPER_BOUNDの間の一様分布
        let avg = self.input.rects.iter().fold(0, |acc, x| acc + x.0 + x.1) / (self.input.N * 2);
        let estimated_lower = (2 * avg - SIZE_UPPER_BOUND).max(SIZE_LOWER_BOUND);
        let prior_mean = (estimated_lower + SIZE_UPPER_BOUND) as f64 / 2.0;
        let prior_var = ((SIZE_UPPER_BOUND - estimated_lower) as f64).powi(2) / 12.0;

        // 事前分布から正則化項の係数を計算
        let lambda_reg = (self.input.sigma as f64).powi(2) / prior_var;

        let AtA = A.transpose() * &A
            + lambda_reg * na::DMatrix::<f64>::identity(self.input.N * 2, self.input.N * 2);
        let AtY = A.transpose() * &y
            + lambda_reg * prior_mean * na::DVector::<f64>::repeat(self.input.N * 2, 1.0);

        let estimated = na::linalg::Cholesky::new(AtA).unwrap().solve(&AtY);

        let mut estimated_width = vec![];
        let mut estimated_height = vec![];

        for i in 0..self.input.N {
            estimated_width.push(estimated[i * 2]);
            estimated_height.push(estimated[i * 2 + 1]);
        }

        let args: Vec<String> = env::args().collect();
        let seed = if args.len() > 1 && args[1].parse::<usize>().is_ok() {
            args[1].parse::<usize>().unwrap()
        } else {
            0
        };
        eprintln!("seed: {}", seed);

        let source_rects = load_source_value(self.input.N, seed);

        // estimated_width,estimated_heightとsource_rectsの2乗誤差を計算
        let mut error_width = 0.0;
        let mut error_height = 0.0;
        for i in 0..self.input.N {
            error_width += (estimated_width[i] - source_rects[i].0 as f64).powi(2);
            error_height += (estimated_height[i] - source_rects[i].1 as f64).powi(2);
        }

        // self.input.rectsとsource_rectsの2乗誤差を計算
        let mut error_width_initial = 0.0;
        let mut error_height_initial = 0.0;
        for i in 0..self.input.N {
            error_width_initial +=
                (self.input.rects[i].0 as f64 - source_rects[i].0 as f64).powi(2);
            error_height_initial +=
                (self.input.rects[i].1 as f64 - source_rects[i].1 as f64).powi(2);
        }

        eprintln!("error_width_initial : {}", error_width_initial);
        eprintln!("error_width         : {}", error_width);
        eprintln!("error_height_initial: {}", error_height_initial);
        eprintln!("error_height        : {}", error_height);

        // 誤差えぐいランキングTop10
        let mut error_ranking = vec![];
        for i in 0..self.input.N {
            error_ranking.push((
                i,
                (estimated_width[i] - source_rects[i].0 as f64).powi(2)
                    + (estimated_height[i] - source_rects[i].1 as f64).powi(2),
            ));
        }
        error_ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for i in 0..10 {
            eprintln!(
                "{}: w_diff:{} h_diff:{}",
                i,
                (estimated_width[error_ranking[i].0] - source_rects[error_ranking[i].0].0 as f64)
                    .abs(),
                (estimated_height[error_ranking[i].0] - source_rects[error_ranking[i].0].1 as f64)
                    .abs()
            );
        }

        #[derive(Clone)]
        struct BeamState {
            score: i32,
            operations: Vec<Operation>,
        }
        let mut beams = vec![BeamState {
            score: i32::MAX,
            operations: vec![],
        }];

        let beam_width = 100;

        for t in 0..self.input.N {
            let mut next_beams = vec![];
            for beam in beams {
                for r in &[Rotation::Stay, Rotation::Rotate] {
                    for d in &[Direction::Up, Direction::Left] {
                        for b in -1..t as isize {
                            let next_op = Operation {
                                p: t,
                                r: r.clone(),
                                d: d.clone(),
                                b,
                            };
                            let mut next_beam = beam.clone();
                            next_beam.operations.push(next_op);
                            // let input = Input {
                            //     N: self.input.N,
                            //     T: self.input.T,
                            //     sigma: self.input.sigma,
                            //     rects: source_rects
                            //         .iter()
                            //         .map(|(x, y)| (*x as usize, *y as usize))
                            //         .collect(),
                            // };
                            let input = self.input;
                            let mut state = State::new(&input);
                            if let Err(err) = state.query(&input, &next_beam.operations) {
                                panic!("{}", err);
                            }
                            next_beams.push(BeamState {
                                score: state.score,
                                operations: next_beam.operations,
                            });
                        }
                    }
                }
            }
            next_beams.sort_by_key(|beam| beam.score);
            next_beams.truncate(beam_width);
            beams = next_beams;
            eprintln!("t: {} best score: {}", t, beams[0].score);
        }

        let best_beam = beams[0].clone();
        eprintln!("best score: {}", best_beam.score);
        self.io.measure(&Query {
            operations: best_beam.operations.clone(),
        });
    }
}
