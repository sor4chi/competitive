use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, BufReader},
};

use proconio::{input, source::line::LineSource};

use crate::Solver;

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

    fn get_dist_map(&self, start: (usize, usize)) -> Vec<Vec<usize>> {
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
                if self.board[nx][ny] == BLANK && dist[nx][ny] == usize::MAX {
                    dist[nx][ny] = dist[x][y] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }

        dist
    }

    fn get_reachable_items(&self, start: (usize, usize)) -> HashMap<(usize, usize), u8> {
        let mut reachable_items = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut visited = vec![vec![false; self.d]; self.d];
        visited[start.0][start.1] = true;
        while let Some((x, y)) = queue.pop_front() {
            if self.board[x][y] != OBJECT && self.board[x][y] != BLANK {
                reachable_items.insert((x, y), self.board[x][y]);
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
                if self.board[nx][ny] != OBJECT {
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }
        }
        reachable_items
    }

    fn debug_board(&self) {
        for x in 0..self.d {
            for y in 0..self.d {
                if self.board[x][y] == OBJECT {
                    eprint!("## ");
                } else if self.board[x][y] == BLANK {
                    eprint!("   ");
                } else {
                    eprint!("{:2} ", self.board[x][y]);
                }
            }
            eprintln!();
        }
    }
}

impl<'a> Solver for GreedySolver<'a> {
    fn solve(&mut self) {
        let input_pos: (usize, usize) = (0, self.d / 2);

        let container_num = self.d.pow(2) - 2 - self.n + 1;
        for _ in 0..container_num {
            input! {
                from &mut self.source,
                d: u8,
            }
            let dist_map = self.get_dist_map(input_pos);
            // board==BLANKで一番遠い場所
            let mut max_dist = 0;
            let mut max_pos = (0, 0);
            for x in 0..self.d {
                for y in 0..self.d {
                    if self.board[x][y] == BLANK && dist_map[x][y] > max_dist {
                        max_dist = dist_map[x][y];
                        max_pos = (x, y);
                    }
                }
            }
            self.board[max_pos.0][max_pos.1] = d;
            println!("{} {}", max_pos.0, max_pos.1);
        }

        self.debug_board();

        for _ in 0..container_num {
            let reachable_items = self.get_reachable_items(input_pos);
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
            println!("{} {}", min_pos.0, min_pos.1);
        }
    }
}
