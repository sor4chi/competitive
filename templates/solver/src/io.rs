use proconio::input;

pub struct Input {
    pub n: usize,
}

pub struct Output {
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

    pub fn write(&self, output: &Output) {
        println!("{}", output.n);
    }
}
