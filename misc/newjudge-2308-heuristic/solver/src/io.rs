use proconio::input_interactive;

pub struct Input {
    pub d: usize,
    pub c: Vec<usize>,
    pub s: Vec<Vec<usize>>,
}

#[derive(Default)]
pub struct IO {}

impl IO {
    pub fn read(&mut self) -> Input {
        input_interactive! {
            d: usize,
            c: [usize; 26],
        }

        Input { d, c, s: vec![] }
    }

    pub fn read_day(&mut self) -> Vec<usize> {
        input_interactive! {
            s: [usize; 26],
        }

        s
    }
}
