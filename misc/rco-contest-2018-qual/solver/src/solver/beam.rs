use std::{collections::HashMap, usize};

use fixedbitset::FixedBitSet;

use crate::io::{Dir, Input, Output, DIRS, IO};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Trap,
    Coin,
    Wall,
    None,
}

#[derive(Debug, Clone)]
struct Board {
    map: Vec<Vec<Cell>>,
    start: (usize, usize),
}

impl Board {
    pub fn from(cmap: Vec<Vec<char>>) -> Self {
        let mut map = Vec::new();
        let mut start = (0, 0);
        for (i, row) in cmap.iter().enumerate() {
            let mut r = Vec::new();
            for (j, c) in row.iter().enumerate() {
                match c {
                    'x' => r.push(Cell::Trap),
                    'o' => r.push(Cell::Coin),
                    '#' => r.push(Cell::Wall),
                    '@' => {
                        r.push(Cell::None);
                        start = (i, j);
                    }
                    _ => panic!("invalid character"),
                }
            }
            map.push(r);
        }

        Board { map, start }
    }
}

struct IDGenerator {
    id: usize,
}

impl IDGenerator {
    pub fn new() -> Self {
        IDGenerator { id: 0 }
    }

    pub fn next(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}

#[derive(Debug, Clone)]
struct BoardState {
    id: u8,
    current: (usize, usize),
    visited: FixedBitSet,
    coin: usize,
}

impl BoardState {
    pub fn from(id: u8, board: &Board) -> Self {
        let mut visited = FixedBitSet::with_capacity(board.map.len() * board.map[0].len());
        visited.insert(board.start.0 * board.map[0].len() + board.start.1);
        BoardState {
            id,
            current: board.start,
            visited,
            coin: 0,
        }
    }
}

struct BeamNode {
    boards: Vec<BoardState>,
    commands: Vec<Dir>,
    score: i32,
}

impl Solver for BeamSolver {
    fn solve(&mut self) -> Output {
        // trapが少ない順にk個選ぶ
        let mut traps = Vec::new();
        for i in 0..self.input.n {
            let mut cnt = 0;
            for j in 0..self.input.h {
                for k in 0..self.input.w {
                    if self.input.maps[i][j][k] == 'x' {
                        cnt += 1;
                    }
                }
            }
            traps.push((i, cnt));
        }

        traps.sort_by_key(|x| x.1);
        let top_board_ids = traps
            .iter()
            .take(self.input.k)
            .map(|x| x.0)
            .collect::<Vec<_>>();

        let mut top_boards = HashMap::new();
        for &i in top_board_ids.iter() {
            let board = Board::from(self.input.maps[i].clone());
            top_boards.insert(i, board);
        }

        let first_beam = BeamNode {
            boards: top_boards
                .iter()
                .map(|(&i, b)| BoardState::from(i as u8, b))
                .collect(),
            commands: Vec::new(),
            score: 0,
        };
        let mut idg = IDGenerator::new();
        let first_beam_id = idg.next();
        let mut beam_cache = HashMap::new();
        beam_cache.insert(first_beam_id, first_beam);
        let mut beams = vec![first_beam_id];

        const BEAM_WIDTH: usize = 400;

        let timer = std::time::Instant::now();
        for ti in 0..self.input.t {
            if timer.elapsed().as_millis() > 3950 {
                break;
            }

            eprintln!("turn = {} / {}", ti, self.input.t);
            let mut next_beams = Vec::new();
            for beam in beams.iter().map(|id| beam_cache.get(id).unwrap()) {
                for dir in DIRS.iter() {
                    let mut next_boards = Vec::new();
                    let mut next_commands = beam.commands.clone();
                    next_commands.push(*dir);

                    for board in beam.boards.iter() {
                        let (dx, dy) = dir.delta();
                        let (nx, ny) = (
                            (board.current.0 as i32 + dx) as usize,
                            (board.current.1 as i32 + dy) as usize,
                        );

                        if nx >= self.input.h || ny >= self.input.w {
                            next_boards.push(board.clone());
                            continue;
                        }

                        let mut next_board = board.clone();

                        let idx = nx * self.input.w + ny;

                        if next_board.visited.contains(idx) {
                            next_board.current = (nx, ny);
                            next_boards.push(next_board);
                            continue;
                        }

                        match top_boards.get(&(next_board.id as usize)).unwrap().map[nx][ny] {
                            Cell::Trap => {
                                continue;
                            }
                            Cell::Coin => {
                                next_board.coin += 1;
                                next_board.visited.insert(idx);
                                next_board.current = (nx, ny);
                            }
                            Cell::Wall => {}
                            Cell::None => {
                                next_board.visited.insert(idx);
                                next_board.current = (nx, ny);
                            }
                        }

                        next_boards.push(next_board);
                    }

                    let next_score = next_boards.iter().map(|b| b.coin).sum::<usize>() as i32;

                    next_beams.push(BeamNode {
                        boards: next_boards,
                        commands: next_commands,
                        score: next_score,
                    });
                }
            }

            // スコアは大きい順にソート
            next_beams.sort_by_key(|b| -b.score);
            beams.clear();
            beam_cache.clear();
            for (i, b) in next_beams.into_iter().take(BEAM_WIDTH).enumerate() {
                let id = idg.next();
                beams.push(id);
                beam_cache.insert(id, b);
            }
        }

        let best_beam_id = beams
            .into_iter()
            .max_by_key(|id| beam_cache.get(id).unwrap().score)
            .unwrap();
        let best_beam = beam_cache.get(&best_beam_id).unwrap();
        // top beamのscoreを表示
        eprintln!("top beam score = {}", best_beam.score);

        Output {
            m: top_board_ids,
            commands: best_beam.commands.clone(),
        }
    }
}
