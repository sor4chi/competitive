use std::process::Command;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::Child,
    time::Instant,
};

use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
};

use super::Solver;

pub struct OneOPSolver {
    io: IO,
    input: Input,
}

impl OneOPSolver {
    pub fn new(io: IO, input: Input) -> Self {
        OneOPSolver { io, input }
    }
}

fn eval(current: &[usize], ss: &[(usize, usize)], tt: &[(usize, usize)]) -> usize {
    let mut score = 0;
    for i in 0..ss.len() {
        let (sx, sy) = ss[i];
        let (tx, ty) = tt[current[i]];
        score += (sx as i32 - tx as i32).unsigned_abs() as usize
            + (sy as i32 - ty as i32).unsigned_abs() as usize;
    }
    score
}

fn visualize_pair(current: &[usize], ss: &[(usize, usize)], tt: &[(usize, usize)], filename: &str) {
    eprintln!("plotting {}...", filename);
    let mut python_code = String::new();
    python_code.push_str("import matplotlib.pyplot as plt\n");
    python_code.push_str("from matplotlib.collections import LineCollection\n");

    python_code.push_str("fig, ax = plt.subplots(1,1, figsize=(10,10))\n");
    python_code.push_str("ax.invert_yaxis()\n");

    let mut s = vec![];
    let mut t = vec![];
    for i in 0..ss.len() {
        let (sx, sy) = ss[i];
        let (tx, ty) = tt[current[i]];
        s.push((sx, sy));
        t.push((tx, ty));
    }

    python_code.push_str(&format!("s = {:?}\n", s));
    python_code.push_str(&format!("t = {:?}\n", t));

    python_code.push_str("lc = LineCollection([[(s[i][1], s[i][0]), (t[i][1], t[i][0])] for i in range(len(s))], color='black')\n");
    python_code.push_str("ax.add_collection(lc)\n");

    python_code.push_str("ax.scatter([s[i][1] for i in range(len(s))], [s[i][0] for i in range(len(s))], color='red')\n");
    python_code.push_str("ax.scatter([t[i][1] for i in range(len(t))], [t[i][0] for i in range(len(t))], color='blue')\n");

    python_code.push_str("plt.savefig('");
    python_code.push_str(filename);
    python_code.push_str("')\n");

    Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .spawn()
        .expect("failed to start `python3`");
}

impl Solver for OneOPSolver {
    fn solve(&mut self) -> Output {
        // self.input.sの点をself.input.tの点に移動させるように対応させる
        // マンハッタン距離が近くなるように焼きなまし
        let mut ss = vec![];
        let mut tt = vec![];
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                if self.input.s[i][j] == self.input.t[i][j] {
                    continue;
                }
                if self.input.s[i][j] {
                    ss.push((i, j));
                }
                if self.input.t[i][j] {
                    tt.push((i, j));
                }
            }
        }
        let mut rng = rand::thread_rng();
        let mut current = (0..ss.len()).collect::<Vec<_>>();
        current.shuffle(&mut rng);
        let mut current_score = eval(&current, &ss, &tt);
        eprintln!("current_score: {}", current_score);
        eprintln!("current: {:?}", current);
        // visualize_pair(&current, &ss, &tt, "current.png");
        let mut best = current.clone();
        let mut best_score = current_score;
        let start = Instant::now();
        let start_temp = 1e2;
        let end_temp = 1e-4;
        let mut temp = start_temp;
        let mut iter = 0;
        let tl = 1900;

        while start.elapsed().as_millis() < tl {
            iter += 1;
            let i = rng.gen_range(0..ss.len());
            let j = rng.gen_range(0..ss.len());
            let mut next = current.clone();
            next.swap(i, j);
            let next_score = eval(&next, &ss, &tt);
            let diff = next_score as i64 - current_score as i64;
            // 最小化
            if diff < 0 || rng.gen::<f64>() < (-diff as f64 / temp).exp() {
                current = next;
                current_score = next_score;
                if current_score < best_score {
                    best = current.clone();
                    best_score = current_score;
                }
            }
            temp = start_temp
                + (end_temp - start_temp) * start.elapsed().as_millis() as f64 / tl as f64;
            iter += 1;
        }

        eprintln!("iter: {}", iter);
        eprintln!("best_score: {}", best_score);
        eprintln!("best: {:?}", best);
        // visualize_pair(&best, &ss, &tt, "best.png");

        // sからtへの移動をOperationに変換
        let mut cur = (0, 0);
        let mut operations = vec![];
        // まだ訪れてないindexのうち最もcurに近いssのindexを選ぶ
        let mut visited = vec![false; ss.len()];
        // curがssに含まれていればoperationsに追加
        if ss.contains(&cur) {
            operations.push(Operation {
                move_to: Move::Stay,
                rotates: vec![],
                actions: vec![Action::PickOrRelease],
            });
        }
        for _ in 0..ss.len() {
            let mut best_dist = usize::MAX;
            let mut best_i = 0;
            for i in 0..ss.len() {
                if visited[i] {
                    continue;
                }
                let (sx, sy) = ss[i];
                let dist = (sx as i32 - cur.0 as i32).unsigned_abs() as usize
                    + (sy as i32 - cur.1 as i32).unsigned_abs() as usize;
                if dist < best_dist {
                    best_dist = dist;
                    best_i = i;
                }
            }
            visited[best_i] = true;
            let (sx, sy) = ss[best_i];
            let (tx, ty) = tt[best[best_i]];
            eprintln!(
                "cur: {:?}, (sx, sy): {:?}, (tx, ty): {:?}",
                cur,
                (sx, sy),
                (tx, ty)
            );
            if (sx, sy) == (tx, ty) {
                continue;
            }
            if (cur.0, cur.1) != (sx, sy) {
                let mut moves = vec![];
                {
                    let dx = sx as i32 - cur.0 as i32;
                    let dy = sy as i32 - cur.1 as i32;
                    while cur.0 != sx {
                        if dx > 0 {
                            moves.push(Move::Shift(Direction::Down));
                            cur.0 += 1;
                        } else if dx < 0 {
                            moves.push(Move::Shift(Direction::Up));
                            cur.0 -= 1;
                        }
                    }
                    while cur.1 != sy {
                        if dy > 0 {
                            moves.push(Move::Shift(Direction::Right));
                            cur.1 += 1;
                        } else if dy < 0 {
                            moves.push(Move::Shift(Direction::Left));
                            cur.1 -= 1;
                        }
                    }
                }
                for m in moves.iter() {
                    operations.push(Operation {
                        move_to: *m,
                        rotates: vec![],
                        actions: vec![Action::Stay],
                    });
                }
                operations.last_mut().unwrap().actions[0] = Action::PickOrRelease;
            }
            if (cur.0, cur.1) != (tx, ty) {
                let mut moves = vec![];
                {
                    let dx = tx as i32 - cur.0 as i32;
                    let dy = ty as i32 - cur.1 as i32;
                    while cur.0 != tx {
                        if dx > 0 {
                            moves.push(Move::Shift(Direction::Down));
                            cur.0 += 1;
                        } else if dx < 0 {
                            moves.push(Move::Shift(Direction::Up));
                            cur.0 -= 1;
                        }
                    }
                    while cur.1 != ty {
                        if dy > 0 {
                            moves.push(Move::Shift(Direction::Right));
                            cur.1 += 1;
                        } else if dy < 0 {
                            moves.push(Move::Shift(Direction::Left));
                            cur.1 -= 1;
                        }
                    }
                }
                for m in moves.iter() {
                    operations.push(Operation {
                        move_to: *m,
                        rotates: vec![],
                        actions: vec![Action::Stay],
                    });
                }
                operations.last_mut().unwrap().actions[0] = Action::PickOrRelease;
            }
        }

        Output {
            flatten_tree: vec![],
            initial_pos: (0, 0),
            operations,
        }
    }
}
