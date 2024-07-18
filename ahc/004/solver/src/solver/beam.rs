use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

use crate::{IDGenerator, Input};

use super::Solver;

pub struct BeamSolver {
    input: Input,
    dict: Vec<String>,
}

impl BeamSolver {
    #[inline]
    pub fn new(input: Input) -> Self {
        let mut dict = vec![];
        for s in &input.ss {
            dict.push(s.clone());
        }
        dict.sort();
        BeamSolver { input, dict }
    }

    #[inline]
    fn bisect_left(&self, s: &str) -> usize {
        let mut left = 0;
        let mut right = self.dict.len();
        while left < right {
            let mid = (left + right) / 2;
            if self.dict[mid] < s.to_string() {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    #[inline]
    fn find_connected_string(&self, banned: &HashSet<usize>) -> HorizontalBeam {
        let mut beams = vec![];
        for (id, first_str) in self.dict.iter().enumerate() {
            let mut next_beam = HorizontalBeam {
                score: 0,
                current: first_str.clone(),
                used: HashSet::new(),
            };
            next_beam.score = evaluate_horizontal(&self.dict, &self.input, &next_beam);
            next_beam.used.insert(id);
            beams.push(next_beam);
        }
        const BEAM_WIDTH: usize = 500;

        let mut iter = 0;
        loop {
            iter += 1;
            eprintln!("iter: {}", iter);
            let mut next_beams: BinaryHeap<HorizontalBeam> = BinaryHeap::new();
            for beam in &beams {
                // suffix n文字を取り出す
                for suffix_num in 2..=beam.current.len().min(11) {
                    let suffix = &beam.current[beam.current.len() - suffix_num..];
                    // suffixと一致するprefixをもつ文字列をdictから取り出す
                    let left = self.bisect_left(suffix);
                    for i in left..self.dict.len() {
                        if beam.used.contains(&i) || banned.contains(&i) {
                            continue;
                        }
                        let next_str = &self.dict[i];
                        // もしnext_strがsuffixをprefixとして持っているなら
                        if next_str.starts_with(suffix) {
                            let mut next_beam = HorizontalBeam {
                                score: 0,
                                current: beam.current.clone() + &next_str[suffix.len()..],
                                used: beam.used.clone(),
                            };
                            next_beam.score =
                                evaluate_horizontal(&self.dict, &self.input, &next_beam);
                            next_beam.used.insert(i);
                            next_beams.push(next_beam);
                            if next_beams.len() >= BEAM_WIDTH {
                                next_beams.pop();
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            let mut next_beams = next_beams.into_iter().collect::<Vec<_>>();
            next_beams.truncate(BEAM_WIDTH);
            // トップ3個を見る
            beams = next_beams;
            for beam in &beams {
                if beam.current.len() >= self.input.n {
                    return beam.clone();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct HorizontalBeam {
    score: usize,
    current: String,
    used: HashSet<usize>,
}

impl PartialOrd for HorizontalBeam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for HorizontalBeam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[inline]
fn evaluate_horizontal(dict: &Vec<String>, input: &Input, beam: &HorizontalBeam) -> usize {
    // current.len()が小さいほど、usedが大きいほど良い
    let mut score = 1000000;
    score += beam.current.len();
    score -= beam.used.len() * input.n;
    score
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct VerticalBeam {
    id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NextVerticalBeam {
    score: usize,
    board: Vec<Vec<char>>,
    used: HashSet<usize>,
    used_row: HashSet<usize>,
}

impl PartialOrd for NextVerticalBeam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for NextVerticalBeam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[inline]
fn evaluate_vertical(beam: &NextVerticalBeam) -> usize {
    // usedの数で評価
    let mut score = 1000000;
    score -= beam.used.len();
    score
}

impl Solver for BeamSolver {
    fn solve(&mut self) -> Vec<Vec<char>> {
        let start = Instant::now();
        let mut rows = vec![vec!['.'; self.input.n]; self.input.n];
        let mut banned = HashSet::new();
        for i in 0..self.input.n {
            let best_beam = self.find_connected_string(&banned);
            rows[i] = best_beam.current.chars().collect();
            rows[i].truncate(self.input.n);
            for &id in &best_beam.used {
                banned.insert(id);
            }
        }

        for row in &rows {
            eprintln!("{}", row.iter().collect::<String>());
        }

        // 残った文字列の数を出力
        eprintln!("left: {:?}", self.dict.len() - banned.len());

        for i in 0..self.dict.len() {
            if banned.contains(&i) {
                self.dict[i].clear();
            }
        }
        self.dict.retain(|s| !s.is_empty());

        let mut idg = IDGenerator::new();

        let mut beam_width = 60;
        let first_beam = VerticalBeam { id: idg.generate() };
        let mut board_map = HashMap::new();
        board_map.insert(first_beam.id, vec![vec!['.'; self.input.n]; self.input.n]);
        let mut used_map = HashMap::new();
        used_map.insert(first_beam.id, HashSet::new());
        let mut used_row_map = HashMap::new();
        used_row_map.insert(first_beam.id, HashSet::new());
        let mut beams: Vec<VerticalBeam> = vec![first_beam];
        let mut best_id = 0;
        for i in 0..self.input.n {
            if start.elapsed().as_millis() > 2700 {
                beam_width = 20;
            }
            eprintln!("iter: {}", i);
            let mut next_beams: BinaryHeap<NextVerticalBeam> = BinaryHeap::new();
            for beam in &beams {
                for (row_id, row) in rows.iter().enumerate() {
                    // 1/2で棄却
                    if rand::random::<f64>() < 0.5 {
                        continue;
                    }
                    let used_row = used_row_map.get(&beam.id).unwrap();
                    if used_row.contains(&row_id) {
                        continue;
                    }
                    for j in 0..self.input.n {
                        // rowをトーラス状に左にjシフトさせる
                        let mut next_row = vec!['.'; self.input.n];
                        for k in 0..self.input.n {
                            next_row[k] = row[(k + j) % self.input.n];
                        }
                        let mut next_board = board_map.get(&beam.id).unwrap().clone();
                        next_board[i] = next_row;
                        let used = used_map.get(&beam.id).unwrap();
                        let mut next_used = used.clone();
                        for col in 0..self.input.n {
                            for suffix_num in 2..=i.min(9) {
                                // i行目以前の同じ列のsuffix_num文字を取り出す
                                let col_str = (0..suffix_num)
                                    .map(|k| next_board[i - k][col])
                                    .rev()
                                    .collect::<String>();
                                let left = self.bisect_left(&col_str);
                                for l in left..self.dict.len() {
                                    if next_used.contains(&l) {
                                        continue;
                                    }
                                    let next_str = &self.dict[l];
                                    if col_str.starts_with(next_str) {
                                        next_used.insert(l);
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                        let mut next_used_row = used_row.clone();
                        next_used_row.insert(row_id);
                        let mut next_beam = NextVerticalBeam {
                            score: 0,
                            board: next_board,
                            used: next_used,
                            used_row: next_used_row,
                        };
                        next_beam.score = evaluate_vertical(&next_beam);
                        next_beams.push(next_beam);
                        if next_beams.len() >= beam_width {
                            next_beams.pop();
                        }
                    }
                }
            }
            let mut next_beams = next_beams.into_iter().collect::<Vec<_>>();
            next_beams.truncate(beam_width);
            beams.clear();
            for beam in next_beams {
                let id = idg.generate();
                board_map.insert(id, beam.board.clone());
                used_map.insert(id, beam.used.clone());
                used_row_map.insert(id, beam.used_row.clone());
                if beams.is_empty() {
                    best_id = id;
                }
                beams.push(VerticalBeam { id });
            }
        }

        board_map.get(&best_id).unwrap().clone()
    }
}
