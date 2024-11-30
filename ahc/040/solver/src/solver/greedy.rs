use std::{fmt::Display, io::Write};

use rand::{seq::SliceRandom, Rng};

use crate::io::{Direction, Input, Operation, Query, Rotation, IO};

use std::fs::File;

use super::Solver;

#[derive(Clone, Debug)]
struct Rect {
    id: usize,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    w: usize,
    h: usize,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect({}: {} {} {} {})",
            self.id, self.x1, self.y1, self.x2, self.y2
        )
    }
}

impl Rect {
    fn new(id: usize, x: usize, y: usize, w: usize, h: usize) -> Rect {
        Rect {
            id,
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
            w,
            h,
        }
    }

    fn overlap(&self, other: &Rect) -> bool {
        self.x1 < other.x2 && self.x2 > other.x1 && self.y1 < other.y2 && self.y2 > other.y1
    }

    fn subtract(&self, other: &Rect) -> Vec<Rect> {
        if !self.overlap(other) {
            return vec![self.clone()];
        }
        let mut rects = vec![];
        if self.x1 < other.x1 {
            rects.push(Rect::new(
                self.id,
                self.x1,
                self.y1,
                other.x1 - self.x1,
                self.h,
            ));
        }
        if self.x2 > other.x2 {
            rects.push(Rect::new(
                self.id,
                other.x2,
                self.y1,
                self.x2 - other.x2,
                self.h,
            ));
        }
        if self.y1 < other.y1 {
            rects.push(Rect::new(
                self.id,
                self.x1,
                self.y1,
                self.w,
                other.y1 - self.y1,
            ));
        }
        if self.y2 > other.y2 {
            rects.push(Rect::new(
                self.id,
                self.x1,
                other.y2,
                self.w,
                self.y2 - other.y2,
            ));
        }
        rects
    }

    fn include(&self, other: &Rect) -> bool {
        self.x1 <= other.x1 && self.y1 <= other.y1 && self.x2 >= other.x2 && self.y2 >= other.y2
    }

    fn larger_than(&self, w: usize, h: usize) -> bool {
        w <= self.w && h <= self.h
    }

    fn x_connected(&self, other: &Rect) -> bool {
        self.x1 == other.x2 || self.x2 == other.x1
    }

    fn y_connected(&self, other: &Rect) -> bool {
        self.y1 == other.y2 || self.y2 == other.y1
    }
}

pub struct GreedySolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl GreedySolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> GreedySolver<'a> {
        GreedySolver { input, io }
    }
}

const COMPRESS_SIZE: usize = 1000;

fn _debug_rects(rects: &[Rect]) {
    let mut svg = String::new();
    let folder = "plots";
    std::fs::remove_dir_all(folder);
    std::fs::create_dir_all(folder);
    svg.push_str(
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 1000" width="1000" height="1000" style="background-color: #eee;">"#
    );
    for (i, rect) in rects.iter().enumerate() {
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="red" fill-opacity="0.5" />"#,
            rect.x1, rect.y1, rect.w, rect.h
        ));
        let svg_text = svg.clone() + "</svg>";
        File::create(format!("{}/{}.svg", folder, i))
            .unwrap()
            .write_all(svg_text.as_bytes())
            .unwrap();
    }
}

impl Solver for GreedySolver<'_> {
    fn solve(&mut self) {
        let total_area: usize = self.input.rects.iter().map(|(w, h)| w * h).sum();
        let expected_area = total_area * 5 / 4;
        let expected_side = (expected_area as f64).sqrt() as usize / COMPRESS_SIZE;
        let mut rects: Vec<Rect> = vec![];
        let mut operations: Vec<Operation> = vec![];
        for (id, (w, h)) in self.input.rects.iter().enumerate() {
            let w = *w / COMPRESS_SIZE;
            let h = *h / COMPRESS_SIZE;
            let mut x = 0;
            let mut y = 0;
            loop {
                {
                    let rect_normal = Rect::new(id, x, y, w, h);
                    if x + w <= expected_side && rects.iter().all(|r| !r.overlap(&rect_normal)) {
                        let mut op = Operation {
                            p: id,
                            r: Rotation::Stay,
                            d: Direction::Left,
                            b: -1,
                        };
                        if rect_normal.y1 == 0 {
                            op = Operation {
                                p: id,
                                r: Rotation::Stay,
                                d: Direction::Left,
                                b: -1,
                            };
                        } else {
                            for r in &rects {
                                if r.y_connected(&rect_normal) {
                                    op = Operation {
                                        p: id,
                                        r: Rotation::Stay,
                                        d: Direction::Left,
                                        b: r.id as isize,
                                    };
                                    break;
                                }
                            }
                        }

                        if rect_normal.x1 == 0 {
                            op = Operation {
                                p: id,
                                r: Rotation::Stay,
                                d: Direction::Up,
                                b: -1,
                            };
                        } else {
                            for r in &rects {
                                if r.x_connected(&rect_normal) {
                                    op = Operation {
                                        p: id,
                                        r: Rotation::Stay,
                                        d: Direction::Up,
                                        b: r.id as isize,
                                    };
                                    break;
                                }
                            }
                        }

                        operations.push(op);
                        rects.push(rect_normal);
                        break;
                    }
                }
                {
                    let rect_rotated = Rect::new(id, x, y, h, w);
                    if x + h <= expected_side && rects.iter().all(|r| !r.overlap(&rect_rotated)) {
                        let mut op = Operation {
                            p: id,
                            r: Rotation::Rotate,
                            d: Direction::Left,
                            b: -1,
                        };
                        if rect_rotated.y1 == 0 {
                            op = Operation {
                                p: id,
                                r: Rotation::Rotate,
                                d: Direction::Left,
                                b: -1,
                            };
                        } else {
                            for r in &rects {
                                if r.y_connected(&rect_rotated) {
                                    op = Operation {
                                        p: id,
                                        r: Rotation::Rotate,
                                        d: Direction::Left,
                                        b: r.id as isize,
                                    };
                                    break;
                                }
                            }
                        }

                        if rect_rotated.x1 == 0 {
                            op = Operation {
                                p: id,
                                r: Rotation::Rotate,
                                d: Direction::Up,
                                b: -1,
                            };
                        } else {
                            for r in &rects {
                                if r.x_connected(&rect_rotated) {
                                    op = Operation {
                                        p: id,
                                        r: Rotation::Rotate,
                                        d: Direction::Up,
                                        b: r.id as isize,
                                    };
                                    break;
                                }
                            }
                        }

                        operations.push(op);
                        rects.push(rect_rotated);
                        break;
                    }
                    x += 1;
                    if x + w > expected_side {
                        x = 0;
                        y += 1;
                    }
                }
            }
        }
        let query = Query { operations };
        // _debug_rects(&rects);
        for _ in 0..self.input.T {
            self.io.measure(&query);
        }
    }
}
