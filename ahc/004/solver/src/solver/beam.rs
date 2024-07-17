use std::{collections::HashSet, hash::Hash};

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

    fn find_connected_string(&self, banned: &HashSet<usize>) -> Beam {
        let mut beams = vec![];
        for (id, first_str) in self.dict.iter().enumerate() {
            let mut next_beam = Beam {
                score: 0,
                current: first_str.clone(),
                used: HashSet::new(),
            };
            next_beam.score = evaluate(&self.input, &next_beam);
            next_beam.used.insert(id);
            beams.push(next_beam);
        }
        const BEAM_WIDTH: usize = 200;
        let mut best_string = String::new();

        while best_string.is_empty() {
            let mut next_beams: Vec<Beam> = vec![];
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
                            let mut next_beam = Beam {
                                score: 0,
                                current: beam.current.clone() + &next_str[suffix.len()..],
                                used: beam.used.clone(),
                            };
                            next_beam.score = evaluate(&self.input, &next_beam);
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
                    eprintln!("beam: {:?}", beam);
                    return beam.clone();
                }
            }
        }

        Beam {
            score: 0,
            current: best_string,
            used: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Beam {
    score: usize,
    current: String,
    used: HashSet<usize>,
}

fn evaluate(input: &Input, beam: &Beam) -> usize {
    // current.len()が小さいほど、usedが大きいほど良い
    let mut score = 1000000;
    score += beam.current.len();
    score -= beam.used.len() * input.n;
    score
}

impl Solver for BeamSolver {
    fn solve(&self) -> Vec<Vec<char>> {
        let mut board = vec![vec!['.'; self.input.n]; self.input.n];
        let mut banned = HashSet::new();
        for i in 0..self.input.n {
            let best_beam = self.find_connected_string(&banned);
            board[i] = best_beam.current.chars().collect();
            for &id in &best_beam.used {
                banned.insert(id);
            }
        }

        board
    }
}
