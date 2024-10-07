use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use rand::{rngs::StdRng, Rng};

use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
    tool::compute_score,
};

use super::Solver;

pub struct MultiArmTreeSolver {
    io: IO,
    input: Input,
}

impl MultiArmTreeSolver {
    pub fn new(io: IO, input: Input) -> Self {
        MultiArmTreeSolver { io, input }
    }
}

const DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Left,
    Direction::Down,
];

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

impl Solver for MultiArmTreeSolver {
    fn solve(&mut self) -> Output {
        let start = Instant::now();
        let tl = 2900;
        let mut best_output = None;
        let mut best_score = i64::MAX;
        let mut seed = [0; 32];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

        let mut iter = 0;
        'outer: loop {
            iter += 1;

            let mut travel = tornado_travel(self.input.n);
            travel.reverse();
            let initial_pos = (self.input.n / 2, self.input.n / 2);

            let mut arms = vec![];
            // split v
            let mut arm_tree = ArmTree::new(initial_pos);
            let mut cur_id = ROOT_ID;
            let tv = self.input.v - 1;
            let vs = vec![tv / 2, tv - tv / 2];
            for v in vs {
                for i in 0..v {
                    cur_id = arm_tree.add_arm(cur_id, rng.gen_range(1..=(self.input.n / 2)));
                }
                arms.push(arm_tree);
                arm_tree = ArmTree::new(initial_pos);
                cur_id = ROOT_ID;
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
            let mut cur_arms = arms;
            let mut is_carryings = vec![false; cur_arms.len()];
            let mut cur_center = initial_pos;
            let mut booked_move = None;

            while !cur_targets.is_empty() {
                loop {
                    let mut best_each_rotates = vec![vec![]; cur_arms.len()];
                    let mut best_each_actions = vec![];
                    let mut stacked = true;
                    let mut booked = HashSet::new();

                    // DFSで探索
                    for i in 0..cur_arms.len() {
                        // stack式DFSをするとarmsの大きいデータを毎回cloneするので遅い。再起で書く
                        fn dfs(
                            arm_tree: &mut ArmTree,
                            rotates: &mut Vec<Rotate>,
                            cur_board: &mut Vec<Vec<bool>>,
                            cur_arms: &mut Vec<ArmTree>,
                            is_carryings: &mut Vec<bool>,
                            cur_targets: &mut HashSet<(usize, usize)>,
                            booked: &mut HashSet<(usize, usize)>,
                            start: Instant,
                            tl: u128,
                            n: usize,
                            i: usize,
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
                                if !is_carryings[i]
                                    && cur_board[x as usize][y as usize]
                                    && !booked.contains(&(x as usize, y as usize))
                                {
                                    cur_board[x as usize][y as usize] = false;
                                    cur_arms[i] = arm_tree.clone();
                                    is_carryings[i] = true;
                                    // eprintln!("Arm[{}]: Pick at ({}, {})", i, x, y);
                                    return (rotates.clone(), true);
                                }
                                if is_carryings[i]
                                    && cur_targets.contains(&(x as usize, y as usize))
                                    && !booked.contains(&(x as usize, y as usize))
                                {
                                    cur_targets.remove(&(x as usize, y as usize));
                                    cur_arms[i] = arm_tree.clone();
                                    is_carryings[i] = false;
                                    // eprintln!("Arm[{}]: Release at ({}, {})", i, x, y);
                                    return (rotates.clone(), true);
                                }
                            }
                            let cur_id = ArmNodeID(rotates.len());
                            let first_child = arm_tree.tree.get(&cur_id).and_then(|v| v.first());
                            if let Some((child, _)) = first_child {
                                let id = child.0;
                                for r in [Rotate::Left, Rotate::Right] {
                                    rotates.push(r);
                                    arm_tree.rotate(ArmNodeID(id), r);
                                    let (new_rotates, is_p) = dfs(
                                        arm_tree,
                                        rotates,
                                        cur_board,
                                        cur_arms,
                                        is_carryings,
                                        cur_targets,
                                        booked,
                                        start,
                                        tl,
                                        n,
                                        i,
                                    );
                                    if is_p {
                                        return (new_rotates, true);
                                    } else {
                                        rotates.pop();
                                        arm_tree.rotate(ArmNodeID(id), r.reverse());
                                    }
                                }
                                rotates.push(Rotate::Stay);
                                let (new_rotates, is_p) = dfs(
                                    arm_tree,
                                    rotates,
                                    cur_board,
                                    cur_arms,
                                    is_carryings,
                                    cur_targets,
                                    booked,
                                    start,
                                    tl,
                                    n,
                                    i,
                                );
                                if is_p {
                                    return (new_rotates, true);
                                } else {
                                    rotates.pop();
                                }
                            }
                            (vec![], false)
                        }

                        let (mut best_rotates, is_p) = dfs(
                            &mut cur_arms[i].clone(),
                            &mut vec![],
                            &mut cur_board,
                            &mut cur_arms,
                            &mut is_carryings,
                            &mut cur_targets,
                            &mut booked,
                            start,
                            tl,
                            self.input.n,
                            i,
                        );

                        if start.elapsed().as_millis() > tl {
                            break 'outer;
                        }

                        if is_p {
                            stacked = false;
                        }

                        // eprintln!("iter: {}", iter);

                        // rotatesがcur_arm[i]の長さより短い場合はStayを追加
                        while best_rotates.len() < cur_arms[i].tree_pos.len() - 1 {
                            best_rotates.push(Rotate::Stay);
                        }

                        best_each_rotates[i] = best_rotates;

                        let mut best_actions = vec![Action::Stay; cur_arms[i].tree_pos.len() - 2];
                        if is_p {
                            best_actions.push(Action::PickOrRelease);
                        } else {
                            best_actions.push(Action::Stay);
                        }
                        best_each_actions.push(best_actions);
                    }

                    if stacked {
                        break;
                    }

                    // eprintln!("i: {}", i);
                    // eprintln!("best_rotates: {:?}", best_rotates);
                    let mut best_rotates = vec![];
                    for rotates in best_each_rotates {
                        best_rotates.extend(rotates);
                    }
                    let mut best_actions = vec![Action::Stay];
                    for actions in best_each_actions {
                        best_actions.extend(actions);
                    }
                    let op = Operation {
                        move_to: if let Some(dir) = booked_move {
                            booked_move = None;
                            Move::Shift(dir)
                        } else {
                            Move::Stay
                        },
                        rotates: best_rotates,
                        actions: best_actions,
                    };
                    operations.push(op);
                }

                if let Some(dir) = booked_move {
                    let op = Operation {
                        move_to: Move::Shift(dir),
                        rotates: vec![Rotate::Stay; self.input.v - 1],
                        actions: vec![Action::Stay; self.input.v],
                    };
                    operations.push(op);
                }

                // cur_centerからd方向に動かし、visitedにない場所がみつかればそこに向かう
                if travel.is_empty() {
                    break;
                }
                let dir = travel.pop().unwrap();
                let d = dir.get_d();
                let new_center = (cur_center.0 as i32 + d.0, cur_center.1 as i32 + d.1);
                cur_center = (new_center.0 as usize, new_center.1 as usize);
                // eprintln!("Move: {:?}", dir);
                // eprintln!("Center: {:?}", cur_center);
                for arm in &mut cur_arms {
                    arm.all_shift(d);
                }
                // operations.push(Operation {
                //     move_to: Move::Shift(dir),
                //     rotates: vec![Rotate::Stay; self.input.v - 1],
                //     actions: vec![Action::Stay; self.input.v],
                // });
                booked_move = Some(dir);
            }

            let mut all_flatten_tree = vec![];
            let mut prefix = 0;
            for arm in &cur_arms {
                let flatten_tree = arm.flatten();
                for (id, length) in &flatten_tree {
                    if *id == ROOT_ID {
                        all_flatten_tree.push((ROOT_ID, *length));
                    } else {
                        all_flatten_tree.push((ArmNodeID(id.0 + prefix), *length));
                    }
                }
                prefix += flatten_tree.len();
            }
            let output = Output {
                flatten_tree: all_flatten_tree,
                initial_pos,
                operations,
            };

            if best_output.is_none() {
                best_output = Some(output);
            } else {
                // let best_score = best_output.as_ref().unwrap().operations.len();
                // let cur_score = output.operations.len();
                // if cur_score < best_score {
                //     best_output = Some(output);
                // }
                let res = compute_score(&self.input, &output);
                eprintln!("score: {}, err: {}", res.0, res.1);
                if res.1.is_empty() && res.0 < best_score {
                    best_output = Some(output);
                    best_score = res.0;
                }
            }
        }

        eprintln!("iter: {}", iter);

        best_output.unwrap()
    }
}
