use proconio::input;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    L,
    R,
    U,
    D,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty, // 0
    L,     // 1
    U,     // 2
    LU,    // 3
    R,     // 4
    LR,    // 5
    RU,    // 6
    LRU,   // 7
    D,     // 8
    LD,    // 9
    UD,    // a
    LUD,   // b
    RD,    // c
    LRD,   // d
    RUD,   // e
    LRUD,  // f
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '0',
            Tile::L => '1',
            Tile::U => '2',
            Tile::LU => '3',
            Tile::R => '4',
            Tile::LR => '5',
            Tile::RU => '6',
            Tile::LRU => '7',
            Tile::D => '8',
            Tile::LD => '9',
            Tile::UD => 'a',
            Tile::LUD => 'b',
            Tile::RD => 'c',
            Tile::LRD => 'd',
            Tile::RUD => 'e',
            Tile::LRUD => 'f',
        }
    }

    fn get_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Empty => vec![],
            Tile::L => vec![Direction::L],
            Tile::U => vec![Direction::U],
            Tile::LU => vec![Direction::L, Direction::U],
            Tile::R => vec![Direction::R],
            Tile::LR => vec![Direction::L, Direction::R],
            Tile::RU => vec![Direction::U, Direction::R],
            Tile::LRU => vec![Direction::L, Direction::U, Direction::R],
            Tile::D => vec![Direction::D],
            Tile::LD => vec![Direction::D, Direction::L],
            Tile::UD => vec![Direction::D, Direction::U],
            Tile::LUD => vec![Direction::D, Direction::L, Direction::U],
            Tile::RD => vec![Direction::D, Direction::R],
            Tile::LRD => vec![Direction::D, Direction::L, Direction::R],
            Tile::RUD => vec![Direction::D, Direction::U, Direction::R],
            Tile::LRUD => vec![Direction::D, Direction::L, Direction::U, Direction::R],
        }
    }
}

impl Tile {
    // dir 方向に other が接続可能かどうか
    fn is_connectable(&self, dir: Direction, other: &Tile) -> bool {
        let self_directions = self.get_directions();
        if !self_directions.contains(&dir) {
            return false;
        }
        let other_directions = other.get_directions();
        match dir {
            Direction::L => other_directions.contains(&Direction::R),
            Direction::R => other_directions.contains(&Direction::L),
            Direction::U => other_directions.contains(&Direction::D),
            Direction::D => other_directions.contains(&Direction::U),
        }
    }
}

struct Input {
    n: usize,
    max_op: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    L,
    R,
    U,
    D,
}

impl Operation {
    fn to_char(&self) -> char {
        match self {
            Operation::L => 'L',
            Operation::R => 'R',
            Operation::U => 'U',
            Operation::D => 'D',
        }
    }
}

fn main() {
    input! {
        N: usize, // 盤面の大きさ
        T: usize, // 最大の操作回数
        t: [String; N], // 盤面
    }

    let mut tiles = vec![vec![Tile::Empty; N]; N];
    for i in 0..N {
        for j in 0..N {
            let row = &t[i];
            tiles[i][j] = match row.chars().nth(j).unwrap() {
                '0' => Tile::Empty,
                '1' => Tile::L,
                '2' => Tile::U,
                '3' => Tile::LU,
                '4' => Tile::R,
                '5' => Tile::LR,
                '6' => Tile::RU,
                '7' => Tile::LRU,
                '8' => Tile::D,
                '9' => Tile::LD,
                'a' => Tile::UD,
                'b' => Tile::LUD,
                'c' => Tile::RD,
                'd' => Tile::LRD,
                'e' => Tile::RUD,
                'f' => Tile::LRUD,
                _ => unreachable!(),
            };
        }
    }

    let input = Input {
        n: N,
        max_op: T,
        tiles,
    };
    let ans = solve(input);

    for op in ans {
        print!("{}", op.to_char());
    }
    println!();
}

fn evaluate(t: &Vec<Vec<Tile>>) -> i64 {
    let n = t.len();
    let mut score = 0;
    // 各マスに対してBFSをして、木の長さの総和を求める
    let mut visited = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if visited[i][j] {
                continue;
            }
            let mut q = std::collections::VecDeque::new();
            q.push_back((i, j, 0));
            visited[i][j] = true;
            let mut visit_cnt = 0;
            while let Some((x, y, d)) = q.pop_front() {
                visit_cnt += 1;
                for dir in [Direction::L, Direction::R, Direction::U, Direction::D] {
                    let (x, y) = (x as isize, y as isize);
                    let (nx, ny) = match dir {
                        Direction::L => (x, y - 1),
                        Direction::R => (x, y + 1),
                        Direction::U => (x - 1, y),
                        Direction::D => (x + 1, y),
                    };
                    if nx < 0 || ny < 0 || nx >= n as isize || ny >= n as isize {
                        continue;
                    }
                    let (x, y, nx, ny) = (x as usize, y as usize, nx as usize, ny as usize);
                    if t[x][y].is_connectable(dir, &t[nx][ny]) && !visited[nx][ny] {
                        visited[nx][ny] = true;
                        q.push_back((nx, ny, d + 1));
                    }
                }
            }
            score += visit_cnt * visit_cnt;
        }
    }
    // score
    score as i64
}

fn solve(input: Input) -> Vec<Operation> {
    let mut initial_empty_tile_pos = (0, 0);
    for i in 0..input.n {
        for j in 0..input.n {
            if input.tiles[i][j] == Tile::Empty {
                initial_empty_tile_pos = (i, j);
            }
        }
    }

    let mut best_score = evaluate(&input.tiles);
    let mut ans = vec![];

    // ビームサーチ
    let beam_width = 50;
    let mut beam = vec![(
        vec![],
        initial_empty_tile_pos,
        input.tiles.clone(),
        best_score,
    )];

    let start = Instant::now();
    for _ in 0..input.max_op {
        if start.elapsed().as_millis() > 2900 {
            break;
        }
        let mut next_beam = vec![];
        for (ops, empty_tile_pos, tiles, _) in beam {
            for &dir in [Direction::L, Direction::R, Direction::U, Direction::D].iter() {
                let (x, y) = empty_tile_pos;
                let (x, y) = (x as isize, y as isize);
                let (nx, ny) = match dir {
                    Direction::L => (x, y - 1),
                    Direction::R => (x, y + 1),
                    Direction::U => (x - 1, y),
                    Direction::D => (x + 1, y),
                };
                if nx < 0 || ny < 0 || nx >= input.n as isize || ny >= input.n as isize {
                    continue;
                }
                let (x, y, nx, ny) = (x as usize, y as usize, nx as usize, ny as usize);
                let mut next_tiles = tiles.clone();
                next_tiles[x][y] = tiles[nx][ny];
                next_tiles[nx][ny] = tiles[x][y];
                let score = evaluate(&next_tiles);
                let mut ops = ops.clone();
                ops.push(match dir {
                    Direction::L => Operation::L,
                    Direction::R => Operation::R,
                    Direction::U => Operation::U,
                    Direction::D => Operation::D,
                });
                next_beam.push((ops, (nx, ny), next_tiles, score));
            }
        }
        next_beam.sort_by_key(|(_, _, _, score)| -(*score as i64));
        next_beam.truncate(beam_width);
        beam = next_beam;
        if beam[0].3 > best_score {
            best_score = beam[0].3;
            ans = beam[0].0.clone();
        }
    }

    ans
}
