#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub struct Input {
    n: usize,
    A: Vec<Vec<i32>>,
}

pub struct Output {
    pub out: Vec<Vec<char>>,
}

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIR: [char; 4] = ['U', 'D', 'L', 'R'];

pub struct State {
    n: usize,
    board: Vec<Vec<i32>>,
    A: Vec<Vec<i32>>,
    B: Vec<Vec<i32>>,
    pos: Vec<(usize, usize, i32)>,
    done: i32,
    turn: i64,
}

impl State {
    fn new(input: &Input) -> Self {
        let mut board = mat![-1; input.n; input.n];
        let mut A = input
            .A
            .iter()
            .map(|a| a.iter().copied().rev().collect_vec())
            .collect_vec();
        for i in 0..input.n {
            board[i][0] = A[i].pop().unwrap();
        }
        State {
            n: input.n,
            board,
            A,
            B: vec![vec![]; input.n],
            pos: (0..input.n).map(|i| (i, 0, -1)).collect_vec(),
            done: 0,
            turn: 0,
        }
    }
    fn apply(&mut self, mv: &[char]) -> Result<(), String> {
        self.turn += 1;
        let mut to = vec![(!0, !0, -1); self.n];
        for i in 0..self.n {
            let (mut x, mut y, mut z) = self.pos[i];
            match mv[i] {
                '.' => (),
                'P' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    } else if z != -1 {
                        return Err(format!("Crane {i} holds a container."));
                    } else if self.board[x][y] == -1 {
                        return Err(format!("No container at ({x}, {y})."));
                    } else {
                        z = self.board[x][y];
                        self.board[x][y] = -1;
                    }
                }
                'Q' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    } else if z == -1 {
                        return Err(format!("Crane {i} does not hold a container."));
                    } else if self.board[x][y] != -1 {
                        return Err(format!("Container already exists at ({x}, {y})."));
                    } else {
                        self.board[x][y] = z;
                        z = -1;
                    }
                }
                'U' | 'D' | 'L' | 'R' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    }
                    let dir = (0..4).find(|&d| DIR[d] == mv[i]).unwrap();
                    let (dx, dy) = DIJ[dir];
                    x += dx;
                    y += dy;
                    if x >= self.n || y >= self.n {
                        return Err(format!("Crane {i} moved out of the board."));
                    } else if i > 0 && z != -1 && self.board[x][y] != -1 {
                        return Err(format!(
                            "Cranes {i} cannot move to a square that contains a container."
                        ));
                    }
                }
                'B' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    }
                    if z != -1 {
                        return Err(format!("Crane {i} holds a container."));
                    }
                    x = !0;
                    y = !0;
                }
                c => {
                    return Err(format!("Invalid move: {}", c));
                }
            }
            to[i] = (x, y, z);
        }
        for i in 0..self.n {
            if to[i].0 == !0 {
                continue;
            }
            for j in 0..i {
                if to[j].0 == !0 {
                    continue;
                }
                if (to[i].0, to[i].1) == (to[j].0, to[j].1) {
                    return Err(format!("Crane {j} and {i} collided."));
                } else if (to[i].0, to[i].1) == (self.pos[j].0, self.pos[j].1)
                    && (to[j].0, to[j].1) == (self.pos[i].0, self.pos[i].1)
                {
                    return Err(format!("Crane {i} and {j} collided."));
                }
            }
        }
        self.pos = to;
        for i in 0..self.n {
            if self.board[i][0] == -1
                && self.A[i].len() > 0
                && self.pos.iter().all(|p| p.2 == -1 || (p.0, p.1) != (i, 0))
            {
                self.board[i][0] = self.A[i].pop().unwrap();
            }
            if self.board[i][self.n - 1] != -1 {
                self.done += 1;
                if (self.n * i) as i32 <= self.board[i][self.n - 1]
                    && self.board[i][self.n - 1] < (self.n * (i + 1)) as i32
                {
                    self.B[i].push(self.board[i][self.n - 1]);
                }
                self.board[i][self.n - 1] = -1;
            }
        }
        Ok(())
    }
    fn score(&self) -> i64 {
        let A = self.turn;
        let mut B = 0;
        let mut C = self.done as i64;
        let D = (self.n * self.n) as i64 - self.done as i64;
        for i in 0..self.n {
            C -= self.B[i].len() as i64;
            for a in 0..self.B[i].len() {
                for b in a + 1..self.B[i].len() {
                    if self.B[i][a] > self.B[i][b] {
                        B += 1;
                    }
                }
            }
        }
        let score = A + B * 100 + C * 10000 + D * 1000000;
        score
    }
}

pub fn compute_score_details(input: &Input, out: &Output, t: usize) -> (i64, String, State) {
    let mut state = State::new(input);
    for k in 0..t {
        let mv = (0..input.n)
            .map(|i| out.out[i].get(k).copied().unwrap_or('.'))
            .collect_vec();
        if let Err(err) = state.apply(&mv) {
            return (0, format!("{err} (turn {k})"), state);
        }
    }
    let score = state.score();
    (score, String::new(), state)
}
