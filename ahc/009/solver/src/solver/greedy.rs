use std::collections::VecDeque;

use crate::{
    board::{Board, DIRECTIONS},
    io::{Input, Operations, Output, BOARD_SIZE, IO},
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

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        let start = (self.input.si, self.input.sj);
        let goal = (self.input.ti, self.input.tj);
        let board = Board::new(BOARD_SIZE, self.input.h.clone(), self.input.v.clone());
        let mut visited = vec![vec![false; BOARD_SIZE]; BOARD_SIZE];
        visited[start.0][start.1] = true;
        let mut queue = VecDeque::new();
        queue.push_back((start, vec![]));

        let mut operations = vec![];
        while let Some((current, path)) = queue.pop_front() {
            if current == goal {
                operations.clone_from(&path);
            }

            for (i, j, dir) in board.nexts(current.0, current.1) {
                if visited[i][j] {
                    continue;
                }

                visited[i][j] = true;
                let mut path = path.clone();
                path.push(dir);
                queue.push_back(((i, j), path));
            }
        }

        Output { operations }
    }
}
