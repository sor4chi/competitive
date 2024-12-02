use std::{collections::HashSet, mem::swap};

use rand::Rng;
use rand_distr::{Distribution, Normal};
use rand_pcg::Pcg64Mcg;

use crate::io::{Direction, Input, Operation, Query, Rotation, IO};

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
        // searchが最も小さくなるような場所を探す
        let row_widths = {
            let mut visited = HashSet::new();
            let mut score_widths = vec![];
            for inv in &[false, true] {
                for width in (0..=1000000).step_by(1000) {
                    let (score, row_counts) = search(width, &self.input.rects, *inv);
                    if !visited.insert((row_counts, *inv)) {
                        continue;
                    }
                    score_widths.push((score, width, *inv));
                }
            }
            score_widths.sort_by_key(|x| x.0);
            score_widths
        };
        for t in 0..self.input.T {
            if t >= row_widths.len() {
                eprintln!("t={} is out of range", t);
                self.io.measure(&Query { operations: vec![] });
                continue;
            }
            let mut operations = vec![];
            let mut cur_width = 0;
            for i in 0..self.input.N {
                let mut rotate = if self.input.rects[i].0 < self.input.rects[i].1 {
                    Rotation::Stay
                } else {
                    Rotation::Rotate
                };
                if row_widths[t].2 {
                    rotate.flip();
                }
                let w = if rotate == Rotation::Stay {
                    self.input.rects[i].0
                } else {
                    self.input.rects[i].1
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
            self.io.measure(&Query {
                operations: operations.clone(),
            });
        }
    }
}
