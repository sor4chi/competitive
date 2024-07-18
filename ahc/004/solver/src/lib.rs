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

pub struct IDGenerator {
    id: usize,
}

impl IDGenerator {
    pub fn new() -> Self {
        IDGenerator { id: 0 }
    }

    #[inline]
    pub fn generate(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}
