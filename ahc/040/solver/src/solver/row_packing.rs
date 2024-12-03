use std::{collections::HashSet, mem::swap, time::Instant};

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use rand_pcg::Pcg64Mcg;

use crate::{
    io::{Direction, Input, Operation, Query, Rotation, IO},
    state::State,
};

use super::Solver;

pub struct RowPackingSolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl RowPackingSolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> RowPackingSolver<'a> {
        RowPackingSolver { input, io }
    }
}

fn search(row_width: usize, rects: &[(usize, usize)], inv: bool) -> (usize, Vec<usize>) {
    let mut width = 0;
    let mut max_width = 0;
    let mut height = 0;
    let mut max_height_in_row = 0;
    let mut row_counts = vec![];
    let mut row_count = 0;
    for (w, h) in rects {
        let (mut w, mut h) = if w < h { (*w, *h) } else { (*h, *w) };
        if inv {
            swap(&mut w, &mut h);
        }
        if width + w > row_width {
            max_width = max_width.max(width);
            width = 0;
            height += max_height_in_row;
            max_height_in_row = 0;
            row_counts.push(row_count);
            row_count = 0;
        }
        width += w;
        max_height_in_row = max_height_in_row.max(h);
        row_count += 1;
    }
    row_counts.push(row_count);
    (max_width + height + max_height_in_row, row_counts)
}

impl Solver for RowPackingSolver<'_> {
    fn solve(&mut self) {
        let mut rects_measured = vec![];
        for rect in &self.input.rects {
            rects_measured.push(vec![(rect.0, rect.1)]);
        }
        let mut perm = (0..self.input.N).collect::<Vec<_>>();
        let mut rng = Pcg64Mcg::new(42);
        perm.shuffle(&mut rng);
        let mut trial = (self.input.T as i32 - self.input.N as i32).max(0) as usize;
        while trial > 0 {
            let idx = trial % self.input.N;
            let (w, h) = self.io.measure(&Query {
                operations: vec![Operation {
                    p: perm[idx],
                    r: Rotation::Stay,
                    d: Direction::Up,
                    b: -1,
                }],
            });
            rects_measured[perm[idx]].push((w, h));
            trial -= 1;
        }
        // average
        let mut rects = vec![];
        for rect in rects_measured.iter() {
            let mut w = 0;
            let mut h = 0;
            for (wi, hi) in rect {
                w += wi;
                h += hi;
            }
            w /= rect.len();
            h /= rect.len();
            rects.push((w, h));
        }
        // searchが最も小さくなるような場所を探す
        let row_widths = {
            let mut visited = HashSet::new();
            let mut score_widths = vec![];
            for inv in &[false, true] {
                for width in (0..=1000000).step_by(1000) {
                    let (score, row_counts) = search(width, &rects, *inv);
                    if !visited.insert((row_counts, *inv)) {
                        continue;
                    }
                    score_widths.push((score, width, *inv));
                }
            }
            score_widths.sort_by_key(|x| x.0);
            score_widths
        };
        let mut state = State::new(self.input);
        let mut best_operations = vec![];
        let _ = state.query(self.input, &best_operations);
        let mut best_score = state.score_t as usize;
        for t in 0..self.input.N.min(self.input.T) - 1 {
            if t >= row_widths.len() {
                eprintln!("t={} is out of range", t);
                self.io.measure(&Query { operations: vec![] });
                continue;
            }
            let mut operations = vec![];
            let mut cur_width = 0;
            for i in 0..self.input.N {
                let mut rotate = if rects[i].0 < rects[i].1 {
                    Rotation::Stay
                } else {
                    Rotation::Rotate
                };
                if row_widths[t].2 {
                    rotate.flip();
                }
                let w = if rotate == Rotation::Stay {
                    rects[i].0
                } else {
                    rects[i].1
                };
                operations.push(Operation {
                    p: i,
                    r: rotate,
                    d: Direction::Up,
                    b: if cur_width + w <= row_widths[t].1 {
                        (i - 1) as isize
                    } else {
                        -1
                    },
                });
                cur_width += w;
                if cur_width > row_widths[t].1 {
                    cur_width = 0;
                }
            }
            let (w, h) = self.io.measure(&Query {
                operations: operations.clone(),
            });
            if w + h < best_score {
                best_score = w + h;
                best_operations.clone_from(&operations);
            }
        }
        // それぞれのrectをひとつづつ回転させていきスコアが良くなったら採用
        let start = Instant::now();
        let mut perm = (0..self.input.N).collect::<Vec<_>>();
        while start.elapsed().as_millis() < 2900 {
            let mut operations = best_operations.clone();
            perm.shuffle(&mut rng);
            let rotates = rng.gen_range(1..=self.input.N);
            for i in 0..rotates {
                operations[perm[i]].r = match operations[perm[i]].r {
                    Rotation::Stay => Rotation::Rotate,
                    Rotation::Rotate => Rotation::Stay,
                };
            }
            let mut state = State::new(self.input);
            let _ = state.query(self.input, &operations);
            let score = state.score_t as usize;
            if score < best_score {
                best_score = score;
                best_operations.clone_from(&operations);
            }
        }

        self.io.measure(&Query {
            operations: best_operations,
        });
    }
}
