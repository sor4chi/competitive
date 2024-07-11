use std::fmt::Display;

#[derive(Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => "F",
                Dir::Down => "B",
                Dir::Left => "L",
                Dir::Right => "R",
            }
        )
    }
}

#[derive(Clone)]
pub struct Game {
    pub a: Vec<usize>,
    pub n: usize,
    pub turn: usize,
    pub board: Vec<Vec<usize>>,
    pub op: Vec<Dir>,
}

impl Game {
    pub fn new(a: Vec<usize>, n: usize) -> Game {
        assert_eq!(a.len(), n * n);
        Self {
            a,
            n,
            turn: 0,
            board: vec![vec![0; n]; n],
            op: vec![],
        }
    }

    pub fn place(&mut self, pos: usize) {
        // 左上から数えてpos番目の空きますにa[turn]を置く
        let mut cnt = 1;
        for y in 0..self.n {
            for x in 0..self.n {
                if self.board[y][x] == 0 {
                    if cnt == pos {
                        self.board[y][x] = self.a[self.turn];
                        return;
                    }
                    cnt += 1;
                }
            }
        }
        unreachable!();
    }

    pub fn slide(&mut self, dir: Dir) {
        let mut new_board = vec![vec![0; self.n]; self.n];
        match dir {
            Dir::Up => {
                for x in 0..self.n {
                    let mut y = 0;
                    for i in 0..self.n {
                        if self.board[i][x] != 0 {
                            new_board[y][x] = self.board[i][x];
                            y += 1;
                        }
                    }
                }
            }
            Dir::Down => {
                for x in 0..self.n {
                    let mut y = self.n - 1;
                    for i in (0..self.n).rev() {
                        if self.board[i][x] != 0 {
                            new_board[y][x] = self.board[i][x];
                            if y > 0 {
                                y -= 1;
                            }
                        }
                    }
                }
            }
            Dir::Left => {
                for y in 0..self.n {
                    let mut x = 0;
                    for i in 0..self.n {
                        if self.board[y][i] != 0 {
                            new_board[y][x] = self.board[y][i];
                            x += 1;
                        }
                    }
                }
            }
            Dir::Right => {
                for y in 0..self.n {
                    let mut x: usize = self.n - 1;
                    for i in (0..self.n).rev() {
                        if self.board[y][i] != 0 {
                            new_board[y][x] = self.board[y][i];
                            if x > 0 {
                                x -= 1;
                            }
                        }
                    }
                }
            }
        }
        self.board = new_board;
        self.op.push(dir);
        self.turn += 1;
    }

    pub fn op_str(&self) -> String {
        self.op
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn board_str(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let a = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        let mut game = Game::new(a, 3);
        game.place(0);
        assert_eq!(
            game.board,
            vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        game.slide(Dir::Down);
        assert_eq!(game.turn, 1);
        assert_eq!(
            game.board,
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![1, 0, 0]]
        );
        game.place(1);
        assert_eq!(
            game.board,
            vec![vec![0, 2, 0], vec![0, 0, 0], vec![1, 0, 0]]
        );
        game.slide(Dir::Up);
        assert_eq!(game.turn, 2);
        assert_eq!(
            game.board,
            vec![vec![1, 2, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        game.place(4);
        assert_eq!(
            game.board,
            vec![vec![1, 2, 0], vec![0, 3, 0], vec![0, 0, 0]]
        );
        game.slide(Dir::Right);
        assert_eq!(game.turn, 3);
        assert_eq!(
            game.board,
            vec![vec![0, 1, 2], vec![0, 0, 3], vec![0, 0, 0]]
        );
    }
}
