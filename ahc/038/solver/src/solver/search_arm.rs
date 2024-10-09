use std::os::unix::process::parent_id;
use std::process::Command;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::Child,
    time::Instant,
};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::original_lib::rand_xorshift;
use crate::util::{generate_cands, tornado_travel};
use crate::{
    game::{ArmNodeID, ArmTree, Direction, ROOT_ID},
    io::{Action, Input, Move, Operation, Output, Rotate, IO},
};

use super::Solver;

pub struct SearchArmSolver {
    io: IO,
    input: Input,
}

impl SearchArmSolver {
    pub fn new(io: IO, input: Input) -> Self {
        SearchArmSolver { io, input }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone)]
struct LeavesState {
    leaves: Vec<bool>,
    // 各葉が向いている方向、座標軸ではなく親要素から見た方向なので注意
    leaves_directions: Vec<Direction>,
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

impl Solver for SearchArmSolver {
    fn solve(&mut self) -> Output {
        assert!(self.input.v >= 7);
        let seed = [0; 32];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

        let mut best_score = usize::MAX;
        let mut best_operations = vec![];
        let start = Instant::now();
        let tl = 2900;

        let initial_pos = (self.input.n / 2, self.input.n / 2);
        let arm_size = 4;
        let mut best_arm_score = 0;
        // let mut best_arm_points: Vec<Vec<Vec<(Direction, Vec<usize>)>>> =
        let mut best_arm_points: Vec<Vec<Vec<OneArmState>>> =
            vec![vec![vec![]; self.input.n]; self.input.n];
        let mut best_placable_counts: Vec<Vec<HashMap<u32, u32>>> = vec![];
        let mut best_pickable_counts: Vec<Vec<HashMap<u32, u32>>> = vec![];
        let mut best_arm_tree = ArmTree::new(initial_pos);
        let travel = tornado_travel(self.input.n);
        let max_travel_idx = travel.len();
        let mut cur_diff_pos = (0, 0);
        let mut travel_sum_diffs = vec![cur_diff_pos];
        for d in &travel {
            let (dx, dy) = d.get_d();
            cur_diff_pos.0 += dx;
            cur_diff_pos.1 += dy;
            travel_sum_diffs.push(cur_diff_pos);
        }

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

        for _ in 0..1000 {
            let mut arm_tree = ArmTree::new(initial_pos);
            let mut cur_id = ROOT_ID;
            for i in 0..arm_size {
                cur_id = arm_tree.add_arm(cur_id, rng.gen_range(1..=(self.input.n / 2)));
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

            // for i in 0..self.input.n {
            //     for j in 0..self.input.n {
            //         if arm_points[i][j] {
            //             arm_points[i][self.input.n - 1 - j] = true;
            //             arm_points[self.input.n - 1 - i][j] = true;
            //             arm_points[self.input.n - 1 - i][self.input.n - 1 - j] = true;
            //         }
            //     }
            // }

            // // 上下左右対称であることをassert
            // for i in 0..self.input.n {
            //     for j in 0..self.input.n {
            //         assert_eq!(arm_points[i][j], arm_points[self.input.n - 1 - i][j]);
            //         assert_eq!(arm_points[i][j], arm_points[i][self.input.n - 1 - j]);
            //         assert_eq!(
            //             arm_points[i][j],
            //             arm_points[self.input.n - 1 - i][self.input.n - 1 - j]
            //         );
            //     }
            // }

            let mut pickable_counts: Vec<Vec<HashMap<u32, u32>>> =
                vec![vec![HashMap::new(); self.input.n]; self.input.n];
            let mut placable_counts: Vec<Vec<HashMap<u32, u32>>> =
                vec![vec![HashMap::new(); self.input.n]; self.input.n];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    // 各点について、そのx座標or y座標が同じ場所にいくつ点があるか
                    for k in 0..self.input.n {
                        if cleaned_s[i][k] && k != j && !arm_points[i][j].is_empty() {
                            let dist = (k as i32 - j as i32).unsigned_abs();
                            // TODO: v-arm_size-1 が正しいか後で確認
                            if dist <= (self.input.v - arm_size - 1) as u32 {
                                *pickable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_s[k][j] && k != i && !arm_points[i][j].is_empty() {
                            let dist = (i as i32 - k as i32).unsigned_abs();
                            if dist <= (self.input.v - arm_size - 1) as u32 {
                                *pickable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_t[i][k] && k != j && !arm_points[i][j].is_empty() {
                            let dist = (k as i32 - j as i32).unsigned_abs();
                            if dist <= (self.input.v - arm_size - 1) as u32 {
                                *placable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                        if cleaned_t[k][j] && k != i && !arm_points[i][j].is_empty() {
                            let dist = (i as i32 - k as i32).unsigned_abs();
                            if dist <= (self.input.v - arm_size - 1) as u32 {
                                *placable_counts[i][j].entry(dist).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            // // countsの非zeroの数を数える
            // let mut score = 0;
            // for i in 0..self.input.n {
            //     for j in 0..self.input.n {
            //         for (_, count) in &pickable_counts[i][j] {
            //             if *count > 0 {
            //                 score += 1;
            //             }
            //         }
            //         for (_, count) in &placable_counts[i][j] {
            //             if *count > 0 {
            //                 score += 1;
            //             }
            //         }
            //     }
            // }

            // countsの合計を数える
            let score = pickable_counts
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|counts| counts.iter().map(|(_, count)| count).sum::<u32>())
                        .sum::<u32>()
                })
                .sum::<u32>()
                + placable_counts
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|counts| counts.iter().map(|(_, count)| count).sum::<u32>())
                            .sum::<u32>()
                    })
                    .sum::<u32>();

            if score > best_arm_score {
                best_arm_score = score;
                best_arm_points = arm_points;
                best_placable_counts = placable_counts;
                best_pickable_counts = pickable_counts;
                best_arm_tree = arm_tree;
            }
        }

        for i in 0..self.input.n {
            for j in 0..self.input.n {
                eprint!(
                    "{:2} ",
                    if !best_arm_points[i][j].is_empty() {
                        best_pickable_counts[i][j].len()
                    } else {
                        0
                    }
                );
            }
            eprintln!();
        }

        eprintln!();

        for i in 0..self.input.n {
            for j in 0..self.input.n {
                eprint!(
                    "{:2} ",
                    if !best_arm_points[i][j].is_empty() {
                        best_placable_counts[i][j].len()
                    } else {
                        0
                    }
                );
            }
            eprintln!();
        }

        eprintln!();

        for i in 0..self.input.n {
            for j in 0..self.input.n {
                eprint!(
                    "{}",
                    if !best_arm_points[i][j].is_empty() {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            eprintln!();
        }

        best_arm_tree.show_info();

        let leaves_size = self.input.v - arm_size - 1;
        let initial_leaves = vec![false; leaves_size];

        let leaves_parent_id = ArmNodeID(arm_size);
        // best_arm_treeに葉を刺す
        for i in 1..=leaves_size {
            best_arm_tree.add_arm(leaves_parent_id, i);
        }

        #[derive(Clone)]
        struct History {
            updates: usize,
            cost: usize,
            is_proceed: bool,
            arm_points_idx: usize,
            leaf_rotates: Vec<Vec<Rotate>>,
            arm_leaf_pos: (usize, usize),
        }
        #[derive(Clone)]
        struct BeamNode {
            leaves_state: LeavesState,
            arm_state: OneArmState,
            s: Vec<Vec<bool>>,
            t: Vec<Vec<bool>>,
            score: usize,
            history: Vec<History>,
            travel_idx: usize,
            turn: usize,
        }
        let initial_beam = BeamNode {
            leaves_state: LeavesState {
                leaves: initial_leaves.clone(),
                leaves_directions: vec![Direction::Up; leaves_size],
            },
            arm_state: OneArmState {
                leaf_direction: Direction::Right,
                root_direction: Direction::Right,
                sub_rotates: vec![Rotate::Stay; arm_size - 1],
            },
            s: cleaned_s.clone(),
            t: cleaned_t.clone(),
            score: 0,
            history: vec![],
            travel_idx: 0,
            turn: 0,
        };
        fn eval_node(node: &BeamNode) -> usize {
            // sとtのboolの数が少ないほど良い。turnも少ないほど良い
            let mut score = 0;
            for i in 0..node.s.len() {
                for j in 0..node.s[i].len() {
                    if node.s[i][j] {
                        score += 1;
                    }
                    if node.t[i][j] {
                        score += 1;
                    }
                }
            }
            score += node.turn;
            score
        }
        // 幅(計算量)はhave_to_move_countに依存する。
        // let beam_width = 1000 / have_to_move_count;
        // eprintln!("beam_width: {}", beam_width);
        let mut beam_width = 1;
        'beam_width_search: loop {
            eprintln!("beam_width: {}", beam_width);
            let mut arm_points_flatten = vec![];
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    for arm_state in &best_arm_points[i][j] {
                        arm_points_flatten.push(((i, j), arm_state.clone()));
                    }
                }
            }

            let mut best_history = vec![];

            let mut beams = vec![initial_beam.clone()];

            loop {
                let mut next_beams: Vec<BeamNode> = vec![];
                for beam in &beams {
                    // 次に進むか止まるかの2択
                    for is_proceed in [true, false] {
                        if start.elapsed().as_millis() > tl {
                            break 'beam_width_search;
                        }
                        // それぞれのarm_pointsについて
                        for (arm_points_idx, ((i, j), arm_state)) in
                            arm_points_flatten.iter().enumerate()
                        {
                            // このarm_pointsに合わせるのに必要な操作数を計算
                            let arm_diff = beam.arm_state.diff(arm_state);
                            if arm_diff == 0 {
                                // TODO: ここ怪しいので後で確認
                                continue;
                            }

                            let mut next_beam: BeamNode = beam.clone();
                            if is_proceed {
                                next_beam.travel_idx += 1;
                            }
                            if next_beam.travel_idx >= max_travel_idx {
                                continue;
                            }
                            // arm_stateを合わせる
                            next_beam.arm_state = arm_state.clone();
                            let arm_center_x = *i as i32 + travel_sum_diffs[next_beam.travel_idx].0;
                            let arm_center_y = *j as i32 + travel_sum_diffs[next_beam.travel_idx].1;
                            if arm_center_x < 0
                                || arm_center_y < 0
                                || arm_center_x >= self.input.n as i32
                                || arm_center_y >= self.input.n as i32
                            {
                                continue;
                            }
                            // それぞれのleafについて
                            let mut updates = 0;
                            // 現在のそれぞれの葉の向きを計算
                            // 先端の向き + 葉の向き
                            let mut leaves_directions = vec![];
                            for leaf in next_beam.leaves_state.leaves_directions.iter() {
                                leaves_directions
                                    .push(next_beam.arm_state.leaf_direction.from_relative(*leaf));
                            }
                            let mut next_leaf_rotates = vec![vec![]; leaves_size];
                            for leaf_idx in 0..leaves_size {
                                let leaf_dist = leaf_idx + 1;
                                let cands_base = vec![
                                    (
                                        (arm_center_x + leaf_dist as i32, arm_center_y),
                                        Direction::Down,
                                    ),
                                    (
                                        (arm_center_x - leaf_dist as i32, arm_center_y),
                                        Direction::Up,
                                    ),
                                    (
                                        (arm_center_x, arm_center_y + leaf_dist as i32),
                                        Direction::Right,
                                    ),
                                    (
                                        (arm_center_x, arm_center_y - leaf_dist as i32),
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
                                        if next_beam.s[*x as usize][*y as usize]
                                            && !next_beam.leaves_state.leaves[leaf_idx]
                                        {
                                            next_beam.s[*x as usize][*y as usize] = false;
                                            next_beam.leaves_state.leaves[leaf_idx] = true;
                                            next_beam.leaves_state.leaves_directions[leaf_idx] =
                                                next_beam
                                                    .arm_state
                                                    .leaf_direction
                                                    .to_relative(*dir);
                                            next_leaf_rotates[leaf_idx] = rotates.clone();
                                            updates += 1;
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
                                        if next_beam.t[*x as usize][*y as usize]
                                            && next_beam.leaves_state.leaves[leaf_idx]
                                        {
                                            next_beam.t[*x as usize][*y as usize] = false;
                                            next_beam.leaves_state.leaves[leaf_idx] = false;
                                            next_beam.leaves_state.leaves_directions[leaf_idx] =
                                                next_beam
                                                    .arm_state
                                                    .leaf_direction
                                                    .to_relative(*dir);
                                            next_leaf_rotates[leaf_idx] = rotates.clone();
                                            updates += 1;
                                            break;
                                        }
                                    }
                                }
                            }
                            // 進まずに更新がない場合はスキップ
                            if updates == 0 && !is_proceed {
                                continue;
                            }
                            next_beam.history.push(History {
                                updates,
                                cost: arm_diff,
                                is_proceed,
                                arm_points_idx,
                                leaf_rotates: next_leaf_rotates,
                                arm_leaf_pos: (arm_center_x as usize, arm_center_y as usize),
                            });
                            next_beam.turn += arm_diff;
                            next_beam.score = eval_node(&next_beam);
                            next_beams.push(next_beam);
                        }
                    }
                }

                next_beams.sort_by_key(|beam| beam.score);

                if next_beams.is_empty() {
                    break;
                }

                // next_beamのtopのスコアを出力
                eprintln!("next_beams[0].score: {}", next_beams[0].score);
                // targetが0かつturnが最小のbeamを取得
                let mut best_idx = None;
                let mut best_turn = usize::MAX;
                for (i, beam) in next_beams.iter().enumerate() {
                    if beam.t.iter().all(|row| row.iter().all(|&b| !b)) && beam.turn < best_turn {
                        best_idx = Some(i);
                        best_turn = beam.turn;
                    }
                }

                if let Some(i) = best_idx {
                    best_history = next_beams[i].history.clone();
                    break;
                }

                next_beams.truncate(beam_width);
                // // next_beams 0のbeamをデバッグ
                // eprintln!("next_beams[0].s:");
                // for row in &next_beams[0].s {
                //     for &b in row {
                //         eprint!("{}", if b { "#" } else { "." });
                //     }
                //     eprintln!();
                // }
                // eprintln!("next_beams[0].t:");
                // for row in &next_beams[0].t {
                //     for &b in row {
                //         eprint!("{}", if b { "#" } else { "." });
                //     }
                //     eprintln!();
                // }
                // eprintln!("next_beams[0].leaves_state.leaves:");
                // eprintln!("{:?}", next_beams[0].leaves_state.leaves);
                // eprintln!("next_beams[0].leaves_state.leaves_directions:");
                // eprintln!("{:?}", next_beams[0].leaves_state.leaves_directions);
                // eprintln!("next_beams[0].arm_state:");
                // eprintln!("{:?}", next_beams[0].arm_state);
                // eprintln!("next_beams[0].travel_idx:");
                // eprintln!("{:?}", next_beams[0].travel_idx);
                // eprintln!("next_beams[0].turn:");
                // eprintln!("{:?}", next_beams[0].turn);
                beams = next_beams;
            }

            eprintln!("best_history.len(): {}", best_history.len());
            let total_cost = best_history.iter().map(|h| h.cost).sum::<usize>();
            eprintln!("total_cost: {}", total_cost);

            // best_historyを辿るようにarm_treeの回転操作をDFSで求める
            let mut operations = vec![];
            let mut cur_arm_tree = best_arm_tree.clone();
            let mut cur_arm_state = OneArmState {
                leaf_direction: Direction::Right,
                root_direction: Direction::Right,
                sub_rotates: vec![Rotate::Stay; arm_size - 1],
            };
            let mut cur_travel_idx = 0;
            let mut cur_board = cleaned_s.clone();
            let mut cur_targets = cleaned_t.clone();
            let mut cur_holding = vec![false; self.input.v];
            for (
                i,
                History {
                    updates,
                    cost,
                    is_proceed,
                    arm_points_idx,
                    leaf_rotates,
                    arm_leaf_pos,
                },
            ) in best_history.iter().enumerate()
            {
                eprintln!("i: {}", i);
                eprintln!("updates: {}", updates);
                eprintln!("cost: {}", cost);
                // まずはcur_arm_stateをarm_points_flatten[arm_points_idx]に合わせる
                let next_arm_state = arm_points_flatten[*arm_points_idx].1.clone();
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
                let mut move_to = if *is_proceed {
                    cur_arm_tree.all_shift(travel[cur_travel_idx].get_d());
                    Move::Shift(travel[cur_travel_idx])
                } else {
                    Move::Stay
                };
                if *is_proceed {
                    cur_travel_idx += 1;
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

            if operations.len() < best_score {
                best_score = operations.len();
                best_operations = operations.clone();
            }

            beam_width *= 2;

            if start.elapsed().as_millis() > tl {
                break;
            }
        }

        Output {
            flatten_tree: best_arm_tree.flatten(),
            initial_pos,
            operations: best_operations,
        }
    }
}
