use proconio::input;

fn main() {
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
        mut t: [usize; d],
        m: usize,
        dq: [(usize, usize); m],
    }

    for (di, qi) in dq {
        t[di - 1] = qi;
        let score = get_score(&c, &s, &t);
        println!("{}", score);
    }
}

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
