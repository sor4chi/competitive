#![allow(non_snake_case, unused_macros)]

use crate::io::{Direction, Input, Operation, Rotation};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cmd {
    p: usize,
    r: bool,
    d: char,
    b: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
    pub r: bool,
    pub t: i32,
}

pub const P0: Pos = Pos {
    x1: -1,
    x2: -1,
    y1: -1,
    y2: -1,
    r: false,
    t: -1,
};

#[derive(Clone)]
pub struct State {
    pub turn: usize,
    /// (x1, x2, y1, y2, rot, t)
    pub pos: Vec<Pos>,
    pub W: i32,
    pub H: i32,
    pub score_t: i32,
    pub score: i32,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let score = input.rects.iter().map(|&(w, h)| w + h).sum::<usize>() as i32;
        Self {
            turn: 0,
            pos: vec![P0; input.N],
            W: 0,
            H: 0,
            score_t: score,
            score,
        }
    }
    pub fn query(&mut self, input: &Input, cmd: &[Operation]) -> Result<(), String> {
        self.pos.fill(P0);
        self.W = 0;
        self.H = 0;
        let mut prev = -1;
        for (t, c) in cmd.iter().enumerate() {
            if !prev.setmax(c.p as i32) {
                return Err(format!("p must be in ascending order."));
            }
            if self.pos[c.p].t >= 0 {
                return Err(format!("Rectangle {} is already used", c.p));
            } else if c.b >= 0 && self.pos[c.b as usize].t < 0 {
                return Err(format!("Rectangle {} is not used", c.b));
            }
            let (mut w, mut h) = (input.rects[c.p].0 as i32, input.rects[c.p].1 as i32);
            if c.r == Rotation::Rotate {
                std::mem::swap(&mut w, &mut h);
            }
            if c.d == Direction::Up {
                let x1 = if c.b < 0 {
                    0
                } else {
                    self.pos[c.b as usize].x2
                };
                let x2 = x1 + w;
                let y1 = self
                    .pos
                    .iter()
                    .filter_map(|q| {
                        if q.t >= 0 && x1.max(q.x1) < x2.min(q.x2) {
                            Some(q.y2)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);
                let y2 = y1 + h;
                self.pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r == Rotation::Rotate,
                    t: t as i32,
                };
            } else {
                let y1 = if c.b < 0 {
                    0
                } else {
                    self.pos[c.b as usize].y2
                };
                let y2 = y1 + h;
                let x1 = self
                    .pos
                    .iter()
                    .filter_map(|q| {
                        if q.t >= 0 && y1.max(q.y1) < y2.min(q.y2) {
                            Some(q.x2)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);
                let x2 = x1 + w;
                self.pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r == Rotation::Rotate,
                    t: t as i32,
                };
            }
            self.W.setmax(self.pos[c.p].x2);
            self.H.setmax(self.pos[c.p].y2);
        }
        self.score_t = self.W + self.H;
        for i in 0..input.N {
            if self.pos[i].t < 0 {
                self.score_t += (input.rects[i].0 + input.rects[i].1) as i32;
            }
        }
        self.score.setmin(self.score_t);
        self.turn += 1;
        Ok(())
    }
}
