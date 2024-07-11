use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashSet,
    io::{stdin, BufReader},
};

use proconio::{input, source::line::LineSource};
use solver::{
    game::{Game, M, N},
    unionfind::UnionFind,
};

extern crate solver;

struct EdgeWithDist {
    i: usize,
    u: usize,
    v: usize,
    dist: usize,
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        points: [(usize, usize); N],
        edges: [(usize, usize); M],
    }

    let game = Game::new(points, edges);
    let mut real_dist = vec![0; M];
    let mut already_used = HashSet::new();
    let mut already_unused = HashSet::new();

    for i in 0..M {
        input! {
            from &mut source,
            j: usize,
        }

        real_dist[i] = j;

        // クラスカル法で最小全域木を求める
        let mut uf = UnionFind::new(N);
        // 採用する辺のindex
        let mut used = HashSet::new();
        let edges = game.edges.clone();
        // コストはuとvの距離
        let mut edge_with_dists = edges
            .iter()
            .enumerate()
            .map(|(k, &(u, v))| EdgeWithDist {
                i: k,
                u,
                v,
                dist: {
                    if real_dist[k] == 0 {
                        game.dist(u, v) * 2
                    } else {
                        real_dist[k]
                    }
                },
            })
            .collect::<Vec<_>>();

        // すでに使われている辺は優先して使う
        for already_used_edge in already_used.iter() {
            let used_edge: (usize, usize) = edges[*already_used_edge];
            uf.unite(used_edge.0, used_edge.1);
            used.insert(*already_used_edge);
        }

        // コストが小さい順にソート、ただしalready_usedに含まれる辺はさらに優先する
        edge_with_dists.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        for edge_with_dist in edge_with_dists {
            if uf.issame(edge_with_dist.u, edge_with_dist.v) {
                continue;
            }
            if already_unused.contains(&edge_with_dist.i) {
                continue;
            }
            uf.unite(edge_with_dist.u, edge_with_dist.v);
            used.insert(edge_with_dist.i);
        }

        eprintln!("turn: {}", i);

        println!("{}", if used.contains(&i) { 1 } else { 0 });
        if used.contains(&i) {
            already_used.insert(i);
        } else {
            already_unused.insert(i);
        }
    }
}
