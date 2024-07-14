use std::collections::{HashSet, VecDeque};

use super::super::{game::Game, game::DIRECTIONS, graph::Point};
use super::{Direction, Strategy};

pub struct GreedyStrategy<'a> {
    game: &'a Game,
}

impl GreedyStrategy<'_> {
    pub fn new(game: &Game) -> GreedyStrategy {
        GreedyStrategy { game }
    }
}

impl Strategy for GreedyStrategy<'_> {
    fn solve(&self) -> Vec<Direction> {
        // とりあえずgraphを頼りに全交差点を通るような経路を作る
        let mut path = vec![];
        let graph = self.game.graph.clone();
        let mut current = Point::from(self.game.input.s);
        // 一番近くの交差点まで移動
        let all_nodes = self.game.graph.get_all_nodes();
        let mut first = Point::from((0, 0));
        let mut queue = VecDeque::from(vec![current]);
        let mut visited = HashSet::new();
        let x_size = self.game.input.c.len();
        let y_size = self.game.input.c[0].len();
        visited.insert(current);
        while let Some(current) = queue.pop_front() {
            if all_nodes.contains(&current) {
                first = current;
                break;
            }
            for (dx, dy) in DIRECTIONS.iter() {
                let (nx, ny) = (current.x as isize + dx, current.y as isize + dy);
                if nx < 0 || ny < 0 || nx >= x_size as isize || ny >= y_size as isize {
                    continue;
                }
                let np = Point::from((nx as usize, ny as usize));
                if visited.contains(&np) {
                    continue;
                }
                visited.insert(np);
                if self.game.input.c[np.x][np.y] == '#' {
                    continue;
                }
                queue.push_back(np);
            }
        }
        while current != first {
            while current.x < first.x {
                path.push(Direction::Down);
                current.x += 1;
            }
            while current.x > first.x {
                path.push(Direction::Up);
                current.x -= 1;
            }
            while current.y < first.y {
                path.push(Direction::Right);
                current.y += 1;
            }
            while current.y > first.y {
                path.push(Direction::Left);
                current.y -= 1;
            }
        }
        let mut have_to_visit = self
            .game
            .graph
            .get_all_nodes()
            .into_iter()
            .collect::<HashSet<_>>();
        have_to_visit.remove(&current);
        while !have_to_visit.is_empty() {
            let dij = graph.dijkstra(current);
            let mut min_dist = std::usize::MAX;
            let mut next = Point::from((0, 0));
            for &p in have_to_visit.iter() {
                if let Some(dist) = dij.get(&p) {
                    if *dist < min_dist {
                        min_dist = *dist;
                        next = p;
                    }
                }
            }
            let to_the_next_path = graph.get_path(current, next, &dij);
            for p in to_the_next_path.iter().skip(1) {
                while current.x < p.x {
                    path.push(Direction::Down);
                    current.x += 1;
                }
                while current.x > p.x {
                    path.push(Direction::Up);
                    current.x -= 1;
                }
                while current.y < p.y {
                    path.push(Direction::Right);
                    current.y += 1;
                }
                while current.y > p.y {
                    path.push(Direction::Left);
                    current.y -= 1;
                }
            }
            have_to_visit.remove(&next);
        }
        // 最後にゴールに向かう
        let to_goal_path = graph.get_path(current, first, &graph.dijkstra(current));
        for p in to_goal_path.iter().skip(1) {
            while current.x < p.x {
                path.push(Direction::Down);
                current.x += 1;
            }
            while current.x > p.x {
                path.push(Direction::Up);
                current.x -= 1;
            }
            while current.y < p.y {
                path.push(Direction::Right);
                current.y += 1;
            }
            while current.y > p.y {
                path.push(Direction::Left);
                current.y -= 1;
            }
        }

        while current.x < self.game.input.s.0 {
            path.push(Direction::Down);
            current.x += 1;
        }
        while current.x > self.game.input.s.0 {
            path.push(Direction::Up);
            current.x -= 1;
        }
        while current.y < self.game.input.s.1 {
            path.push(Direction::Right);
            current.y += 1;
        }
        while current.y > self.game.input.s.1 {
            path.push(Direction::Left);
            current.y -= 1;
        }

        path
    }
}
