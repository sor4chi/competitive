use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use rand::Rng;

use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
    util::tornado_travel,
};

use super::Solver;

pub struct OneArmTreeSolver<'a> {
    io: IO,
    input: Input,
    start: &'a Instant,
}

impl<'a> OneArmTreeSolver<'a> {
    pub fn new(io: IO, input: Input, start: &'a Instant) -> Self {
        OneArmTreeSolver { io, input, start }
    }
}

impl Solver for OneArmTreeSolver<'_> {
    fn solve(&mut self) -> Output {
        let mut best_output = None;
        let mut best_score = usize::MAX;
        let mut rng = rand::thread_rng();

        let mut iter = 0;
        'outer: loop {
            iter += 1;

            let mut travel = tornado_travel(self.input.n);
            travel.reverse();
            let initial_pos = (self.input.n / 2, self.input.n / 2);
            let mut arm_tree = ArmTree::new(initial_pos);
            let mut cur_id = ROOT_ID;
            let v = self.input.v.min(11);
            for i in 0..v - 1 {
                // cur_id = arm_tree.add_arm(cur_id, v - i - 1);
                // cur_id = arm_tree.add_arm(
                //     cur_id,
                //     (2i32.pow((v - i - 2) as u32) as usize).min(self.input.n / 2),
                // );
                // lengthをランダムに決める
                cur_id = arm_tree.add_arm(cur_id, rng.gen_range(1..=(self.input.n / 2)));
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

            let mut operations = vec![];
            let mut is_carrying = false;
            let mut cur_arm_tree = arm_tree;
            let mut cur_move_to = Move::Stay;
            let mut cur_center = initial_pos;

            while !cur_targets.is_empty() {
                loop {
                    fn dfs(
                        arm_tree: &mut ArmTree,
                        rotates: &mut Vec<Rotate>,
                        cur_board: &mut Vec<Vec<bool>>,
                        cur_targets: &mut HashSet<(usize, usize)>,
                        is_carrying: &mut bool,
                        start: &Instant,
                        tl: u128,
                        n: usize,
                    ) -> (Vec<Rotate>, bool) {
                        if start.elapsed().as_millis() > tl {
                            return (vec![], false);
                        }

                        // arm_treeのleavesがcur_boardにかぶっていたらそれをbestとして終了
                        for leaf_id in &arm_tree.leaves {
                            let (x, y) = arm_tree.tree_pos[leaf_id];
                            if x < 0 || y < 0 || x >= n as i32 || y >= n as i32 {
                                continue;
                            }
                            if !*is_carrying && cur_board[x as usize][y as usize] {
                                cur_board[x as usize][y as usize] = false;
                                *is_carrying = true;
                                return (rotates.clone(), true);
                            }
                            if *is_carrying && cur_targets.contains(&(x as usize, y as usize)) {
                                cur_targets.remove(&(x as usize, y as usize));
                                *is_carrying = false;
                                return (rotates.clone(), true);
                            }
                        }
                        let cur_id = ArmNodeID(rotates.len());
                        if let Some((child, _)) = arm_tree.tree.get(&cur_id).and_then(|v| v.first())
                        {
                            let id = child.0;
                            for r in [Rotate::Left, Rotate::Right] {
                                rotates.push(r);
                                arm_tree.rotate(ArmNodeID(id), r);
                                let (res, found) = dfs(
                                    arm_tree,
                                    rotates,
                                    cur_board,
                                    cur_targets,
                                    is_carrying,
                                    start,
                                    tl,
                                    n,
                                );
                                if found {
                                    return (res, true);
                                } else {
                                    rotates.pop();
                                    arm_tree.rotate(ArmNodeID(id), r.reverse());
                                }
                            }
                            rotates.push(Rotate::Stay);
                            let (res, found) = dfs(
                                arm_tree,
                                rotates,
                                cur_board,
                                cur_targets,
                                is_carrying,
                                start,
                                tl,
                                n,
                            );
                            if found {
                                return (res, true);
                            } else {
                                rotates.pop();
                            }
                        }
                        (vec![], false)
                    }

                    let (mut best_rotates, found) = dfs(
                        &mut cur_arm_tree,
                        &mut vec![],
                        &mut cur_board,
                        &mut cur_targets,
                        &mut is_carrying,
                        self.start,
                        self.input.tl,
                        self.input.n,
                    );

                    if self.start.elapsed().as_millis() > self.input.tl {
                        break 'outer;
                    }

                    if !found {
                        break;
                    }

                    // pad with Stay
                    while best_rotates.len() < v - 1 {
                        best_rotates.push(Rotate::Stay);
                    }
                    let mut actions = vec![Action::Stay; v - 1];
                    actions.push(Action::PickOrRelease);
                    let op = Operation {
                        move_to: cur_move_to,
                        rotates: best_rotates,
                        actions,
                    };
                    cur_move_to = Move::Stay; // 最初だけ移動を引き継ぐため、使ったらリセット
                    operations.push(op);

                    // operations.lengthがbest_scoreを超えていたら打ち切り
                    if operations.len() >= best_score {
                        continue 'outer;
                    }
                }

                if cur_move_to != Move::Stay {
                    let op = Operation {
                        move_to: cur_move_to,
                        rotates: vec![Rotate::Stay; v - 1],
                        actions: vec![Action::Stay; v],
                    };
                    operations.push(op);
                }

                // cur_centerからd方向に動かし、visitedにない場所がみつかればそこに向かう
                if travel.is_empty() {
                    break;
                }
                let dir = travel.pop().unwrap();
                cur_move_to = Move::Shift(dir);
                let d = dir.get_d();
                let new_center = (cur_center.0 as i32 + d.0, cur_center.1 as i32 + d.1);
                cur_center = (new_center.0 as usize, new_center.1 as usize);
                cur_arm_tree.all_shift(d);
            }

            let output = Output {
                flatten_tree: cur_arm_tree.flatten(),
                initial_pos,
                operations,
            };

            if best_output.is_none() {
                best_output = Some(output);
            } else {
                let cur_score = output.operations.len();
                if cur_score < best_score {
                    best_score = cur_score;
                    best_output = Some(output);
                }
            }
        }

        eprintln!("iter: {}", iter);

        best_output.unwrap()
    }
}
