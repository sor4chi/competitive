mod original_lib;
use std::cmp::min;

use crate::original_lib::{gen as original_gen, Input};
use original_lib::{can_move, compute_diff, parse_input, Output, DIJ};
use proconio::input;
use svg::node::element::{Circle, Group, Line, Rectangle, Text};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let i = original_gen(seed as u64);
    format!("{}", i)
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

const SVG_WIDTH: usize = 800;
const SVG_HEIGHT: usize = 800;
const COLOR_HOTTEST_HSLA: &str = "hsl(349, 100%, 56%, 0.8)"; // #ff1e46 * 0.8
const COLOR_COOLEST_HSLA: &str = "hsl(210, 100%, 56%, 0.8)"; // #1e90ff * 0.8

#[derive(Debug, Clone, Copy)]
struct HslaColor {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

fn decode_to_hsla(s: &str) -> HslaColor {
    let s2 = s
        .trim_start_matches("hsl(")
        .trim_end_matches(')')
        .split(',')
        .collect::<Vec<_>>();
    let h = s2[0].parse::<f64>().unwrap();
    let s = s2[1].trim().trim_end_matches('%').parse::<f64>().unwrap();
    let l = s2[2].trim().trim_end_matches('%').parse::<f64>().unwrap();
    let a = s2[3].trim().parse::<f64>().unwrap();
    HslaColor { h, s, l, a }
}

fn encode_to_hsla(c: HslaColor) -> String {
    format!("hsla({}, {}%, {}%, {})", c.h, c.s, c.l, c.a)
}

fn get_colors(cnt: usize) -> Vec<HslaColor> {
    let mut colors = vec![];
    let hottest = decode_to_hsla(COLOR_HOTTEST_HSLA);
    let coolest = decode_to_hsla(COLOR_COOLEST_HSLA);
    let mut h = coolest.h;
    let mut s = coolest.s;
    let mut l = coolest.l;
    let mut a = coolest.a;
    let dh = (coolest.h - hottest.h + 360.0) / (cnt as f64);
    let ds = (hottest.s - coolest.s) / (cnt as f64);
    let dl = (hottest.l - coolest.l) / (cnt as f64);
    let da = (hottest.a - coolest.a) / (cnt as f64);
    for _ in 0..cnt {
        colors.push(HslaColor { h, s, l, a });
        h = (h - dh) % 360.0;
        s += ds;
        l += dl;
        a += da;
    }
    colors
}

struct MapState {
    a: Vec<Vec<i32>>,
    p1: (usize, usize),
    p2: (usize, usize),
}

fn compute_map_state(
    input: &Input,
    start: (usize, usize, usize, usize),
    out: &[(bool, usize, usize)],
) -> MapState {
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);
    for &(do_swap, dir1, dir2) in out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                unreachable!();
            }
            p1.0 += DIJ[dir1].0;
            p1.1 += DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                unreachable!();
            }
            p2.0 += DIJ[dir2].0;
            p2.1 += DIJ[dir2].1;
        }
    }
    MapState { a, p1, p2 }
}

fn generate_svg(input: &Input, output: &Output) -> String {
    let state = compute_map_state(input, output.start, &output.out);
    let colors = get_colors(input.n * input.n);
    let cell_width = min(SVG_WIDTH / input.n, 50);
    let cell_height = min(SVG_HEIGHT / input.n, 50);
    let mut group = Group::new();
    for i in 0..input.n {
        for j in 0..input.n {
            let x = j * cell_width;
            let y = i * cell_height;
            let cell_num = state.a[i][j];
            let color_index = (cell_num - 1) as usize;
            let rect = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", cell_width)
                .set("height", cell_height)
                .set("fill", encode_to_hsla(colors[color_index]))
                .set("stroke", "#4444")
                .set("stroke-width", 1);
            group = group.add(rect);
            let text = Text::new()
                .set("x", x + cell_width / 2)
                .set("y", y + cell_height / 2)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("font-size", cell_height / 3)
                .set("fill", "black")
                .add(svg::node::Text::new(format!("{}", cell_num)));
            group = group.add(text);
        }
    }
    for i in 0..input.n {
        for j in 0..input.n - 1 {
            let ch = input.vs[i][j];
            if ch == '0' {
                continue;
            }
            let x = j * cell_width + cell_width;
            let y = i * cell_height;
            let line = Line::new()
                .set("x1", x)
                .set("y1", y)
                .set("x2", x)
                .set("y2", y + cell_height)
                .set("stroke", "black")
                .set("stroke-width", 3);
            group = group.add(line);
        }
    }
    for i in 0..input.n - 1 {
        for j in 0..input.n {
            let ch = input.hs[i][j];
            if ch == '0' {
                continue;
            }
            let x = j * cell_width;
            let y = i * cell_height + cell_height;
            let line = Line::new()
                .set("x1", x)
                .set("y1", y)
                .set("x2", x + cell_width)
                .set("y2", y)
                .set("stroke", "black")
                .set("stroke-width", 3);
            group = group.add(line);
        }
    }
    let (p1x, p1y) = state.p1;
    let (p2x, p2y) = state.p2;
    let p1circle = Circle::new()
        .set("cx", p1y * cell_width + cell_width / 2)
        .set("cy", p1x * cell_height + cell_height / 2)
        .set("r", cell_width / 3)
        .set("fill", "#ff1e46aa")
        .set("stroke", "#ff1e46")
        .set("stroke-width", 2);
    let p2circle = Circle::new()
        .set("cx", p2y * cell_width + cell_width / 2)
        .set("cy", p2x * cell_height + cell_height / 2)
        .set("r", cell_width / 3)
        .set("fill", "#1e90ffaa")
        .set("stroke", "#1e90ff")
        .set("stroke-width", 2);
    group = group.add(p1circle).add(p2circle);
    let svg = svg::Document::new()
        .set("width", SVG_WIDTH)
        .set("height", SVG_HEIGHT)
        .add(group);
    svg.to_string()
}

fn generate_yakinamashi_svg(input: &Input, output: &YakinamashiOutput, turn: usize) -> String {
    let state = output.steps[turn].clone();
    let colors = get_colors(input.n * input.n);
    let cell_width = min(SVG_WIDTH / input.n, 50);
    let cell_height = min(SVG_HEIGHT / input.n, 50);
    let mut group = Group::new();
    for i in 0..input.n {
        for j in 0..input.n {
            let x = j * cell_width;
            let y = i * cell_height;
            let cell_num = state[i][j];
            let color_index = (cell_num - 1) as usize;
            let rect = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", cell_width)
                .set("height", cell_height)
                .set("fill", encode_to_hsla(colors[color_index]))
                .set("stroke", "#4444")
                .set("stroke-width", 1);
            group = group.add(rect);
            let text = Text::new()
                .set("x", x + cell_width / 2)
                .set("y", y + cell_height / 2)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("font-size", cell_height / 3)
                .set("fill", "black")
                .add(svg::node::Text::new(format!("{}", cell_num)));
            group = group.add(text);
        }
    }
    for i in 0..input.n {
        for j in 0..input.n - 1 {
            let ch = input.vs[i][j];
            if ch == '0' {
                continue;
            }
            let x = j * cell_width + cell_width;
            let y = i * cell_height;
            let line = Line::new()
                .set("x1", x)
                .set("y1", y)
                .set("x2", x)
                .set("y2", y + cell_height)
                .set("stroke", "black")
                .set("stroke-width", 3);
            group = group.add(line);
        }
    }
    for i in 0..input.n - 1 {
        for j in 0..input.n {
            let ch = input.hs[i][j];
            if ch == '0' {
                continue;
            }
            let x = j * cell_width;
            let y = i * cell_height + cell_height;
            let line = Line::new()
                .set("x1", x)
                .set("y1", y)
                .set("x2", x + cell_width)
                .set("y2", y)
                .set("stroke", "black")
                .set("stroke-width", 3);
            group = group.add(line);
        }
    }
    let svg = svg::Document::new()
        .set("width", SVG_WIDTH)
        .set("height", SVG_HEIGHT)
        .add(group);
    svg.to_string()
}

struct YakinamashiOutput {
    steps: Vec<Vec<Vec<i32>>>,
}

fn parse_yakinamashi_output(input: &Input, f: &str) -> Result<YakinamashiOutput, String> {
    // split f to lines
    let f = f
        .trim()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    let iter = f.len() / input.n;
    // parse f to steps, f includes n * n's array * iter times
    let mut steps = vec![];
    for i in 0..iter {
        let mut step = vec![];
        for j in 0..input.n {
            let line = &f[i * input.n + j];
            let row = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            step.push(row);
        }
        steps.push(step);
    }
    Ok(YakinamashiOutput { steps })
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parse_input(&_input);
    let out = parse_yakinamashi_output(&input, &_output);
    let before = compute_diff(&input, &input.a);
    let (score, err, svg) = match out {
        Ok(out) => {
            let after = compute_diff(&input, &out.steps[turn]);
            let score = ((1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round()
                as i64)
                .max(1);
            let svg = generate_yakinamashi_svg(&input, &out, turn);
            (score, "".to_string(), svg)
        }
        Err(err) => (0, err, "".to_string()),
    };
    Ret { score, err, svg }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = parse_input(&_input);
    let out = parse_yakinamashi_output(&input, &_output);
    match out {
        Ok(out) => out.steps.len(),
        Err(_) => 0,
    }
}
