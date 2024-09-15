use proconio::input;

pub struct Input {
    pub n: usize,
    pub drinks: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
            drinks: [(usize, usize); n],
        }

        Input { n, drinks }
    }
}

pub struct Output {
    pub operations: Vec<((usize, usize), (usize, usize))>,
}

impl Output {
    pub fn write(&self) {
        println!("{}", self.operations.len());
        for ((a1, b1), (a2, b2)) in &self.operations {
            println!("{} {} {} {}", a1, b1, a2, b2);
        }
    }
}
