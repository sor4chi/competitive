mod original_lib;
use wasm_bindgen::prelude::*;

use crate::original_lib::gen as original_gen;
use original_lib::{parse_input, parse_output, vis as original_vis};

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let input = original_gen(seed as u64);
    input.to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parse_input(_input.as_str());
    let output = parse_output(&input, _output.as_str());
    let (score, err, svg) = match output {
        Ok(out) => original_vis(&input, &out, turn),
        Err(err) => (0, err, String::new()),
    };
    Ret { score, err, svg }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = parse_input(_input.as_str());
    let output = parse_output(&input, _output.as_str());
    match output {
        Ok(output) => {
            let mut max_len = 0;
            for i in 0..output.out.len() {
                max_len = max_len.max(output.out[i].len());
            }
            max_len
        }
        Err(_) => 0,
    }
}
