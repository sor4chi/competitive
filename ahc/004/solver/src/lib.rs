use proconio::input;

pub mod solver;

pub struct Input {
    pub n: usize,
    pub m: usize,
    pub ss: Vec<String>,
}

impl Default for Input {
    fn default() -> Self {
        input! {
            n: usize,
            m: usize,
            ss: [String; m],
        }
        Input { n, m, ss }
    }
}
