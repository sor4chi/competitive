use std::time::Instant;

use crate::game::{Direction, DIRS};

// n=3, k=4 の時 [0,0,0] から [3,3,3] までの組み合わせを生成する
pub fn generate_cands(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut cands = vec![];
    let mut current = vec![0; n];
    loop {
        cands.push(current.clone());
        let mut i = n;
        while i > 0 && current[i - 1] == k - 1 {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        current[i - 1] += 1;
        for j in i..n {
            current[j] = 0;
        }
    }
    cands
}

#[test]
fn test_generate_cands() {
    let mut time = Instant::now();
    generate_cands(10, 3);
    eprintln!("{:?}", time.elapsed());
}

pub fn tornado_travel(n: usize) -> Vec<Direction> {
    let mut res = vec![];
    let mut x = n / 2;
    let mut y = n / 2;
    let mut d = 0;
    let mut l = 1;
    let mut c = 0;
    let mut i = 0;
    while i < n * n - 1 {
        res.push(DIRS[d as usize]);
        let n = DIRS[d as usize].get_d();
        x = (x as i32 + n.0) as usize;
        y = (y as i32 + n.1) as usize;
        i += 1;
        c += 1;
        if c == l {
            c = 0;
            d = (d + 1) % 4;
            if d % 2 == 0 {
                l += 1;
            }
        }
    }
    res
}

#[test]
fn test_tornado_travel() {
    let n = 5;
    let res = tornado_travel(n);
    assert_eq!(res.len(), n * n);
    eprintln!("{:?}", res);
}

// nをm個に均等に分割する
pub fn split_n(n: usize, m: usize) -> Vec<usize> {
    let mut res = vec![n / m; m];
    for i in 0..n % m {
        res[i] += 1;
    }
    res
}

#[test]
fn test_split_n() {
    let n = 10;
    let m = 3;
    let res = split_n(n, m);
    assert_eq!(res.len(), m);
    assert_eq!(res.iter().sum::<usize>(), n);
    let min_v = *res.iter().min().unwrap();
    let max_v = *res.iter().max().unwrap();
    assert!(max_v - min_v <= 1);
    eprintln!("{:?}", res);
}
