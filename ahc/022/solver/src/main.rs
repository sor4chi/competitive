use proconio::source::line::LineSource;
use rand;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::io::BufReader;
use std::{collections::VecDeque, io};

use proconio::input;

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        l: usize,
        n: usize,
        s: usize,
        yx: [(usize, usize); n],
        // a: [usize; n],
        // f: [isize; 10000],
    }

    let mut p = vec![vec![0; l]; l];
    let mut pred = vec![0; n];

    // {
    //     let mut cnt = 0;
    //     let mut inc = true;
    //     let mut visited = vec![vec![false; l]; l];
    //     let mut queue = VecDeque::new();
    //     queue.push_back((l / 2, l / 2));
    //     visited[l / 2][l / 2] = true;
    //     let dir: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    //     // N*Nマスの空間に対して、最大でN*N回の探索が発生する
    //     while let Some((y, x)) = queue.pop_front() {
    //         if cnt == 1000 {
    //             inc = false;
    //         }
    //         if cnt == 0 {
    //             inc = true;
    //         }
    //         p[y][x] = cnt;
    //         if inc {
    //             cnt += 1;
    //         } else {
    //             cnt -= 1;
    //         }
    //         for &(dy, dx) in dir.iter() {
    //             let ny = y as isize + dy;
    //             let nx = x as isize + dx;
    //             if ny < 0 || ny >= l as isize || nx < 0 || nx >= l as isize {
    //                 continue;
    //             }
    //             let (ny, nx) = (ny as usize, nx as usize);
    //             if visited[ny][nx] {
    //                 continue;
    //             }
    //             visited[ny][nx] = true;
    //             queue.push_back((ny, nx));
    //         }
    //     }
    // }

    // if s > 25 {
    //     // 中心からの距離を0~1000スケールにして初期化
    //     // 半径がl/2なので1/2で1000になるように線形に
    //     for i in 0..l {
    //         for j in 0..l {
    //             // マンハッタン距離を出し、lで割って1000倍
    //             let (i, j) = (i, j);
    //             let (y, x) = (l as isize / 2, l as isize / 2);
    //             let dist = (i as isize - y).abs() + (j as isize - x).abs();
    //             p[i][j] = (dist * 1000 / (l as isize)) as usize;
    //         }
    //     }
    // }

    if s > 25 {
        // pをランダムな値で初期化
        for i in 0..l {
            for j in 0..l {
                p[i][j] = rand::random::<usize>() % 1000;
            }
        }
    }

    if s <= 25 {
        // pを2次元の正弦波で初期化
        // 振幅は250、周期はl
        for i in 0..l {
            for j in 0..l {
                let noize = rand::random::<usize>() % 10;
                p[i][j] = ((((250.0 * (1.0 * PI * i as f64 / l as f64).sin()) + 250.0) as usize
                    + ((250.0 * (1.0 * PI * j as f64 / l as f64).sin()) + 250.0) as usize)
                    + noize)
                    .min(1000)
                    .max(0);
            }
        }
    }

    // print P
    for i in 0..l {
        let mut s = String::new();
        for j in 0..l {
            s.push_str(&format!("{} ", p[i][j]));
        }
        s.pop();
        println!("{}", s);
    }

    // p[x][y]を観測したときの値は $max(0,min(1000,round(P_{x,y} + f(S))))$ である
    // この時、$f(S)$は平均0、標準偏差Sの正規分布。
    // 尤度を求めて自己位置をベイズ推定する
    // 尤度はそのマスの値が観測値になる確率
    fn likelihood(p: usize, s: usize, f: isize) -> f64 {
        let (f, p, s) = (f as f64, p as f64, s as f64);
        let a = 1.0 / (s * (2.0 * PI).sqrt());
        let b = (-1.0 * (f - p).powi(2) / (2.0 * s.powi(2))).exp();
        a * b
    }

    // let mut found = HashSet::new();

    for ni in 0..n {
        // ベイズ推定
        // 最初は全てのマスに均等に確率を割り振る
        let mut prob = vec![vec![0.0; l]; l];
        // let mut not_found = HashSet::new();
        // ますは必ずyxに該当するため、それらを初期値にする
        for i in 0..n {
            let (y, x) = yx[i];
            // if found.contains(&(y, x)) {
            //     continue;
            // } else {
            //     not_found.insert((y, x));
            // }
            prob[y][x] = 1.0;
        }
        // norm
        let sum: f64 = prob.iter().map(|row| row.iter().sum::<f64>()).sum();
        for i in 0..l {
            for j in 0..l {
                prob[i][j] /= sum;
            }
        }

        eprintln!("=== {} ===", ni);
        // eprintln!("not found {:?}", not_found);
        // RANGEをlog_10(S)の整数部分にする
        let RANGE = (s as f64).log10() as isize + 1;
        'measure_loop: for measure_i in 0..RANGE * 2 + 1 {
            for measure_j in 0..RANGE * 2 + 1 {
                let (measure_i, measure_j) =
                    (measure_i as isize - RANGE, measure_j as isize - RANGE);
                println!("{} {} {}", ni, measure_i, measure_j);
                input! {
                    measured: usize,
                }
                // 現在の位置を観測したときの尤度を計算
                for i in 0..l {
                    for j in 0..l {
                        let (i, j, l) = (i as isize, j as isize, l as isize);
                        // i = (i + 1) % l
                        let torus_i = (i + measure_i + l) % l;
                        // j = (j + 1) % l
                        let torus_j = (j + measure_j + l) % l;
                        let (torus_i, torus_j) = (torus_i as usize, torus_j as usize);
                        // prob[torus_i][torus_j] *= likelihood(p[torus_i][torus_j], s, measured as isize);
                        let lh: f64 = likelihood(p[torus_i][torus_j], s, measured as isize);
                        prob[i as usize][j as usize] *= lh;
                    }
                }

                // 正規化
                let sum: f64 = prob.iter().map(|row| row.iter().sum::<f64>()).sum();
                for i in 0..l {
                    for j in 0..l {
                        prob[i][j] /= sum;
                    }
                }

                // もし一番確率が高いマスが7割以上なら、そのマスをyx[ni]にする
                let mut max = 0.0;
                let mut max_i = 0;
                let mut max_j = 0;
                for i in 0..l {
                    for j in 0..l {
                        if prob[i][j] > max {
                            max = prob[i][j];
                            max_i = i;
                            max_j = j;
                        }
                    }
                }
                eprintln!("{} {} {}", max_i, max_j, max);
                // yxからijのindexを求める
                let yx_idx = yx.iter().position(|&yx| yx == (max_i, max_j));
                eprintln!("{:?}", yx_idx);

                pred[ni] = yx_idx.unwrap();
                if max > 0.95 {
                    // found.insert((max_i, max_j));
                    break 'measure_loop;
                }
            }
        }
    }

    println!("-1 -1 -1");

    // print pred
    for i in 0..n {
        println!("{}", pred[i]);
    }
}
