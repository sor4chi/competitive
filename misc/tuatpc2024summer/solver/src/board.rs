use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    mem,
};

use rand::Rng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    hash_table: Vec<Vec<[usize; 5]>>, // zobrist hash
    pub hash: usize,
    h: usize,
    w: usize,
    grid: Vec<Vec<Option<usize>>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.h {
            for c in 0..self.w {
                match self.grid[self.h - r - 1][c] {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Board {
    pub fn new(h: usize, w: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut hash_table = vec![vec![[0; 5]; w]; h];
        let mut hash = 0;
        for row in hash_table.iter_mut() {
            for cell in row.iter_mut() {
                for x in cell.iter_mut() {
                    *x = rng.gen();
                    hash ^= *x;
                }
            }
        }
        Board {
            hash_table,
            h,
            w,
            grid: vec![vec![None; w]; h],
            hash,
        }
    }

    pub fn tick(&mut self) {
        for c in 0..self.w {
            let mut bottom = 0;
            for r in 0..self.h {
                if self.grid[r][c].is_some() {
                    self.grid[bottom][c] = self.grid[r][c];
                    self.hash ^= self.hash_table[bottom][c][self.grid[bottom][c].unwrap_or(0)];
                    if bottom != r {
                        self.clear(r, c);
                    }
                    bottom += 1;
                }
            }
        }
    }

    pub fn organize(&mut self) -> usize {
        let mut count = 0;
        let mut score = 0;

        loop {
            count += 1;

            self.tick();

            let mut removed = HashSet::new();
            let mut removed_count = 0;
            let mut visited = vec![vec![false; self.w]; self.h];
            for r in 0..self.h {
                for c in 0..self.w {
                    if visited[r][c] {
                        continue;
                    }
                    if self.grid[r][c].is_none() {
                        continue;
                    }
                    let jewel = self.grid[r][c];
                    let mut q = VecDeque::new();
                    q.push_back((r, c));
                    let mut comp = vec![];
                    while let Some((r, c)) = q.pop_front() {
                        if visited[r][c] {
                            continue;
                        }
                        visited[r][c] = true;
                        comp.push((r, c));
                        for (dr, dc) in DIR.iter() {
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr < 0 || nr >= self.h as i32 || nc < 0 || nc >= self.w as i32 {
                                continue;
                            }
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if visited[nr][nc] {
                                continue;
                            }
                            if self.grid[r][c] == self.grid[nr][nc] {
                                q.push_back((nr, nc));
                            }
                        }
                    }
                    if comp.len() >= 3 {
                        removed.insert(jewel);
                        removed_count += comp.len();
                        for (r, c) in comp {
                            self.clear(r, c);
                        }
                    }
                }
            }

            let f = removed_count.pow(2)
                * ((512_f32 * (1_f32 - 0.99_f32.powi(2_i32.pow(count)))).round() as usize
                    + removed.len().pow(2));

            if f == 0 {
                break;
            }

            score += f;
        }

        // 盤面に残っている場合はscoreを0にする
        if !self.is_all_empty() {
            score = 0;
        }

        score
    }

    pub fn place(&mut self, r: usize, c: usize, x: usize) {
        assert!(r < self.h && c < self.w);
        assert!(self.grid[r][c].is_none());
        self.grid[r][c] = Some(x);
        self.hash ^= self.hash_table[r][c][x];
    }

    pub fn clear(&mut self, r: usize, c: usize) {
        assert!(r < self.h && c < self.w);
        assert!(self.grid[r][c].is_some());
        self.grid[r][c] = None;
        self.hash ^= self.hash_table[r][c][self.grid[r][c].unwrap_or(0)];
    }

    pub fn is_placable(&self, r: usize, c: usize) -> bool {
        r < self.h && c < self.w && self.grid[r][c].is_none()
    }

    pub fn is_all_filled(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&x| x.is_some()))
    }

    pub fn empty_size(&self) -> usize {
        self.grid.iter().flatten().filter(|&&x| x.is_none()).count()
    }

    pub fn is_all_empty(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&x| x.is_none()))
    }

    pub fn get(&self, r: usize, c: usize) -> Option<usize> {
        self.grid[r][c]
    }

    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        let tmp = self.grid[r1][c1];
        self.grid[r1][c1] = self.grid[r2][c2];
        self.grid[r2][c2] = tmp;
        self.hash ^= self.hash_table[r1][c1][self.grid[r1][c1].unwrap_or(0)];
        self.hash ^= self.hash_table[r2][c2][self.grid[r2][c2].unwrap_or(0)];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_fmt() {
        let mut board = Board::new(3, 3);
        board.grid[0][0] = Some(1);
        board.grid[1][0] = Some(2);
        board.grid[2][0] = Some(3);
        board.grid[2][1] = Some(4);
        board.grid[2][2] = Some(5);
        assert_eq!(
            board.to_string(),
            "345\n\
             2..\n\
             1..\n"
        );
    }

    #[test]
    fn test_board_tick() {
        let mut board = Board::new(3, 3);
        board.grid[0][0] = Some(1);
        board.grid[1][0] = Some(2);
        board.grid[2][0] = Some(3);
        board.grid[1][1] = Some(4);
        board.grid[2][1] = Some(5);
        board.grid[2][2] = Some(6);
        assert_eq!(
            board.to_string(),
            "356\n\
             24.\n\
             1..\n"
        );
        board.tick();
        assert_eq!(
            board.to_string(),
            "3..\n\
             25.\n\
             146\n"
        );
    }

    #[test]
    fn test_board_clear() {
        let mut board = Board::new(3, 3);
        board.grid[0][0] = Some(1);
        board.grid[1][0] = Some(1);
        board.grid[2][0] = Some(1);
        board.grid[0][1] = Some(2);
        board.grid[1][1] = Some(1);
        board.grid[2][1] = Some(2);
        board.grid[2][2] = Some(2);
        assert_eq!(
            board.to_string(),
            "122\n\
             11.\n\
             12.\n"
        );
        eprintln!("organize score: {}", board.organize());
        assert_eq!(
            board.to_string(),
            "...\n\
             ...\n\
             ...\n"
        );
    }
}
