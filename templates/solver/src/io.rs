use proconio::input;

pub struct Input {
    pub n: usize,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input! {
            n: usize,
        }

        Input { n }
    }
}

pub struct Output {
    n: usize,
}

impl Output {
    pub fn write(&self) {
        println!("{}", self.n);
    }
}
