use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use crate::solver::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let n = self.x * 70 + self.y;
        n.hash(state);
    }
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

    // startから各点への最短距離を返す
    pub fn dijkstra(&self, start: Point) -> HashMap<Point, usize> {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        dist.insert(start, 0);
        heap.push((Reverse(0), start));
        while let Some((Reverse(d), p)) = heap.pop() {
            if dist[&p] < d {
                continue;
            }
            for &(np, w) in &self.graph[&p] {
                if !dist.contains_key(&np) || dist[&np] > d + w {
                    dist.insert(np, d + w);
                    heap.push((Reverse(d + w), np));
                }
            }
        }
        dist
    }

    pub fn get_path(&self, start: Point, goal: Point, dist: &HashMap<Point, usize>) -> Vec<Point> {
        let mut path = vec![goal];
        let mut current = goal;
        while current != start {
            let mut next = Point::from((0, 0));
            for &(np, w) in &self.graph[&current] {
                if dist[&np] + w == dist[&current] {
                    next = np;
                    break;
                }
            }
            path.push(next);
            current = next;
        }
        path.reverse();
        path
    }

    pub fn get_all_nodes(&self) -> Vec<Point> {
        self.graph.keys().cloned().collect()
    }

    pub fn remove_node(&mut self, p: Point) {
        self.graph.remove(&p);
        for (_, edges) in self.graph.iter_mut() {
            edges.retain(|&(np, _)| np != p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = WeightedUndirectedGraph::new();
        graph.add_edge(Point::from((0, 0)), Point::from((1, 0)), 3);
        graph.add_edge(Point::from((1, 0)), Point::from((1, 1)), 2);
        graph.add_edge(Point::from((0, 0)), Point::from((0, 1)), 4);
        graph.add_edge(Point::from((0, 1)), Point::from((1, 1)), 2);
        let dist = graph.dijkstra(Point::from((0, 0)));
        assert_eq!(dist[&Point::from((0, 0))], 0);
        assert_eq!(dist[&Point::from((1, 0))], 3);
        assert_eq!(dist[&Point::from((1, 1))], 5);
    }
}
