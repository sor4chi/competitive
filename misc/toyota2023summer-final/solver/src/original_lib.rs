#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::input;
use rand::prelude::*;

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

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub rs: Vec<(usize, usize)>,
    pub ts: Vec<usize>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.D, self.rs.len())?;
        for &(i, j) in &self.rs {
            writeln!(f, "{} {}", i, j)?;
        }
        for &i in &self.ts {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        D: usize, N: usize,
        rs: [(usize, usize); N],
        ts: [usize; D * D - 1 - N],
    }
    Input { D, rs, ts }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
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

const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

fn reachable(
    n: usize,
    cs: &Vec<usize>,
    ti: usize,
    tj: usize,
    visited: &mut Vec<usize>,
    iter: &mut usize,
) -> bool {
    if cs[n / 2] != !0 {
        return false;
    }
    *iter += 1;
    let mut stack = vec![(0, n / 2)];
    visited[n / 2] = *iter;
    while let Some((i, j)) = stack.pop() {
        if (i, j) == (ti, tj) {
            return true;
        }
        for (di, dj) in DIJ {
            let i2 = i + di;
            let j2 = j + dj;
            if i2 < n && j2 < n && cs[i2 * n + j2] == !0 && visited[i2 * n + j2].setmax(*iter) {
                stack.push((i2, j2));
            }
        }
    }
    false
}

pub fn compute_score(input: &Input, out: &[(usize, usize)]) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(
    input: &Input,
    out: &[(usize, usize)],
) -> (
    i64,
    String,
    (Vec<usize>, Vec<usize>, Option<(usize, usize)>),
) {
    let mut cs = vec![!0; input.D * input.D];
    let mut visited = vec![0; input.D * input.D];
    let mut iter = 0;
    let mut num_put = 0;
    let mut order = vec![];
    let mut last = None;
    for &(i, j) in &input.rs {
        cs[i * input.D + j] = !1;
    }
    for &a in out {
        last = Some(a.clone());
        if num_put < input.D * input.D - 1 - input.rs.len() {
            if a == (0, input.D / 2) {
                return (
                    0,
                    format!(
                        "You cannot put containers on the entrance ({}, {})",
                        0,
                        input.D / 2
                    ),
                    (cs, order, last),
                );
            } else if cs[a.0 * input.D + a.1] == !1 {
                return (
                    0,
                    format!("({}, {}) contains an obstacle", a.0, a.1),
                    (cs, order, last),
                );
            } else if cs[a.0 * input.D + a.1] != !0 {
                return (
                    0,
                    format!("({}, {}) already contains a container", a.0, a.1),
                    (cs, order, last),
                );
            } else if !reachable(input.D, &cs, a.0, a.1, &mut visited, &mut iter) {
                return (
                    0,
                    format!("({}, {}) is not reachalbe", a.0, a.1),
                    (cs, order, last),
                );
            }
            cs[a.0 * input.D + a.1] = input.ts[num_put];
            num_put += 1;
        } else {
            if cs[a.0 * input.D + a.1] == !0 || cs[a.0 * input.D + a.1] == !1 {
                return (
                    0,
                    format!("({}, {}) does not contain a container", a.0, a.1),
                    (cs, order, last),
                );
            }
            let c = cs[a.0 * input.D + a.1];
            cs[a.0 * input.D + a.1] = !0;
            if !reachable(input.D, &cs, a.0, a.1, &mut visited, &mut iter) {
                cs[a.0 * input.D + a.1] = c;
                return (
                    0,
                    format!("({}, {}) is not reachalbe", a.0, a.1),
                    (cs, order, last),
                );
            }
            order.push(c);
        }
    }
    let mut inv = 0;
    for i in 0..order.len() {
        for j in i + 1..order.len() {
            if order[i] > order[j] {
                inv += 1;
            }
        }
    }
    let K = (input.D * input.D - input.rs.len()) * (input.D * input.D - 1 - input.rs.len()) / 2;
    let score = (K - inv) as f64 / K as f64;
    let err = if order.len() < input.D * input.D - 1 - input.rs.len() {
        format!("Containers are still remaining.")
    } else {
        String::new()
    };
    ((1e9 * score).round() as i64, err, (cs, order, last))
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}
