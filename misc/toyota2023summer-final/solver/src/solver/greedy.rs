use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    io::{stdin, BufReader},
};

use itertools::Itertools;
use proconio::{input, source::line::LineSource};

use crate::{
    original_lib::{compute_score, compute_score_details, Input},
    IDGenerator, Solver,
};

pub struct GreedySolver<'a> {
    d: usize,
    n: usize,
    r: Vec<(usize, usize)>,
    board: Vec<Vec<u8>>,
    source: LineSource<BufReader<std::io::StdinLock<'a>>>,
}

const OBJECT: u8 = 255;
const BLANK: u8 = 254;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl<'a> GreedySolver<'a> {
    pub fn new() -> Self {
        let stdin = stdin();
        let mut source: LineSource<BufReader<std::io::StdinLock<'_>>> =
            LineSource::new(BufReader::new(stdin.lock()));

        input! {
            from &mut source,
            d: usize,
            n: usize,
            r: [(usize, usize); n],
        }

        let mut board = vec![vec![BLANK; d]; d];
        for (x, y) in r.iter() {
            board[*x][*y] = OBJECT;
        }

        Self {
            d,
            n,
            r,
            source,
            board,
        }
    }

    fn get_dist_map(&self, start: (usize, usize), board: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
        let mut dist = vec![vec![usize::MAX; self.d]; self.d];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        dist[start.0][start.1] = 0;

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIRECTIONS.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= self.d as i32 || ny < 0 || ny >= self.d as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if board[nx][ny] == BLANK && dist[nx][ny] == usize::MAX {
                    dist[nx][ny] = dist[x][y] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }

        dist
    }

    fn get_ideal_board(&self) -> Vec<Vec<u8>> {
        let mut ideal_board = vec![vec![BLANK; self.d]; self.d];
        for (x, y) in self.r.iter() {
            ideal_board[*x][*y] = OBJECT;
        }
        let mut counter = 0;
        let start = (0, self.d / 2);
        let dist_map = self.get_dist_map(start, &self.board);
        let mut dist_vec = vec![];
        for x in 0..self.d {
            for y in 0..self.d {
                if self.board[x][y] == BLANK {
                    dist_vec.push((dist_map[x][y], (x, y)));
                }
            }
        }
        dist_vec.sort();
        for (_, (x, y)) in dist_vec {
            if (x, y) == start {
                continue;
            }
            ideal_board[x][y] = counter;
            counter += 1;
        }
        ideal_board
    }

    fn get_reachable_items(
        &self,
        start: (usize, usize),
        board: &Vec<Vec<u8>>,
    ) -> HashMap<(usize, usize), u8> {
        let mut reachable_items = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut visited = vec![vec![false; self.d]; self.d];
        visited[start.0][start.1] = true;
        while let Some((x, y)) = queue.pop_front() {
            if board[x][y] != OBJECT && board[x][y] != BLANK {
                reachable_items.insert((x, y), board[x][y]);
                continue;
            }
            for (dx, dy) in DIRECTIONS.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= self.d as i32 || ny < 0 || ny >= self.d as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if visited[nx][ny] {
                    continue;
                }
                if board[nx][ny] != OBJECT {
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }
        }
        reachable_items
    }

    fn debug_board(&self, board: &Vec<Vec<u8>>) {
        for x in 0..self.d {
            for y in 0..self.d {
                if board[x][y] == OBJECT {
                    eprint!("## ");
                } else if board[x][y] == BLANK {
                    eprint!("   ");
                } else {
                    eprint!("{:2} ", board[x][y]);
                }
            }
            eprintln!();
        }
    }

    fn debug_dist_map(&self, dist_map: &Vec<Vec<usize>>) {
        for x in 0..self.d {
            for y in 0..self.d {
                if dist_map[x][y] == usize::MAX {
                    eprint!("## ");
                } else {
                    eprint!("{:2} ", dist_map[x][y]);
                }
            }
            eprintln!();
        }
    }

    // ボードに穴がないか
    fn is_board_has_hole(&self, board: &Vec<Vec<u8>>) -> bool {
        let start = (0, self.d / 2);
        // boardがBLANKなのにdist_mapがusize::MAXなら穴がある
        let dist_map = self.get_dist_map(start, board);
        for x in 0..self.d {
            for y in 0..self.d {
                if board[x][y] == BLANK && dist_map[x][y] == usize::MAX {
                    return true;
                }
            }
        }
        false
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as i32 - b.0 as i32).abs() as usize + (a.1 as i32 - b.1 as i32).abs() as usize
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct BeamNode {
    id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NextBeamNode {
    score: i64,
    board: Vec<Vec<u8>>,
    operations: Vec<u8>,
}

impl PartialOrd for NextBeamNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // スコアは大きいほど良い
        // Some(other.score.cmp(&self.score))
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for NextBeamNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other.score.cmp(&self.score)
        self.score.cmp(&other.score)
    }
}

fn calc_inversion(operations: &Vec<u8>) -> i64 {
    let mut inversion = 0;
    for i in 0..operations.len() {
        for j in i + 1..operations.len() {
            if operations[i] > operations[j] {
                inversion += 1;
            }
        }
    }
    inversion
}

impl<'a> Solver for GreedySolver<'a> {
    fn solve(&mut self) {
        let input_pos: (usize, usize) = (0, self.d / 2);

        let container_num = self.d.pow(2) - 1 - self.n;
        let mut left_items = (0..container_num as u8).collect::<Vec<_>>();
        let mut place_ops = vec![];
        // self.debug_board(&ideal_board);
        for _ in 0..container_num {
            input! {
                from &mut self.source,
                d: u8,
            }
            let ideal_board = self.get_ideal_board();
            self.debug_board(&ideal_board);
            let ideal_pos = {
                let mut pos = (0, 0);
                // left_itemsのdのindex
                let idx = left_items.iter().position(|&x| x == d).unwrap();
                for x in 0..self.d {
                    for y in 0..self.d {
                        if ideal_board[x][y] == idx as u8 {
                            pos = (x, y);
                        }
                    }
                }
                pos
            };
            // 行ける場所の中で一番マンハッタン距離が小さいもの
            let mut min_dist = usize::MAX;
            let mut min_pos = (0, 0);
            let dist_map = self.get_dist_map(input_pos, &self.board);
            // self.debug_dist_map(&dist_map);
            for x in 0..self.d {
                for y in 0..self.d {
                    if (x, y) == input_pos {
                        continue;
                    }
                    if dist_map[x][y] != usize::MAX {
                        let mut board_clone = self.board.clone();
                        board_clone[x][y] = d;
                        if self.is_board_has_hole(&board_clone) {
                            continue;
                        }
                        // let dist = manhattan_distance((x, y), ideal_pos);
                        let tmp_dist_map = self.get_dist_map(ideal_pos, &self.board);
                        let dist = tmp_dist_map[x][y];
                        if dist < min_dist {
                            min_dist = dist;
                            min_pos = (x, y);
                        }
                    }
                }
            }
            self.board[min_pos.0][min_pos.1] = d;
            println!("{} {}", min_pos.0, min_pos.1);
            place_ops.push(min_pos);
            left_items.retain(|&item| item != d);
        }

        self.debug_board(&self.board);

        let mut greedy_ans = vec![];
        for _ in 0..container_num {
            let reachable_items = self.get_reachable_items(input_pos, &self.board);
            // reachable_itemsの中で一番小さいもの
            let mut min_item = u8::MAX;
            let mut min_pos = (0, 0);
            for (pos, item) in reachable_items.iter() {
                if *item < min_item {
                    min_item = *item;
                    min_pos = *pos;
                }
            }
            self.board[min_pos.0][min_pos.1] = BLANK;
            greedy_ans.push(min_pos);
        }
        for (x, y) in greedy_ans.clone() {
            println!("{} {}", x, y);
        }

        // let mut greedy_all_ops = vec![];
        // for (x, y) in place_ops.iter() {
        //     greedy_all_ops.push((*x, *y));
        // }
        // for (x, y) in greedy_ans.iter() {
        //     greedy_all_ops.push((*x, *y));
        // }

        // eprintln!(
        //     "greedy_ans score = {}",
        //     compute_score(&input_original, &greedy_all_ops).0
        // );
        // eprintln!(
        //     "place_ops score = {}",
        //     compute_score(&input_original, &greedy_all_ops).0
        // );

        // let mut idg = IDGenerator::new();
        // let mut beams = vec![BeamNode { id: idg.generate() }];
        // let mut beam_cache = HashMap::new();
        // beam_cache.insert(
        //     beams[0].id,
        //     NextBeamNode {
        //         score: 0,
        //         board: self.board.clone(),
        //         operations: vec![],
        //     },
        // );
        // let beam_width = 10000;
        // for i in 0..container_num {
        //     eprintln!("i = {}", i);
        //     let mut next_beams = BinaryHeap::new();
        //     for beam in beams {
        //         let board = beam_cache[&beam.id].board.clone();
        //         let reachable_items = self.get_reachable_items(input_pos, &board);
        //         for (pos, item) in reachable_items.iter() {
        //             let mut next_board = board.clone();
        //             next_board[pos.0][pos.1] = BLANK;
        //             let mut operations = beam_cache[&beam.id].operations.clone();
        //             operations.push(*item);
        //             // operationsの転倒数をスコアとする
        //             let next_beam = NextBeamNode {
        //                 score: calc_inversion(&operations),
        //                 board: next_board,
        //                 operations,
        //             };
        //             next_beams.push(next_beam);
        //             if next_beams.len() >= beam_width {
        //                 next_beams.pop();
        //             }
        //         }
        //     }
        //     // eprintln!("next_beams.len() = {}", next_beams.len());
        //     // // トップを表示
        //     // eprintln!("next_beams.top() = {:?}", next_beams.peek().unwrap());
        //     // self.debug_board(&next_beams.peek().unwrap().board);
        //     beams = next_beams
        //         .into_iter()
        //         // .unique_by(|beam| beam.score)
        //         .map(|next_beam| {
        //             let id = idg.generate();
        //             beam_cache.insert(id, next_beam);
        //             BeamNode { id }
        //         })
        //         .collect();
        // }

        // let mut max_score = 0;
        // let mut max_beam = None;
        // for beam in beams {
        //     let score = beam_cache[&beam.id].score;
        //     if score > max_score {
        //         max_score = score;
        //         max_beam = Some(beam);
        //     }
        // }

        // let max_beam = max_beam.unwrap();
        // let max_beam = beam_cache[&max_beam.id].clone();
        // self.debug_board(&max_beam.board);
        // // 各数字に対応するboardの座標を取得
        // let mut pos_map = HashMap::new();
        // for x in 0..self.d {
        //     for y in 0..self.d {
        //         if self.board[x][y] != OBJECT && self.board[x][y] != BLANK {
        //             pos_map.insert(self.board[x][y], (x, y));
        //         }
        //     }
        // }

        // for item in max_beam.operations {
        //     let pos = pos_map[&item];
        //     println!("{} {}", pos.0, pos.1);
        // }
    }
}
