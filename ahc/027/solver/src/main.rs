use proconio::input;
use rand;
use std::time::Instant;

struct Input {
    n: usize,
    h: Vec<Vec<u32>>,
    v: Vec<Vec<u32>>,
    d: Vec<Vec<u32>>,
}

#[derive(PartialEq, Clone)]
enum Operation {
    L,
    R,
    U,
    D,
}

#[derive(Clone, PartialEq, Debug)]
struct State {
    operations: Vec<u8>,
    current: (usize, usize),
    total: u64,
    each: u32,
    d: Vec<Vec<u32>>,
    turn: u32,
}

impl State {
    // 初期状態の生成
    fn new(input: &Input) -> State {
        let current = (0, 0);
        let operations = vec![];

        // dの合計
        let mut each = 0;
        for i in 0..input.n {
            for j in 0..input.n {
                each += input.d[i][j];
            }
        }

        State {
            operations,
            current,
            total: 0,
            each,
            d: input.d.clone(),
            turn: 0,
        }
    }

    // 差分更新で適用する
    fn apply(&mut self, node: &Node) {
        // 操作を適用
        self.operations.push(node.op);
        match node.op {
            0 => self.current.1 -= 1,
            1 => self.current.1 += 1,
            2 => self.current.0 -= 1,
            3 => self.current.0 += 1,
            _ => unreachable!(),
        }
        self.total += (self.each - self.d[self.current.0][self.current.1]) as u64;
        self.turn += 1;
    }

    // 差分更新で元に戻す
    fn revert(&mut self, node: &Node) {
        self.total -= (self.each - self.d[self.current.0][self.current.1]) as u64;
        // 操作を戻す
        match node.op {
            0 => self.current.1 += 1,
            1 => self.current.1 -= 1,
            2 => self.current.0 += 1,
            3 => self.current.0 -= 1,
            _ => unreachable!(),
        }
        self.operations.pop();
        self.turn -= 1;
    }
}

#[derive(Clone)]
struct Cand {
    op: u8,
    parent: usize,
    eval_score: u64,
    hash: u64,
}

impl Cand {
    fn raw_score(&self, input: &Input) -> i64 {
        todo!();
    }

    fn to_node(&self) -> Node {
        Node {
            child: !0,
            prev: !0,
            next: !0,
            op: self.op,
            parent: self.parent,
        }
    }
}

#[derive(Clone, Default)]
struct Node {
    op: u8,
    parent: usize, // 親Node
    child: usize,  // 代表の子Node
    prev: usize,   // 前の兄弟Node
    next: usize,   // 次の兄弟Node
}

const MAX_WIDTH: usize = 1000;
const TURN: usize = 100;

struct BeamSearch {
    state: State,
    leaf: Vec<usize>, // 子が存在しないNodeのindex
    next_leaf: Vec<usize>,
    nodes: Vec<Node>,
    cur_node: usize,
    free: Vec<usize>, // nodesのうち使われていないindex
}
impl BeamSearch {
    fn new(state: State, node: Node) -> BeamSearch {
        const MAX_NODES: usize = MAX_WIDTH * 5;
        let mut nodes = vec![Node::default(); MAX_NODES];
        nodes[0] = node;
        let free = (1..MAX_NODES as usize).rev().collect();

        BeamSearch {
            state,
            nodes,
            free,
            leaf: vec![0],
            next_leaf: vec![],
            cur_node: 0,
        }
    }

    // 頂点を新たに追加する
    // 代表の子Nodeの前に挿入する形で実装
    fn add_node(&mut self, cand: Cand) {
        let next = self.nodes[cand.parent as usize].child;
        let new = self.free.pop().expect("MAX_NODEが足りないよ") as usize;
        if next != !0 {
            self.nodes[next as usize].prev = new;
        }
        self.nodes[cand.parent as usize].child = new;

        self.next_leaf.push(new);
        self.nodes[new as usize] = Node {
            next,
            ..cand.to_node()
        };
    }

    // 既に探索済みのノードで葉のノードを再帰的に消していく
    fn del_node(&mut self, mut idx: usize) {
        loop {
            self.free.push(idx);
            let Node {
                prev, next, parent, ..
            } = self.nodes[idx as usize];
            assert_ne!(parent, !0, "全てのノードを消そうとしています");
            // 兄弟がいないなら親を消しに行く
            if prev & next == !0 {
                idx = parent;
                continue;
            }

            if prev != !0 {
                self.nodes[prev as usize].next = next;
            } else {
                self.nodes[parent as usize].child = next;
            }
            if next != !0 {
                self.nodes[next as usize].prev = prev;
            }

            break;
        }
    }

    // dfsで木を走査
    // 一本道の場合戻る必要はないのでそれをsingleで管理
    fn dfs(&mut self, input: &Input, cands: &mut Vec<Cand>, single: bool) {
        if self.nodes[self.cur_node].child == !0 {
            self.append_cands(input, self.cur_node, cands);
            return;
        }

        let node = self.cur_node;
        let mut child = self.nodes[node].child;
        let next_single = single & (self.nodes[child as usize].next == !0);

        // let prev_state=self.state.clone();
        loop {
            self.cur_node = child as usize;
            self.state.apply(&self.nodes[child as usize]);
            self.dfs(input, cands, next_single);

            if !next_single {
                self.state.revert(&self.nodes[child as usize]);
                // assert!(prev_state==self.state);
            }
            child = self.nodes[child as usize].next;
            if child == !0 {
                break;
            }
        }

        if !next_single {
            self.cur_node = node;
        }
    }

    // 走査の非再帰実装
    fn no_dfs(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        // 1本道でなくなるまで潜る
        loop {
            let Node { next, child, .. } = self.nodes[self.cur_node];
            if next == !0 || child == !0 {
                break;
            }
            self.cur_node = child as usize;
            self.state.apply(&self.nodes[self.cur_node]);
        }

        let root = self.cur_node;
        loop {
            let child = self.nodes[self.cur_node].child;
            if child == !0 {
                self.append_cands(input, self.cur_node, cands);
                loop {
                    if self.cur_node == root {
                        return;
                    }
                    let node = &self.nodes[self.cur_node];
                    self.state.revert(&node);
                    if node.next != !0 {
                        self.cur_node = node.next as usize;
                        self.state.apply(&self.nodes[self.cur_node]);
                        break;
                    }
                    self.cur_node = node.parent as usize;
                }
            } else {
                self.cur_node = child as usize;
                self.state.apply(&self.nodes[self.cur_node]);
            }
        }
    }

    fn enum_cands(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        // self.dfs(input,cands,true);
        self.no_dfs(input, cands);
    }

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I) {
        self.next_leaf.clear();
        for cand in cands {
            self.add_node(cand);
        }

        for i in 0..self.leaf.len() {
            let n = self.leaf[i];
            // 子が存在しないノードは無駄なので消す
            if self.nodes[n as usize].child == !0 {
                self.del_node(n);
            }
        }

        std::mem::swap(&mut self.leaf, &mut self.next_leaf);
    }

    fn restore(&self, mut idx: usize) -> Vec<u8> {
        let mut ret = vec![];
        loop {
            let Node { op, parent, .. } = self.nodes[idx as usize];
            if op == !0 {
                break;
            }
            ret.push(op);
            idx = parent;
        }

        ret.reverse();
        ret
    }

    // self.stateがself.nodes[idx]のノードが表す状態になっている
    // self.nodes[idx]からのCandをcandsに積む
    fn append_cands(&self, input: &Input, idx: usize, cands: &mut Vec<Cand>) {
        let node = &self.nodes[idx];
        assert_eq!(node.child, !0);

        // self.state.currentがいけるOperationを列挙
        let mut ops = vec![];
        if self.state.current.1 > 0 {
            ops.push(0);
        }
        if self.state.current.1 < input.n - 1 {
            ops.push(1);
        }
        if self.state.current.0 > 0 {
            ops.push(2);
        }
        if self.state.current.0 < input.n - 1 {
            ops.push(3);
        }

        for &op in ops.iter() {
            let mut new_state = self.state.clone();
            new_state.apply(&Node { op, ..*node });
            // hashは前回の位置と移動後の位置から作る. 位置はnで表現できるのでhashはn^2で表現できる
            let prev_pos_num = self.state.current.0 * input.n + self.state.current.1 as usize;
            let new_pos_num = new_state.current.0 * input.n + new_state.current.1;
            let hash = prev_pos_num * input.n * input.n + new_pos_num;
            let eval_score = self.state.total / new_state.turn as u64;
            cands.push(Cand {
                op,
                parent: idx,
                eval_score,
                hash,
            });
        }
    }
}

fn main() {
    input! {
        n: usize,
        h: [String; n-1],
        v: [String; n],
        d: [[u32; n]; n],
    }

    let mut parsed_h = vec![];
    for i in 0..n - 1 {
        let mut row = vec![];
        for c in h[i].chars() {
            row.push(c.to_digit(10).unwrap());
        }
        parsed_h.push(row);
    }

    let mut parsed_v = vec![];
    for i in 0..n {
        let mut row = vec![];
        for c in v[i].chars() {
            row.push(c.to_digit(10).unwrap());
        }
        parsed_v.push(row);
    }

    let input = Input {
        n,
        h: parsed_h,
        v: parsed_v,
        d,
    };

    let state = State::new(&input);
    eprintln!("state: {:?}", state);
}
