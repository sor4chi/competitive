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

pub struct MultiOPSolver {
    io: IO,
    input: Input,
}

impl MultiOPSolver {
    pub fn new(io: IO, input: Input) -> Self {
        MultiOPSolver { io, input }
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

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

impl Solver for MultiOPSolver {
    fn solve(&mut self) -> Output {
        let initial_pos = (0, 0);
        // sからtへの移動をOperationに変換
        let mut arm_tree = ArmTree::new((0, 0));
        for i in 1..self.input.v {
            arm_tree.add_arm(ROOT_ID, i);
        }
        let flatten_tree = arm_tree.flatten();
        let mut cur_arm_holdings = vec![false; self.input.v];
        let mut cur_s = self.input.s.clone();
        let mut cur_t = self.input.t.clone();
        // sとtが一致する場所はもうクリアしたことにする
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                if cur_s[i][j] && cur_t[i][j] {
                    cur_s[i][j] = false;
                    cur_t[i][j] = false;
                }
            }
        }
        let mut operations = vec![];
        let mut cur_pos = initial_pos;
        let mut cur_dir = Direction::Right;
        let full_stay_rotates = vec![Rotate::Stay; self.input.v - 1];
        let full_left_rotates = vec![Rotate::Left; self.input.v - 1];
        let full_right_rotates = vec![Rotate::Right; self.input.v - 1];
        let full_stay_actions = vec![Action::Stay; self.input.v];
        let mut tree_width = self.input.v;

        loop {
            let mut best_source_match_pos = None;
            let mut best_source_match_score = 0;
            let mut best_source_match_dir = None;
            // cur_sの横方向の順方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if j + k >= self.input.n {
                            break;
                        }
                        if cur_s[i][j + k] && !cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_source_match_score {
                        best_source_match_score = score;
                        best_source_match_pos = Some((i, j));
                        best_source_match_dir = Some(Direction::Right);
                    }
                }
            }
            // cur_sの縦方向の順方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if i + k >= self.input.n {
                            break;
                        }
                        if cur_s[i + k][j] && !cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_source_match_score {
                        best_source_match_score = score;
                        best_source_match_pos = Some((i, j));
                        best_source_match_dir = Some(Direction::Down);
                    }
                }
            }
            // cur_sの横方向の逆方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if (j as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_s[i][j - k] && !cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_source_match_score {
                        best_source_match_score = score;
                        best_source_match_pos = Some((i, j));
                        best_source_match_dir = Some(Direction::Left);
                    }
                }
            }
            // cur_sの縦方向の逆方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if (i as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_s[i - k][j] && !cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_source_match_score {
                        best_source_match_score = score;
                        best_source_match_pos = Some((i, j));
                        best_source_match_dir = Some(Direction::Up);
                    }
                }
            }

            if let Some(next_pos) = best_source_match_pos {
                let mut current_operations = vec![];
                let need_rotates = {
                    // 必要な回転数を計算
                    // cur_dirとbest_source_match_dirの差分を計算
                    let mut need_rotates_right = vec![];
                    let mut right_trial_dir = cur_dir;
                    // 右回転の場合
                    while right_trial_dir != best_source_match_dir.unwrap() {
                        need_rotates_right.push(Rotate::Right);
                        right_trial_dir = right_trial_dir.rotate_right();
                    }
                    let mut need_rotates_left = vec![];
                    let mut left_trial_dir = cur_dir;
                    // 左回転の場合
                    while left_trial_dir != best_source_match_dir.unwrap() {
                        need_rotates_left.push(Rotate::Left);
                        left_trial_dir = left_trial_dir.rotate_left();
                    }
                    if need_rotates_right.len() < need_rotates_left.len() {
                        need_rotates_right
                    } else {
                        need_rotates_left
                    }
                };

                // arm_treeを回転
                for rotate in &need_rotates {
                    for id in arm_tree.leaves.clone() {
                        arm_tree.rotate(id, *rotate);
                    }
                }

                let prev_pos = cur_pos;
                while cur_pos.0 != next_pos.0 {
                    if cur_pos.0 < next_pos.0 {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Down),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.0 += 1;
                    } else {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Up),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.0 -= 1;
                    }
                }
                while cur_pos.1 != next_pos.1 {
                    if cur_pos.1 < next_pos.1 {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Right),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.1 += 1;
                    } else {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Left),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.1 -= 1;
                    }
                }
                arm_tree.all_shift((
                    next_pos.0 as i32 - prev_pos.0 as i32,
                    next_pos.1 as i32 - prev_pos.1 as i32,
                ));
                let mut actions = full_stay_actions.clone();
                for id in &arm_tree.leaves {
                    let pos = arm_tree.tree_pos[id];
                    if pos.0 < 0
                        || pos.1 < 0
                        || pos.0 >= self.input.n as i32
                        || pos.1 >= self.input.n as i32
                    {
                        continue;
                    }
                    if cur_s[pos.0 as usize][pos.1 as usize] && !cur_arm_holdings[id.0] {
                        actions[id.0] = Action::PickOrRelease;
                        cur_arm_holdings[id.0] = true;
                        cur_s[pos.0 as usize][pos.1 as usize] = false;
                    }
                }

                current_operations.push(Operation {
                    move_to: Move::Stay,
                    rotates: full_stay_rotates.clone(),
                    actions,
                });

                for i in 0..need_rotates.len() {
                    if current_operations.len() > i {
                        //current_operationsのi番目の要素をneed_rotates[i]のrotateに変更
                        current_operations[i].rotates = match need_rotates[i] {
                            Rotate::Left => full_left_rotates.clone(),
                            Rotate::Right => full_right_rotates.clone(),
                            Rotate::Stay => full_stay_rotates.clone(),
                        };
                    } else {
                        //current_operationsに要素が足りない場合は追加
                        current_operations.push(Operation {
                            move_to: Move::Stay,
                            rotates: match need_rotates[i] {
                                Rotate::Left => full_left_rotates.clone(),
                                Rotate::Right => full_right_rotates.clone(),
                                Rotate::Stay => full_stay_rotates.clone(),
                            },
                            actions: full_stay_actions.clone(),
                        });
                    }
                }

                cur_pos = next_pos;
                cur_dir = best_source_match_dir.unwrap();
                operations.append(&mut current_operations);
            }

            let mut best_target_match_pos = None;
            let mut best_target_match_score = 0;
            let mut best_target_match_dir = None;
            // cur_tの横方向の順方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if j + k >= self.input.n {
                            break;
                        }
                        if cur_t[i][j + k] && cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_target_match_score {
                        best_target_match_score = score;
                        best_target_match_pos = Some((i, j));
                        best_target_match_dir = Some(Direction::Right);
                    }
                }
            }
            // cur_tの縦方向の順方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if i + k >= self.input.n {
                            break;
                        }
                        if cur_t[i + k][j] && cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_target_match_score {
                        best_target_match_score = score;
                        best_target_match_pos = Some((i, j));
                        best_target_match_dir = Some(Direction::Down);
                    }
                }
            }
            // cur_tの横方向の逆方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if (j as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_t[i][j - k] && cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_target_match_score {
                        best_target_match_score = score;
                        best_target_match_pos = Some((i, j));
                        best_target_match_dir = Some(Direction::Left);
                    }
                }
            }
            // cur_tの縦方向の逆方向にtree_width-1個の部分列で最も一致するものを探す
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut score = 0;
                    for k in 1..tree_width {
                        if (i as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_t[i - k][j] && cur_arm_holdings[k] {
                            score += 1;
                        }
                    }
                    if score > best_target_match_score {
                        best_target_match_score = score;
                        best_target_match_pos = Some((i, j));
                        best_target_match_dir = Some(Direction::Up);
                    }
                }
            }

            if let Some(next_pos) = best_target_match_pos {
                let mut current_operations = vec![];
                let need_rotates = {
                    // 必要な回転数を計算
                    // cur_dirとbest_target_match_dirの差分を計算
                    let mut need_rotates_right = vec![];
                    let mut right_trial_dir = cur_dir;
                    // 右回転の場合
                    while right_trial_dir != best_target_match_dir.unwrap() {
                        need_rotates_right.push(Rotate::Right);
                        right_trial_dir = right_trial_dir.rotate_right();
                    }
                    let mut need_rotates_left = vec![];
                    let mut left_trial_dir = cur_dir;
                    // 左回転の場合
                    while left_trial_dir != best_target_match_dir.unwrap() {
                        need_rotates_left.push(Rotate::Left);
                        left_trial_dir = left_trial_dir.rotate_left();
                    }
                    if need_rotates_right.len() < need_rotates_left.len() {
                        need_rotates_right
                    } else {
                        need_rotates_left
                    }
                };

                // arm_treeを回転
                for rotate in &need_rotates {
                    for id in arm_tree.leaves.clone() {
                        arm_tree.rotate(id, *rotate);
                    }
                }

                let prev_pos = cur_pos;
                while cur_pos.0 != next_pos.0 {
                    if cur_pos.0 < next_pos.0 {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Down),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.0 += 1;
                    } else {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Up),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.0 -= 1;
                    }
                }
                while cur_pos.1 != next_pos.1 {
                    if cur_pos.1 < next_pos.1 {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Right),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.1 += 1;
                    } else {
                        current_operations.push(Operation {
                            move_to: Move::Shift(Direction::Left),
                            rotates: full_stay_rotates.clone(),
                            actions: full_stay_actions.clone(),
                        });
                        cur_pos.1 -= 1;
                    }
                }
                arm_tree.all_shift((
                    next_pos.0 as i32 - prev_pos.0 as i32,
                    next_pos.1 as i32 - prev_pos.1 as i32,
                ));

                let mut actions = full_stay_actions.clone();
                for id in &arm_tree.leaves {
                    let pos = arm_tree.tree_pos[id];
                    if pos.0 < 0
                        || pos.1 < 0
                        || pos.0 >= self.input.n as i32
                        || pos.1 >= self.input.n as i32
                    {
                        continue;
                    }
                    if cur_t[pos.0 as usize][pos.1 as usize] && cur_arm_holdings[id.0] {
                        actions[id.0] = Action::PickOrRelease;
                        cur_arm_holdings[id.0] = false;
                        cur_t[pos.0 as usize][pos.1 as usize] = false;
                    }
                }
                current_operations.push(Operation {
                    move_to: Move::Stay,
                    rotates: full_stay_rotates.clone(),
                    actions,
                });

                for i in 0..need_rotates.len() {
                    if current_operations.len() > i {
                        //current_operationsのi番目の要素をneed_rotates[i]のrotateに変更
                        current_operations[i].rotates = match need_rotates[i] {
                            Rotate::Left => full_left_rotates.clone(),
                            Rotate::Right => full_right_rotates.clone(),
                            Rotate::Stay => full_stay_rotates.clone(),
                        };
                    } else {
                        //current_operationsに要素が足りない場合は追加
                        current_operations.push(Operation {
                            move_to: Move::Stay,
                            rotates: match need_rotates[i] {
                                Rotate::Left => full_left_rotates.clone(),
                                Rotate::Right => full_right_rotates.clone(),
                                Rotate::Stay => full_stay_rotates.clone(),
                            },
                            actions: full_stay_actions.clone(),
                        });
                    }
                }

                cur_pos = next_pos;
                cur_dir = best_target_match_dir.unwrap();
                operations.append(&mut current_operations);
            }

            if best_source_match_pos.is_none() && best_target_match_pos.is_none() {
                let mut all_t_false = true;
                for i in 0..self.input.n {
                    for j in 0..self.input.n {
                        if cur_t[i][j] {
                            all_t_false = false;
                        }
                    }
                }
                let mut all_s_false = true;
                for i in 0..self.input.n {
                    for j in 0..self.input.n {
                        if cur_s[i][j] {
                            all_s_false = false;
                        }
                    }
                }

                if !all_t_false || !all_s_false {
                    // 今持っている物を離したい
                    // 持っているものを離せる最短距離に移動
                    let mut queue = VecDeque::new();
                    let mut visited = HashSet::new();
                    // 持っているものの今の座標を計算
                    let holding_poses = {
                        let mut holding_poses = vec![];
                        for id in &arm_tree.leaves {
                            let pos = arm_tree.tree_pos[id];
                            if cur_arm_holdings[id.0] {
                                holding_poses.push(pos);
                            }
                        }
                        holding_poses
                    };
                    queue.push_back((cur_pos, holding_poses));
                    visited.insert(cur_pos);
                    let mut best_pos = None;
                    while let Some((pos, holding_poses)) = queue.pop_front() {
                        // もしholding_posesの全ての要素がcur_tでない場所にあれば終了、best_posを更新
                        let mut all_t = true;
                        for holding_pos in &holding_poses {
                            if holding_pos.0 < 0
                                || holding_pos.1 < 0
                                || holding_pos.0 >= self.input.n as i32
                                || holding_pos.1 >= self.input.n as i32
                                || cur_t[holding_pos.0 as usize][holding_pos.1 as usize]
                            {
                                all_t = false;
                            }
                        }
                        if all_t {
                            best_pos = Some(pos);
                            break;
                        }
                        for dir in &[
                            Direction::Up,
                            Direction::Down,
                            Direction::Left,
                            Direction::Right,
                        ] {
                            let d = dir.get_d();
                            let next_pos = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);
                            if next_pos.0 < 0
                                || next_pos.1 < 0
                                || next_pos.0 >= self.input.n as i32
                                || next_pos.1 >= self.input.n as i32
                            {
                                continue;
                            }
                            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                            if visited.contains(&next_pos) {
                                continue;
                            }
                            visited.insert(next_pos);
                            let mut next_holding_poses = vec![];
                            for holding_pos in &holding_poses {
                                next_holding_poses.push((holding_pos.0 + d.0, holding_pos.1 + d.1));
                            }
                            queue.push_back((next_pos, next_holding_poses));
                        }
                    }

                    if best_pos.is_none() {
                        eprintln!("[WARNING]: unexpected error");
                        break;
                    }
                    let best_pos = best_pos.unwrap();

                    // cur_posからbest_posまで移動
                    let mut current_operations = vec![];
                    let prev_pos = cur_pos;
                    while best_pos.0 != cur_pos.0 {
                        if best_pos.0 > cur_pos.0 {
                            current_operations.push(Operation {
                                move_to: Move::Shift(Direction::Down),
                                rotates: full_stay_rotates.clone(),
                                actions: full_stay_actions.clone(),
                            });
                            cur_pos.0 += 1;
                        } else {
                            current_operations.push(Operation {
                                move_to: Move::Shift(Direction::Up),
                                rotates: full_stay_rotates.clone(),
                                actions: full_stay_actions.clone(),
                            });
                            cur_pos.0 -= 1;
                        }
                    }
                    while best_pos.1 != cur_pos.1 {
                        if best_pos.1 > cur_pos.1 {
                            current_operations.push(Operation {
                                move_to: Move::Shift(Direction::Right),
                                rotates: full_stay_rotates.clone(),
                                actions: full_stay_actions.clone(),
                            });
                            cur_pos.1 += 1;
                        } else {
                            current_operations.push(Operation {
                                move_to: Move::Shift(Direction::Left),
                                rotates: full_stay_rotates.clone(),
                                actions: full_stay_actions.clone(),
                            });
                            cur_pos.1 -= 1;
                        }
                    }
                    arm_tree.all_shift((
                        best_pos.0 as i32 - prev_pos.0 as i32,
                        best_pos.1 as i32 - prev_pos.1 as i32,
                    ));
                    let mut actions = full_stay_actions.clone();
                    for id in &arm_tree.leaves {
                        let pos = arm_tree.tree_pos[id];
                        if pos.0 < 0
                            || pos.1 < 0
                            || pos.0 >= self.input.n as i32
                            || pos.1 >= self.input.n as i32
                        {
                            continue;
                        }
                        if cur_arm_holdings[id.0] {
                            actions[id.0] = Action::PickOrRelease;
                            cur_arm_holdings[id.0] = false;
                            cur_s[pos.0 as usize][pos.1 as usize] = true;
                        }
                    }

                    // leavesの中で一番長いものを削除
                    let mut max_length = 0;
                    let mut max_id = None;
                    for id in arm_tree.leaves.clone() {
                        if id.0 > max_length {
                            max_length = id.0;
                            max_id = Some(id);
                        }
                    }
                    if let Some(max_id) = max_id {
                        arm_tree.remove_arm(max_id);
                        // tree_widthを更新
                        if tree_width > 2 {
                            tree_width -= 1;
                        }
                    }

                    current_operations.push(Operation {
                        move_to: Move::Stay,
                        rotates: full_stay_rotates.clone(),
                        actions,
                    });

                    operations.append(&mut current_operations);

                    continue;
                }

                break;
            }
        }

        Output {
            flatten_tree,
            initial_pos,
            operations,
        }
    }
}
