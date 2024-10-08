use std::process::Command;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::Child,
    time::Instant,
};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
};

use super::Solver;

pub struct BulkArmSolver {
    io: IO,
    input: Input,
}

impl BulkArmSolver {
    pub fn new(io: IO, input: Input) -> Self {
        BulkArmSolver { io, input }
    }
}

// n=3, k=4 の時 [0,0,0] から [3,3,3] までの組み合わせを生成する
fn generate_cands(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut cands = vec![];
    let mut current = vec![0; n];
    loop {
        cands.push(current.clone());
        let mut i = n;
        while i > 0 && current[i - 1] == k - 1 {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        current[i - 1] += 1;
        for j in i..n {
            current[j] = 0;
        }
    }
    cands
}

const DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Left,
    Direction::Down,
];

#[test]
fn test_generate_cands() {
    let mut time = Instant::now();
    generate_cands(10, 3);
    eprintln!("{:?}", time.elapsed());
}

fn tornado_travel(n: usize) -> Vec<Direction> {
    let mut res = vec![];
    let mut x = n / 2;
    let mut y = n / 2;
    let mut d = 0;
    let mut l = 1;
    let mut c = 0;
    let mut i = 0;
    while i < n * n - 1 {
        res.push(DIRS[d as usize]);
        let n = DIRS[d as usize].get_d();
        x = (x as i32 + n.0) as usize;
        y = (y as i32 + n.1) as usize;
        i += 1;
        c += 1;
        if c == l {
            c = 0;
            d = (d + 1) % 4;
            if d % 2 == 0 {
                l += 1;
            }
        }
    }
    res
}

#[test]
fn test_tornado_travel() {
    let n = 5;
    let res = tornado_travel(n);
    assert_eq!(res.len(), n * n);
    eprintln!("{:?}", res);
}

impl Solver for BulkArmSolver {
    fn solve(&mut self) -> Output {
        assert!(self.input.v >= 7);
        let mut best_operations = vec![];
        let mut best_score = usize::MAX;
        let initial_pos = (self.input.n / 2, self.input.n / 2);
        let mut best_arm_tree = ArmTree::new(initial_pos);
        let start = Instant::now();
        let tl = 2900;
        let seed = [0; 32];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

        let mut iter = 0;
        'outer: loop {
            iter += 1;

            if start.elapsed().as_millis() > tl {
                break;
            }

            let mut operations = vec![];
            let mut tour = tornado_travel(self.input.n);
            tour.reverse(); // popで使うので逆順にしておく

            // split v
            let mut arm_tree = ArmTree::new(initial_pos);
            let mut cur_id = ROOT_ID;
            let arm_size = 5;
            for i in 0..arm_size {
                cur_id = arm_tree.add_arm(cur_id, rng.gen_range(1..=(self.input.n / 2)));
                // cur_id = arm_tree.add_arm(cur_id, 1 << (arm_size - i));
            }
            for i in 0..self.input.v - arm_size - 1 {
                arm_tree.add_arm(cur_id, i + 1);
            }

            let mut cur_board = self.input.s.clone();
            let mut cur_targets = HashSet::new();
            // もし既にself.input.sとself.input.tが一致しているものは埋めておく
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    if self.input.t[i][j] {
                        if self.input.s[i][j] {
                            cur_board[i][j] = false;
                        } else {
                            cur_targets.insert((i, j));
                        }
                    }
                }
            }

            let mut cur_holding = vec![false; self.input.v];
            let leaves = arm_tree.leaves.clone();
            let mut cur_arm_tree = arm_tree.clone();
            let mut booked_move = None;

            loop {
                while !cur_targets.is_empty() {
                    if start.elapsed().as_millis() > tl {
                        break 'outer;
                    }
                    let mut best_rotates_score = 0;
                    let mut best_rotates = vec![];
                    let mut best_actions = vec![];
                    let mut best_arm_tree = cur_arm_tree.clone();
                    let cands = generate_cands(arm_size, 3);

                    for after_rotate in 0..2 {
                        for rotates in &cands {
                            let mut try_rotates = rotates.to_vec();
                            let mut try_arm_tree = cur_arm_tree.clone();
                            for (i, r) in try_rotates.iter().enumerate() {
                                let rotate = match r {
                                    0 => Rotate::Left,
                                    1 => Rotate::Right,
                                    2 => Rotate::Stay,
                                    _ => unreachable!(),
                                };
                                if rotate == Rotate::Stay {
                                    continue;
                                }
                                try_arm_tree.rotate(ArmNodeID(i + 1), rotate);
                            }

                            for leaf_id in &leaves {
                                let rotate = match after_rotate {
                                    0 => Rotate::Left,
                                    1 => Rotate::Right,
                                    2 => Rotate::Stay,
                                    _ => unreachable!(),
                                };
                                if rotate == Rotate::Stay {
                                    continue;
                                }
                                try_arm_tree.rotate(*leaf_id, rotate);
                                try_rotates.push(after_rotate);
                            }

                            let mut try_rotates_score = 0;
                            let mut try_actions = vec![Action::Stay; self.input.v];
                            for leaf_id in &leaves {
                                // 葉がcur_boardにどれだけかぶっているかを計算
                                let (x, y) = try_arm_tree.tree_pos[leaf_id];
                                if x < 0
                                    || y < 0
                                    || x >= self.input.n as i32
                                    || y >= self.input.n as i32
                                {
                                    continue;
                                }
                                if !cur_holding[leaf_id.0] && cur_board[x as usize][y as usize] {
                                    try_actions[leaf_id.0] = Action::PickOrRelease;
                                    try_rotates_score += 1;
                                    continue;
                                }
                                if cur_holding[leaf_id.0]
                                    && cur_targets.contains(&(x as usize, y as usize))
                                {
                                    try_actions[leaf_id.0] = Action::PickOrRelease;
                                    try_rotates_score += 1;
                                    continue;
                                }
                            }

                            if try_rotates_score > best_rotates_score {
                                best_rotates_score = try_rotates_score;
                                best_rotates = try_rotates.to_vec();
                                best_arm_tree = try_arm_tree;
                                best_actions = try_actions;
                            }
                        }
                    }

                    if best_rotates_score == 0 {
                        break;
                    }

                    let mut rotates = best_rotates
                        .iter()
                        .map(|&r| match r {
                            0 => Rotate::Left,
                            1 => Rotate::Right,
                            2 => Rotate::Stay,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>();
                    while rotates.len() < self.input.v - 1 {
                        rotates.push(Rotate::Stay);
                    }

                    for leaf_id in &leaves {
                        let (x, y) = best_arm_tree.tree_pos[leaf_id];
                        if x < 0 || y < 0 || x >= self.input.n as i32 || y >= self.input.n as i32 {
                            continue;
                        }
                        if !cur_holding[leaf_id.0] && cur_board[x as usize][y as usize] {
                            cur_board[x as usize][y as usize] = false;
                            cur_holding[leaf_id.0] = true;
                            continue;
                        }
                        if cur_holding[leaf_id.0] && cur_targets.contains(&(x as usize, y as usize))
                        {
                            cur_targets.remove(&(x as usize, y as usize));
                            cur_holding[leaf_id.0] = false;
                            continue;
                        }
                    }

                    cur_arm_tree = best_arm_tree;

                    operations.push(Operation {
                        move_to: if let Some(dir) = booked_move {
                            booked_move = None;
                            Move::Shift(dir)
                        } else {
                            Move::Stay
                        },
                        rotates,
                        actions: best_actions,
                    });

                    // ベストスコア以上に探索するメリットがないのでこの時点で打ち切る
                    if operations.len() >= best_score {
                        continue 'outer;
                    }
                }

                if let Some(dir) = booked_move {
                    let op = Operation {
                        move_to: Move::Shift(dir),
                        rotates: vec![Rotate::Stay; self.input.v - 1],
                        actions: vec![Action::Stay; self.input.v],
                    };
                    operations.push(op);
                }

                if cur_targets.is_empty() {
                    break;
                }

                if tour.is_empty() {
                    break;
                }

                let dir = tour.pop().unwrap();
                let shift = dir.get_d();
                cur_arm_tree.all_shift(shift);
                booked_move = Some(dir);
            }

            let score = operations.len();
            if score < best_score {
                best_score = score;
                best_operations = operations;
                best_arm_tree = cur_arm_tree;
            }
        }

        eprintln!("iter: {}", iter);

        Output {
            flatten_tree: best_arm_tree.flatten(),
            initial_pos,
            operations: best_operations,
        }
    }
}
