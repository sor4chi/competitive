use std::collections::{HashMap, HashSet};

use crate::{util::IdGenerator, Input};

#[derive(Debug)]
struct Line {
    a: (usize, usize),
    b: (usize, usize),
}

impl Line {
    fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        if a.0 > b.0 || (a.0 == b.0 && a.1 > b.1) {
            Self { a: b, b: a }
        } else {
            Self { a, b }
        }
    }

    // 線分上に点pが存在するかどうか
    fn is_on(&self, p: (usize, usize)) -> bool {
        let (x1, y1, x2, y2, x, y) = (
            self.a.0 as isize,
            self.a.1 as isize,
            self.b.0 as isize,
            self.b.1 as isize,
            p.0 as isize,
            p.1 as isize,
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
    fn get_cross_point(&self, other: &Line) -> Option<(usize, usize)> {
        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            self.a.0 as isize,
            self.a.1 as isize,
            self.b.0 as isize,
            self.b.1 as isize,
            other.a.0 as isize,
            other.a.1 as isize,
            other.b.0 as isize,
            other.b.1 as isize,
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

        let x = (x1 * t + x2 * s) / (s + t);
        let y = (y1 * t + y2 * s) / (s + t);

        if x < 0 || y < 0 {
            return None;
        }

        Some((x as usize, y as usize))
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_map(c: Vec<Vec<char>>) -> Vec<Line> {
    let mut lines = Vec::new();
    let x_size = c.len();
    let y_size = c[0].len();

    // 縦のラインを作成
    for y in 0..y_size {
        let mut cur = None;
        for x in 0..x_size {
            if x == x_size - 1 || c[x + 1][y] == '#' {
                if let Some((x1, y1)) = cur {
                    if x1 < x - 1 {
                        lines.push(Line::new((x1, y1), (x, y)));
                    }
                    cur = None;
                }
            } else {
                if cur.is_none() && c[x][y] != '#' && x < x_size - 1 {
                    cur = Some((x, y));
                }
            }
        }
    }

    // 横のラインを作成
    for x in 0..x_size {
        let mut cur = None;
        for y in 0..y_size {
            if y == y_size - 1 || c[x][y + 1] == '#' {
                if let Some((x1, y1)) = cur {
                    if y1 < y - 1 {
                        lines.push(Line::new((x1, y1), (x, y)));
                    }
                    cur = None;
                }
            } else {
                if cur.is_none() && c[x][y] != '#' && y < y_size - 1 {
                    cur = Some((x, y));
                }
            }
        }
    }

    lines
}

fn get_resolve_map(lines: &Vec<Line>) -> HashMap<usize, HashSet<(usize, usize)>> {
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

pub struct Game {
    /// 入力
    input: Input,
    /// ゲームボードが保持しているラインのリスト
    lines: Vec<Line>,
    /// ラインのID(linesのインデックス)とそこを通ることで消せる座標のコレクション
    resolve_map: HashMap<usize, HashSet<(usize, usize)>>,
}

impl Game {
    pub fn new(input: Input) -> Self {
        let mut lines = Vec::new();
        lines.append(&mut parse_map(input.c.clone()));
        let resolve_map = get_resolve_map(&lines);
        Self {
            input,
            lines,
            resolve_map,
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
        let a = (1, 1);
        let b = (2, 2);
        let line = Line::new(a, b);
        assert_eq!(line.a, (1, 1));
        assert_eq!(line.b, (2, 2));

        let a = (2, 2);
        let b = (1, 1);
        let line = Line::new(a, b);
        assert_eq!(line.a, (1, 1));
        assert_eq!(line.b, (2, 2));
    }

    #[test]
    fn test_line_is_on() {
        let a = (1, 1);
        let b = (1, 3);
        let line = Line::new(a, b);
        assert!(line.is_on((1, 2)));
        assert!(!line.is_on((1, 4)));
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
}
