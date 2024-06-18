use proconio::input;
use rand;

#[derive(Clone, PartialEq)]
struct State {
    h: Vec<Vec<i32>>,
    diff: i32,
    ops: Vec<u8>,
    cur: (usize, usize),
    cost: i32,
    hold: i32,
}
impl State {
    // 初期状態の生成
    fn new(input: &Input) -> State {
        let h = input.h.clone();
        let mut diff = 0;
        for i in 0..input.n {
            for j in 0..input.n {
                diff += h[i][j].abs();
            }
        }
        State {
            h,
            diff,
            ops: vec![],
            cur: (0, 0),
            cost: 0,
            hold: 0,
        }
    }

    // 差分更新で適用する
    fn apply(&mut self, node: &Node) {
        self.ops.push(node.op);
        if node.op == 0 {
            self.cur.0 -= 1;
            self.cost += self.hold + 100;
        } else if node.op == 1 {
            self.cur.0 += 1;
            self.cost += self.hold + 100;
        } else if node.op == 2 {
            self.cur.1 -= 1;
            self.cost += self.hold + 100;
        } else if node.op == 3 {
            self.cur.1 += 1;
            self.cost += self.hold + 100;
        } else if node.op == 4 {
            // HOLD_AMOUNT掴むのでdiffから掴む前のabsの値を引く
            self.diff -= self.h[self.cur.0][self.cur.1].abs();
            // HOLD_AMOUNT掴むので盤面からHOLD_AMOUNT減らす
            self.h[self.cur.0][self.cur.1] -= HOLD_AMOUNT;
            // HOLD_AMOUNT掴んだのでholdにHOLD_AMOUNT加える
            self.hold += HOLD_AMOUNT;
            // HOLD_AMOUNT掴むのでコストにHOLD_AMOUNT加える
            self.cost += HOLD_AMOUNT;
            // HOLD_AMOUNT掴むのでdiffに掴んだ後のabsの値を加える
            self.diff += self.h[self.cur.0][self.cur.1].abs();
        } else if node.op == 5 {
            // HOLD_AMOUNT離すのでdiffから離す前のabsの値を引く
            self.diff -= self.h[self.cur.0][self.cur.1].abs();
            // HOLD_AMOUNT離すので盤面にHOLD_AMOUNT加える
            self.h[self.cur.0][self.cur.1] += HOLD_AMOUNT;
            // HOLD_AMOUNT離したのでholdからHOLD_AMOUNT減らす
            self.hold -= HOLD_AMOUNT;
            // HOLD_AMOUNT離すのでコストにHOLD_AMOUNT加える
            self.cost += HOLD_AMOUNT;
            // HOLD_AMOUNT離すのでdiffに離した後のabsの値を加える
            self.diff += self.h[self.cur.0][self.cur.1].abs();
        }
    }

    // 差分更新で元に戻す
    fn revert(&mut self, node: &Node) {
        let op = node.op;
        if op == 0 {
            self.cur.0 += 1;
            self.cost -= self.hold + 100;
        } else if op == 1 {
            self.cur.0 -= 1;
            self.cost -= self.hold + 100;
        } else if op == 2 {
            self.cur.1 += 1;
            self.cost -= self.hold + 100;
        } else if op == 3 {
            self.cur.1 -= 1;
            self.cost -= self.hold + 100;
        } else if op == 4 {
            self.diff -= self.h[self.cur.0][self.cur.1].abs();
            self.h[self.cur.0][self.cur.1] += HOLD_AMOUNT;
            self.hold -= HOLD_AMOUNT;
            self.cost -= HOLD_AMOUNT;
            self.diff += self.h[self.cur.0][self.cur.1].abs();
        } else if op == 5 {
            self.diff -= self.h[self.cur.0][self.cur.1].abs();
            self.h[self.cur.0][self.cur.1] -= HOLD_AMOUNT;
            self.hold += HOLD_AMOUNT;
            self.cost -= HOLD_AMOUNT;
            self.diff += self.h[self.cur.0][self.cur.1].abs();
        }
        self.ops.pop();
    }

    fn score(&self) -> i64 {
        10000 + 100 * self.diff as i64 + self.cost as i64
    }
}

#[derive(Clone)]
struct Cand {
    op: u8, // 0:上, 1:下, 2:左, 3:右 4:HOLD_AMOUNT掴む 5:HOLD_AMOUNT離す
    parent: usize,
    eval_score: i64,
    hash: u64,
}
impl Cand {
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

const MAX_WIDTH: usize = 10000;
const TURN: usize = 100;
const HOLD_AMOUNT: i32 = 3;

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

        // LRUD,HOLD_AMOUNT掴む,HOLD_AMOUNT離すの順に評価する
        for op in 0..6 {
            match op {
                0..=3 => {
                    // 前の操作の逆操作は無視
                    if (op as i32 - node.op as i32).abs() == 1 {
                        continue;
                    }
                    let (dx, dy) = match op {
                        0 => (-1, 0),
                        1 => (1, 0),
                        2 => (0, -1),
                        3 => (0, 1),
                        _ => unreachable!(),
                    };
                    let (nx, ny) = (self.state.cur.0 as i32 + dx, self.state.cur.1 as i32 + dy);
                    if nx < 0 || nx >= input.n as i32 || ny < 0 || ny >= input.n as i32 {
                        continue;
                    }
                    let op = op as u8;
                    // let hash = (nx as u64) * input.n as u64 + ny as u64 + self.state.hold as u64 * 100;
                    // とりあえずhashは乱数で
                    let hash = rand::random();
                    // 差分は前回のスコアに移動コスト(hold+100)を加えたもの
                    let eval_score = self.state.score() + self.state.hold as i64 + 100;
                    cands.push(Cand {
                        op,
                        parent: idx,
                        eval_score,
                        hash,
                    });
                }
                4 => {
                    // すでにhが負の値の場合は掴めない
                    if self.state.h[self.state.cur.0][self.state.cur.1] < HOLD_AMOUNT {
                        continue;
                    }
                    // let hash = (self.state.cur.0 as u64) * input.n as u64 + self.state.cur.1 as u64;
                    let hash = rand::random();
                    // 差分は前回のスコアに掴む前のabsの値を引いて掴んだ後のabsの値を加え、掴むコスト(HOLD_AMOUNT)を加える
                    let eval_score = self.state.score()
                        - self.state.h[self.state.cur.0][self.state.cur.1].abs() as i64
                        + (self.state.h[self.state.cur.0][self.state.cur.1] - HOLD_AMOUNT).abs()
                            as i64
                        + HOLD_AMOUNT as i64;
                    cands.push(Cand {
                        op: 4,
                        parent: idx,
                        eval_score,
                        hash,
                    });
                }
                5 => {
                    // すでにhが正の値の場合は離せない
                    if self.state.h[self.state.cur.0][self.state.cur.1] > -HOLD_AMOUNT {
                        continue;
                    }
                    if self.state.hold < HOLD_AMOUNT {
                        continue;
                    }
                    // let hash = (self.state.cur.0 as u64) * input.n as u64 + self.state.cur.1 as u64;
                    let hash = rand::random();
                    // 差分は前回のスコアに離す前のabsの値を引いて離した後のabsの値を加え、離すコスト(HOLD_AMOUNT)を加える
                    let eval_score = self.state.score()
                        - self.state.h[self.state.cur.0][self.state.cur.1].abs() as i64
                        + (self.state.h[self.state.cur.0][self.state.cur.1] + HOLD_AMOUNT).abs()
                            as i64
                        + HOLD_AMOUNT as i64;
                    cands.push(Cand {
                        op: 5,
                        parent: idx,
                        eval_score,
                        hash,
                    });
                }
                _ => unreachable!(),
            }
        }
    }

    fn solve(&mut self, input: &Input) -> Vec<u8> {
        let mut cands = vec![];

        for i in 0..TURN {
            eprintln!("turn: {}, cand: {}", i, cands.len());
            self.enum_cands(input, &mut cands);
            cands.sort_by_key(|c| std::cmp::Reverse(c.eval_score));
            cands.truncate(MAX_WIDTH);
            self.update(cands.drain(..));
        }

        let mut best = 0;
        let mut best_score = 0;
        for &idx in &self.leaf {
            let score = self.state.score();
            if score > best_score {
                best_score = score;
                best = idx;
            }
        }

        self.restore(best)
    }
}

struct Input {
    n: usize,
    h: Vec<Vec<i32>>,
}

fn main() {
    input! {
        n: usize,
        h: [[i32; n]; n]
    };

    let input = Input { n, h };

    let state = State::new(&input);
    let node = Node {
        op: !0,
        parent: !0,
        child: !0,
        prev: !0,
        next: !0,
    };
    let mut bs = BeamSearch::new(state, node);
    let ans = bs.solve(&input);

    for &op in &ans {
        let op = match op {
            0 => "U",
            1 => "D",
            2 => "L",
            3 => "R",
            4 => "+10",
            5 => "-10",
            _ => unreachable!(),
        };
        println!("{}", op);
    }
}
