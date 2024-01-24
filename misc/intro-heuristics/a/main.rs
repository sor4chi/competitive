use proconio::input;

#[derive(Debug, Clone)]
struct Solver {
    d: usize,
    c: Vec<i32>,
    s: Vec<Vec<i32>>,
    t: Vec<usize>,
}

impl Solver {
    fn new(d: usize, c: Vec<i32>, s: Vec<Vec<i32>>) -> Solver {
        Solver { d, c, s, t: vec![] }
    }

    fn solve(&mut self, k: usize) {
        for _ in 0..self.d {
            let mut score = i32::min_value();
            // find the vest day to use
            let mut best_day = 0;
            for d in 0..26 {
                self.t.push(d + 1);
                let new_score = self.get_score(k);
                if new_score > score {
                    score = new_score;
                    best_day = d;
                }
                self.t.pop();
            }
            self.t.push(best_day + 1);
        }
    }

    fn output(&self) {
        for a in &self.t {
            println!("{}", a);
        }
    }

    fn get_score(&self, k: usize) -> i32 {
        let mut last_day = vec![0; 26];
        let mut score = 0;
        for (index, &ti) in self.t.iter().enumerate() {
            score += self.s[index][ti - 1];
            last_day[ti - 1] = index + 1;
            for (i, &li) in last_day.iter().enumerate() {
                score -= self.c[i] * ((index + 1) - li) as i32;
            }
        }
        for d in self.t.len()..(self.t.len() + k).min(self.d) {
            for (i, &li) in last_day.iter().enumerate() {
                score -= self.c[i] * ((d + 1) - li) as i32;
            }
        }
        score
    }
}

fn main() {
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
    }

    let mut max_score_solver = Solver::new(d, c.clone(), s.clone());
    let mut max_score = i32::min_value();
    for k in 0..26 {
        let mut solver = Solver::new(d, c.clone(), s.clone());
        solver.solve(k);
        let score = solver.get_score(k);
        if score > max_score {
            max_score = score;
            max_score_solver = solver;
        }
    }

    max_score_solver.output();
}
