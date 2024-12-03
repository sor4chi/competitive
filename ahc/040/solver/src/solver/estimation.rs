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
const SIZE_UPPER_BOUND: usize = 50000;

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

impl Solver for EstimationSolver<'_> {
    fn solve(&mut self) {
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
        for _ in 0..self.input.T - 1 {
            // split in 2
            let mut width_measure_group = vec![];
            let mut height_measure_group = vec![];
            let mut perm = (0..self.input.N).collect::<Vec<_>>();
            perm.shuffle(&mut rng);
            perm.truncate(self.input.N);
            perm.sort();
            for i in 1..self.input.N {
                if rng.gen_bool(0.5) {
                    width_measure_group.push(perm[i]);
                } else {
                    height_measure_group.push(perm[i]);
                }
            }
            let mut operations = vec![Operation {
                p: perm[0],
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
            width_measure_group.insert(0, 0);
            height_measure_group.insert(0, 0);
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

        // debug print A_width
        for i in 0..A_width.nrows() {
            for j in 0..A_width.ncols() {
                eprint!("{}", if A_width[(i, j)] == 1.0 { "1" } else { "0" });
            }
            eprintln!();
        }

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

        // let estimated_width = na::linalg::Cholesky::new(AtA_width)
        //     .unwrap()
        //     .solve(&AtY_width);
        // let estimated_height = na::linalg::Cholesky::new(AtA_height)
        //     .unwrap()
        //     .solve(&AtY_height);

        let estimated_width = na::linalg::Cholesky::new(AtA_width)
            .unwrap()
            .solve(&AtY_width);

        let estimated_height = na::linalg::Cholesky::new(AtA_height)
            .unwrap()
            .solve(&AtY_height);

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
