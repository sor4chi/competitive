use std::io::{stdin, BufReader};
use proconio::{input, source::line::LineSource};
use solver::{
    game::Game,
    solver::{greedy::GreedySolver, random::RandomSolver, Solver},
};
pub mod solver {
pub mod game {
use std::fmt::Display;
#[derive(Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => "F",
                Dir::Down => "B",
                Dir::Left => "L",
                Dir::Right => "R",
            }
        )
    }
}
#[derive(Clone)]
pub struct Game {
    pub a: Vec<usize>,
    pub n: usize,
    pub turn: usize,
    pub board: Vec<Vec<usize>>,
    pub op: Vec<Dir>,
}
impl Game {
    pub fn new(a: Vec<usize>, n: usize) -> Game {
        assert_eq!(a.len(), n * n);
        Self {
            a,
            n,
            turn: 0,
            board: vec![vec![0; n]; n],
            op: vec![],
        }
    }
    pub fn place(&mut self, pos: usize) {
        let mut cnt = 1;
        for y in 0..self.n {
            for x in 0..self.n {
                if self.board[y][x] == 0 {
                    if cnt == pos {
                        self.board[y][x] = self.a[self.turn];
                        return;
                    }
                    cnt += 1;
                }
            }
        }
        unreachable!();
    }
    pub fn slide(&mut self, dir: Dir) {
        let mut new_board = vec![vec![0; self.n]; self.n];
        match dir {
            Dir::Up => {
                for x in 0..self.n {
                    let mut y = 0;
                    for i in 0..self.n {
                        if self.board[i][x] != 0 {
                            new_board[y][x] = self.board[i][x];
                            y += 1;
                        }
                    }
                }
            }
            Dir::Down => {
                for x in 0..self.n {
                    let mut y = self.n - 1;
                    for i in (0..self.n).rev() {
                        if self.board[i][x] != 0 {
                            new_board[y][x] = self.board[i][x];
                            if y > 0 {
                                y -= 1;
                            }
                        }
                    }
                }
            }
            Dir::Left => {
                for y in 0..self.n {
                    let mut x = 0;
                    for i in 0..self.n {
                        if self.board[y][i] != 0 {
                            new_board[y][x] = self.board[y][i];
                            x += 1;
                        }
                    }
                }
            }
            Dir::Right => {
                for y in 0..self.n {
                    let mut x: usize = self.n - 1;
                    for i in (0..self.n).rev() {
                        if self.board[y][i] != 0 {
                            new_board[y][x] = self.board[y][i];
                            if x > 0 {
                                x -= 1;
                            }
                        }
                    }
                }
            }
        }
        self.board = new_board;
        self.op.push(dir);
        self.turn += 1;
    }
    pub fn op_str(&self) -> String {
        self.op
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn board_str(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game() {
        let a = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        let mut game = Game::new(a, 3);
        game.place(0);
        assert_eq!(
            game.board,
            vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        game.slide(Dir::Down);
        assert_eq!(game.turn, 1);
        assert_eq!(
            game.board,
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![1, 0, 0]]
        );
        game.place(1);
        assert_eq!(
            game.board,
            vec![vec![0, 2, 0], vec![0, 0, 0], vec![1, 0, 0]]
        );
        game.slide(Dir::Up);
        assert_eq!(game.turn, 2);
        assert_eq!(
            game.board,
            vec![vec![1, 2, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        game.place(4);
        assert_eq!(
            game.board,
            vec![vec![1, 2, 0], vec![0, 3, 0], vec![0, 0, 0]]
        );
        game.slide(Dir::Right);
        assert_eq!(game.turn, 3);
        assert_eq!(
            game.board,
            vec![vec![0, 1, 2], vec![0, 0, 3], vec![0, 0, 0]]
        );
    }
}
}
pub mod solver {
pub mod greedy {
use std::time::Instant;
use rand::{rngs::StdRng, Rng, SeedableRng};
use super::super::game::{Dir, Game};
use super::Solver;
pub struct GreedySolver {}
impl GreedySolver {
    pub fn new() -> Self {
        Self {}
    }
}
impl Solver for GreedySolver {
    fn get_move(&self, _game: &Game) -> Dir {
        let mut left_scores = vec![];
        let mut right_scores = vec![];
        let mut up_scores = vec![];
        let mut down_scores = vec![];
        let left = _game.n * _game.n - _game.turn - 1;
        if left >= 4 {
            let start = Instant::now();
            let mut rng = StdRng::from_entropy();
            while start.elapsed().as_millis() < 19 {
                let mut game = _game.clone();
                let i = rng.gen_range(1..=left);
                let j = rng.gen_range(1..=left - 1);
                let k = rng.gen_range(1..=left - 2);
                let l = rng.gen_range(1..=left - 3);
                game.place(i);
                for dir_1 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    let mut game = game.clone();
                    game.slide(dir_1.clone());
                    game.place(j);
                    for dir_2 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                        let mut game = game.clone();
                        game.slide(dir_2.clone());
                        game.place(k);
                        for dir_3 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                            let mut game = game.clone();
                            game.slide(dir_3.clone());
                            game.place(l);
                            for dir_4 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                                let mut game = game.clone();
                                game.slide(dir_4.clone());
                                let score = self.raw_eval(&game);
                                match dir_1 {
                                    Dir::Up => up_scores.push(score),
                                    Dir::Down => down_scores.push(score),
                                    Dir::Left => left_scores.push(score),
                                    Dir::Right => right_scores.push(score),
                                }
                            }
                        }
                    }
                }
            }
        } else {
            for i in 1..=left {
                let mut game = _game.clone();
                game.place(i);
                for dir in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    let mut game = game.clone();
                    game.slide(dir.clone());
                    let score = self.raw_eval(&game);
                    match dir {
                        Dir::Up => up_scores.push(score),
                        Dir::Down => down_scores.push(score),
                        Dir::Left => left_scores.push(score),
                        Dir::Right => right_scores.push(score),
                    }
                }
            }
        }
        let up_avg = up_scores.iter().sum::<usize>() as f64 / up_scores.len() as f64;
        let down_avg = down_scores.iter().sum::<usize>() as f64 / down_scores.len() as f64;
        let left_avg = left_scores.iter().sum::<usize>() as f64 / left_scores.len() as f64;
        let right_avg = right_scores.iter().sum::<usize>() as f64 / right_scores.len() as f64;
        if up_avg.is_nan() || down_avg.is_nan() || left_avg.is_nan() || right_avg.is_nan() {
            let mut rng = StdRng::from_entropy();
            match rng.gen_range(0..4) {
                0 => return Dir::Up,
                1 => return Dir::Down,
                2 => return Dir::Left,
                3 => return Dir::Right,
                _ => unreachable!(),
            }
        }
        let max = vec![
            (Dir::Up, up_avg),
            (Dir::Down, down_avg),
            (Dir::Left, left_avg),
            (Dir::Right, right_avg),
        ]
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
        max.0
    }
}
}
pub mod random {
use rand::{rngs::StdRng, Rng};
use super::super::game::{Dir, Game};
use super::Solver;
pub struct RandomSolver {
    rng: StdRng,
}
impl RandomSolver {
    pub fn new(seed: u8) -> Self {
        Self {
            rng: rand::SeedableRng::from_seed([seed; 32]),
        }
    }
}
impl Solver for RandomSolver {
    fn get_move(&self, _game: &Game) -> Dir {
        let mut rng = self.rng.clone();
        match rng.gen_range(0..4) {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!(),
        }
    }
}
}
use std::collections::{HashMap, HashSet};
use super::game::{Dir, Game};
pub trait Solver {
    fn get_move(&self, game: &Game) -> Dir;
    fn raw_eval(&self, game: &Game) -> usize {
        let mut score = 0.0;
        let mut connections = vec![];
        let mut visited = HashSet::new();
        for y in 0..game.n {
            for x in 0..game.n {
                if visited.contains(&(y, x)) {
                    continue;
                }
                if game.board[y][x] == 0 {
                    continue;
                }
                let mut size = 0;
                let mut stack = vec![(y, x)];
                while let Some((y, x)) = stack.pop() {
                    if visited.contains(&(y, x)) {
                        continue;
                    }
                    visited.insert((y, x));
                    size += 1;
                    for &(dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny < 0 || ny >= game.n as i32 || nx < 0 || nx >= game.n as i32 {
                            continue;
                        }
                        let ny = ny as usize;
                        let nx = nx as usize;
                        if game.board[y][x] == game.board[ny][nx] {
                            stack.push((ny, nx));
                        }
                    }
                }
                connections.push(size);
            }
        }
        for connection in connections {
            score += (connection * connection) as f64;
        }
        let mut cnts = HashMap::new();
        for &x in &game.a {
            *cnts.entry(x).or_insert(0) += 1;
        }
        let mut squared_sum = 0.0;
        for &x in cnts.values() {
            squared_sum += (x * x) as f64;
        }
        score /= squared_sum;
        (1e6 * score).round() as usize
    }
}
}
}
const N: usize = 10;
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        a: [usize; N * N],
    }
    let mut game = Game::new(a, N);
    let solver = GreedySolver::new();
    while game.turn < N * N {
        input! {
            from &mut source,
            pos: usize,
        }
        game.place(pos);
        eprintln!("{}", game.board_str());
        eprintln!("{}", solver.raw_eval(&game));
        let op = solver.get_move(&game);
        game.slide(op.clone());
        println!("{}", op);
    }
}
