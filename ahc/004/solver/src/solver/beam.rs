use std::{collections::HashSet, hash::Hash, time::Instant};

use crate::Input;

use super::Solver;

pub struct BeamSolver {
    input: Input,
    dict: Vec<String>,
}

impl BeamSolver {
    pub fn new(input: Input) -> Self {
        let mut dict = vec![];
        for s in &input.ss {
            dict.push(s.clone());
        }
        dict.sort();
        BeamSolver { input, dict }
    }

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

    fn bisect_right(&self, s: &str) -> usize {
        let mut left = 0;
        let mut right = self.dict.len();
        while left < right {
            let mid = (left + right) / 2;
            if self.dict[mid] <= s.to_string() {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn find_connected_string(&self, banned: &HashSet<usize>) -> HorizontalBeam {
        let mut beams = vec![];
        for (id, first_str) in self.dict.iter().enumerate() {
            let mut next_beam = HorizontalBeam {
                score: 0,
                current: first_str.clone(),
                used: HashSet::new(),
            };
            next_beam.score = evaluate_horizontal(&self.input, &next_beam);
            next_beam.used.insert(id);
            beams.push(next_beam);
        }
        const BEAM_WIDTH: usize = 150;
        let mut best_string = String::new();

        let mut iter = 0;
        while best_string.is_empty() {
            iter += 1;
            eprintln!("iter: {}", iter);
            let mut next_beams: Vec<HorizontalBeam> = vec![];
            for beam in &beams {
                // suffix n文字を取り出す
                for suffix_num in 1..=beam.current.len().min(12) {
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
                            next_beam.score = evaluate_horizontal(&self.input, &next_beam);
                            next_beam.used.insert(i);
                            next_beams.push(next_beam);
                        } else {
                            break;
                        }
                    }
                }
            }
            next_beams.sort_by(|a, b| {
                if a.current == b.current {
                    a.used.len().cmp(&b.used.len())
                } else {
                    a.score.cmp(&b.score)
                }
            });
            next_beams.dedup_by(|a, b| a.current == b.current);
            next_beams.sort_by_key(|beam| beam.score);
            next_beams.truncate(BEAM_WIDTH);
            // トップ3個を見る
            beams = next_beams;
            for beam in &beams {
                if beam.current.len() >= self.input.n {
                    return beam.clone();
                }
            }
        }

        HorizontalBeam {
            score: 0,
            current: best_string,
            used: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct HorizontalBeam {
    score: usize,
    current: String,
    used: HashSet<usize>,
}

fn evaluate_horizontal(input: &Input, beam: &HorizontalBeam) -> usize {
    // current.len()が小さいほど、usedが大きいほど良い
    let mut score = 1000000;
    score += beam.current.len();
    score -= beam.used.len() * input.n;
    score
}

#[derive(Debug, Clone)]
struct VerticalBeam {
    score: usize,
    board: Vec<Vec<char>>,
    used: HashSet<usize>,
    used_row: HashSet<usize>,
}

fn evaluate_vertical(input: &Input, beam: &VerticalBeam) -> usize {
    // usedの数で評価
    let mut score = 1000000;
    score -= beam.used.len();
    score
}

impl Solver for BeamSolver {
    fn solve(&mut self) -> Vec<Vec<char>> {
        let mut rows = vec![vec!['.'; self.input.n]; self.input.n];
        let mut banned = HashSet::new();
        for i in 0..self.input.n {
            let best_beam = self.find_connected_string(&banned);
            rows[i] = best_beam.current.chars().collect();
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

        let beam_width = 10;
        let mut beams: Vec<VerticalBeam> = vec![VerticalBeam {
            score: 0,
            board: vec![vec!['.'; self.input.n]; self.input.n],
            used: HashSet::new(),
            used_row: HashSet::new(),
        }];
        for i in 0..self.input.n {
            eprintln!("iter: {}", i);
            let mut next_beams = vec![];
            for beam in beams {
                for (row_id, row) in rows.iter().enumerate() {
                    if beam.used_row.contains(&row_id) {
                        continue;
                    }
                    for j in 0..self.input.n {
                        // rowをトーラス状に左にjシフトさせる
                        let mut next_row = vec!['.'; self.input.n];
                        for k in 0..self.input.n {
                            next_row[k] = row[(k + j) % self.input.n];
                        }
                        let mut next_board = beam.board.clone();
                        next_board[i] = next_row;
                        let mut next_used = beam.used.clone();
                        for col in 0..self.input.n {
                            for suffix_num in 1..=i.min(12) {
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
                        let mut next_used_row = beam.used_row.clone();
                        next_used_row.insert(row_id);
                        let mut next_beam = VerticalBeam {
                            score: 0,
                            board: next_board,
                            used: next_used,
                            used_row: next_used_row,
                        };
                        next_beam.score = evaluate_vertical(&self.input, &next_beam);
                        next_beams.push(next_beam);
                    }
                }
            }
            next_beams.sort_by_key(|beam| beam.score);
            next_beams.truncate(beam_width);
            beams = next_beams;
            for row in &beams[0].board {
                eprintln!("{}", row.iter().collect::<String>());
            }
        }

        // 残った文字列の数を出力
        eprintln!("left: {:?}", self.dict.len() - beams[0].used.len());

        beams[0].board.clone()
    }
}
