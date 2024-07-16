use rand::prelude::SliceRandom;
use std::time::Instant;

use rand::Rng;

pub fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    let (ax, ay) = a;
    let (bx, by) = b;
    ((ax as isize - bx as isize).abs() + (ay as isize - by as isize).abs()) as usize
}

pub fn manhattan_16(a: (u16, u16), b: (u16, u16)) -> u16 {
    let (ax, ay) = a;
    let (bx, by) = b;
    ((ax as i16 - bx as i16).abs() + (ay as i16 - by as i16).abs()) as u16
}

pub fn tsp(v: Vec<(usize, usize)>, limit: usize) -> Vec<usize> {
    fn score(v: &Vec<(usize, usize)>, order: &Vec<usize>) -> usize {
        let mut score = 0;
        for i in 0..order.len() - 1 {
            score += manhattan(v[order[i]], v[order[i + 1]]);
        }
        score
    }

    let mut order = (0..v.len()).collect::<Vec<usize>>();
    let mut best_order = order.clone();
    let mut best_score = score(&v, &order);
    let start = Instant::now();
    let start_temp = 100.0;
    let end_temp = 0.1;
    let mut rng = rand::thread_rng();
    let mut temp = start_temp;

    loop {
        let elapsed = start.elapsed().as_millis();
        if elapsed > limit as u128 {
            break;
        }

        let i = rng.gen_range(0..v.len());
        let j = rng.gen_range(0..v.len());
        let mut new_order = order.clone();
        new_order.swap(i, j);
        let new_score = score(&v, &new_order);

        let diff = new_score as f64 - best_score as f64;
        if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
            // eprintln!("score: {}, temp: {}", new_score, temp);
            order = new_order;
            best_order = order.clone();
            best_score = new_score;
        }

        temp = start_temp + (end_temp - start_temp) * elapsed as f64 / limit as f64;
    }

    best_order
}

pub fn tsp(
    v: Vec<(usize, usize)>,
    limit: usize,
    validator: &dyn Fn(&Vec<usize>) -> bool,
) -> Vec<usize> {
    fn score(v: &Vec<(usize, usize)>, order: &Vec<usize>) -> usize {
        let mut score = 0;
        for i in 0..order.len() - 1 {
            score += manhattan(v[order[i]], v[order[i + 1]]);
        }
        score
    }

    let mut order = (0..v.len()).collect::<Vec<usize>>();
    let mut best_order = order.clone();
    let mut best_score = score(&v, &order);
    let start = Instant::now();
    let start_temp = 10.0;
    let end_temp = 0.1;
    let mut rng = rand::thread_rng();
    let mut temp = start_temp;

    loop {
        let elapsed = start.elapsed().as_millis();
        if elapsed > limit as u128 {
            break;
        }

        // let i = rng.gen_range(1..v.len() - 1);
        // let j = rng.gen_range(1..v.len() - 1);
        // let mut new_order = order.clone();
        // new_order.swap(i, j);
        // let new_score = score(&v, &new_order);

        // 2-opt
        let i = rng.gen_range(1..v.len() - 2);
        let j = rng.gen_range(i + 1..v.len() - 1);
        let mut new_order = order.clone();
        new_order[i..=j].reverse();
        let new_score = score(&v, &new_order);

        let diff = new_score as f64 - best_score as f64;
        if diff < 0.0 || rng.gen_bool((-diff / temp).exp()) {
            // eprintln!("score: {}, temp: {}", new_score, temp);
            if validator(&new_order) {
                order = new_order;
                best_order = order.clone();
                best_score = new_score;
            }
        }

        temp = start_temp + (end_temp - start_temp) * elapsed as f64 / limit as f64;
    }

    best_order
}

pub fn output(used_orders: Vec<usize>, ops: Vec<(usize, usize)>) {
    print!("{} ", used_orders.len());
    for order in used_orders {
        print!("{} ", order + 1);
    }
    println!();

    print!("{} ", ops.len());
    for op in ops {
        print!("{} {} ", op.0, op.1);
    }
    println!();
}
