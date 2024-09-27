use std::{
    collections::{HashMap, VecDeque},
    process::Command,
};

pub fn visualize_score_transition(scores: &[usize], filename: &str) {
    eprintln!("plotting {}...", filename);
    let mut python_code = String::new();
    python_code.push_str("import matplotlib.pyplot as plt\n");
    python_code.push_str("fig, ax = plt.subplots(1,1, figsize=(10,10))\n");
    python_code.push_str("ax.plot([");
    python_code.push_str(
        &scores
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    python_code.push_str("])\n");
    python_code.push_str(format!("plt.savefig('{}')\n", filename).as_str());

    Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .spawn()
        .unwrap();
}
