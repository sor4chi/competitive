pub const N: usize = 400;
pub const M: usize = 1995;

pub struct Game {
    pub nodes: Vec<(usize, usize)>,
    pub edges: Vec<(usize, usize)>,
}

impl Game {
    pub fn new(nodes: Vec<(usize, usize)>, edges: Vec<(usize, usize)>) -> Self {
        Self { nodes, edges }
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        let (x1, y1) = self.nodes[u];
        let (x2, y2) = self.nodes[v];
        ((((x1 as i32 - x2 as i32).pow(2) + (y1 as i32 - y2 as i32).pow(2)) as f64).sqrt()).round()
            as usize
    }
}
