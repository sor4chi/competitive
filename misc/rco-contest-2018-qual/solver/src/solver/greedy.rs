use crate::io::{Dir, Input, Output, DIRS, IO};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Trap,
    Coin,
    Wall,
    None,
}

#[derive(Debug, Clone)]
struct Board {
    current: (usize, usize),
    map: Vec<Vec<Cell>>,
}

impl Board {
    pub fn from(cmap: Vec<Vec<char>>) -> Self {
        let mut map = Vec::new();
        let mut current = (0, 0);
        for (i, row) in cmap.iter().enumerate() {
            let mut r = Vec::new();
            for (j, c) in row.iter().enumerate() {
                match c {
                    'x' => r.push(Cell::Trap),
                    'o' => r.push(Cell::Coin),
                    '#' => r.push(Cell::Wall),
                    '@' => {
                        r.push(Cell::None);
                        current = (i, j);
                    }
                    _ => panic!("invalid character"),
                }
            }
            map.push(r);
        }

        Board { current, map }
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self) -> Output {
        // なるべくx(罠)が少ない方がいいので、少ない順にk個選ぶ
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

        let mut use_maps = Vec::new();
        let mut use_maps_ids = Vec::new();
        for (map_id, _) in traps.iter().take(self.input.k) {
            let board = Board::from(self.input.maps[*map_id].clone());
            use_maps.push(board);
            use_maps_ids.push(*map_id);
        }

        let mut is_trapped = vec![false; self.input.k];
        let mut commands = Vec::new();

        // DPで毎ターン一番多くコインを取れる方向に進む
        for t in 0..self.input.t {
            eprintln!("t = {}", t);
            let mut max = 0;
            let mut max_dir = Dir::Up;
            for dir in DIRS.iter() {
                let (dx, dy) = dir.delta();
                let mut cnt = 0;
                for board in use_maps.iter() {
                    let (x, y) = board.current;
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || nx >= self.input.h as i32 || ny < 0 || ny >= self.input.w as i32 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;
                    if board.map[nx][ny] == Cell::Wall {
                        continue;
                    }

                    if board.map[nx][ny] == Cell::Trap {
                        cnt += 1;
                    }

                    if board.map[nx][ny] == Cell::Coin {
                        cnt += 1;
                    }
                }

                if cnt > max {
                    max = cnt;
                    max_dir = *dir;
                }
            }

            commands.push(max_dir);

            for (i, board) in use_maps.iter_mut().enumerate() {
                if is_trapped[i] {
                    continue;
                }

                let (dx, dy) = max_dir.delta();
                let (x, y) = board.current;
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= self.input.h as i32 || ny < 0 || ny >= self.input.w as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                if board.map[nx][ny] == Cell::Wall {
                    continue;
                }

                if board.map[nx][ny] == Cell::Trap {
                    is_trapped[i] = true;
                }

                board.current = (nx, ny);
                if board.map[nx][ny] == Cell::Coin {
                    board.map[nx][ny] = Cell::None;
                }
            }
        }

        eprintln!("commands = {:?}", commands);


        Output {
            m: use_maps_ids,
            commands,
        }
    }
}
