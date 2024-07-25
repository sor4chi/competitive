use core::fmt;
use std::{
    fmt::{Debug, Formatter},
    io::{self, BufReader},
};

use proconio::{input, source::line::LineSource};

#[derive(Clone)]
pub struct Mino {
    pub id: usize,
    pub shape: Vec<Vec<bool>>,
    pub d: usize,
    pub width: usize,
    pub height: usize,
}

impl Debug for Mino {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "mino id: {}", self.id)?;
        for row in &self.shape {
            for cell in row {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Input {
    pub n: usize,
    pub m: usize,
    pub eps: f64,
    pub minos: Vec<Mino>,
}

pub struct IO {
    source: LineSource<BufReader<io::Stdin>>,
}

impl IO {
    pub fn new() -> Self {
        let source = LineSource::new(BufReader::new(io::stdin()));
        Self { source }
    }

    pub fn read(&mut self) -> Input {
        input! {
            from &mut self.source,
            n: usize,
            m: usize,
            eps: f64,
        }

        let mut minos = Vec::with_capacity(n);

        for i in 0..m {
            input! {
                from &mut self.source,
                d: usize,
            }

            let mut cells = Vec::with_capacity(d);
            let mut xm = 0;
            let mut ym = 0;

            for _ in 0..d {
                input! {
                    from &mut self.source,
                    x: usize,
                    y: usize,
                }

                cells.push((x, y));

                xm = xm.max(x);
                ym = ym.max(y);
            }

            let mut shape = vec![vec![false; ym + 1]; xm + 1];

            for (x, y) in cells {
                shape[x][y] = true;
            }

            minos.push(Mino {
                id: i,
                shape,
                d,
                width: ym + 1,
                height: xm + 1,
            });
        }

        Input { n, m, eps, minos }
    }

    pub fn query_dig(&mut self, x: usize, y: usize) -> usize {
        println!("q 1 {} {}", x, y);
        input! {
            from &mut self.source,
            d: usize,
        }
        d
    }

    pub fn query_divination(&mut self, v: Vec<(usize, usize)>) -> usize {
        let s = v
            .iter()
            .map(|(x, y)| format!("{} {}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        println!("q {} {}", v.len(), s);
        input! {
            from &mut self.source,
            d_hat: usize,
        }
        d_hat
    }

    pub fn debug_colorize(&self, x: usize, y: usize, color: &str) {
        println!("#c {} {} {}", x, y, color);
    }

    pub fn debug_clear(&self, input: &Input) {
        for i in 0..input.n {
            for j in 0..input.n {
                println!("#c {} {} white", i, j);
            }
        }
    }

    pub fn answer(&mut self, v: Vec<(usize, usize)>) -> bool {
        let s = v
            .iter()
            .map(|(x, y)| format!("{} {}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        println!("a {} {}", v.len(), s);
        input! {
            from &mut self.source,
            ok: usize,
        }
        ok == 1
    }
}
