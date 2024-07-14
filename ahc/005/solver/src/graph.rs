use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct WeightedUndirectedGraph {
    pub graph: HashMap<Point, Vec<(Point, usize)>>,
}

impl WeightedUndirectedGraph {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: Point, to: Point, weight: usize) {
        self.graph
            .entry(from)
            .or_insert(Vec::new())
            .push((to, weight));
        self.graph
            .entry(to)
            .or_insert(Vec::new())
            .push((from, weight));
    }

    pub fn dijkstra(&self, start: Point) -> HashMap<Point, usize> {
        let mut dist = HashMap::new();
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, start)));
        while let Some(Reverse((d, p))) = pq.pop() {
            if dist.contains_key(&p) {
                continue;
            }
            dist.insert(p, d);
            for &(np, w) in &self.graph[&p] {
                if !dist.contains_key(&np) {
                    pq.push(Reverse((d + w, np)));
                }
            }
        }
        dist
    }
}
