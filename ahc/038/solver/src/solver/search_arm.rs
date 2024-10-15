use std::collections::BinaryHeap;
use std::os::unix::process::parent_id;
use std::process::Command;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::Child,
    time::Instant,
};

use fixedbitset::FixedBitSet;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::original_lib::id::IncrementalIDGenerator;
use crate::original_lib::rand_xorshift;
use crate::tool::compute_score;
use crate::util::{generate_cands, tornado_travel};
use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
};

use super::Solver;

pub struct SearchArmSolver<'a> {
    io: IO,
    input: Input,
    start: &'a Instant,
}

impl<'a> SearchArmSolver<'a> {
    pub fn new(io: IO, input: Input, start: &'a Instant) -> Self {
        SearchArmSolver { io, input, start }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OneArmState {
    root_direction: Direction,
    leaf_direction: Direction,
    sub_rotates: Vec<Rotate>,
}

impl OneArmState {
    // このOneArmStateをotherに合わせるための操作数を返す
    fn diff(&self, other: &OneArmState) -> usize {
        let mut diff = self.root_direction.diff(other.root_direction);
        for (r1, r2) in self.sub_rotates.iter().zip(other.sub_rotates.iter()) {
            diff = diff.max(r1.diff(*r2));
        }
        diff
    }

    // 目標にあわせるための回転操作列を返す
    fn align(&self, other: &OneArmState) -> Vec<Vec<Rotate>> {
        let mut rotates = vec![];
        let root_align_rotates = self.root_direction.align(other.root_direction);
        rotates.push(root_align_rotates);
        for (me, target) in self.sub_rotates.iter().zip(other.sub_rotates.iter()) {
            let align_rotates = me.align(*target);
            rotates.push(align_rotates);
        }
        rotates
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LeavesState {
    leaves: FixedBitSet,
    // 各葉が向いている方向、座標軸ではなく親要素から見た方向なので注意
    // leaves_directions: Vec<Direction>,
    leaves_directions: FixedBitSet,
}

impl LeavesState {
    fn new(leaves_size: usize) -> Self {
        LeavesState {
            leaves: FixedBitSet::with_capacity(leaves_size),
            leaves_directions: FixedBitSet::with_capacity(leaves_size * 2), // Directionが4つなので2bitで表現できる
        }
    }

    // leaves_directionsを返す
    fn get_leaves_directions(&self) -> Vec<Direction> {
        let mut res = vec![];
        for i in 0..self.leaves.len() {
            let bits_1 = self.leaves_directions[i * 2];
            let bits_2 = self.leaves_directions[i * 2 + 1];
            let bits = (bits_1 as usize) << 1 | bits_2 as usize;
            res.push(Direction::from_idx(bits));
        }
        res
    }

    fn set_leaves_direction(&mut self, idx: usize, direction: Direction) {
        let num = direction.idx();
        self.leaves_directions.set(idx * 2, num >> 1 == 1);
        self.leaves_directions.set(idx * 2 + 1, num & 1 == 1);
    }
}

// impl LeavesState {
//     // このLeavesStateをotherに合わせるための操作数を返す
//     fn diff(&self, other: &LeavesState) -> usize {
//         let mut diff = 0;
//         for (r1, r2) in self.leaves_directions.iter().zip(other.leaves_directions.iter()) {
//             diff = diff.max(r1.diff(*r2));
//         }
//         diff
//     }

//     // 目標にあわせるための回転操作列を返す
//     fn align(&self, other: &LeavesState) -> Vec<Vec<Rotate>> {
//         let mut rotates = vec![];
//         let mut max_operation_length = 0;
//         for (me, target) in self.leaves_directions.iter().zip(other.leaves_directions.iter()) {
//             let align_rotates = me.align(*target);
//             max_operation_length = max_operation_length.max(align_rotates.len());
//             rotates.push(align_rotates);
//         }
//         for rotate in rotates.iter_mut() {
//             while rotate.len() < max_operation_length {
//                 rotate.push(Rotate::Stay);
//             }
//         }
//         rotates
//     }
// }

impl Solver for SearchArmSolver<'_> {
    fn solve(&mut self) -> Output {
        let seed = [0; 32];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

        let mut best_score = usize::MAX;
        let mut best_operations = vec![];
        let tl = self.input.tl;

        let central_pos = (self.input.n / 2, self.input.n / 2);
        let arm_size = if self.input.v >= 7 { 4 } else { 3 };
        let mut best_arm_score = 0;
        let mut best_arm_points: Vec<Vec<Vec<OneArmState>>> =
            vec![vec![vec![]; self.input.n]; self.input.n];
        let mut best_placable_counts: Vec<Vec<HashMap<usize, usize>>> = vec![];
        let mut best_pickable_counts: Vec<Vec<HashMap<usize, usize>>> = vec![];
        let mut best_arm_tree = ArmTree::new(central_pos);
        let mut best_initial_pos = central_pos;

        let leaves_size = self.input.v - arm_size - 1;

        let mut cleaned_s = self.input.s.clone();
        let mut cleaned_t = self.input.t.clone();
        let mut have_to_move_count = 0;
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                if cleaned_s[i][j] && cleaned_t[i][j] {
                    cleaned_s[i][j] = false;
                    cleaned_t[i][j] = false;
                }
                if cleaned_s[i][j] && !cleaned_t[i][j] {
                    have_to_move_count += 1;
                }
            }
        }

        let mut leaf_dists = vec![];

        let leaves_size = self.input.v - arm_size - 1;
        let initial_leaves = vec![false; leaves_size];

        // 葉を伸ばす距離を計算
        if self.input.v < 7 {
            for i in 1..=leaves_size {
                leaf_dists.push(self.input.n / (1 << i));
            }
        } else {
            for i in 1..=leaves_size {
                leaf_dists.push(i);
            }
        }

        let max_leaf_dist = leaf_dists.iter().max().unwrap();

        let initial_arm_variant_size = 10;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        struct ArmId(usize);
        impl From<usize> for ArmId {
            fn from(id: usize) -> Self {
                ArmId(id)
            }
        }
        let mut arm_idg = IncrementalIDGenerator::<ArmId>::new();
        let mut all_arm_points: HashMap<ArmId, Vec<Vec<Vec<OneArmState>>>> = HashMap::new();
        let mut all_arm_trees: HashMap<ArmId, ArmTree> = HashMap::new();

        let mut arm_candidates = vec![];

        for _ in 0..1000 {
            let mut arm_tree = ArmTree::new(central_pos);
            let mut cur_id = ROOT_ID;
            let mut perm = (1..self.input.n - 1).collect::<Vec<_>>();
            perm.shuffle(&mut rng);
            for _ in 0..arm_size {
                cur_id = arm_tree.add_arm(cur_id, perm.pop().unwrap());
            }

            // armが行ける点とその行き方(アームの曲げ方)
            let mut arm_points: Vec<Vec<Vec<OneArmState>>> =
                vec![vec![vec![]; self.input.n]; self.input.n];
            let cands = generate_cands(arm_size - 1, 3);
            for d in 0..4 {
                let mut directed_arm_tree = arm_tree.clone();
                // 中心から4方向に曲げる
                for _ in 0..d {
                    directed_arm_tree.rotate(ArmNodeID(1), Rotate::Right);
                }
                let direction = match d {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => unreachable!(),
                };

                for rotates in cands.iter() {
                    let mut new_arm_tree = directed_arm_tree.clone();
                    let mut new_rotates = vec![];
                    for i in 0..arm_size - 1 {
                        let rotate = match rotates[i] {
                            0 => Rotate::Stay,
                            1 => Rotate::Right,
                            2 => Rotate::Left,
                            _ => unreachable!(),
                        };
                        new_rotates.push(rotate);
                        if rotate == Rotate::Stay {
                            continue;
                        }
                        new_arm_tree.rotate(ArmNodeID(i + 2), rotate);
                    }
                    // この時点で葉がどこを向いているかを確認
                    let leaf_direction = new_arm_tree.direction(ArmNodeID(arm_size));
                    for leaf_id in new_arm_tree.leaves {
                        let (x, y) = new_arm_tree.tree_pos[&leaf_id];
                        if x < 0 || y < 0 || x >= self.input.n as i32 || y >= self.input.n as i32 {
                            continue;
                        }
                        arm_points[x as usize][y as usize].push(OneArmState {
                            root_direction: direction,
                            leaf_direction,
                            sub_rotates: new_rotates.clone(),
                        });
                    }
                }
            }

            let mut pickable_counts: Vec<Vec<HashMap<usize, usize>>> =
                vec![vec![HashMap::new(); self.input.n]; self.input.n];
            let mut placable_counts: Vec<Vec<HashMap<usize, usize>>> =
                vec![vec![HashMap::new(); self.input.n]; self.input.n];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    // 各点について、そのx座標or y座標が同じ場所にいくつ点があるか
                    for k in 0..self.input.n {
                        if cleaned_s[i][k] && k != j && !arm_points[i][j].is_empty() {
                            let dist = (k as i32 - j as i32).unsigned_abs() as usize;
                            // TODO: v-arm_size-1 が正しいか後で確認
                            if dist <= *max_leaf_dist {
                                *pickable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_s[k][j] && k != i && !arm_points[i][j].is_empty() {
                            let dist = (i as i32 - k as i32).unsigned_abs() as usize;
                            if dist <= *max_leaf_dist {
                                *pickable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_t[i][k] && k != j && !arm_points[i][j].is_empty() {
                            let dist = (k as i32 - j as i32).unsigned_abs() as usize;
                            if dist <= *max_leaf_dist {
                                *placable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_t[k][j] && k != i && !arm_points[i][j].is_empty() {
                            let dist = (i as i32 - k as i32).unsigned_abs() as usize;
                            if dist <= *max_leaf_dist {
                                *placable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            // countsの非zeroの数を数える
            let mut score = 0;
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    for (_, count) in &pickable_counts[i][j] {
                        if *count > 0 {
                            score += 1;
                        }
                    }
                    for (_, count) in &placable_counts[i][j] {
                        if *count > 0 {
                            score += 1;
                        }
                    }
                }
            }

            arm_candidates.push((score, arm_points, arm_tree));
        }

        arm_candidates.sort_by_key(|(score, _arm_points, _arm_tree)| *score);
        arm_candidates.reverse();
        arm_candidates.truncate(initial_arm_variant_size);

        for (_, arm_points, arm_tree) in &arm_candidates {
            let arm_id = arm_idg.generate();
            all_arm_points.insert(arm_id, arm_points.clone());
            all_arm_trees.insert(arm_id, arm_tree.clone());
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        struct HistoryId(usize);
        impl From<usize> for HistoryId {
            fn from(id: usize) -> Self {
                HistoryId(id)
            }
        }
        let mut history_idg = IncrementalIDGenerator::<HistoryId>::new();

        #[derive(Clone, PartialEq, Eq)]
        struct History {
            updates: usize,
            cost: usize,
            move_to: Move,
            arm_points_idx: usize,
            leaf_rotates: Vec<Vec<Rotate>>,
            parent_id: HistoryId,
        }
        #[derive(Clone, PartialEq, Eq)]
        struct BeamNode {
            leaves_state: LeavesState,
            arm_state: OneArmState,
            s: FixedBitSet,
            t: FixedBitSet,
            visited: FixedBitSet,
            score: usize,
            id: HistoryId,
            diff_pos: (i32, i32),
            initial_pos: (usize, usize),
            turn: usize,
            left_s: usize,
            left_t: usize,
            arm_id: ArmId,
        }
        let initial_id = history_idg.generate();
        // let flattened_s = cleaned_s.iter().flatten().copied().collect::<Vec<bool>>();
        // let flattened_t = cleaned_t.iter().flatten().copied().collect::<Vec<bool>>();
        let mut initial_s = FixedBitSet::with_capacity(self.input.n * self.input.n);
        let mut initial_t = FixedBitSet::with_capacity(self.input.n * self.input.n);
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                initial_s.set(i * self.input.n + j, cleaned_s[i][j]);
                initial_t.set(i * self.input.n + j, cleaned_t[i][j]);
            }
        }
        // let mut initial_visited = FixedBitSet::with_capacity(self.input.n * self.input.n);
        // initial_visited.set(initial_pos.0 * self.input.n + initial_pos.1, true);
        // let initial_beam = BeamNode {
        //     leaves_state: LeavesState {
        //         leaves: initial_leaves,
        //         leaves_directions: vec![Direction::Up; leaves_size],
        //     },
        //     arm_state: OneArmState {
        //         leaf_direction: Direction::Right,
        //         root_direction: Direction::Right,
        //         sub_rotates: vec![Rotate::Stay; arm_size - 1],
        //     },
        //     s: initial_s.clone(),
        //     t: initial_t.clone(),
        //     visited: initial_visited.clone(),
        //     score: 0,
        //     id: initial_id,
        //     diff_pos: (0, 0),
        //     turn: 0,
        //     left: have_to_move_count,
        // };

        // initial_diff_posが(n/4,n/4) ~ (3n/4,3n/4)の9マスになるようにする
        let mut initial_beams = vec![];
        let splits = 3;
        for i in 1..=splits {
            for j in 1..=splits {
                let real_pos = (
                    (i * self.input.n / (splits + 1)),
                    (j * self.input.n / (splits + 1)),
                );
                let mut initial_visited = FixedBitSet::with_capacity(self.input.n * self.input.n);
                initial_visited.set(
                    real_pos.0 as usize * self.input.n + real_pos.1 as usize,
                    true,
                );
                let initial_beam = BeamNode {
                    leaves_state: LeavesState::new(leaves_size),
                    arm_state: OneArmState {
                        leaf_direction: Direction::Right,
                        root_direction: Direction::Right,
                        sub_rotates: vec![Rotate::Stay; arm_size - 1],
                    },
                    s: initial_s.clone(),
                    t: initial_t.clone(),
                    visited: initial_visited.clone(),
                    score: 0,
                    id: initial_id,
                    diff_pos: (
                        real_pos.0 as i32 - central_pos.0 as i32,
                        real_pos.1 as i32 - central_pos.1 as i32,
                    ),
                    turn: 0,
                    left_s: have_to_move_count,
                    left_t: have_to_move_count,
                    initial_pos: real_pos,
                    arm_id: ArmId(usize::MAX),
                };
                for (arm_id, arm_points) in &all_arm_points {
                    let mut initial_beam = initial_beam.clone();
                    initial_beam.arm_id = *arm_id;
                    initial_beams.push(initial_beam);
                }
            }
        }
        let mut history_cache = HashMap::new();
        fn eval_node(node: &BeamNode, n: usize) -> usize {
            // sとtのboolの数が少ないほど良い。turnも少ないほど良い
            let mut score = node.left_s + node.left_t;
            score += node.turn.pow(2);
            score
        }
        // 幅(計算量)はhave_to_move_countに依存する。
        // let beam_width = 1000 / have_to_move_count;
        // eprintln!("beam_width: {}", beam_width);

        let mut all_arm_points_flatten: HashMap<ArmId, Vec<((usize, usize), OneArmState)>> =
            HashMap::new();
        for (arm_id, arm_points) in &all_arm_points {
            let mut arm_points_flatten = vec![];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    for arm_state in &arm_points[i][j] {
                        arm_points_flatten.push(((i, j), arm_state.clone()));
                    }
                }
            }
            all_arm_points_flatten.insert(*arm_id, arm_points_flatten);
        }

        let mut beam_width = 4;
        'beam_width_search: loop {
            eprintln!("beam_width: {}", beam_width);
            let mut beams = initial_beams.clone();
            let mut best_beam = None;
            let mut best_turn = usize::MAX;
            let mut last_update = None;

            let mut iter = 0;
            #[derive(Eq, PartialEq)]
            struct NextState(BeamNode, History);
            impl Ord for NextState {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.0.score.cmp(&other.0.score)
                }
            }
            impl PartialOrd for NextState {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            loop {
                iter += 1;
                // let mut next_beams: Vec<NextState> = vec![];
                let mut next_beams = BinaryHeap::new();

                for beam in &beams {
                    // それぞれのarm_pointsについて
                    for (arm_points_idx, ((i, j), arm_state)) in
                        all_arm_points_flatten[&beam.arm_id].iter().enumerate()
                    {
                        // 次に進むか止まるかの2択
                        // for is_proceed in [true, false] {
                        // for is_proceed in [false, true] {
                        for move_to in [
                            Move::Shift(Direction::Right),
                            Move::Shift(Direction::Left),
                            Move::Shift(Direction::Down),
                            Move::Shift(Direction::Up),
                            Move::Stay,
                        ] {
                            if self.start.elapsed().as_millis() > tl {
                                break 'beam_width_search;
                            }
                            // このarm_pointsに合わせるのに必要な操作数を計算
                            let arm_diff = beam.arm_state.diff(arm_state);
                            if arm_diff == 0 {
                                // TODO: ここ怪しいので後で確認
                                continue;
                            }

                            if arm_diff == 2 {
                                continue;
                            }

                            let mut next_beam: BeamNode = beam.clone();
                            let diff_pos = if let Move::Shift(dir) = move_to {
                                dir.get_d()
                            } else {
                                (0, 0)
                            };
                            let next_diff_pos =
                                (beam.diff_pos.0 + diff_pos.0, beam.diff_pos.1 + diff_pos.1);
                            let real_center = (
                                central_pos.0 as i32 + next_diff_pos.0,
                                central_pos.1 as i32 + next_diff_pos.1,
                            );
                            if real_center.0 < 0
                                || real_center.1 < 0
                                || real_center.0 >= self.input.n as i32
                                || real_center.1 >= self.input.n as i32
                            {
                                continue;
                            }
                            // arm_stateを合わせる
                            next_beam.arm_state = arm_state.clone();
                            let arm_center_x = *i as i32 + next_diff_pos.0;
                            let arm_center_y = *j as i32 + next_diff_pos.1;
                            if arm_center_x < 0
                                || arm_center_y < 0
                                || arm_center_x >= self.input.n as i32
                                || arm_center_y >= self.input.n as i32
                            {
                                continue;
                            }
                            if move_to != Move::Stay
                                && next_beam.visited
                                    [arm_center_x as usize * self.input.n + arm_center_y as usize]
                            {
                                continue;
                            }
                            next_beam.diff_pos = next_diff_pos;
                            // それぞれのleafについて
                            let mut updates = 0;
                            // 現在のそれぞれの葉の向きを計算
                            // 先端の向き + 葉の向き
                            let mut leaves_directions = vec![];
                            for leaf in next_beam.leaves_state.get_leaves_directions().iter() {
                                leaves_directions
                                    .push(next_beam.arm_state.leaf_direction.from_relative(*leaf));
                            }
                            let mut next_leaf_rotates = vec![vec![]; leaves_size];
                            for (leaf_idx, leaf_dist) in leaf_dists.iter().enumerate() {
                                let cands_base = vec![
                                    (
                                        (arm_center_x + *leaf_dist as i32, arm_center_y),
                                        Direction::Down,
                                    ),
                                    (
                                        (arm_center_x - *leaf_dist as i32, arm_center_y),
                                        Direction::Up,
                                    ),
                                    (
                                        (arm_center_x, arm_center_y + *leaf_dist as i32),
                                        Direction::Right,
                                    ),
                                    (
                                        (arm_center_x, arm_center_y - *leaf_dist as i32),
                                        Direction::Left,
                                    ),
                                ];
                                // leaves_directions[leaf_idx]とcandsの差に着目
                                // もしarm_diffが1以下なら、leaves_directions[leaf_idx]とcandsの差が1以下であるものだけを残す
                                // arm_diffが2以上ならどの方向も指せる
                                let mut cands = vec![];
                                if arm_diff == 1 {
                                    for (pos, dir) in &cands_base {
                                        if leaves_directions[leaf_idx].diff(*dir) <= arm_diff {
                                            cands.push((
                                                *pos,
                                                *dir,
                                                leaves_directions[leaf_idx].align(*dir),
                                            ));
                                        }
                                    }
                                } else {
                                    for (pos, dir) in &cands_base {
                                        cands.push((
                                            *pos,
                                            *dir,
                                            leaves_directions[leaf_idx].align(*dir),
                                        ));
                                    }
                                }

                                // もしleaves[leaf_idx]がfalseなら
                                // (i, j)からleaf_distだけはなれた場所にある点を1つsから選びleaves[leaf_idx]をtrueにする
                                if !next_beam.leaves_state.leaves[leaf_idx] {
                                    for ((x, y), dir, rotates) in &cands {
                                        if *x < 0
                                            || *y < 0
                                            || *x >= self.input.n as i32
                                            || *y >= self.input.n as i32
                                        {
                                            continue;
                                        }
                                        if next_beam.s[(*x as usize) * self.input.n + *y as usize]
                                            && !next_beam.leaves_state.leaves[leaf_idx]
                                        {
                                            next_beam.s.set(
                                                (*x as usize) * self.input.n + *y as usize,
                                                false,
                                            );
                                            next_beam.leaves_state.leaves.set(leaf_idx, true);
                                            next_beam.leaves_state.set_leaves_direction(
                                                leaf_idx,
                                                next_beam
                                                    .arm_state
                                                    .leaf_direction
                                                    .to_relative(*dir),
                                            );
                                            next_leaf_rotates[leaf_idx] = rotates.clone();
                                            updates += 1;
                                            next_beam.left_s -= 1;
                                            break;
                                        }
                                    }
                                }
                                // もしleaves[leaf_idx]がtrueなら
                                // (i, j)からleaf_distだけはなれた場所にある点を1つtから選びleaves[leaf_idx]をfalseにする
                                else {
                                    for ((x, y), dir, rotates) in &cands {
                                        if *x < 0
                                            || *y < 0
                                            || *x >= self.input.n as i32
                                            || *y >= self.input.n as i32
                                        {
                                            continue;
                                        }
                                        if next_beam.t[(*x as usize) * self.input.n + *y as usize]
                                            && next_beam.leaves_state.leaves[leaf_idx]
                                        {
                                            next_beam.t.set(
                                                (*x as usize) * self.input.n + *y as usize,
                                                false,
                                            );
                                            next_beam.leaves_state.leaves.set(leaf_idx, false);
                                            next_beam.leaves_state.set_leaves_direction(
                                                leaf_idx,
                                                next_beam
                                                    .arm_state
                                                    .leaf_direction
                                                    .to_relative(*dir),
                                            );
                                            next_leaf_rotates[leaf_idx] = rotates.clone();
                                            updates += 1;
                                            next_beam.left_t -= 1;
                                            break;
                                        }
                                    }
                                }
                            }
                            // 進まずに更新がない場合はスキップ
                            if updates == 0 && move_to == Move::Stay {
                                continue;
                            }
                            next_beam.turn += arm_diff;
                            next_beam.score = eval_node(&next_beam, self.input.n);
                            next_beam.visited.set(
                                arm_center_x as usize * self.input.n + arm_center_y as usize,
                                true,
                            );
                            next_beams.push(NextState(
                                next_beam,
                                History {
                                    updates,
                                    cost: arm_diff,
                                    move_to,
                                    arm_points_idx,
                                    leaf_rotates: next_leaf_rotates,
                                    parent_id: beam.id,
                                },
                            ));
                            if next_beams.len() > beam_width {
                                next_beams.pop();
                            }
                        }
                    }
                }

                // next_beams.sort_by_key(|NextState(beam, _)| beam.score);

                if next_beams.is_empty() {
                    break;
                }

                // next_beamのtopのスコアを出力
                // eprintln!("next_beams[0].score: {}", next_beams[0].0.score);

                // next_beams.truncate(beam_width);
                beams.clear();
                for NextState(mut next_beam, history) in next_beams {
                    let id = history_idg.generate();
                    history_cache.insert(id, history);
                    next_beam.id = id;
                    if next_beam.turn < best_turn && next_beam.left_t == 0 {
                        last_update = Some(iter);
                        best_beam = Some(next_beam.clone());
                        best_turn = next_beam.turn;
                    }
                    beams.push(next_beam);
                }

                if let Some(last_update) = last_update {
                    if iter - last_update >= 0 {
                        break;
                    }
                }
            }

            if best_beam.is_none() {
                beam_width *= 2;
                continue;
            }
            let best_beam = best_beam.unwrap();

            let mut best_history = vec![];
            let mut cur_id = best_beam.id;
            while let Some(history) = history_cache.get(&cur_id) {
                best_history.push(history.clone());
                cur_id = history.parent_id;
            }
            best_history.reverse();
            eprintln!("best_history.len(): {}", best_history.len());
            let total_cost = best_history.iter().map(|h| h.cost).sum::<usize>();
            eprintln!("total_cost: {}", total_cost);
            eprintln!("best_arm_id: {:?}", best_beam.arm_id);
            let best_arm_id = best_beam.arm_id;

            let mut arm_tree = all_arm_trees[&best_arm_id].clone();
            let leaves_parent_id = ArmNodeID(arm_size);

            for &leaf in leaf_dists.iter() {
                arm_tree.add_arm(leaves_parent_id, leaf);
            }

            arm_tree.show_info();

            // best_historyを辿るようにarm_treeの回転操作をDFSで求める
            let mut operations = vec![];
            let mut cur_arm_tree = arm_tree.clone();
            cur_arm_tree.all_shift((
                best_beam.initial_pos.0 as i32 - central_pos.0 as i32,
                best_beam.initial_pos.1 as i32 - central_pos.1 as i32,
            ));
            let mut cur_arm_state = OneArmState {
                leaf_direction: Direction::Right,
                root_direction: Direction::Right,
                sub_rotates: vec![Rotate::Stay; arm_size - 1],
            };
            let mut cur_board = cleaned_s.clone();
            let mut cur_targets = cleaned_t.clone();
            let mut cur_holding = vec![false; self.input.v];
            let mut cur_pos = best_beam.initial_pos;
            for (
                i,
                History {
                    updates: _,
                    cost: _,
                    move_to,
                    arm_points_idx,
                    leaf_rotates,
                    parent_id: _,
                },
            ) in best_history.iter().enumerate()
            {
                // まずはcur_arm_stateをarm_points_flatten[arm_points_idx]に合わせる
                let next_arm_state = all_arm_points_flatten[&best_beam.arm_id][*arm_points_idx]
                    .1
                    .clone();
                let mut arm_rotates = cur_arm_state.align(&next_arm_state);
                let mut leaf_rotates = leaf_rotates.clone();
                // rotatesの最大数を取得
                let mut max_rotates = 0;
                for arm_rotate in arm_rotates.iter() {
                    max_rotates = max_rotates.max(arm_rotate.len());
                }
                for leaf_rotate in leaf_rotates.iter() {
                    max_rotates = max_rotates.max(leaf_rotate.len());
                }
                // 不足分をStayでpad
                for arm_rotate in arm_rotates.iter_mut() {
                    while arm_rotate.len() < max_rotates {
                        arm_rotate.push(Rotate::Stay);
                    }
                }
                for leaf_rotate in leaf_rotates.iter_mut() {
                    while leaf_rotate.len() < max_rotates {
                        leaf_rotate.push(Rotate::Stay);
                    }
                }
                // armとleafのrotateをガッチャンコする
                let mut all_rotates: Vec<Vec<Rotate>> = vec![];
                for i in 0..max_rotates {
                    let mut rotates = vec![];
                    for arm_rotate in arm_rotates.iter() {
                        rotates.push(arm_rotate[i]);
                    }
                    for leaf_rotate in leaf_rotates.iter() {
                        rotates.push(leaf_rotate[i]);
                    }
                    all_rotates.push(rotates);
                }
                let mut move_to = *move_to;
                if let Move::Shift(dir) = move_to {
                    cur_arm_tree.all_shift(dir.get_d());
                    cur_pos.0 = (cur_pos.0 as i32 + dir.get_d().0) as usize;
                    cur_pos.1 = (cur_pos.1 as i32 + dir.get_d().1) as usize;
                }

                for (i, rotates) in all_rotates.iter().enumerate() {
                    if i == all_rotates.len() - 1 {
                        for i in 0..rotates.len() {
                            let rotate = rotates[i];
                            if rotate != Rotate::Stay {
                                cur_arm_tree.rotate(ArmNodeID(i + 1), rotate);
                            }
                        }
                        let mut actions = vec![Action::Stay; self.input.v];
                        for leaf_id in &cur_arm_tree.leaves {
                            let (x, y) = cur_arm_tree.tree_pos[leaf_id];
                            if x < 0
                                || y < 0
                                || x >= self.input.n as i32
                                || y >= self.input.n as i32
                            {
                                continue;
                            }
                            if cur_holding[leaf_id.0] && cur_targets[x as usize][y as usize] {
                                actions[leaf_id.0] = Action::PickOrRelease;
                                cur_holding[leaf_id.0] = false;
                                cur_targets[x as usize][y as usize] = false;
                                continue;
                            }
                            if !cur_holding[leaf_id.0] && cur_board[x as usize][y as usize] {
                                actions[leaf_id.0] = Action::PickOrRelease;
                                cur_holding[leaf_id.0] = true;
                                cur_board[x as usize][y as usize] = false;
                                continue;
                            }
                        }
                        operations.push(Operation {
                            move_to,
                            rotates: rotates.clone(),
                            actions,
                        });
                        move_to = Move::Stay;
                    } else {
                        for i in 0..rotates.len() {
                            let rotate = rotates[i];
                            if rotate != Rotate::Stay {
                                cur_arm_tree.rotate(ArmNodeID(i + 1), rotate);
                            }
                        }
                        operations.push(Operation {
                            move_to,
                            rotates: rotates.clone(),
                            actions: vec![Action::Stay; self.input.v],
                        });
                        move_to = Move::Stay;
                    }
                }

                cur_arm_state = next_arm_state;
            }
            let score = operations.len();
            if score < best_score {
                best_score = score;
                best_operations = operations.clone();
                best_initial_pos = best_beam.initial_pos;
                best_arm_tree = cur_arm_tree.clone();
            }

            beam_width *= 2;

            if self.start.elapsed().as_millis() > tl {
                break;
            }
        }

        Output {
            flatten_tree: best_arm_tree.flatten(),
            initial_pos: best_initial_pos,
            operations: best_operations,
        }
    }
}
