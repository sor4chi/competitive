use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    f32::consts::E,
    time::Instant,
    vec,
};

use itertools::Itertools;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{
    board::Board,
    io::{Input, Operation, Output, IO},
};

use super::Solver;

pub struct ConstructSolver {
    io: IO,
    input: Input,
}

impl ConstructSolver {
    pub fn new(io: IO, input: Input) -> Self {
        ConstructSolver { io, input }
    }
}
const CONNECTION_WIDTH: usize = 3;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Eq, PartialEq, Clone)]
struct BeamNode {
    board: Board,
    connect: Vec<(usize, usize)>,
    last_color: usize,
    score: usize,
}

impl Ord for BeamNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for BeamNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solver for ConstructSolver {
    fn solve(&mut self) -> Output {
        let start_all = Instant::now();
        let initial_board = Board::new(self.input.h, self.input.w);

        // cand_shapeを作る。(0,0)を基準として長さ3の形状をすべて
        let mut cand_shapes = vec![];
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        let start = (0, 0);
        visited.insert(start);
        q.push_back((vec![start], visited));
        while let Some((shape, visited)) = q.pop_front() {
            if shape.len() == CONNECTION_WIDTH {
                cand_shapes.push(shape);
                continue;
            }
            for dir in &DIRS {
                let next = (
                    shape.last().unwrap().0 + dir.0,
                    shape.last().unwrap().1 + dir.1,
                );
                if visited.contains(&next) {
                    continue;
                }
                let mut next_visited = visited.clone();
                next_visited.insert(next);
                let mut next_shape = shape.clone();
                next_shape.push(next);
                q.push_back((next_shape, next_visited));
            }
        }

        let mut initial_beams = vec![];
        let mut initial_visited = HashSet::new();
        for column in 0..self.input.w {
            for color in 1..=4 {
                for cand_shape in cand_shapes.iter() {
                    let mut new_board = initial_board.clone();
                    let mut last_connection = vec![];
                    // placable_check
                    let mut ok = true;
                    for i in 0..CONNECTION_WIDTH {
                        let (r, c) = cand_shape[i];
                        let col = column as i32 + c;
                        if r < 0
                            || col < 0
                            || r >= self.input.h as i32
                            || col >= self.input.w as i32
                        {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    // apply
                    for i in 0..CONNECTION_WIDTH {
                        let (r, c) = cand_shape[i];
                        let col = column as i32 + c;
                        new_board.place(r as usize, col as usize, color);
                        last_connection.push((r as usize, column + c as usize));
                    }
                    let new_score = {
                        let mut board = new_board.clone();
                        board.organize()
                    };
                    if initial_visited.contains(&new_board.hash) {
                        continue;
                    }
                    initial_visited.insert(new_board.hash);
                    initial_beams.push(BeamNode {
                        board: new_board,
                        connect: last_connection,
                        last_color: color,
                        score: new_score,
                    });
                }
            }
        }

        let mut beam_width = 100;
        let mut best_each_best_template = vec![];
        let mut best_score = 0;
        let tl = 2500;

        'beam_width_search: loop {
            eprintln!("beam_width = {}", beam_width);
            let mut each_best_template = vec![];
            let mut beams = initial_beams.clone();
            let mut current_best_score = 0;
            let mut visited = initial_visited.clone();
            loop {
                if start_all.elapsed().as_millis() > tl {
                    break 'beam_width_search;
                }
                let mut next_beam = BinaryHeap::new();
                for beam in &beams {
                    for color in 1..=4 {
                        if beam.last_color == color {
                            continue;
                        }
                        for connection_pos in beam.connect.iter() {
                            'cand_shape_find: for cand_shape in cand_shapes.iter() {
                                let mut ok = true;
                                let mut required_margin = vec![0; self.input.w];
                                let mut max_height = vec![0; self.input.w];
                                for i in 0..CONNECTION_WIDTH {
                                    let (r, c) = cand_shape[i];
                                    let row = connection_pos.0 as i32 + r;
                                    let col = connection_pos.1 as i32 + c;
                                    if row < 0
                                        || col < 0
                                        || row >= self.input.h as i32
                                        || col >= self.input.w as i32
                                    {
                                        ok = false;
                                        break;
                                    }
                                    required_margin[col as usize] += 1;
                                    max_height[col as usize] = max_height[col as usize].max(row);
                                }
                                if !ok {
                                    continue;
                                }
                                let mut ok = true;
                                for c in 0..self.input.w {
                                    // boardのmax_heightよりも上にrequired_margin個の空きがあるか
                                    let mut row = max_height[c] + 1;
                                    while row < self.input.h as i32 && required_margin[c] > 0 {
                                        if beam.board.is_placable(row as usize, c) {
                                            required_margin[c] -= 1;
                                        }
                                        row += 1;
                                    }
                                    if required_margin[c] > 0 {
                                        ok = false;
                                        break;
                                    }
                                }
                                if !ok {
                                    continue;
                                }
                                let mut new_board = beam.board.clone();
                                let mut new_last_connection = vec![];
                                for i in 0..CONNECTION_WIDTH {
                                    let (r, c) = cand_shape[i];
                                    let row = connection_pos.0 as i32 + r;
                                    let col = connection_pos.1 as i32 + c;
                                    let (row, col) = (row as usize, col as usize);
                                    if !new_board.is_placable(row, col) {
                                        let mut r = self.input.h - 1;
                                        while r > row {
                                            new_board.swap(r, col, r - 1, col);
                                            r -= 1;
                                        }
                                    }
                                    // assert!(new_board.is_placable(row, col));
                                    if !new_board.is_placable(row, col) {
                                        // TODO: ここなおす
                                        eprintln!("WARNING: not placable");
                                        continue 'cand_shape_find;
                                    }
                                    new_board.place(row, col, color);
                                    new_last_connection.push((row, col));
                                }
                                let new_score = {
                                    let mut board = new_board.clone();
                                    board.organize()
                                };
                                if visited.contains(&new_board.hash) {
                                    continue;
                                }
                                visited.insert(new_board.hash);
                                next_beam.push(BeamNode {
                                    board: new_board,
                                    connect: new_last_connection.clone(),
                                    last_color: color,
                                    score: new_score,
                                });
                                if next_beam.len() > beam_width {
                                    next_beam.pop();
                                }
                            }
                        }
                    }
                }

                eprintln!("next_beam.len() = {}", next_beam.len());

                if next_beam.is_empty() {
                    break;
                }

                eprintln!(
                    "next_beam.top().score = {}",
                    next_beam.peek().unwrap().score
                );

                beams.clear();
                for beam in next_beam.into_sorted_vec() {
                    beams.push(beam);
                }

                let template_board = beams[0].board.clone();
                let mut color_distribution = vec![0; 4];
                for r in 0..self.input.h {
                    for c in 0..self.input.w {
                        let color = template_board.get(r, c);
                        if let Some(color) = color {
                            color_distribution[color - 1] += 1;
                        }
                    }
                }

                each_best_template.push((beams[0].score, template_board, color_distribution));
                current_best_score = current_best_score.max(beams[0].score);
            }

            if current_best_score > best_score {
                best_score = current_best_score;
                best_each_best_template = each_best_template.clone();
            }

            beam_width *= 2;
        }

        best_each_best_template.sort_by_key(|(score, _, _)| *score);

        let mut t = 0;
        let mut score = 0;
        let mut operations = vec![];
        let mut board = Board::new(self.input.h, self.input.w);

        while t < self.input.n {
            let mut cur_each_best_template = best_each_best_template.clone();
            // template_boardの1~4に何の色を対応させるのがいいか全探索
            let mut best_trash_count = usize::MAX;
            let mut best_trash_idxs = vec![];
            let mut best_color_map = vec![];
            let mut best_template_board = None;

            while let Some((expected_score, template_board, color_distribution)) =
                cur_each_best_template.pop()
            {
                eprintln!("expected_score = {}", expected_score);
                let mut ok = false;
                'find_best_color_map: for color_map in
                    (1..=4).collect::<Vec<_>>().into_iter().permutations(4)
                {
                    let mut cur_t = t;
                    let mut left_color_distribution = color_distribution.clone();
                    let mut trash_count = 0;
                    let mut trash_idxs = vec![];
                    while left_color_distribution.iter().sum::<usize>() > 0 {
                        if cur_t >= self.input.n {
                            continue 'find_best_color_map;
                        }
                        let color = self.input.a[cur_t];
                        let mapped_color = color_map[color - 1];
                        if left_color_distribution[mapped_color - 1] == 0 {
                            trash_count += 1;
                            trash_idxs.push(cur_t);
                        } else {
                            left_color_distribution[mapped_color - 1] -= 1;
                        }
                        cur_t += 1;
                    }
                    if trash_count < best_trash_count {
                        ok = true;
                        best_trash_count = trash_count;
                        best_trash_idxs = trash_idxs;
                        best_color_map.clone_from(&color_map);
                        best_template_board = Some(template_board.clone());
                    }
                }

                if ok {
                    eprintln!("apply score = {}", expected_score);
                    break;
                }
            }

            if best_trash_count == usize::MAX {
                break;
            }

            let best_template_board = best_template_board.unwrap();

            // 実際に操作を生成
            'jewel_placement: while t < self.input.n {
                if best_trash_idxs.contains(&t) {
                    operations.push(Operation {
                        place: None,
                        organize: false,
                    });
                    score += 100;
                    t += 1;
                    continue;
                }
                let color = self.input.a[t];
                let mapped_color = best_color_map[color - 1];
                for r in 0..self.input.h {
                    for c in 0..self.input.w {
                        if best_template_board.get(r, c) == Some(mapped_color)
                            && board.is_placable(r, c)
                        {
                            board.place(r, c, mapped_color);
                            operations.push(Operation {
                                place: Some((r + 1, c + 1)),
                                organize: false,
                            });
                            t += 1;
                            continue 'jewel_placement;
                        }
                    }
                }
                break;
            }

            // best_boardとboardが同じことを確認
            for r in 0..self.input.h {
                for c in 0..self.input.w {
                    assert_eq!(best_template_board.get(r, c), board.get(r, c));
                }
            }

            operations.last_mut().unwrap().organize = true;

            score += board.organize();
        }

        // 残りターン数を出力
        eprintln!("WARNING: turn {} left", self.input.n - t);
        // 残りターンはgreedy solverを使う
        for ti in t..self.input.n {
            let color = self.input.a[ti];
            let mut best_pos = None;
            // jewelを置く場所を探す
            let mut q = VecDeque::new();
            let first = match color {
                1 => (0, 0),
                2 => (0, self.input.w - 1),
                3 => (self.input.h - 1, 0),
                4 => (self.input.h - 1, self.input.w - 1),
                _ => unreachable!(),
            };
            let mut visited = vec![vec![false; self.input.w]; self.input.h];
            q.push_back(first);
            visited[first.0][first.1] = true;
            while let Some((r, c)) = q.pop_front() {
                if board.is_placable(r, c) {
                    best_pos = Some((r, c));
                    break;
                }
                for (dr, dc) in DIRS.iter() {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= self.input.h as i32 || nc < 0 || nc >= self.input.w as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if visited[nr][nc] {
                        continue;
                    }
                    visited[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }

            let mut place = None;
            if let Some(best_pos) = best_pos {
                place = Some((best_pos.0 + 1, best_pos.1 + 1));

                board.place(best_pos.0, best_pos.1, color);
            } else {
                score += 100; // 捨てる
            }

            // すべて盤面が埋まっているか、もし最後のターンなら整理する
            let organize = board.is_all_filled() || ti == self.input.n - 1;
            if organize {
                score += board.organize();
            }

            operations.push(Operation { place, organize });
        }

        eprintln!("construct = {}", score);

        Output { operations, score }
    }
}
