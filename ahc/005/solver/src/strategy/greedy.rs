use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::game::Line;
use crate::graph::WeightedUndirectedGraph;

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

// // WeightedUndirectedGraphから指定した点を含む辺を削除する
// fn remove_line_shared_point(
//     lines: &[Line],
//     resolve_map_rev: &HashMap<Point, HashSet<usize>>,
//     graph: &WeightedUndirectedGraph,
//     p: Point,
// ) -> WeightedUndirectedGraph {
//     let mut graph = graph.clone();
//     if let Some(crossing_lines) = resolve_map_rev.get(&p) {
//         for line_id in crossing_lines.iter() {
//             let line = &lines[*line_id];
//             for point in line.get_inner_points() {
//                 graph.remove_node(point);
//             }
//         }
//     }
//     graph
// }

impl Strategy for GreedyStrategy<'_> {
    fn solve(&self) -> Vec<Direction> {
        let mut game = self.game.clone();
        // とりあえずgraphを頼りに全交差点を通るような経路を作る
        let mut path = vec![];
        let mut graph = game.graph.clone();
        let mut current = Point::from(game.input.s);
        // 一番近くの交差点まで移動
        let all_nodes = game.graph.get_all_nodes();
        let mut first = Point::from((0, 0));
        let mut queue = VecDeque::from(vec![current]);
        let mut visited = HashSet::new();
        let x_size = game.input.c.len();
        let y_size = game.input.c[0].len();
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
                if game.input.c[np.x][np.y] == '#' {
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
        let mut have_to_visit = graph.get_all_nodes().into_iter().collect::<HashSet<_>>();
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
                let mut dir = 0; // 0はVertical, 1はHorizontal
                while current.x < p.x {
                    path.push(Direction::Down);
                    current.x += 1;
                    dir = 0;
                }
                while current.x > p.x {
                    path.push(Direction::Up);
                    current.x -= 1;
                    dir = 0;
                }
                while current.y < p.y {
                    path.push(Direction::Right);
                    current.y += 1;
                    dir = 1;
                }
                while current.y > p.y {
                    path.push(Direction::Left);
                    current.y -= 1;
                    dir = 1;
                }
                // 通過した点と、そのラインの全ての点をhave_to_visitから削除
                have_to_visit.remove(p);
                if let Some(crossing_lines) = game.resolve_map_rev.clone().get(p) {
                    for line_id in crossing_lines.iter() {
                        game.resolve_map.remove(line_id);
                        for (point, line_ids) in &game.resolve_map_rev.clone() {
                            if line_ids.contains(line_id) {
                                game.resolve_map_rev.get_mut(point).unwrap().remove(line_id);
                            }
                        }
                    }
                }
                for (point, line_ids) in &game.resolve_map_rev.clone() {
                    if line_ids.is_empty() {
                        have_to_visit.remove(point);
                    }
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

        while current.x < game.input.s.0 {
            path.push(Direction::Down);
            current.x += 1;
        }
        while current.x > game.input.s.0 {
            path.push(Direction::Up);
            current.x -= 1;
        }
        while current.y < game.input.s.1 {
            path.push(Direction::Right);
            current.y += 1;
        }
        while current.y > game.input.s.1 {
            path.push(Direction::Left);
            current.y -= 1;
        }

        path
    }
}
