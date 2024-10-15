// Partial copy from tools

#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use rand::prelude::*;
use std::{fmt::Pointer, ops::RangeBounds};

use crate::io::{Input, Output};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.n, self.m, self.v)?;
        for i in 0..self.n {
            writeln!(
                f,
                "{}",
                self.s[i]
                    .iter()
                    .map(|&b| if b { '1' } else { '0' })
                    .collect::<String>()
            )?;
        }
        for i in 0..self.n {
            writeln!(
                f,
                "{}",
                self.t[i]
                    .iter()
                    .map(|&b| if b { '1' } else { '0' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub fn gen(seed: u64, fix_N: Option<usize>, fix_M: Option<usize>, fix_V: Option<usize>) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed ^ 3);
    let mut N = rng.gen_range(15i32..=30) as usize;
    if let Some(fix_N) = fix_N {
        N = fix_N;
    }
    let mut M = rng.gen_range(((N * N + 9) / 10) as i32..=(N * N / 2) as i32) as usize;
    if let Some(fix_M) = fix_M {
        M = fix_M.min(N * N / 2);
    }
    let mut V = rng.gen_range(5i32..=15) as usize;
    if let Some(fix_V) = fix_V {
        V = fix_V;
    }
    let mut st;
    loop {
        st = vec![mat![false; N; N]; 2];
        for s in &mut st {
            let mut w = mat![0.0; N; N];
            let c = rng.gen_range(1..=5);
            for _ in 0..c {
                let cx = rng.gen_range(-1.0..=N as f64);
                let cy = rng.gen_range(-1.0..=N as f64);
                let a = rng.gen::<f64>();
                let sigma = rng.gen_range(2.0..=5.0);
                for i in 0..N {
                    for j in 0..N {
                        let dx = i as f64 - cx;
                        let dy = j as f64 - cy;
                        w[i][j] += a * (-(dx * dx + dy * dy) / (2.0 * sigma * sigma)).exp();
                    }
                }
            }
            let mut ps = vec![];
            for i in 0..N {
                for j in 0..N {
                    ps.push((i, j));
                }
            }
            for _ in 0..M {
                let &(i, j) = ps.choose_weighted(&mut rng, |&(i, j)| w[i][j]).unwrap();
                s[i][j] = true;
                ps.retain(|&p| p != (i, j));
            }
        }
        let mut diff = 0;
        for i in 0..N {
            for j in 0..N {
                if st[0][i][j] != st[1][i][j] {
                    diff += 1;
                }
            }
        }
        if diff >= M {
            break;
        }
    }
    Input {
        n: N,
        m: M,
        v: V,
        s: st[0].clone(),
        t: st[1].clone(),
        tl: 2900,
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out, out.operations.len());
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub const DIJ: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct State {
    pub N: usize,
    pub V: usize,
    pub r: (i32, i32),
    pub pL: Vec<(usize, usize)>,
    pub is_leaf: Vec<bool>,
    pub dirs: Vec<usize>,
    pub has: Vec<bool>,
    pub board: Vec<Vec<bool>>,
}

impl State {
    pub fn new(input: &Input, r: (i32, i32), pL: &Vec<(usize, usize)>) -> Self {
        let V = pL.len() + 1;
        let mut is_leaf = vec![true; V];
        for &(p, _) in pL {
            is_leaf[p] = false;
        }
        State {
            N: input.n,
            V,
            r,
            pL: pL.clone(),
            is_leaf,
            dirs: vec![0; V],
            has: vec![false; V],
            board: input.s.clone(),
        }
    }
    pub fn get(&self, mut u: usize) -> (i32, i32) {
        let mut vs = vec![];
        while u > 0 {
            let (v, l) = self.pL[u - 1];
            vs.push((self.dirs[u], l));
            u = v;
        }
        let mut p = self.r;
        let mut dir = 0;
        for &(d, l) in vs.iter().rev() {
            dir = (dir + d) % 4;
            let (dx, dy) = DIJ[dir];
            p.0 += l as i32 * dx;
            p.1 += l as i32 * dy;
        }
        p
    }
    pub fn apply(&mut self, s: &[char]) -> Result<(), String> {
        match s[0] {
            'U' => {
                self.r.0 -= 1;
                if self.r.0 < 0 {
                    return Err(format!("The root coordinate is out of range."));
                }
            }
            'D' => {
                self.r.0 += 1;
                if self.r.0 == self.N as i32 {
                    return Err(format!("The root coordinate is out of range."));
                }
            }
            'L' => {
                self.r.1 -= 1;
                if self.r.1 < 0 {
                    return Err(format!("The root coordinate is out of range."));
                }
            }
            'R' => {
                self.r.1 += 1;
                if self.r.1 == self.N as i32 {
                    return Err(format!("The root coordinate is out of range."));
                }
            }
            '.' => {}
            _ => {
                return Err(format!("Invalid operation: {}", s[0]));
            }
        }
        for i in 1..self.V {
            match s[i] {
                'L' => {
                    self.dirs[i] = (self.dirs[i] + 3) % 4;
                }
                'R' => {
                    self.dirs[i] = (self.dirs[i] + 1) % 4;
                }
                '.' => {}
                _ => {
                    return Err(format!("Invalid operation: {}", s[i]));
                }
            }
        }
        for i in 0..self.V {
            match s[self.V + i] {
                'P' => {
                    if !self.is_leaf[i] {
                        return Err(format!("The vertex {} is not a leaf.", i));
                    } else {
                        let (x, y) = self.get(i);
                        if x < 0 || y < 0 || x >= self.N as i32 || y >= self.N as i32 {
                            return Err(format!("The leaf coordinate is out of range."));
                        }
                        if self.has[i] {
                            if self.board[x as usize][y as usize] {
                                return Err(format!(
                                    "You cannot put multiple takoyaki on the same square."
                                ));
                            }
                            self.has[i] = false;
                            self.board[x as usize][y as usize] = true;
                        } else {
                            if !self.board[x as usize][y as usize] {
                                return Err(format!("({}, {}) does not contain takoyaki.", x, y));
                            }
                            self.has[i] = true;
                            self.board[x as usize][y as usize] = false;
                        }
                    }
                }
                '.' => {}
                _ => {
                    return Err(format!("Invalid operation: {}", s[self.V + i]));
                }
            }
        }
        Ok(())
    }
}

pub fn compute_score_details(input: &Input, out: &Output, t: usize) -> (i64, String, State) {
    let mut state = State::new(
        input,
        (out.initial_pos.0 as i32, out.initial_pos.1 as i32),
        &out.flatten_tree.iter().map(|&(p, l)| (p.0, l)).collect(),
    );
    for s in &out.operations[..t] {
        let s = s.to_string();
        let s = s.chars().collect::<Vec<_>>();
        if let Err(err) = state.apply(&s) {
            return (0, err, state);
        }
    }
    let mut M2 = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            if input.t[i][j] && state.board[i][j] {
                M2 += 1;
            }
        }
    }
    let score = if M2 == input.m {
        t as i64
    } else {
        100000 + 1000 * (input.m as i64 - M2 as i64)
    };
    (score, String::new(), state)
}
