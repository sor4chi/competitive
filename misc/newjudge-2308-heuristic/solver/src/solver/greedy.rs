use crate::io::{Input, IO};

use super::Solver;

pub struct GreedySolver {
    io: IO,
    input: Input,
}

impl GreedySolver {
    pub fn new(io: IO, input: Input) -> Self {
        GreedySolver { io, input }
    }
}

fn compute_sat(input: &Input, out: &[usize]) -> (i64, (Vec<Vec<usize>>, Vec<Vec<i64>>)) {
    let mut sat = 0;
    let mut last = vec![0; 26];
    let mut lasts = vec![];
    let mut dsat = vec![vec![0; 26]; out.len()];
    for d in 0..out.len() {
        sat += input.s[d][out[d]] as i64;
        last[out[d]] = d + 1;
        lasts.push(last.clone());
        for i in 0..26 {
            dsat[d][i] = input.c[i] as i64 * (d + 1 - last[i]) as i64;
            sat -= dsat[d][i];
        }
    }
    (sat, (lasts, dsat))
}

pub fn compute_score(
    input: &Input,
    out: &[usize],
) -> (i64, String, (Vec<Vec<usize>>, Vec<Vec<i64>>)) {
    let mut base = vec![0; out.len()];
    for d in 0..out.len() {
        base[d] = d % 26;
    }
    let (B, _) = compute_sat(input, &base);
    let (S, dsat) = compute_sat(input, out);
    ((S - B + 1).max(0), String::new(), dsat)
}

impl Solver for GreedySolver {
    fn solve(&mut self) {
        let mut out = vec![];
        let mut last = vec![0; 26];
        for t in 0..self.input.d {
            let today_ss = self.io.read_day();
            self.input.s.push(today_ss);

            // 貪欲に評価関数を最大化するように次のコンテストを選ぶ
            let mut best = 0;
            let mut best_score = -1;
            for i in 0..26 {
                let id = i;
                let mut new_out = out.clone();
                new_out.push(id);
                let (mut score, _, _) = compute_score(&self.input, &new_out);
                score += (t + 1 - last[id]).pow(2) as i64 * 10;
                if score > best_score {
                    best_score = score;
                    best = id;
                }
            }

            out.push(best);
            last[best] = t + 1;
            println!("{}", best + 1);
        }
    }
}
