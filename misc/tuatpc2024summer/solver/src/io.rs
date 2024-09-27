use proconio::input;

#[derive(Clone)]
pub struct Input {
    pub n: usize,
    pub h: usize,
    pub w: usize,
    pub a: Vec<usize>,
}

#[derive(Default, Clone)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
            h: usize,
            w: usize,
            a: [usize; n]
        }

        Input { n, h, w, a }
    }
}

pub struct Operation {
    pub place: Option<(usize, usize)>, // (r, c) に置く, None なら捨てる
    pub organize: bool,                // 整理フェーズを行うかどうか
}

pub struct Output {
    pub operations: Vec<Operation>,
    pub score: usize,
}

impl Output {
    pub fn write(&self) {
        for operation in &self.operations {
            let (r, c) = match operation.place {
                Some((r, c)) => (r as i32, c as i32),
                None => (-1, -1),
            };
            println!("{} {} {}", r, c, if operation.organize { 1 } else { 0 });
        }
    }
}
