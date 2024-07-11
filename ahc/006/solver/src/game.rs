pub const N: usize = 1000;

pub struct Input {
    pub a: Vec<usize>,
    pub b: Vec<usize>,
    pub c: Vec<usize>,
    pub d: Vec<usize>,
}

impl Input {
    pub fn new(row: Vec<(usize, usize, usize, usize)>) -> Self {
        let mut a = vec![];
        let mut b = vec![];
        let mut c = vec![];
        let mut d = vec![];

        row.iter().for_each(|&(a_, b_, c_, d_)| {
            a.push(a_);
            b.push(b_);
            c.push(c_);
            d.push(d_);
        });

        Self { a, b, c, d }
    }
}

pub struct Game {
    pub input: Input,
}

impl Game {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}
