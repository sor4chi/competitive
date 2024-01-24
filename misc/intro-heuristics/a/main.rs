use proconio::input;

fn get_score(c: &[i32], s: &[Vec<i32>], t: &[usize]) -> i32 {
    let mut last_day = vec![0; 26];
    let mut score = 0;
    for (index, &ti) in t.iter().enumerate() {
        score += s[index][ti - 1];
        last_day[ti - 1] = index + 1;
        for (i, &li) in last_day.iter().enumerate() {
            score -= c[i] * ((index + 1) - li) as i32;
        }
    }
    score
}

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

    fn solve(&mut self) {
        for _ in 0..self.d {
            let mut score = i32::min_value();
            // find the vest day to use
            let mut best_day = 0;
            for d in 0..26 {
                self.t.push(d + 1);
                let new_score = get_score(&self.c, &self.s, &self.t);
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
}

fn main() {
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
    }

    let mut solver = Solver::new(d, c, s);
    solver.solve();
    solver.output();
}
