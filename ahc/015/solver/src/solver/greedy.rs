use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game::{Dir, Game};

use super::Solver;

pub struct GreedySolver {}

impl GreedySolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for GreedySolver {
    fn get_move(&self, _game: &Game) -> Dir {
        // 次の入力が任意の位置に置かれた時、どの方向にスライドすると最も得点が高くなるかを計算する
        let mut left_scores = vec![];
        let mut right_scores = vec![];
        let mut up_scores = vec![];
        let mut down_scores = vec![];

        let left = _game.n * _game.n - _game.turn - 1;

        if left >= 4 {
            let start = Instant::now();
            let mut rng = StdRng::from_entropy();
            // 19msに達するまで
            while start.elapsed().as_millis() < 19 {
                let mut game = _game.clone();
                let i = rng.gen_range(1..=left);
                let j = rng.gen_range(1..=left - 1);
                let k = rng.gen_range(1..=left - 2);
                let l = rng.gen_range(1..=left - 3);

                game.place(i);
                for dir_1 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    let mut game = game.clone();
                    game.slide(dir_1.clone());
                    game.place(j);
                    for dir_2 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                        let mut game = game.clone();
                        game.slide(dir_2.clone());
                        game.place(k);
                        for dir_3 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                            let mut game = game.clone();
                            game.slide(dir_3.clone());
                            game.place(l);
                            for dir_4 in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                                let mut game = game.clone();
                                game.slide(dir_4.clone());
                                let score = self.raw_eval(&game);
                                match dir_1 {
                                    Dir::Up => up_scores.push(score),
                                    Dir::Down => down_scores.push(score),
                                    Dir::Left => left_scores.push(score),
                                    Dir::Right => right_scores.push(score),
                                }
                            }
                        }
                    }
                }
            }
        } else {
            for i in 1..=left {
                let mut game = _game.clone();
                game.place(i);
                for dir in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    let mut game = game.clone();
                    game.slide(dir.clone());
                    let score = self.raw_eval(&game);
                    match dir {
                        Dir::Up => up_scores.push(score),
                        Dir::Down => down_scores.push(score),
                        Dir::Left => left_scores.push(score),
                        Dir::Right => right_scores.push(score),
                    }
                }
            }
        }

        let up_avg = up_scores.iter().sum::<usize>() as f64 / up_scores.len() as f64;
        let down_avg = down_scores.iter().sum::<usize>() as f64 / down_scores.len() as f64;
        let left_avg = left_scores.iter().sum::<usize>() as f64 / left_scores.len() as f64;
        let right_avg = right_scores.iter().sum::<usize>() as f64 / right_scores.len() as f64;
        // 最後はどこも行く場所がないのでNaNになる。その場合はランダムに動く
        if up_avg.is_nan() || down_avg.is_nan() || left_avg.is_nan() || right_avg.is_nan() {
            let mut rng = StdRng::from_entropy();
            match rng.gen_range(0..4) {
                0 => return Dir::Up,
                1 => return Dir::Down,
                2 => return Dir::Left,
                3 => return Dir::Right,
                _ => unreachable!(),
            }
        }
        let max = vec![
            (Dir::Up, up_avg),
            (Dir::Down, down_avg),
            (Dir::Left, left_avg),
            (Dir::Right, right_avg),
        ]
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

        max.0
    }
}
