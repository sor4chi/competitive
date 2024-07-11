pub mod greedy;
pub mod random;

use std::collections::{HashMap, HashSet};

use crate::game::{Dir, Game};

pub trait Solver {
    fn get_move(&self, game: &Game) -> Dir;
    fn raw_eval(&self, game: &Game) -> usize {
        let mut score = 0.0;
        // boardをグラフとしてみて、連結成分を列挙する
        let mut connections = vec![];
        let mut visited = HashSet::new();
        for y in 0..game.n {
            for x in 0..game.n {
                if visited.contains(&(y, x)) {
                    continue;
                }
                if game.board[y][x] == 0 {
                    continue;
                }
                let mut size = 0;
                let mut stack = vec![(y, x)];
                while let Some((y, x)) = stack.pop() {
                    if visited.contains(&(y, x)) {
                        continue;
                    }
                    visited.insert((y, x));
                    size += 1;
                    for &(dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny < 0 || ny >= game.n as i32 || nx < 0 || nx >= game.n as i32 {
                            continue;
                        }
                        let ny = ny as usize;
                        let nx = nx as usize;
                        if game.board[y][x] == game.board[ny][nx] {
                            stack.push((ny, nx));
                        }
                    }
                }
                connections.push(size);
            }
        }
        // それぞれの連結成分の2乗の和をスコアとする
        for connection in connections {
            score += (connection * connection) as f64;
        }
        // aからそれぞれの種類をカウントする
        let mut cnts = HashMap::new();
        for &x in &game.a {
            *cnts.entry(x).or_insert(0) += 1;
        }
        let mut squared_sum = 0.0;
        for &x in cnts.values() {
            squared_sum += (x * x) as f64;
        }
        score /= squared_sum;
        (1e6 * score).round() as usize
    }
}
