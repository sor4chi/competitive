use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    graph::{Point, WeightedUndirectedGraph},
    Input,
};

#[derive(Debug, Clone)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        assert!(
            (a.x == b.x && a.y != b.y) || (a.x != b.x && a.y == b.y),
            "Line must be horizontal or vertical"
        );
        if a.x > b.x || (a.x == b.x && a.y > b.y) {
            Self { a: b, b: a }
        } else {
            Self { a, b }
        }
    }

    // 線分上に点pが存在するかどうか
    fn is_on(&self, p: Point) -> bool {
        let (x1, y1, x2, y2, x, y) = (
            self.a.x as isize,
            self.a.y as isize,
            self.b.x as isize,
            self.b.y as isize,
            p.x as isize,
            p.y as isize,
        );

        let (dx1, dy1) = (x - x1, y - y1);
        let (dx2, dy2) = (x2 - x, y2 - y);

        if dx1 * dy2 == dx2 * dy1 {
            if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                return true;
            }
        }
        false
    }

    /// 交差点を取得する, 交差しない場合はNoneを返す
    fn get_cross_point(&self, other: &Line) -> Option<Point> {
        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            self.a.x as isize,
            self.a.y as isize,
            self.b.x as isize,
            self.b.y as isize,
            other.a.x as isize,
            other.a.y as isize,
            other.b.x as isize,
            other.b.y as isize,
        );

        let (dx1, dy1) = (x2 - x1, y2 - y1);
        let (dx2, dy2) = (x4 - x3, y4 - y3);

        let s = dx1 * (y3 - y1) - dy1 * (x3 - x1);
        let t = dx1 * (y4 - y1) - dy1 * (x4 - x1);

        if s * t > 0 {
            return None;
        }

        let s = dx2 * (y1 - y3) - dy2 * (x1 - x3);
        let t = dx2 * (y2 - y3) - dy2 * (x2 - x3);

        if s * t > 0 {
            return None;
        }

        let s = s.abs();
        let t = t.abs();
        if s + t == 0 {
            return None;
        }

        let x = (x1 * t + x2 * s) / (s + t);
        let y = (y1 * t + y2 * s) / (s + t);

        if x < 0 || y < 0 {
            return None;
        }

        Some(Point::new(x as usize, y as usize))
    }

    pub fn get_inner_points(&self) -> HashSet<Point> {
        let mut set = HashSet::new();
        for x in self.a.x..=self.b.x {
            for y in self.a.y..=self.b.y {
                set.insert(Point::new(x, y));
            }
        }
        set
    }

    pub fn get_dir(&self) -> usize {
        // 0はVertical, 1はHorizontal
        if self.a.x == self.b.x {
            0
        } else {
            1
        }
    }
}

pub const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_map(c: Vec<Vec<char>>) -> Vec<Line> {
    let mut lines = Vec::new();
    let x_size = c.len();
    let y_size = c[0].len();

    // 縦のラインを作成
    for y in 0..y_size {
        let mut cur: Option<Point> = None;
        for x in 0..x_size {
            if x == x_size - 1 || c[x + 1][y] == '#' {
                if let Some(p) = cur {
                    if p.x < x - 1 {
                        lines.push(Line::new(p, Point::new(x, y)));
                    }
                    cur = None;
                }
            } else {
                if cur.is_none() && c[x][y] != '#' && x < x_size - 1 {
                    cur = Some(Point::new(x, y));
                }
            }
        }
    }

    // 横のラインを作成
    for x in 0..x_size {
        let mut cur: Option<Point> = None;
        for y in 0..y_size {
            if y == y_size - 1 || c[x][y + 1] == '#' {
                if let Some(p) = cur {
                    if p.y < y - 1 {
                        lines.push(Line::new(p, Point::new(x, y)));
                    }
                    cur = None;
                }
            } else {
                if cur.is_none() && c[x][y] != '#' && y < y_size - 1 {
                    cur = Some(Point::new(x, y));
                }
            }
        }
    }

    lines
}

fn get_resolve_map(lines: &Vec<Line>) -> HashMap<usize, HashSet<Point>> {
    let mut resolve_map = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let mut set = HashSet::new();
        for (j, other) in lines.iter().enumerate() {
            if i == j {
                continue;
            }
            if let Some(p) = line.get_cross_point(other) {
                set.insert(p);
            }
        }
        resolve_map.insert(i, set);
    }
    resolve_map
}

fn construct_graph(input: &Input) -> WeightedUndirectedGraph {
    let mut graph = WeightedUndirectedGraph::new();
    // BFSをする。交差点を見つけたら、その点をグラフに追加する
    struct State {
        current: Point,
        point: Point,
        cost: usize,
    }
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    fn is_cross_point(input: &Input, x: usize, y: usize) -> bool {
        let mut is_x_dir = false;
        let mut is_y_dir = false;
        for d in DIRECTIONS.iter() {
            let nx = x as isize + d.0;
            let ny = y as isize + d.1;
            if nx < 0 || ny < 0 || nx >= input.n as isize || ny >= input.n as isize {
                continue;
            }
            if input.c[nx as usize][ny as usize] == '#' {
                continue;
            }
            if nx == x as isize {
                is_y_dir = true;
            }
            if ny == y as isize {
                is_x_dir = true;
            }
        }
        is_x_dir && is_y_dir
    }
    let start = {
        let mut tmp = Point::new(0, 0);
        for x in 0..input.n {
            for y in 0..input.n {
                if input.c[x][y] == '#' {
                    continue;
                }
                if is_cross_point(input, x, y) {
                    tmp = Point::new(x, y);
                    break;
                }
            }
        }
        tmp
    };
    queue.push_back(State {
        point: start,
        current: start,
        cost: 0,
    });
    visited.insert(start);
    while let Some(State {
        point,
        current,
        cost,
    }) = queue.pop_front()
    {
        for (dx, dy) in DIRECTIONS.iter() {
            let np = Point::new(
                (point.x as isize + dx) as usize,
                (point.y as isize + dy) as usize,
            );
            if visited.contains(&np) {
                continue;
            }
            if np.x >= input.n || np.y >= input.n {
                continue;
            }
            visited.insert(np);
            if input.c[np.x][np.y] == '#' {
                continue;
            }
            let next_cost = cost + input.c[np.x][np.y].to_digit(10).unwrap() as usize;
            if is_cross_point(input, np.x, np.y) {
                graph.add_edge(current, np, next_cost);
                queue.push_back(State {
                    point: np,
                    current: np,
                    cost: 0,
                });
            } else {
                queue.push_back(State {
                    point: np,
                    current,
                    cost: next_cost,
                });
            }
        }
    }

    graph
}

#[derive(Clone)]
pub struct Game {
    /// 入力
    pub input: Input,
    /// ゲームボードが保持しているラインのリスト
    pub lines: Vec<Line>,
    /// ラインのID(linesのインデックス)とそこを通ることで消せる座標のコレクション
    pub resolve_map: HashMap<usize, HashSet<Point>>,
    /// 逆に、座標とそこを通るラインのIDのコレクション
    pub resolve_map_rev: HashMap<Point, HashSet<usize>>,
    /// 隣接リスト
    pub graph: WeightedUndirectedGraph,
}

impl Game {
    pub fn new(input: Input) -> Self {
        let mut lines = Vec::new();
        lines.append(&mut parse_map(input.c.clone()));
        let resolve_map = get_resolve_map(&lines);
        let graph = construct_graph(&input);
        let mut resolve_map_rev = HashMap::new();
        for (id, set) in resolve_map.iter() {
            for p in set.iter() {
                resolve_map_rev
                    .entry(*p)
                    .or_insert(HashSet::new())
                    .insert(*id);
            }
        }
        Self {
            input,
            lines,
            resolve_map,
            resolve_map_rev,
            graph,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _parse_map_str(c: &str) -> Vec<Vec<char>> {
        c.trim()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect()
    }

    #[test]
    fn test_line_new() {
        let a = Point::new(1, 1);
        let b = Point::new(2, 2);
        let line = Line::new(a, b);
        assert_eq!(line.a, Point::new(1, 1));
        assert_eq!(line.b, Point::new(2, 2));

        let a = Point::new(2, 2);
        let b = Point::new(1, 1);
        let line = Line::new(a, b);
        assert_eq!(line.a, Point::new(1, 1));
        assert_eq!(line.b, Point::new(2, 2));
    }

    #[test]
    fn test_line_is_on() {
        let a = Point::new(1, 1);
        let b = Point::new(1, 3);
        let line = Line::new(a, b);
        assert!(line.is_on(Point::new(1, 2)));
        assert!(!line.is_on(Point::new(1, 4)));
    }

    #[test]
    fn test_parse_map() {
        let c = r#"
55555
5#5#5
5#5#5
5#5#5
55555
"#;
        let lines = parse_map(_parse_map_str(c));
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_get_resolve_map() {
        let c = r#"
55555
5#5#5
5#5#5
5#5#5
55555
"#;
        let lines = parse_map(_parse_map_str(c));
        let resolve_map = get_resolve_map(&lines);
        assert_eq!(resolve_map.len(), 5);
        for (id, set) in resolve_map.iter() {
            eprintln!("id: {}, set: {:?}", id, set);
        }
    }

    #[test]
    fn test_construct_graph() {
        let c = r#"
55555
5#5#5
5#5#5
5#5#5
55555
"#;
        let input = Input {
            n: 5,
            s: (0, 0),
            c: _parse_map_str(c),
        };
        let wug = construct_graph(&input);
        for (point, edges) in wug.graph.iter() {
            eprintln!("point: {:?}, edges: {:?}", point, edges);
        }
    }
}
