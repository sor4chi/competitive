use std::{
    collections::{HashMap, VecDeque},
    process::Command,
};

use crate::original_lib::color::get_rgba_gradient;

pub fn visualize_a(
    order: &[usize],
    nodes: &[(usize, usize)],
    graph: &HashMap<usize, Vec<usize>>,
    filename: &str,
) {
    let mut components = vec![];
    let mut queue = order.iter().copied().collect::<VecDeque<_>>();
    let mut current_component = vec![];
    while let Some(cur) = queue.pop_front() {
        if current_component.is_empty() {
            current_component.push(cur);
        } else {
            let last = *current_component.last().unwrap();
            if graph[&last].contains(&cur) {
                current_component.push(cur);
            } else {
                components.push(current_component.clone());
                current_component.clear();
                current_component.push(cur);
            }
        }
    }
    if !current_component.is_empty() {
        components.push(current_component);
    }

    visualize_components(&components, nodes, graph, filename);
}

pub fn visualize_components(
    components: &[Vec<usize>],
    nodes: &[(usize, usize)],
    graph: &HashMap<usize, Vec<usize>>,
    filename: &str,
) {
    eprintln!("plotting {}...", filename);
    let mut python_code = String::new();
    python_code.push_str("import matplotlib.pyplot as plt\n");
    python_code.push_str("fig, ax = plt.subplots(1,1, figsize=(10,10))\n");
    python_code.push_str("ax.invert_yaxis()\n");
    // 最初に薄い灰色の点で全ての点と線を描画
    let x = nodes.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let y = nodes.iter().map(|&(_, y)| y).collect::<Vec<_>>();
    python_code.push_str(&format!(
        "ax.scatter({:?}, {:?}, color='#cccccc', s=5)\n",
        x, y
    ));
    for (i, j) in nodes.iter().enumerate() {
        for &k in graph.get(&i).unwrap() {
            let (x1, y1) = j;
            let (x2, y2) = nodes[k];
            python_code.push_str(&format!(
                "ax.plot([{}, {}], [{}, {}], color='#eeeeee', linewidth=0.5)\n",
                x1, x2, y1, y2
            ));
        }
    }
    // tspの要領で座標がnodesに格納されているのでorderに従って線で結びながら描画
    // orderを最初から順に見て、order[i]とorder[i+1]がgraph上でつながっているなら線を引く
    let mut colors = get_rgba_gradient(components.len());
    for component in components.iter() {
        // 色を変えながら描画
        let color = colors.pop().unwrap();
        let color = format!(
            "({}, {}, {}, {})",
            (color.r as f64) / 255.0,
            (color.g as f64) / 255.0,
            (color.b as f64) / 255.0,
            color.a
        );

        let nodes = component.iter().map(|&i| nodes[i]).collect::<Vec<_>>();
        let x = nodes.iter().map(|&(x, _)| x).collect::<Vec<_>>();
        let y = nodes.iter().map(|&(_, y)| y).collect::<Vec<_>>();
        python_code.push_str(&format!(
            "ax.plot({:?}, {:?}, color={}, linewidth=1)\n",
            x, y, color
        ));
        python_code.push_str(&format!(
            "ax.scatter({:?}, {:?}, color={}, s=10)\n",
            x, y, color
        ));
    }

    python_code.push_str(format!("plt.savefig('{}')\n", filename).as_str());

    Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .spawn()
        .unwrap();
}
