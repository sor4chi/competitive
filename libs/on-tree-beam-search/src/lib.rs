/// terry_u16さんの木上ビームサーチの実装を拝借
use std::{
    cmp::Reverse,
    fmt::Display,
    hash::Hash,
    ops::{Index, IndexMut},
};

use rustc_hash::FxHashSet;

/// コピー可能な小さい状態を表すトレイト
pub trait SmallState {
    type Score: Ord + Display;
    type Hash: Hash + Eq;
    type LargeState;
    type Action;

    /// ビームサーチ用スコア（大きいほど良い）
    /// デフォルトでは生スコアをそのまま返す
    fn beam_score(&self) -> Self::Score {
        self.raw_score()
    }

    // 生スコア
    fn raw_score(&self) -> Self::Score;

    /// ハッシュ値
    fn hash(&self) -> Self::Hash;

    /// stateにこの差分を作用させる
    fn apply(&self, state: &mut Self::LargeState);

    /// stateに作用させたこの差分をロールバックする
    fn rollback(&self, state: &mut Self::LargeState);

    /// 実行した行動を返す
    fn action(&self) -> Self::Action;
}

/// 現在のstateからの遷移先を列挙するトレイト
pub trait ActGen<S: SmallState> {
    /// 現在のstateからの遷移先をnext_satesに格納する
    fn generate(&self, small_state: &S, large_state: &S::LargeState, next_states: &mut Vec<S>);
}

/// ビームの次の遷移候補
struct Cancidate<S: SmallState> {
    /// 実行後のsmall_state
    small_state: S,
    /// 親となるノードのインデックス
    parent: NodeIndex,
}

impl<S: SmallState> Cancidate<S> {
    fn new(small_state: S, parent: NodeIndex) -> Self {
        Self {
            small_state,
            parent,
        }
    }

    fn to_node(
        self,
        child: NodeIndex,
        left_sibling: NodeIndex,
        right_sibling: NodeIndex,
    ) -> Node<S> {
        Node {
            small_state: self.small_state,
            parent: self.parent,
            child,
            left_sibling,
            right_sibling,
        }
    }
}

/// 重複除去を行うトレイト
pub trait Deduplicator<S: SmallState> {
    /// 重複除去に使った情報をクリアし、次の重複除去の準備をする
    fn clear(&mut self);

    /// 重複チェックを行い、残すべきならtrue、重複していればfalseを返す
    fn filter(&mut self, state: &S) -> bool;
}

/// 重複除去を行わず素通しするDeduplicator
pub struct NoOpDeduplicator;

impl<S: SmallState> Deduplicator<S> for NoOpDeduplicator {
    fn clear(&mut self) {
        // do nothing
    }

    fn filter(&mut self, _state: &S) -> bool {
        // 常に素通しする
        true
    }
}

/// 同じハッシュ値を持つ状態を1つだけに制限するDeduplicator
pub struct HashSingleDeduplicator<S: SmallState> {
    set: FxHashSet<S::Hash>,
}

impl<S: SmallState> Default for HashSingleDeduplicator<S> {
    fn default() -> Self {
        Self {
            set: FxHashSet::default(),
        }
    }
}

impl<S: SmallState> Deduplicator<S> for HashSingleDeduplicator<S> {
    fn clear(&mut self) {
        self.set.clear();
    }

    fn filter(&mut self, state: &S) -> bool {
        // ハッシュが重複していなければ通す
        self.set.insert(state.hash())
    }
}

/// ビームサーチ木のノード
#[derive(Debug, Default, Clone)]
struct Node<S: SmallState> {
    /// 実行後のsmall_state
    small_state: S,
    /// （N分木と考えたときの）親ノード
    parent: NodeIndex,
    /// （二重連鎖木と考えたときの）子ノード
    child: NodeIndex,
    /// （二重連鎖木と考えたときの）左の兄弟ノード
    left_sibling: NodeIndex,
    /// （二重連鎖木と考えたときの）右の兄弟ノード
    right_sibling: NodeIndex,
}

impl<S: SmallState> Node<S> {
    fn new(
        small_state: S,
        parent: NodeIndex,
        child: NodeIndex,
        left_sibling: NodeIndex,
        right_sibling: NodeIndex,
    ) -> Self {
        Self {
            small_state,
            parent,
            child,
            left_sibling,
            right_sibling,
        }
    }
}

/// NodeVec用のindex
/// 型安全性と、indexの内部的な型(u32 or u16)の変更を容易にすることが目的
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NodeIndex(u16);

impl NodeIndex {
    /// 何も指していないことを表す定数
    const NULL: NodeIndex = NodeIndex(!0);
}

impl Default for NodeIndex {
    fn default() -> Self {
        Self::NULL
    }
}

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<NodeIndex> for usize {
    fn from(val: NodeIndex) -> Self {
        val.0 as usize
    }
}

/// Nodeのコレクション
#[derive(Debug)]
struct NodeVec<S: SmallState> {
    nodes: Vec<Node<S>>,
    free_indices: Vec<usize>,
}

impl<S: SmallState + Default + Clone> NodeVec<S> {
    fn new(capacity: usize) -> Self {
        Self {
            nodes: vec![Default::default(); capacity],
            free_indices: (0..capacity).rev().collect(),
        }
    }

    fn push(&mut self, node: Node<S>) -> NodeIndex {
        let index = self
            .free_indices
            .pop()
            .expect("ノードプールの容量制限に達しました。");

        self.nodes[index] = node;

        NodeIndex::from(index)
    }

    fn delete(&mut self, index: NodeIndex) {
        self.free_indices.push(index.into());
    }
}

impl<S: SmallState> Index<NodeIndex> for NodeVec<S> {
    type Output = Node<S>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        let index: usize = index.into();
        self.nodes.index(index)
    }
}

impl<S: SmallState> IndexMut<NodeIndex> for NodeVec<S> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        let index: usize = index.into();
        self.nodes.index_mut(index)
    }
}

/// 保持する最大ノード数。65536個にするとNULLノードと被るため65535個に抑えている
const MAX_NODES: usize = u16::MAX as usize - 1;

#[derive(Debug)]
pub struct BeamSearch<S: SmallState, G: ActGen<S>> {
    state: S::LargeState,
    act_gen: G,
    nodes: NodeVec<S>,
    current_index: NodeIndex,
    leaves: Vec<NodeIndex>,
    next_leaves: Vec<NodeIndex>,
    action_buffer: Vec<S>,
}

impl<S: SmallState + Default + Clone, G: ActGen<S>> BeamSearch<S, G> {
    /// ビーム木を指定された容量で初期化する
    pub fn new(large_state: S::LargeState, small_state: S, act_gen: G) -> Self {
        let mut nodes = NodeVec::new(MAX_NODES);
        nodes.push(Node::new(
            small_state,
            NodeIndex::NULL,
            NodeIndex::NULL,
            NodeIndex::NULL,
            NodeIndex::NULL,
        ));

        Self {
            state: large_state,
            act_gen,
            nodes,
            current_index: NodeIndex(0),
            leaves: vec![NodeIndex(0)],
            next_leaves: vec![],
            action_buffer: vec![],
        }
    }

    pub fn run<P: Deduplicator<S>>(
        &mut self,
        max_turn: usize,
        beam_width: usize,
        mut deduplicator: P,
    ) -> (Vec<S::Action>, S::Score) {
        let mut candidates = vec![];

        for turn in 0..max_turn {
            eprintln!("turn: {}, beam width: {}", turn, beam_width);
            candidates.clear();
            self.dfs(&mut candidates, true);

            if turn + 1 == max_turn {
                break;
            }

            assert_ne!(
                candidates.len(),
                0,
                "次の状態の候補が見つかりませんでした。"
            );

            // 重複除去を行ったのち、次の遷移先を確定させる
            // glidesortが速いらしいが、多様性を確保したいため敢えて不安定ソートを採用している
            candidates.sort_unstable_by_key(|c| Reverse(c.small_state.beam_score()));

            deduplicator.clear();
            self.update_tree(
                candidates
                    .drain(..)
                    .filter(|c| deduplicator.filter(&c.small_state))
                    .take(beam_width),
            );
        }

        let Cancidate {
            small_state,
            parent,
            ..
        } = candidates
            .into_iter()
            .max_by_key(|c| c.small_state.beam_score())
            .expect("最終状態となる候補が見つかりませんでした。");

        // 操作列の復元
        let mut actions = self.restore_actions(parent);
        actions.push(small_state.action());
        (actions, small_state.raw_score())
    }

    /// ノードを追加する
    fn add_node(&mut self, candidate: Cancidate<S>) {
        let parent = candidate.parent;
        let node_index =
            self.nodes
                .push(candidate.to_node(NodeIndex::NULL, NodeIndex::NULL, NodeIndex::NULL));

        // 親の子、すなわち一番左にいる兄弟ノード
        let sibling = self.nodes[parent].child;

        // 既に兄弟がいる場合、その左側に入る
        if sibling != NodeIndex::NULL {
            self.nodes[sibling].left_sibling = node_index;
        }

        // 兄弟を1マス右に押し出して、自分が一番左に入る
        self.next_leaves.push(node_index);
        self.nodes[parent].child = node_index;
        self.nodes[node_index].right_sibling = sibling;
    }

    /// 指定されたインデックスのノードを削除する
    /// 必要に応じてビーム木の辺を繋ぎ直す
    fn remove_node(&mut self, mut index: NodeIndex) {
        loop {
            let Node {
                left_sibling,
                right_sibling,
                parent,
                ..
            } = self.nodes[index];
            self.nodes.delete(index);

            // 親は生きているはず
            assert_ne!(parent, NodeIndex::NULL, "rootノードを消そうとしています。");

            // もう兄弟がいなければ親へ
            if left_sibling == NodeIndex::NULL && right_sibling == NodeIndex::NULL {
                index = parent;
                continue;
            }

            // 左右の連結リストを繋ぎ直す
            if left_sibling != NodeIndex::NULL {
                self.nodes[left_sibling].right_sibling = right_sibling;
            } else {
                self.nodes[parent].child = right_sibling;
            }

            if right_sibling != NodeIndex::NULL {
                self.nodes[right_sibling].left_sibling = left_sibling;
            }

            return;
        }
    }

    /// DFSでビームサーチ木を走査し、次の状態の一覧をcandidatesに詰める
    /// ビームサーチ木が一本道の場合は戻る必要がないため、is_single_pathで管理
    fn dfs(&mut self, candidates: &mut Vec<Cancidate<S>>, is_single_path: bool) {
        // 葉ノードであれば次の遷移を行う
        if self.nodes[self.current_index].child == NodeIndex::NULL {
            self.act_gen.generate(
                &self.nodes[self.current_index].small_state,
                &self.state,
                &mut self.action_buffer,
            );

            while let Some(state) = self.action_buffer.pop() {
                candidates.push(Cancidate::new(state, self.current_index));
            }

            return;
        }

        let current_index = self.current_index;
        let mut child_index = self.nodes[current_index].child;
        let next_is_single_path =
            is_single_path & (self.nodes[child_index].right_sibling == NodeIndex::NULL);

        // デバッグ用
        //let prev_state = self.state.clone();

        // 兄弟ノードを全て走査する
        loop {
            self.current_index = child_index;
            self.nodes[child_index].small_state.apply(&mut self.state);
            self.dfs(candidates, next_is_single_path);

            if !next_is_single_path {
                self.nodes[child_index]
                    .small_state
                    .rollback(&mut self.state);

                // デバッグ用
                //assert!(prev_state == self.state);
            }

            child_index = self.nodes[child_index].right_sibling;

            if child_index == NodeIndex::NULL {
                break;
            }
        }

        if !next_is_single_path {
            self.current_index = current_index;
        }
    }

    /// 木を更新する
    /// 具体的には以下の処理を行う
    ///
    /// - 新しいcandidatesを葉に追加する
    /// - 1ターン前のノードであって葉のノード（今後参照されないノード）を削除する
    fn update_tree(&mut self, candidates: impl Iterator<Item = Cancidate<S>>) {
        self.next_leaves.clear();
        for candidate in candidates {
            self.add_node(candidate);
        }

        for i in 0..self.leaves.len() {
            let node_index = self.leaves[i];

            if self.nodes[node_index].child == NodeIndex::NULL {
                self.remove_node(node_index);
            }
        }

        std::mem::swap(&mut self.leaves, &mut self.next_leaves);
    }

    /// 操作列を復元する
    fn restore_actions(&self, mut index: NodeIndex) -> Vec<S::Action> {
        let mut actions = vec![];

        while self.nodes[index].parent != NodeIndex::NULL {
            actions.push(self.nodes[index].small_state.action());
            index = self.nodes[index].parent;
        }

        actions.reverse();
        actions
    }
}

#[cfg(test)]
mod test {
    //! TSPをビームサーチで解くテスト
    use super::{ActGen, BeamSearch, NoOpDeduplicator, SmallState};

    #[derive(Debug, Clone)]
    struct Input {
        n: usize,
        distances: Vec<Vec<i32>>,
    }

    impl Input {
        fn gen_testcase() -> Self {
            let n = 4;
            let distances = vec![
                vec![0, 2, 3, 10],
                vec![2, 0, 1, 3],
                vec![3, 1, 0, 2],
                vec![10, 3, 2, 0],
            ];

            Self { n, distances }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct TspSmallState {
        distance: i32,
        position: usize,
        visited_count: usize,
    }

    impl TspSmallState {
        fn new(distance: i32, position: usize, visited_count: usize) -> Self {
            Self {
                distance,
                position,
                visited_count,
            }
        }
    }

    impl Default for TspSmallState {
        fn default() -> Self {
            Self {
                distance: 0,
                position: 0,
                visited_count: 1,
            }
        }
    }

    impl SmallState for TspSmallState {
        type Score = i32;
        type Hash = u64;
        type LargeState = TspLargeState;
        type Action = usize;

        fn raw_score(&self) -> Self::Score {
            self.distance
        }

        fn beam_score(&self) -> Self::Score {
            // 大きいほど良いとする
            -self.distance
        }

        fn hash(&self) -> Self::Hash {
            // 適当に0を返す
            0
        }

        fn apply(&self, state: &mut Self::LargeState) {
            // 現在地を訪問済みにする
            state.visited[self.position] = true;
        }

        fn rollback(&self, state: &mut Self::LargeState) {
            // 現在地を未訪問にする
            state.visited[self.position] = false;
        }

        fn action(&self) -> Self::Action {
            self.position
        }
    }

    #[derive(Debug, Clone)]
    struct TspLargeState {
        visited: Vec<bool>,
    }

    impl TspLargeState {
        fn new(n: usize) -> Self {
            let mut visited = vec![false; n];
            visited[0] = true;
            Self { visited }
        }
    }

    #[derive(Debug, Clone)]
    struct ActionGenerator<'a> {
        input: &'a Input,
    }

    impl<'a> ActionGenerator<'a> {
        fn new(input: &'a Input) -> Self {
            Self { input }
        }
    }

    impl<'a> ActGen<TspSmallState> for ActionGenerator<'a> {
        fn generate(
            &self,
            small_state: &TspSmallState,
            large_state: &TspLargeState,
            next_states: &mut Vec<TspSmallState>,
        ) {
            if small_state.visited_count == self.input.n {
                // 頂点0に戻るしかない
                let next_pos = 0;
                let next_dist =
                    small_state.distance + self.input.distances[small_state.position][0];
                let next_visited_count = small_state.visited_count + 1;
                let next_state = TspSmallState::new(next_dist, next_pos, next_visited_count);
                next_states.push(next_state);
                return;
            }

            // 未訪問の頂点に移動
            for next_pos in 0..self.input.n {
                if large_state.visited[next_pos] {
                    continue;
                }

                let next_dist =
                    small_state.distance + self.input.distances[small_state.position][next_pos];
                let next_visited_count = small_state.visited_count + 1;
                let next_state = TspSmallState::new(next_dist, next_pos, next_visited_count);
                next_states.push(next_state);
            }
        }
    }

    #[test]
    fn beam_tsp_test() {
        let input = Input::gen_testcase();
        let small_state = TspSmallState::default();
        let large_state = TspLargeState::new(input.n);
        let action_generator = ActionGenerator::new(&input);
        let mut beam = BeamSearch::new(large_state, small_state, action_generator);

        // hashを適当に全て0としているため、重複除去は行わない
        let deduplicator = NoOpDeduplicator;
        let beam_width = 100;

        let (actions, score) = beam.run(input.n, beam_width, deduplicator);

        eprintln!("score: {}", score);
        eprintln!("actions: {:?}", actions);
        assert_eq!(score, 10);
        assert!(actions == vec![1, 3, 2, 0] || actions == vec![2, 3, 1, 0]);
    }
}
