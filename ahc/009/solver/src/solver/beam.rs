use std::collections::{BinaryHeap, VecDeque};

use rand::Rng;

use crate::{
    board::{Board, DIRECTIONS},
    io::{Input, Operations, Output, BOARD_SIZE, IO, MAX_OPERATIONS},
    solver::beam,
};

use super::Solver;

pub struct BeamSolver {
    io: IO,
    input: Input,
}

impl BeamSolver {
    pub fn new(io: IO, input: Input) -> Self {
        BeamSolver { io, input }
    }
}

struct BeamState {
    board: [[f64; BOARD_SIZE]; BOARD_SIZE],
    operations: Vec<Operations>,
}
impl Eq for BeamState {}
impl PartialEq for BeamState {
    fn eq(&self, other: &Self) -> bool {
        self.operations == other.operations
    }
}
#[derive(Eq, PartialEq)]
struct NextBeam {
    beam: BeamState,
    score: usize,
}
impl Ord for NextBeam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for NextBeam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solver for BeamSolver {
    fn solve(&mut self) -> Output {
        let start = (self.input.si, self.input.sj);
        let goal = (self.input.ti, self.input.tj);
        let board = Board::new(BOARD_SIZE, self.input.h.clone(), self.input.v.clone());
        let mut dist_map = vec![vec![usize::MAX; BOARD_SIZE]; BOARD_SIZE];
        let mut queue = VecDeque::new();
        queue.push_back((goal, 0));
        dist_map[goal.0][goal.1] = 0;
        let mut max_dist = 0;
        while let Some((current, dist)) = queue.pop_front() {
            for (i, j, _) in board.nexts(current.0, current.1) {
                let new_dist = dist + 1;
                if new_dist < dist_map[i][j] {
                    dist_map[i][j] = new_dist;
                    max_dist = max_dist.max(new_dist);
                    queue.push_back(((i, j), new_dist));
                }
            }
        }

        let mut rng = rand::thread_rng();
        let mut eval = |state: &BeamState| {
            let mut score = 0.0;
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    score += state.board[i][j] * ((max_dist - dist_map[i][j]) as f64).powf(2.0);
                }
            }
            (score * 1000.0) as usize
        };

        let mut first_state = BeamState {
            board: [[0.0; BOARD_SIZE]; BOARD_SIZE],
            operations: vec![],
        };
        first_state.board[start.0][start.1] = 1.0;
        let mut beam = vec![first_state];
        let beam_width = 900;

        for t in 0..MAX_OPERATIONS {
            eprintln!("turn: {}, beam: {}", t, beam.len());
            let mut next_beam = BinaryHeap::new();
            for state in beam {
                for &(di, dj, dir) in DIRECTIONS {
                    let mut next_board = [[0.0; BOARD_SIZE]; BOARD_SIZE];
                    for i in 0..BOARD_SIZE {
                        for j in 0..BOARD_SIZE {
                            if (i, j) == (goal.0, goal.1) {
                                next_board[goal.0][goal.1] += state.board[i][j];
                                continue;
                            }
                            if state.board[i][j] > 0.0 {
                                let next_i = i as i32 + di;
                                let next_j = j as i32 + dj;
                                if board.can_move(i, j, dir) {
                                    let current_p = state.board[i][j];
                                    next_board[i][j] += current_p * self.input.p; // その場にとどまる確率
                                    next_board[next_i as usize][next_j as usize] +=
                                        current_p * (1.0 - self.input.p); // 移動する確率
                                } else {
                                    next_board[i][j] += state.board[i][j];
                                }
                            }
                        }
                    }

                    let mut next_operations = state.operations.clone();
                    next_operations.push(dir);
                    let next_state = BeamState {
                        board: next_board,
                        operations: next_operations,
                    };
                    next_beam.push(NextBeam {
                        score: eval(&next_state),
                        beam: next_state,
                    });
                    if next_beam.len() > beam_width {
                        next_beam.pop();
                    }
                }
            }

            beam = next_beam.into_iter().map(|nb| nb.beam).collect();
        }

        let operations = beam[0].operations.clone();

        Output { operations }
    }
}
