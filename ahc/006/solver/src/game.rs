use std::collections::HashMap;

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
    serach_restaurant_idx: HashMap<(usize, usize), usize>,
    serach_house_idx: HashMap<(usize, usize), usize>,
}

impl Game {
    pub fn new(input: Input) -> Self {
        let mut serach_restaurant_idx = HashMap::new();
        let mut serach_house_idx = HashMap::new();
        for i in 0..N {
            serach_restaurant_idx.insert((input.a[i], input.b[i]), i);
            serach_house_idx.insert((input.c[i], input.d[i]), i);
        }
        Self {
            input,
            serach_restaurant_idx,
            serach_house_idx,
        }
    }

    pub fn validate(&self, ops: &[(usize, usize)]) -> bool {
        // レストランに行く前に配達していないか確認
        let mut picked = vec![false; N];
        for &(x, y) in ops.iter() {
            if let Some(&idx) = self.serach_restaurant_idx.get(&(x, y)) {
                picked[idx] = true;
            }
            if let Some(&idx) = self.serach_house_idx.get(&(x, y)) {
                if !picked[idx] {
                    return false;
                }
            }
        }
        true
    }
}
