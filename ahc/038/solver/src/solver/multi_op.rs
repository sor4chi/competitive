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
        let mut arm_tree = ArmTree::new((0, 0));
        for i in 1..self.input.v {
            arm_tree.add_arm(ROOT_ID, i);
        }
        let flatten_tree = arm_tree.flatten();
        let mut cur_arm_holdings = vec![false; self.input.v];
        let mut cur_s = self.input.s.clone();
        let mut cur_t = self.input.t.clone();
        let mut cur_filled = vec![vec![false; self.input.n]; self.input.n];
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                if cur_s[i][j] {
                    cur_filled[i][j] = true;
                }
            }
        }
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
            let mut source_match_poses = vec![];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut fillable_count = 0;
                    for k in 1..tree_width {
                        if j + k >= self.input.n {
                            break;
                        }
                        if cur_s[i][j + k] && !cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        source_match_poses.push((i, j, Direction::Right, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if i + k >= self.input.n {
                            break;
                        }
                        if cur_s[i + k][j] && !cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        source_match_poses.push((i, j, Direction::Down, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if (j as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_s[i][j - k] && !cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        source_match_poses.push((i, j, Direction::Left, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if (i as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_s[i - k][j] && !cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        source_match_poses.push((i, j, Direction::Up, fillable_count));
                    }
                }
            }
            source_match_poses.sort_by_key(|(i, j, dir, fillable_count)| {
                let dx = *i as i32 - cur_pos.0 as i32;
                let dy = *j as i32 - cur_pos.1 as i32;
                let dist = dx.unsigned_abs() + dy.unsigned_abs();
                (-(*fillable_count), dist)
            });
            let best_source_match_pos = source_match_poses.first().map(|(i, j, dir, _)| (*i, *j));
            let best_source_match_dir = source_match_poses.first().map(|(_, _, dir, _)| *dir);
            if let Some(next_pos) = best_source_match_pos {
                let mut current_operations = vec![];
                let need_rotates = {
                    let mut need_rotates_right = vec![];
                    let mut right_trial_dir = cur_dir;
                    while right_trial_dir != best_source_match_dir.unwrap() {
                        need_rotates_right.push(Rotate::Right);
                        right_trial_dir = right_trial_dir.rotate_right();
                    }
                    let mut need_rotates_left = vec![];
                    let mut left_trial_dir = cur_dir;
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
                        cur_filled[pos.0 as usize][pos.1 as usize] = false;
                    }
                }
                for i in 0..need_rotates.len() {
                    if current_operations.len() > i {
                        current_operations[i].rotates = match need_rotates[i] {
                            Rotate::Left => full_left_rotates.clone(),
                            Rotate::Right => full_right_rotates.clone(),
                            Rotate::Stay => full_stay_rotates.clone(),
                        };
                    } else {
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
                if !current_operations.is_empty() {
                    current_operations.last_mut().unwrap().actions = actions;
                } else {
                    current_operations.push(Operation {
                        move_to: Move::Stay,
                        rotates: full_stay_rotates.clone(),
                        actions,
                    });
                }
                cur_pos = next_pos;
                cur_dir = best_source_match_dir.unwrap();
                operations.append(&mut current_operations);
            }
            let mut target_match_poses = vec![];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    let mut fillable_count = 0;
                    for k in 1..tree_width {
                        if j + k >= self.input.n {
                            break;
                        }
                        if cur_t[i][j + k] && cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        target_match_poses.push((i, j, Direction::Right, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if i + k >= self.input.n {
                            break;
                        }
                        if cur_t[i + k][j] && cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        target_match_poses.push((i, j, Direction::Down, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if (j as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_t[i][j - k] && cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        target_match_poses.push((i, j, Direction::Left, fillable_count));
                    }
                    fillable_count = 0;
                    for k in 1..tree_width {
                        if (i as i32 - k as i32) < 0 {
                            break;
                        }
                        if cur_t[i - k][j] && cur_arm_holdings[k] {
                            fillable_count += 1;
                        }
                    }
                    if fillable_count > 0 {
                        target_match_poses.push((i, j, Direction::Up, fillable_count));
                    }
                }
            }
            target_match_poses.sort_by_key(|(i, j, dir, fillable_count)| {
                let dx = *i as i32 - cur_pos.0 as i32;
                let dy = *j as i32 - cur_pos.1 as i32;
                let dist = dx.unsigned_abs() + dy.unsigned_abs();
                (-(*fillable_count), dist)
            });
            let best_target_match_pos = target_match_poses.first().map(|(i, j, dir, _)| (*i, *j));
            let best_target_match_dir = target_match_poses.first().map(|(_, _, dir, _)| *dir);
            if let Some(next_pos) = best_target_match_pos {
                let mut current_operations = vec![];
                let need_rotates = {
                    let mut need_rotates_right = vec![];
                    let mut right_trial_dir = cur_dir;
                    while right_trial_dir != best_target_match_dir.unwrap() {
                        need_rotates_right.push(Rotate::Right);
                        right_trial_dir = right_trial_dir.rotate_right();
                    }
                    let mut need_rotates_left = vec![];
                    let mut left_trial_dir = cur_dir;
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
                        cur_filled[pos.0 as usize][pos.1 as usize] = true;
                    }
                }
                for i in 0..need_rotates.len() {
                    if current_operations.len() > i {
                        current_operations[i].rotates = match need_rotates[i] {
                            Rotate::Left => full_left_rotates.clone(),
                            Rotate::Right => full_right_rotates.clone(),
                            Rotate::Stay => full_stay_rotates.clone(),
                        };
                    } else {
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
                if !current_operations.is_empty() {
                    current_operations.last_mut().unwrap().actions = actions;
                } else {
                    current_operations.push(Operation {
                        move_to: Move::Stay,
                        rotates: full_stay_rotates.clone(),
                        actions,
                    });
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
                    let mut queue = VecDeque::new();
                    let mut visited = HashSet::new();
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
                        let mut is_ok = true;
                        for holding_pos in &holding_poses {
                            if holding_pos.0 < 0
                                || holding_pos.1 < 0
                                || holding_pos.0 >= self.input.n as i32
                                || holding_pos.1 >= self.input.n as i32
                            {
                                is_ok = false;
                                break;
                            }
                            if cur_filled[holding_pos.0 as usize][holding_pos.1 as usize] {
                                is_ok = false;
                                break;
                            }
                        }
                        if is_ok {
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
                            cur_filled[pos.0 as usize][pos.1 as usize] = true;
                        }
                    }
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
