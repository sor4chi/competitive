use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::strategy::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }

    pub fn move_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::from((self.x, self.y - 1)),
            Direction::Down => Self::from((self.x, self.y + 1)),
            Direction::Left => Self::from((self.x - 1, self.y)),
            Direction::Right => Self::from((self.x + 1, self.y)),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn get(&self, p: Point) -> Option<&Vec<(Point, usize)>> {
        self.graph.get(&p)
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

    pub fn get_path(&self, start: Point, goal: Point, dist: &HashMap<Point, usize>) -> Vec<Point> {
        let mut path = vec![goal];
        let mut current = goal;
        while current != start {
            for &(np, w) in &self.graph[&current] {
                if dist[&current] == dist[&np] + w {
                    path.push(np);
                    current = np;
                    break;
                }
            }
        }
        path.reverse();
        path
    }

    pub fn get_all_nodes(&self) -> Vec<Point> {
        self.graph.keys().cloned().collect()
    }
}
