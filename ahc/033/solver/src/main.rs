use proconio::input;

use game::{
    get_direct_path, manhattan_distance, simulate_operations, CraneId, EscapeMode, Game, Input,
    Operation, Position, TickError, Value,
};

use std::time::Instant;

mod game {
    use std::collections::{BTreeSet, HashMap, VecDeque};
    use std::fmt;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct CraneId(pub usize);

    impl fmt::Display for CraneId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Value(pub usize);

    impl fmt::Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub struct Input {
        pub n: usize,
        pub a: Vec<Vec<Value>>,
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Position {
        pub row: usize,
        pub col: usize,
    }

    impl Position {
        pub fn new(row: usize, col: usize) -> Self {
            Self { row, col }
        }
    }

    pub fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
        (p1.row as isize - p2.row as isize).unsigned_abs()
            + (p1.col as isize - p2.col as isize).unsigned_abs()
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn reverse(&self) -> Self {
            match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }

    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    #[derive(Clone, PartialEq, Debug)]
    pub enum Operation {
        Stay,
        Move(Direction),
        Hold,
        Release,
        Crush,
    }

    impl fmt::Display for Operation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Operation::Stay => write!(f, "."),
                Operation::Move(Direction::Up) => write!(f, "U"),
                Operation::Move(Direction::Down) => write!(f, "D"),
                Operation::Move(Direction::Left) => write!(f, "L"),
                Operation::Move(Direction::Right) => write!(f, "R"),
                Operation::Hold => write!(f, "P"),
                Operation::Release => write!(f, "Q"),
                Operation::Crush => write!(f, "B"),
            }
        }
    }

    pub fn get_direct_path(p1: &Position, p2: &Position) -> Vec<Direction> {
        let mut path = Vec::new();
        let mut current = *p1;
        while current.row != p2.row {
            if current.row < p2.row {
                path.push(Direction::Down);
                current.row += 1;
            } else {
                path.push(Direction::Up);
                current.row -= 1;
            }
        }
        while current.col != p2.col {
            if current.col < p2.col {
                path.push(Direction::Right);
                current.col += 1;
            } else {
                path.push(Direction::Left);
                current.col -= 1;
            }
        }
        path
    }

    pub fn simulate_operations(p1: &Position, operations: Vec<Operation>) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut current = *p1;
        positions.push(current);
        for operation in operations {
            match operation {
                Operation::Stay => {}
                Operation::Move(direction) => match direction {
                    Direction::Up => current.row -= 1,
                    Direction::Down => current.row += 1,
                    Direction::Left => current.col -= 1,
                    Direction::Right => current.col += 1,
                },
                Operation::Hold => {}
                Operation::Release => {}
                Operation::Crush => {}
            }
            positions.push(current);
        }
        positions
    }

    #[derive(Clone)]
    pub struct BoardCell {
        pub value: Option<Value>,
        pub lock: Option<CraneId>,
    }

    impl fmt::Debug for BoardCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.value {
                Some(value) => {
                    if self.lock.is_some() {
                        write!(f, "{:>2}<{}", value, self.lock.unwrap())
                    } else {
                        write!(f, " {:>2} ", value)
                    }
                }
                None => write!(f, " -1 "),
            }
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct Crane {
        pub pos: Position,
        holding: Option<Value>,
        operations: Vec<Operation>,
    }

    impl fmt::Debug for Crane {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Crane {{")?;
            writeln!(f, "    pos: {:?}", self.pos)?;
            writeln!(f, "    holding: {:?}", self.holding)?;
            writeln!(f, "    operations: {:?}", self.operations)?;
            write!(f, "  }}")
        }
    }

    #[derive(Clone)]
    pub struct Game {
        n: usize,
        turn: usize,
        pub board: Vec<Vec<BoardCell>>,
        input_queues: Vec<VecDeque<Value>>,
        pub output_stacks: Vec<Vec<Value>>,
        pub big_crane: Option<Crane>,
        pub small_crane: HashMap<CraneId, Crane>,
        pub requests: Vec<Option<Value>>,
        history: HashMap<CraneId, Vec<Operation>>,
        pub timing_slots: Vec<Vec<BTreeSet<usize>>>,
    }

    #[derive(PartialEq)]
    pub enum EscapeMode {
        Flying,  // コンテナを避けず上を通過
        Walking, // コンテナを避けて通過
    }

    impl fmt::Debug for Game {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Game (turn: {}) {{", self.turn)?;
            writeln!(f, "  n: {}", self.n)?;
            writeln!(f, "  board: [")?;
            for row in &self.board {
                writeln!(f, "    {:?}", row)?;
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  input_queues: [")?;
            for queue in &self.input_queues {
                writeln!(f, "    {:?}", queue)?;
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  output_stacks: [")?;
            for stack in &self.output_stacks {
                writeln!(f, "    {:?}", stack)?;
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  big_crane: {:?}", self.big_crane)?;
            writeln!(f, "  small_crane: {:?}", self.small_crane)?;
            writeln!(f, "  requests: {:?}", self.requests)?;
            writeln!(f, "  timing: {:?}", self.timing_slots)?;
            writeln!(f, "  latest_timing:")?;
            for row in 0..self.n {
                write!(f, " ")?;
                for col in 0..self.n {
                    write!(
                        f,
                        " {}",
                        if let Some(timing) = self.timing_slots[row][col].iter().next() {
                            format!("{:>2}", timing)
                        } else {
                            "-1".to_string()
                        }
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f, "}}")?;
            writeln!(f, "Answer:")?;
            writeln!(f, "{}", self.answer())
        }
    }

    #[derive(Debug)]
    pub enum TickError {
        PathBlocked(CraneId),
    }

    impl Game {
        pub fn new(input: &Input) -> Self {
            let n = input.n;
            let board = vec![
                vec![
                    BoardCell {
                        value: None,
                        lock: None
                    };
                    n
                ];
                n
            ];
            let input_queues = input
                .a
                .iter()
                .map(|row| row.iter().copied().collect())
                .collect();
            let output_stacks = vec![Vec::new(); n];
            let big_crane = Some(Crane {
                pos: Position::new(0, 0),
                holding: None,
                operations: Vec::new(),
            });

            let mut small_crane = HashMap::new();
            (1..n).for_each(|row| {
                small_crane.insert(
                    CraneId(row),
                    Crane {
                        pos: Position::new(row, 0),
                        holding: None,
                        operations: Vec::new(),
                    },
                );
            });
            let requests = (0..n).map(|i| Some(Value(i * n))).collect();
            let history = (0..n).map(|i| (CraneId(i), Vec::new())).collect();
            let timing_slots = vec![vec![BTreeSet::new(); n]; n];
            Self {
                n,
                turn: 0,
                board,
                input_queues,
                output_stacks,
                big_crane,
                small_crane,
                requests,
                history,
                timing_slots,
            }
        }

        pub fn get_crane(&self, crane_id: CraneId) -> Option<&Crane> {
            if crane_id == CraneId(0) {
                self.big_crane.as_ref()
            } else {
                self.small_crane.get(&crane_id)
            }
        }

        fn get_crane_mut(&mut self, crane_id: CraneId) -> Option<&mut Crane> {
            if crane_id == CraneId(0) {
                self.big_crane.as_mut()
            } else {
                self.small_crane.get_mut(&crane_id)
            }
        }

        fn move_crane(&mut self, crane_id: CraneId, direction: Direction) -> Result<(), &str> {
            // クレーンが存在しない場合はエラー
            let crane = self.get_crane(crane_id).ok_or("Invalid crane ID")?;
            // 移動先の座標を計算、範囲外の場合はエラー
            let new_pos = match direction {
                Direction::Up => {
                    if crane.pos.row == 0 {
                        return Err("Invalid move");
                    }
                    Position::new(crane.pos.row - 1, crane.pos.col)
                }
                Direction::Down => {
                    if crane.pos.row == self.n - 1 {
                        return Err("Invalid move");
                    }
                    Position::new(crane.pos.row + 1, crane.pos.col)
                }
                Direction::Left => {
                    if crane.pos.col == 0 {
                        return Err("Invalid move");
                    }
                    Position::new(crane.pos.row, crane.pos.col - 1)
                }
                Direction::Right => {
                    if crane.pos.col == self.n - 1 {
                        return Err("Invalid move");
                    }
                    Position::new(crane.pos.row, crane.pos.col + 1)
                }
            };
            if crane_id == CraneId(0) {
                self.big_crane.as_mut().unwrap().pos = new_pos;
            } else {
                // 小さいクレーンはコンテナを持っている時次の移動場所にコンテナがある場合はエラーになる
                if self.small_crane.get(&crane_id).unwrap().holding.is_some()
                    && self.board[new_pos.row][new_pos.col].value.is_some()
                {
                    return Err("Path blocked");
                }
                self.small_crane.get_mut(&crane_id).unwrap().pos = new_pos;
            }
            self.history
                .get_mut(&crane_id)
                .unwrap()
                .push(Operation::Move(direction));
            Ok(())
        }

        fn hold(&mut self, crane_id: CraneId) -> Result<(), &str> {
            // クレーンが存在しない場合はエラー
            let crane = self.get_crane(crane_id).ok_or("Invalid crane ID")?;
            let pos = crane.pos;

            // 既に値を持っている場合はエラー
            if crane.holding.is_some() {
                return Err("Already holding a value");
            }

            // クレーンの位置に値がない場合はエラー
            let value = self.board[pos.row][pos.col]
                .value
                .ok_or("No value to hold")?;
            if crane_id == CraneId(0) {
                self.big_crane.as_mut().unwrap().holding = Some(value);
            } else {
                self.small_crane.get_mut(&crane_id).unwrap().holding = Some(value);
            }
            self.board[pos.row][pos.col].value = None;
            self.board[pos.row][pos.col].lock = None;
            self.history
                .get_mut(&crane_id)
                .unwrap()
                .push(Operation::Hold);
            Ok(())
        }

        fn release(&mut self, crane_id: CraneId) -> Result<(), &str> {
            let crane = self.get_crane(crane_id).ok_or("Invalid crane ID")?;
            let pos = crane.pos;
            let value = if crane_id == CraneId(0) {
                self.big_crane.as_mut().unwrap().holding
            } else {
                self.small_crane.get_mut(&crane_id).unwrap().holding
            }
            .ok_or("No value to release")?;
            if self.board[pos.row][pos.col].value.is_some() {
                return Err("Cell is not empty");
            }
            self.board[pos.row][pos.col].value = Some(value);
            if crane_id == CraneId(0) {
                self.big_crane.as_mut().unwrap().holding = None;
            } else {
                self.small_crane.get_mut(&crane_id).unwrap().holding = None;
            }
            self.history
                .get_mut(&crane_id)
                .unwrap()
                .push(Operation::Release);
            Ok(())
        }

        fn crush(&mut self, crane_id: CraneId) -> Result<(), &str> {
            if crane_id == CraneId(0) {
                // 既に破壊済みの場合は破壊できない
                if self.big_crane.is_none() {
                    return Err("Already crushed");
                }
                // 値を持っている場合は破壊できない
                if self.big_crane.as_ref().unwrap().holding.is_some() {
                    return Err("Cannot crush while holding a value");
                }
                self.big_crane = None;
            } else {
                // 既に破壊済みの場合は破壊できない
                if !self.small_crane.contains_key(&crane_id) {
                    return Err("Already crushed");
                }
                // 値を持っている場合は破壊できない
                if self.small_crane.get(&crane_id).unwrap().holding.is_some() {
                    return Err("Cannot crush while holding a value");
                }
                self.small_crane.remove(&crane_id);
            }
            self.history
                .get_mut(&crane_id)
                .unwrap()
                .push(Operation::Crush);
            Ok(())
        }

        fn stay(&mut self, crane_id: CraneId) {
            self.history
                .get_mut(&crane_id)
                .unwrap()
                .push(Operation::Stay);
        }

        fn get_crane_ids(&self) -> Vec<CraneId> {
            let mut ids = Vec::new();
            if self.big_crane.is_some() {
                ids.push(CraneId(0));
            }
            ids.extend(self.small_crane.keys().copied());
            ids.sort();
            ids
        }

        pub fn clear_operations(&mut self, crane_id: CraneId) {
            self.get_crane_mut(crane_id)
                .map(|crane| crane.operations.clear());
        }

        pub fn add_operation(
            &mut self,
            crane_id: CraneId,
            operation: Operation,
        ) -> Result<(), &str> {
            if let Some(crane) = self.get_crane_mut(crane_id) {
                crane.operations.push(operation);
                Ok(())
            } else {
                Err("Invalid crane ID")
            }
        }

        pub fn find_value(&self, value: Value) -> Option<Position> {
            for row in 0..self.n {
                for col in 0..self.n {
                    if self.board[row][col].value == Some(value) {
                        return Some(Position::new(row, col));
                    }
                }
            }
            None
        }

        pub fn is_request_completed(&self) -> bool {
            self.requests.iter().all(|request| request.is_none())
        }

        pub fn is_crane_operations_empty(&self, crane_id: CraneId) -> bool {
            self.get_crane(crane_id)
                .map(|crane| crane.operations.is_empty())
                .unwrap_or(true)
        }

        // col+1が空いているセルをfloating_positionとして探す
        pub fn get_floating_positions(&self) -> Vec<Position> {
            let mut floating_positions = Vec::new();
            for row in 0..self.n {
                for col in 0..self.n - 2 {
                    if self.board[row][col].value.is_some()
                        && self.board[row][col + 1].value.is_none()
                        && self.board[row][col].lock.is_none()
                        && !self.crane_exists(&Position::new(row, col))
                        && !self.crane_exists(&Position::new(row, col + 1))
                    {
                        floating_positions.push(Position::new(row, col));
                    }
                }
            }
            floating_positions
        }

        // 座標にクレーンがあるか
        fn crane_exists(&self, pos: &Position) -> bool {
            if let Some(big_crane) = &self.big_crane {
                if big_crane.pos == *pos {
                    return true;
                }
            }
            self.small_crane.values().any(|crane| crane.pos == *pos)
        }

        pub fn find_no_timing_slot_cells(&self, pos: &Position) -> Vec<Position> {
            let mut no_timing_slot_cells = Vec::new();
            for row in 0..self.n {
                for col in 0..self.n {
                    if self.timing_slots[row][col].is_empty()
                        && !self.crane_exists(&Position::new(row, col))
                    {
                        no_timing_slot_cells.push(Position::new(row, col));
                    }
                }
            }
            no_timing_slot_cells.sort_by_key(|empty_pos| manhattan_distance(pos, empty_pos));
            no_timing_slot_cells
        }

        // timing_slotsを使って衝突を避けるようなpathを生成して返す
        pub fn get_escape_path(
            &self,
            from: &Position,
            to: &Position,
            mode: EscapeMode,
            from_dist: usize,
        ) -> Result<Vec<Direction>, &str> {
            eprintln!("from:{:?}, to:{:?}", from, to);
            // BFS
            struct BFSNode {
                pos: Position,
                dist: usize,
            }
            let mut queue = VecDeque::new();
            let mut visited = vec![vec![false; self.n]; self.n];
            let mut dist = vec![vec![std::usize::MAX; self.n]; self.n];
            queue.push_back(BFSNode {
                pos: *from,
                dist: from_dist,
            });
            visited[from.row][from.col] = true;
            dist[from.row][from.col] = from_dist;
            while let Some(node) = queue.pop_front() {
                if node.pos == *to {
                    break;
                }

                for direction in &DIRECTIONS {
                    let next_pos = match direction {
                        Direction::Up => {
                            if node.pos.row == 0 {
                                continue;
                            }
                            Position::new(node.pos.row - 1, node.pos.col)
                        }
                        Direction::Down => {
                            if node.pos.row == self.n - 1 {
                                continue;
                            }
                            Position::new(node.pos.row + 1, node.pos.col)
                        }
                        Direction::Left => {
                            if node.pos.col == 0 {
                                continue;
                            }
                            Position::new(node.pos.row, node.pos.col - 1)
                        }
                        Direction::Right => {
                            if node.pos.col == self.n - 1 {
                                continue;
                            }
                            Position::new(node.pos.row, node.pos.col + 1)
                        }
                    };
                    let next_dist = node.dist + 1;
                    if visited[next_pos.row][next_pos.col] {
                        continue;
                    }
                    if self.timing_slots[next_pos.row][next_pos.col].contains(&next_dist) {
                        continue;
                    }
                    if self.timing_slots[next_pos.row][next_pos.col].contains(&(next_dist - 1)) {
                        continue;
                    }
                    if mode == EscapeMode::Walking
                        && self.board[next_pos.row][next_pos.col].value.is_some()
                    {
                        continue;
                    }
                    visited[next_pos.row][next_pos.col] = true;
                    dist[next_pos.row][next_pos.col] = next_dist;
                    queue.push_back(BFSNode {
                        pos: next_pos,
                        dist: next_dist,
                    });
                }
            }

            let mut current = *to;
            let mut path = Vec::new();
            let mut current_dist = dist[current.row][current.col];
            let mut prev = Position::new(usize::MAX, usize::MAX);
            while current != *from {
                if prev == current {
                    return Err("No path found");
                }
                prev = current;
                for direction in &DIRECTIONS {
                    let next_pos = match direction {
                        Direction::Up => {
                            if current.row == 0 {
                                continue;
                            }
                            Position::new(current.row - 1, current.col)
                        }
                        Direction::Down => {
                            if current.row == self.n - 1 {
                                continue;
                            }
                            Position::new(current.row + 1, current.col)
                        }
                        Direction::Left => {
                            if current.col == 0 {
                                continue;
                            }
                            Position::new(current.row, current.col - 1)
                        }
                        Direction::Right => {
                            if current.col == self.n - 1 {
                                continue;
                            }
                            Position::new(current.row, current.col + 1)
                        }
                    };
                    if next_pos.row >= self.n || next_pos.col >= self.n {
                        continue;
                    }
                    if dist[next_pos.row][next_pos.col] == current_dist - 1 {
                        path.push(direction.reverse());
                        current = next_pos;
                        current_dist -= 1;
                        break;
                    }
                }
            }
            path.reverse();
            Ok(path)
        }

        pub fn tick(&mut self) -> Result<(), TickError> {
            let crane_ids = self.get_crane_ids();
            let all_operations_empty = crane_ids
                .iter()
                .all(|&id| self.get_crane(id).unwrap().operations.is_empty());
            let mut operation_err = None;
            if !all_operations_empty {
                crane_ids.iter().for_each(|&id| {
                    let crane = self.get_crane(id).unwrap();
                    // operationsが空の場合はSTAYとする
                    let operation = crane.operations.first().unwrap_or(&Operation::Stay);
                    match operation {
                        Operation::Stay => {
                            self.stay(id);
                        }
                        Operation::Move(direction) => {
                            let res = self.move_crane(id, direction.clone());
                            // もしエラーだったらPathBlockedを返す
                            if res.is_err() {
                                operation_err = Some(TickError::PathBlocked(id));
                            }
                        }
                        Operation::Hold => {
                            // 提出のためエラーを無視する
                            self.hold(id);
                        }
                        Operation::Release => {
                            // 提出のためエラーを無視する
                            self.release(id);
                        }
                        Operation::Crush => {
                            // 提出のためエラーを無視する
                            self.crush(id);
                        }
                    }
                });
                self.turn += 1;
            }
            if let Some(err) = operation_err {
                return Err(err);
            }
            (0..self.n).for_each(|row| {
                if self.board[row][0].value.is_none() && !self.input_queues[row].is_empty() {
                    self.board[row][0].value = Some(self.input_queues[row].pop_front().unwrap());
                }
            });
            (0..self.n).for_each(|row| {
                if self.board[row][self.n - 1].value.is_some() {
                    if self.requests[row] == self.board[row][self.n - 1].value {
                        self.output_stacks[row].push(self.board[row][self.n - 1].value.unwrap());
                        if self.requests[row] == Some(Value(row * self.n + self.n - 1)) {
                            self.requests[row] = None;
                        } else {
                            self.requests[row] = Some(Value(self.requests[row].unwrap().0 + 1));
                        }
                    }
                    self.board[row][self.n - 1].value = None;
                }
            });
            // operationsを消費
            crane_ids.iter().for_each(|&id| {
                let crane = self.get_crane_mut(id);
                if let Some(crane) = crane {
                    if !crane.operations.is_empty() {
                        crane.operations.remove(0);
                    }
                }
            });
            // timing_slotsを更新
            (0..self.n).for_each(|row| {
                (0..self.n).for_each(|col| {
                    let timing_slots = self.timing_slots[row][col].clone();
                    self.timing_slots[row][col].clear();
                    timing_slots.iter().for_each(|timing| {
                        // timingを1減らす
                        if *timing > 0 {
                            self.timing_slots[row][col].insert(timing - 1);
                        }
                    });
                });
            });
            Ok(())
        }

        pub fn answer(&self) -> String {
            let mut answer = String::new();
            (0..self.n).for_each(|crane_id| {
                let operations = self.history.get(&CraneId(crane_id)).unwrap();
                for operation in operations {
                    answer.push_str(&format!("{}", operation));
                }
                answer.push('\n');
            });
            answer.pop();
            answer
        }

        pub fn debug_lock(&self) {
            println!("lock,turn:{}", self.turn);
            for row in 0..self.n {
                let mut s = String::new();
                for col in 0..self.n {
                    if let Some(lock) = self.board[row][col].lock {
                        s.push_str(&format!("{} ", lock));
                    } else {
                        s.push_str(". ");
                    }
                }
                s.pop();
                println!("{}", s);
            }
        }

        pub fn debug_timing(&self) {
            println!("timing,turn:{}", self.turn);
            for row in 0..self.n {
                let mut s = String::new();
                for col in 0..self.n {
                    if self.timing_slots[row][col].is_empty() {
                        s.push_str(". ");
                    } else {
                        for timing in &self.timing_slots[row][col] {
                            s.push_str(&format!("{},", timing));
                        }
                        s.pop();
                        s.push(' ');
                    }
                }
                s.pop();
                println!("{}", s);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_manhattan_distance() {
            let p1 = Position::new(0, 0);
            let p2 = Position::new(0, 0);
            assert_eq!(manhattan_distance(&p1, &p2), 0);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(0, 1);
            assert_eq!(manhattan_distance(&p1, &p2), 1);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 0);
            assert_eq!(manhattan_distance(&p1, &p2), 1);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 1);
            assert_eq!(manhattan_distance(&p1, &p2), 2);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 2);
            assert_eq!(manhattan_distance(&p1, &p2), 3);
        }

        #[test]
        fn test_get_direct_path() {
            let p1 = Position::new(0, 0);
            let p2 = Position::new(0, 0);
            assert_eq!(get_direct_path(&p1, &p2), Vec::new());

            let p1 = Position::new(0, 0);
            let p2 = Position::new(0, 1);
            assert_eq!(get_direct_path(&p1, &p2), vec![Direction::Right]);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 0);
            assert_eq!(get_direct_path(&p1, &p2), vec![Direction::Down]);

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 1);
            assert_eq!(
                get_direct_path(&p1, &p2),
                vec![Direction::Down, Direction::Right]
            );

            let p1 = Position::new(0, 0);
            let p2 = Position::new(1, 2);
            assert_eq!(
                get_direct_path(&p1, &p2),
                vec![Direction::Down, Direction::Right, Direction::Right]
            );
        }

        #[test]
        fn test_new_game() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let game = Game::new(&input);
            assert_eq!(game.n, 3);
            assert_eq!(game.board.len(), 3);
            assert_eq!(game.board[0].len(), 3);
            assert_eq!(game.input_queues.len(), 3);
            assert_eq!(game.input_queues[0].len(), 3);
            assert_eq!(game.input_queues[0][0], Value(1));
            assert_eq!(game.input_queues[0][1], Value(2));
            assert_eq!(game.input_queues[0][2], Value(3));
            assert_eq!(game.output_stacks.len(), 3);
            assert_eq!(game.output_stacks[0].len(), 0);
            assert_eq!(game.big_crane.as_ref().unwrap().pos, Position::new(0, 0));
            assert_eq!(game.big_crane.as_ref().unwrap().holding, None);
            assert_eq!(game.big_crane.as_ref().unwrap().operations.len(), 0);
            assert_eq!(game.small_crane.len(), 2);
            assert_eq!(game.small_crane[&CraneId(1)].pos, Position::new(1, 0));
            assert_eq!(game.small_crane[&CraneId(1)].holding, None);
            assert_eq!(game.small_crane[&CraneId(1)].operations.len(), 0);
            assert_eq!(game.small_crane[&CraneId(2)].pos, Position::new(2, 0));
            assert_eq!(game.small_crane[&CraneId(2)].holding, None);
            assert_eq!(game.small_crane[&CraneId(2)].operations.len(), 0);
            assert_eq!(game.requests.len(), 3);
            assert_eq!(game.requests[0], Some(Value(0)));
            assert_eq!(game.requests[1], Some(Value(3)));
            assert_eq!(game.requests[2], Some(Value(6)));
            assert_eq!(game.history.len(), 3);
        }

        #[test]
        fn test_get_crane() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let game = Game::new(&input);

            // 0, 1, 2のクレーンが存在する
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 0));
            assert_eq!(game.get_crane(CraneId(1)).unwrap().pos, Position::new(1, 0));
            assert_eq!(game.get_crane(CraneId(2)).unwrap().pos, Position::new(2, 0));

            // 3は存在しない
            assert_eq!(game.get_crane(CraneId(3)), None);
        }

        #[test]
        fn test_move_crane() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            // 0は下右上左に続けて移動することができる
            let res = game.move_crane(CraneId(0), Direction::Down);
            assert!(res.is_ok());
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(1, 0));
            let res = game.move_crane(CraneId(0), Direction::Right);
            assert!(res.is_ok());
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(1, 1));
            let res = game.move_crane(CraneId(0), Direction::Up);
            assert!(res.is_ok());
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 1));
            let res = game.move_crane(CraneId(0), Direction::Left);
            assert!(res.is_ok());
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 0));
            assert_eq!(
                *game.history.get(&CraneId(0)).unwrap(),
                vec![
                    Operation::Move(Direction::Down),
                    Operation::Move(Direction::Right),
                    Operation::Move(Direction::Up),
                    Operation::Move(Direction::Left)
                ]
            );

            // 2は下には移動できない
            let res = game.move_crane(CraneId(2), Direction::Down);
            assert_eq!(res, Err("Invalid move"));
            assert_eq!(game.get_crane(CraneId(2)).unwrap().pos, Position::new(2, 0));
            assert_eq!(game.history.get(&CraneId(2)).unwrap().len(), 0);
        }

        #[test]
        fn test_hold() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            // クレーン0の位置には値がないため持ち上げられない
            let res = game.hold(CraneId(0));
            assert_eq!(res, Err("No value to hold"));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, None);

            // クレーン0の位置に値を置く
            game.board[0][0].value = Some(Value(1));

            // クレーン0が値を持ち上げる
            let res = game.hold(CraneId(0));
            assert!(res.is_ok());
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, Some(Value(1)));
            assert_eq!(game.board[0][0].value, None);
            assert_eq!(
                *game.history.get(&CraneId(0)).unwrap(),
                vec![Operation::Hold]
            );

            // クレーン0が値を持ち上げている状態にする
            game.big_crane.as_mut().unwrap().holding = Some(Value(2));
            // クレーン0の位置に値を置く
            game.board[0][0].value = Some(Value(1));

            // クレーン0が値を持ち上げる
            let res = game.hold(CraneId(0));
            assert_eq!(res, Err("Already holding a value"));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, Some(Value(2)));
            assert_eq!(game.board[0][0].value, Some(Value(1)));
        }

        #[test]
        fn test_release() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            // クレーン0は値を持ち上げていないため置けない
            let res = game.release(CraneId(0));
            assert_eq!(res, Err("No value to release"));
            assert_eq!(game.board[0][0].value, None);
            assert_eq!(game.big_crane.as_ref().unwrap().holding, None);

            // クレーン0が値を持ち上げている状態にする
            game.big_crane.as_mut().unwrap().holding = Some(Value(1));

            // クレーン0が値を置く
            let res = game.release(CraneId(0));
            assert!(res.is_ok());
            assert_eq!(game.board[0][0].value, Some(Value(1)));
            assert_eq!(game.big_crane.as_ref().unwrap().holding, None);
            assert_eq!(
                *game.history.get(&CraneId(0)).unwrap(),
                vec![Operation::Release]
            );

            // クレーン0が値を持ち上げている状態にする
            game.big_crane.as_mut().unwrap().holding = Some(Value(2));

            // クレーン0が値を置く
            let res = game.release(CraneId(0));
            assert_eq!(res, Err("Cell is not empty"));
            assert_eq!(game.board[0][0].value, Some(Value(1)));
            assert_eq!(game.big_crane.as_ref().unwrap().holding, Some(Value(2)));
        }

        #[test]
        fn test_crush() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            // クレーン0を破壊する
            let res = game.crush(CraneId(0));
            assert!(res.is_ok());
            assert_eq!(game.big_crane, None);
            assert_eq!(game.get_crane(CraneId(0)), None);
            assert_eq!(
                *game.history.get(&CraneId(0)).unwrap(),
                vec![Operation::Crush]
            );

            // クレーン0を破壊する
            let res = game.crush(CraneId(0));
            assert_eq!(res, Err("Already crushed"));

            let mut game = Game::new(&input);

            // クレーン0が値を持ち上げている状態にする
            game.big_crane.as_mut().unwrap().holding = Some(Value(1));

            // クレーン0を破壊する
            let res = game.crush(CraneId(0));
            assert_eq!(res, Err("Cannot crush while holding a value"));
            assert_eq!(game.big_crane.as_ref().unwrap().holding, Some(Value(1)));
        }

        #[test]
        fn test_stay() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            // クレーン0の位置には値がないため置けない
            game.stay(CraneId(0));
            assert_eq!(
                *game.history.get(&CraneId(0)).unwrap(),
                vec![Operation::Stay]
            );
        }

        #[test]
        fn test_get_crane_ids() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let game = Game::new(&input);

            // クレーン0, 1, 2が存在する
            assert_eq!(
                game.get_crane_ids(),
                vec![CraneId(0), CraneId(1), CraneId(2)]
            );

            // クレーン0を破壊する
            let mut game = Game::new(&input);
            game.big_crane = None;
            assert_eq!(game.get_crane_ids(), vec![CraneId(1), CraneId(2)]);

            // クレーン1を破壊する
            let mut game = Game::new(&input);
            game.small_crane.remove(&CraneId(1));
            assert_eq!(game.get_crane_ids(), vec![CraneId(0), CraneId(2)]);
        }

        #[test]
        fn test_tick_board_update() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(0), Value(1), Value(2)],
                    vec![Value(3), Value(4), Value(5)],
                    vec![Value(6), Value(7), Value(8)],
                ],
            };
            let mut game = Game::new(&input);

            assert_eq!(game.board[0][0].value, None);
            assert_eq!(game.board[1][0].value, None);
            assert_eq!(game.board[2][0].value, None);

            // Tickすると入力キューの先頭が各行の先頭に移動する
            game.tick();

            assert_eq!(game.board[0][0].value, Some(Value(0)));
            assert_eq!(game.board[1][0].value, Some(Value(3)));
            assert_eq!(game.board[2][0].value, Some(Value(6)));
            assert_eq!(game.input_queues[0][0], Value(1));
            assert_eq!(game.input_queues[1][0], Value(4));
            assert_eq!(game.input_queues[2][0], Value(7));

            // Tickすると各行の最後の値が出力スタックに移動する
            game.board[0][2].value = game.board[0][0].value;
            game.board[1][2].value = game.board[1][0].value;
            game.board[2][2].value = game.board[2][0].value;
            game.board[0][0].value = None;
            game.board[1][0].value = None;
            game.board[2][0].value = None;

            game.tick();

            assert_eq!(game.board[0][2].value, None);
            assert_eq!(game.board[1][2].value, None);
            assert_eq!(game.board[2][2].value, None);
            assert_eq!(game.output_stacks[0][0], Value(0));
            assert_eq!(game.output_stacks[1][0], Value(3));
            assert_eq!(game.output_stacks[2][0], Value(6));
        }

        #[test]
        fn test_tick_crane_operations() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            assert_eq!(game.board[0][0].value, None);
            game.tick();

            // クレーン0が今いるマスの値を持ち上げる
            game.add_operation(CraneId(0), Operation::Hold).unwrap();
            // クレーン0が持ち上げた値を右に移動する
            game.add_operation(CraneId(0), Operation::Move(Direction::Right))
                .unwrap();
            // クレーン0が持ち上げた値を置く
            game.add_operation(CraneId(0), Operation::Release).unwrap();

            assert_eq!(game.board[0][0].value, Some(Value(1)));
            game.tick();
            // 次のinputが入る
            assert_eq!(game.board[0][0].value, Some(Value(2)));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, Some(Value(1)));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 0));
            game.tick();
            assert_eq!(game.board[0][1].value, None);
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, Some(Value(1)));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 1));
            game.tick();
            assert_eq!(game.board[0][1].value, Some(Value(1)));
            assert_eq!(game.get_crane(CraneId(0)).unwrap().holding, None);
            assert_eq!(game.get_crane(CraneId(0)).unwrap().pos, Position::new(0, 1));
        }

        #[test]
        fn test_find_value() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);
            for row in 0..3 {
                for col in 0..3 {
                    game.board[row][col].value = Some(Value(row * 3 + col + 1));
                }
            }

            assert_eq!(game.find_value(Value(1)), Some(Position::new(0, 0)));
            assert_eq!(game.find_value(Value(5)), Some(Position::new(1, 1)));
            assert_eq!(game.find_value(Value(9)), Some(Position::new(2, 2)));
            assert_eq!(game.find_value(Value(10)), None);
        }

        #[test]
        fn test_is_request_completed() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            assert!(!game.is_request_completed());

            game.requests[0] = None;
            game.requests[1] = None;
            game.requests[2] = None;

            assert!(game.is_request_completed());
        }

        #[test]
        fn test_is_crane_operations_empty() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let mut game = Game::new(&input);

            assert!(game.is_crane_operations_empty(CraneId(0)));

            game.add_operation(CraneId(0), Operation::Move(Direction::Down))
                .unwrap();

            assert!(!game.is_crane_operations_empty(CraneId(0)));
        }

        #[test]
        fn test_get_floating_positions() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };
            let game = Game::new(&input);

            let floating_positions = game.get_floating_positions();
            assert_eq!(floating_positions.len(), 0);

            let mut game = Game::new(&input);
            game.board[0][0].value = Some(Value(1));
            game.board[0][1].value = Some(Value(2));
            game.board[0][2].value = None;
            game.board[1][0].value = Some(Value(4));
            game.board[1][1].value = Some(Value(5));
            game.board[1][2].value = None;
            game.board[2][0].value = Some(Value(7));
            game.board[2][1].value = Some(Value(8));
            game.board[2][2].value = None;

            let floating_positions = game.get_floating_positions();
            assert_eq!(floating_positions.len(), 0);

            game.board[0][0].value = None;
            game.board[0][1].value = None;
            game.board[0][2].value = None;
            game.board[1][0].value = None;
            game.board[1][1].value = None;
            game.board[1][2].value = None;
            game.board[2][0].value = None;
            game.board[2][1].value = None;
            game.board[2][2].value = None;

            let floating_positions = game.get_floating_positions();
            assert_eq!(floating_positions.len(), 0);

            game.board[0][0].value = Some(Value(1));
            game.board[0][1].value = None;
            game.board[0][2].value = Some(Value(3));
            game.board[1][0].value = None;
            game.board[1][1].value = Some(Value(5));
            game.board[1][2].value = None;
            game.board[2][0].value = Some(Value(7));
            game.board[2][1].value = None;
            game.board[2][2].value = Some(Value(9));

            let floating_positions = game.get_floating_positions();
            assert_eq!(floating_positions.len(), 2);
            assert_eq!(floating_positions[0], Position::new(0, 0));
            assert_eq!(floating_positions[1], Position::new(2, 0));
        }

        #[test]
        fn test_get_escape_path_flying() {
            let input = Input {
                n: 3,
                a: vec![
                    vec![Value(1), Value(2), Value(3)],
                    vec![Value(4), Value(5), Value(6)],
                    vec![Value(7), Value(8), Value(9)],
                ],
            };

            let from = Position::new(0, 0);
            let to = Position::new(0, 2);

            // timing_slotsが空の場合は直線的なpathが返る
            let game = Game::new(&input);
            let path = game
                .get_escape_path(&from, &to, EscapeMode::Flying, 0)
                .unwrap();
            assert_eq!(path, vec![Direction::Right, Direction::Right]);

            // timing_slotsが埋まっている場合は避けるようなpathが返る
            let mut game = Game::new(&input);
            game.timing_slots[0][1].insert(1);
            let path = game
                .get_escape_path(&from, &to, EscapeMode::Flying, 0)
                .unwrap();
            assert_eq!(
                path,
                vec![
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up
                ]
            );

            // timing_slotsが交差している場合も衝突になるため避けるようなpathが返る
            // 盤面が狭いのでn=4用のinputを使う
            let input = Input {
                n: 4,
                a: vec![
                    vec![Value(1), Value(2), Value(3), Value(4)],
                    vec![Value(5), Value(6), Value(7), Value(8)],
                    vec![Value(9), Value(10), Value(11), Value(12)],
                    vec![Value(13), Value(14), Value(15), Value(16)],
                ],
            };
            let from = Position::new(0, 0);
            let to = Position::new(0, 3);
            let mut game = Game::new(&input);
            game.timing_slots[0][1].insert(2);
            game.timing_slots[0][2].insert(1);
            game.timing_slots[0][3].insert(0);
            let path = game
                .get_escape_path(&from, &to, EscapeMode::Flying, 0)
                .unwrap();
            assert_eq!(
                path,
                vec![
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up
                ]
            );
        }
    }
}

use itertools::Itertools;

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub struct ToolInput {
    n: usize,
    A: Vec<Vec<i32>>,
}

pub struct ToolOutput {
    pub out: Vec<Vec<char>>,
}

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIR: [char; 4] = ['U', 'D', 'L', 'R'];

pub struct ToolState {
    n: usize,
    board: Vec<Vec<i32>>,
    A: Vec<Vec<i32>>,
    B: Vec<Vec<i32>>,
    pos: Vec<(usize, usize, i32)>,
    done: i32,
    turn: i64,
}

impl ToolState {
    fn new(input: &ToolInput) -> Self {
        let mut board = mat![-1; input.n; input.n];
        let mut A = input
            .A
            .iter()
            .map(|a| a.iter().copied().rev().collect_vec())
            .collect_vec();
        for i in 0..input.n {
            board[i][0] = A[i].pop().unwrap();
        }
        ToolState {
            n: input.n,
            board,
            A,
            B: vec![vec![]; input.n],
            pos: (0..input.n).map(|i| (i, 0, -1)).collect_vec(),
            done: 0,
            turn: 0,
        }
    }
    fn apply(&mut self, mv: &[char]) -> Result<(), String> {
        self.turn += 1;
        let mut to = vec![(!0, !0, -1); self.n];
        for i in 0..self.n {
            let (mut x, mut y, mut z) = self.pos[i];
            match mv[i] {
                '.' => (),
                'P' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    } else if z != -1 {
                        return Err(format!("Crane {i} holds a container."));
                    } else if self.board[x][y] == -1 {
                        return Err(format!("No container at ({x}, {y})."));
                    } else {
                        z = self.board[x][y];
                        self.board[x][y] = -1;
                    }
                }
                'Q' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    } else if z == -1 {
                        return Err(format!("Crane {i} does not hold a container."));
                    } else if self.board[x][y] != -1 {
                        return Err(format!("Container already exists at ({x}, {y})."));
                    } else {
                        self.board[x][y] = z;
                        z = -1;
                    }
                }
                'U' | 'D' | 'L' | 'R' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    }
                    let dir = (0..4).find(|&d| DIR[d] == mv[i]).unwrap();
                    let (dx, dy) = DIJ[dir];
                    x += dx;
                    y += dy;
                    if x >= self.n || y >= self.n {
                        return Err(format!("Crane {i} moved out of the board."));
                    } else if i > 0 && z != -1 && self.board[x][y] != -1 {
                        return Err(format!(
                            "Cranes {i} cannot move to a square that contains a container."
                        ));
                    }
                }
                'B' => {
                    if x == !0 {
                        return Err(format!("Crane {i} has already bombed."));
                    }
                    if z != -1 {
                        return Err(format!("Crane {i} holds a container."));
                    }
                    x = !0;
                    y = !0;
                }
                c => {
                    return Err(format!("Invalid move: {}", c));
                }
            }
            to[i] = (x, y, z);
        }
        for i in 0..self.n {
            if to[i].0 == !0 {
                continue;
            }
            for j in 0..i {
                if to[j].0 == !0 {
                    continue;
                }
                if (to[i].0, to[i].1) == (to[j].0, to[j].1) {
                    return Err(format!("Crane {j} and {i} collided."));
                } else if (to[i].0, to[i].1) == (self.pos[j].0, self.pos[j].1)
                    && (to[j].0, to[j].1) == (self.pos[i].0, self.pos[i].1)
                {
                    return Err(format!("Crane {i} and {j} collided."));
                }
            }
        }
        self.pos = to;
        for i in 0..self.n {
            if self.board[i][0] == -1
                && self.A[i].len() > 0
                && self.pos.iter().all(|p| p.2 == -1 || (p.0, p.1) != (i, 0))
            {
                self.board[i][0] = self.A[i].pop().unwrap();
            }
            if self.board[i][self.n - 1] != -1 {
                self.done += 1;
                if (self.n * i) as i32 <= self.board[i][self.n - 1]
                    && self.board[i][self.n - 1] < (self.n * (i + 1)) as i32
                {
                    self.B[i].push(self.board[i][self.n - 1]);
                }
                self.board[i][self.n - 1] = -1;
            }
        }
        Ok(())
    }
    fn score(&self) -> i64 {
        let A = self.turn;
        let mut B = 0;
        let mut C = self.done as i64;
        let D = (self.n * self.n) as i64 - self.done as i64;
        for i in 0..self.n {
            C -= self.B[i].len() as i64;
            for a in 0..self.B[i].len() {
                for b in a + 1..self.B[i].len() {
                    if self.B[i][a] > self.B[i][b] {
                        B += 1;
                    }
                }
            }
        }
        let score = A + B * 100 + C * 10000 + D * 1000000;
        score
    }
}

pub fn compute_score_details(
    input: &ToolInput,
    out: &ToolOutput,
    t: usize,
) -> (i64, String, ToolState) {
    let mut state = ToolState::new(input);
    for k in 0..t {
        let mv = (0..input.n)
            .map(|i| out.out[i].get(k).copied().unwrap_or('.'))
            .collect_vec();
        if let Err(err) = state.apply(&mv) {
            return (0, format!("{err} (turn {k})"), state);
        }
    }
    let score = state.score();
    (score, String::new(), state)
}

pub fn compute_score(input: &ToolInput, out: &ToolOutput) -> (i64, String) {
    let (mut score, err, _) =
        compute_score_details(input, out, out.out.iter().map(|s| s.len()).max().unwrap());
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let a = a
        .iter()
        .map(|row| row.iter().map(|&value| Value(value)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answers = Vec::new();

    (1..n).for_each(|use_small| {
        let input = Input { n, a: a.clone() };
        let mut game = Game::new(&input);
        game.tick();

        let mut times = 0;
        for col in (1..input.n - 1).rev() {
            for row in 0..input.n {
                let mut operations = Vec::new();
                operations.push(Operation::Hold);
                let start_pos = game.get_crane(CraneId(row)).unwrap().pos;
                let hold_pos = Position::new(row, col);
                let start_col = if col == 1 { 1 } else { 0 };
                let release_pos = Position::new(row, start_col);
                get_direct_path(&start_pos, &hold_pos)
                    .iter()
                    .for_each(|direction| {
                        operations.push(Operation::Move(direction.clone()));
                    });
                operations.push(Operation::Release);
                get_direct_path(&hold_pos, &release_pos)
                    .iter()
                    .for_each(|direction| {
                        operations.push(Operation::Move(direction.clone()));
                    });
                let path_positions = simulate_operations(&start_pos, operations.clone());
                operations.iter().for_each(|operation| {
                    game.add_operation(CraneId(row), operation.clone()).unwrap();
                });
                assert_eq!(path_positions.len(), operations.len() + 1);
                for (i, path_pos) in path_positions.iter().enumerate() {
                    game.timing_slots[path_pos.row][path_pos.col].insert(times + i);
                }
                if row == input.n - 1 {
                    times += operations.len();
                }
            }
        }

        for _ in 0..times {
            game.tick();
            eprintln!("{:?}", game);
        }

        assert!(input.n >= use_small + 1);

        for i in use_small + 1..input.n {
            game.add_operation(CraneId(i), Operation::Crush).unwrap();
        }

        // 万が一のために実行が700msを超えたらwhileを抜ける
        let start = std::time::Instant::now();
        while !game.is_request_completed() && start.elapsed().as_millis() < 700 {
            let mut snapshot = game.clone();

            if game.is_crane_operations_empty(CraneId(0)) {
                let big_crane = game.big_crane.as_ref().unwrap();
                // game.requestsの値の中で、盤面に存在する値を持っているクレーンを探し、一番近いものを探す
                let mut min_distance = std::usize::MAX;
                let mut min_hold_pos = None;
                let mut min_release_pos = None;
                for (row, request) in game.requests.iter().enumerate() {
                    let release_pos = Position::new(row, input.n - 1);
                    if let Some(request) = request {
                        eprintln!("TRY REQUEST: {} {:?}", request, release_pos);
                        if let Some(hold_pos) = game.find_value(*request) {
                            eprintln!(
                                "CRANE 0 TRY HOLDING: {} {:?} {:?}",
                                request, hold_pos, release_pos
                            );
                            if game.board[hold_pos.row][hold_pos.col].lock.is_some() {
                                continue;
                            }
                            let mut distance = manhattan_distance(&big_crane.pos, &hold_pos)
                                + manhattan_distance(&hold_pos, &release_pos);
                            // もしoutput_stacksが空のrowだったら優先したいため、空でない場合はdistanceを大きくする
                            if !game.output_stacks[row].is_empty() {
                                distance += 10;
                                // distance += 5;
                            }
                            // もしhold_posがrow==2だったら優先したいため、row!=2の場合はdistanceを大きくする
                            if hold_pos.row != 2 {
                                distance += 5;
                                // distance += 10;
                            }
                            if distance < min_distance {
                                min_distance = distance;
                                min_hold_pos = Some(hold_pos);
                                min_release_pos = Some(release_pos);
                            }
                        }
                    }
                }
                if let (Some(min_hold_pos), Some(min_release_pos)) = (min_hold_pos, min_release_pos)
                {
                    eprintln!(
                        "CRANE 0 current: {:?} hold: {:?} release: {:?}",
                        big_crane.pos, min_hold_pos, min_release_pos
                    );
                    let hold_path =
                        game.get_escape_path(&big_crane.pos, &min_hold_pos, EscapeMode::Flying, 0);
                    if hold_path.is_ok() {
                        let hold_path = hold_path.unwrap();
                        let release_path = game.get_escape_path(
                            &min_hold_pos,
                            &min_release_pos,
                            EscapeMode::Flying,
                            hold_path.len(),
                        );
                        if release_path.is_ok() {
                            let release_path = release_path.unwrap();
                            let mut operations = Vec::new();
                            for direction in hold_path {
                                operations.push(Operation::Move(direction));
                            }
                            operations.push(Operation::Hold);
                            for direction in release_path {
                                operations.push(Operation::Move(direction));
                            }
                            operations.push(Operation::Release);
                            let path_positions =
                                simulate_operations(&big_crane.pos, operations.clone());
                            assert_eq!(path_positions.len(), operations.len() + 1);
                            let mut is_conflicted = false;
                            for (i, path_pos) in path_positions.iter().enumerate() {
                                if game.timing_slots[path_pos.row][path_pos.col].contains(&i)
                                    && i != 0
                                {
                                    is_conflicted = true;
                                    break;
                                }
                            }
                            if !is_conflicted {
                                for (i, path_pos) in path_positions.iter().enumerate() {
                                    game.timing_slots[path_pos.row][path_pos.col].insert(i);
                                }
                                game.board[min_hold_pos.row][min_hold_pos.col].lock =
                                    Some(CraneId(0));
                                operations.iter().for_each(|operation| {
                                    game.add_operation(CraneId(0), operation.clone()).unwrap();
                                });
                            }
                        }
                    }
                } else {
                    // 自分のいる位置にtiming_slotがある場合にない場所へ移動
                    let current_pos = big_crane.pos;
                    let no_timing_slot_cells = game.find_no_timing_slot_cells(&current_pos);
                    if let Some(no_timing_slot_pos) = no_timing_slot_cells.first() {
                        let path = game.get_escape_path(
                            &current_pos,
                            no_timing_slot_pos,
                            EscapeMode::Flying,
                            0,
                        );
                        if path.is_ok() {
                            let path = path.unwrap();
                            let mut operations = Vec::new();
                            for direction in path {
                                operations.push(Operation::Move(direction));
                            }
                            let path_positions =
                                simulate_operations(&big_crane.pos, operations.clone());
                            assert_eq!(path_positions.len(), operations.len() + 1);
                            let mut is_conflicted = false;
                            for (i, path_pos) in path_positions.iter().enumerate() {
                                if game.timing_slots[path_pos.row][path_pos.col].contains(&i)
                                    && i != 0
                                {
                                    is_conflicted = true;
                                    break;
                                }
                            }
                            if !is_conflicted {
                                for (i, path_pos) in path_positions.iter().enumerate() {
                                    game.timing_slots[path_pos.row][path_pos.col].insert(i);
                                }
                                operations.iter().for_each(|operation| {
                                    game.add_operation(CraneId(0), operation.clone()).unwrap();
                                });
                            }
                        }
                    }
                }
            }

            (1..use_small + 1).for_each(|id| {
                let small_crane_id = CraneId(id);
                if game.is_crane_operations_empty(small_crane_id) {
                    eprintln!("1 STACK DETECTED");
                    let mut is_job_found = false;
                    // requestそれぞれについて、EscapeMode::Walkingで処理できるものがないか探す
                    for row in 0..input.n {
                        if game.requests[row].is_none() {
                            continue;
                        }
                        if let Some(hold_pos) = game.find_value(game.requests[row].unwrap()) {
                            if game.board[hold_pos.row][hold_pos.col].lock.is_some() {
                                continue;
                            }
                            eprintln!(
                                "CRANE {} TRY HOLDING: {} {:?}",
                                small_crane_id.0,
                                game.requests[row].unwrap(),
                                hold_pos
                            );
                            let release_pos = Position::new(row, input.n - 1);
                            let hold_path = game.get_escape_path(
                                &game.small_crane.get(&small_crane_id).unwrap().pos,
                                &hold_pos,
                                EscapeMode::Flying,
                                0,
                            );
                            if hold_path.is_err() {
                                continue;
                            }
                            let hold_path = hold_path.unwrap();
                            let release_path = game.get_escape_path(
                                &hold_pos,
                                &release_pos,
                                EscapeMode::Walking,
                                hold_path.len(),
                            );
                            if release_path.is_err() {
                                continue;
                            }
                            let release_path = release_path.unwrap();
                            eprintln!("TRY RELEASE: {:?} {:?}", hold_pos, release_pos);
                            let mut operations = Vec::new();
                            for direction in hold_path {
                                operations.push(Operation::Move(direction));
                            }
                            operations.push(Operation::Hold);
                            for direction in release_path {
                                operations.push(Operation::Move(direction));
                            }
                            operations.push(Operation::Release);
                            let path_positions = simulate_operations(
                                &game.small_crane.get(&small_crane_id).unwrap().pos,
                                operations.clone(),
                            );
                            assert_eq!(path_positions.len(), operations.len() + 1);
                            let mut is_conflicted = false;
                            for (i, path_pos) in path_positions.iter().enumerate() {
                                if game.timing_slots[path_pos.row][path_pos.col].contains(&i)
                                    && i != 0
                                {
                                    is_conflicted = true;
                                    break;
                                }
                            }
                            if !is_conflicted {
                                for (i, path_pos) in path_positions.iter().enumerate() {
                                    game.timing_slots[path_pos.row][path_pos.col].insert(i);
                                }
                                game.board[hold_pos.row][hold_pos.col].lock = Some(small_crane_id);
                                operations.iter().for_each(|operation| {
                                    game.add_operation(small_crane_id, operation.clone())
                                        .unwrap();
                                });
                                is_job_found = true;
                                break;
                            }
                        }
                    }

                    let floating_positions = game.get_floating_positions();
                    eprintln!("{:?}", game);
                    eprintln!("floating_positions: {:?}", floating_positions);

                    // 複数試すようにする
                    struct FloatingJob {
                        distance: usize,
                        hold_pos: Position,
                        release_pos: Position,
                    }

                    let mut escapes = Vec::new();
                    for floating_pos in floating_positions {
                        // floating posに他のクレーンがいる場合はスキップ
                        let mut release_pos = Position::new(floating_pos.row, floating_pos.col + 1);
                        // // マスが空いている限り右にrelease_posを移動
                        // while game.board[release_pos.row][release_pos.col + 1]
                        //     .value
                        //     .is_none()
                        //     && release_pos.col + 1 < input.n - 1
                        // {
                        //     release_pos.col += 1;
                        // }
                        let distance = manhattan_distance(
                            &game.small_crane.get(&small_crane_id).unwrap().pos,
                            &floating_pos,
                        ) + manhattan_distance(&floating_pos, &release_pos);
                        escapes.push(FloatingJob {
                            distance,
                            hold_pos: floating_pos,
                            release_pos,
                        });
                    }
                    escapes.sort_by_key(|escape| escape.distance);

                    let mut is_stacked = true;

                    if !escapes.is_empty() && !is_job_found {
                        for escape in escapes {
                            let hold_path = game.get_escape_path(
                                &game.small_crane.get(&small_crane_id).unwrap().pos,
                                &escape.hold_pos,
                                EscapeMode::Flying,
                                0,
                            );
                            if hold_path.is_ok() {
                                let hold_path = hold_path.unwrap();
                                let release_path = game.get_escape_path(
                                    &escape.hold_pos,
                                    &escape.release_pos,
                                    EscapeMode::Walking,
                                    hold_path.len(),
                                );
                                eprintln!("TRY RELEASE: path: {:?} {:?}", hold_path, release_path);
                                if release_path.is_ok() {
                                    let release_path = release_path.unwrap();
                                    let mut operations = Vec::new();
                                    for direction in hold_path {
                                        operations.push(Operation::Move(direction));
                                    }
                                    operations.push(Operation::Hold);
                                    for direction in release_path {
                                        operations.push(Operation::Move(direction));
                                    }
                                    operations.push(Operation::Release);
                                    let path_positions = simulate_operations(
                                        &game.small_crane.get(&small_crane_id).unwrap().pos,
                                        operations.clone(),
                                    );
                                    let mut is_conflicted = false;
                                    for (i, path_pos) in path_positions.iter().enumerate() {
                                        if game.timing_slots[path_pos.row][path_pos.col]
                                            .contains(&i)
                                            && i != 0
                                        {
                                            is_conflicted = true;
                                            break;
                                        }
                                    }
                                    eprintln!("conflict: {}", is_conflicted);
                                    eprintln!("{:?} {:?}", path_positions, operations);
                                    if !is_conflicted {
                                        for (i, path_pos) in path_positions.iter().enumerate() {
                                            game.timing_slots[path_pos.row][path_pos.col].insert(i);
                                        }
                                        game.board[escape.hold_pos.row][escape.hold_pos.col].lock =
                                            Some(small_crane_id);
                                        operations.iter().for_each(|operation| {
                                            game.add_operation(small_crane_id, operation.clone())
                                                .unwrap();
                                        });
                                        is_stacked = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if is_stacked && !is_job_found {
                        // 自分のいる位置にtiming_slotがある場合にない場所へ移動
                        let current_pos = game.small_crane.get(&small_crane_id).unwrap().pos;
                        let no_timing_slot_cells = game.find_no_timing_slot_cells(&current_pos);
                        if let Some(no_timing_slot_pos) = no_timing_slot_cells.first() {
                            eprintln!("TRY ESCAPE: {:?} {:?}", current_pos, no_timing_slot_pos);
                            let path = game.get_escape_path(
                                &current_pos,
                                no_timing_slot_pos,
                                EscapeMode::Flying,
                                0,
                            );
                            if path.is_ok() {
                                let path = path.unwrap();
                                let mut operations = Vec::new();
                                for direction in path {
                                    operations.push(Operation::Move(direction));
                                }
                                let path_positions = simulate_operations(
                                    &game.small_crane.get(&small_crane_id).unwrap().pos,
                                    operations.clone(),
                                );
                                assert_eq!(path_positions.len(), operations.len() + 1);
                                let mut is_conflicted = false;
                                // path_positionsの0番目は自分の位置なので1から
                                for (i, path_pos) in path_positions.iter().enumerate() {
                                    if game.timing_slots[path_pos.row][path_pos.col].contains(&i)
                                        && i != 0
                                    {
                                        eprintln!("conflict: {:?} {:?}", path_pos, i);
                                        is_conflicted = true;
                                        break;
                                    }
                                }

                                eprintln!("{:?} {:?}", path_positions, operations);
                                eprintln!("is_conflicted: {}", is_conflicted);
                                if !is_conflicted {
                                    for (i, path_pos) in path_positions.iter().enumerate() {
                                        game.timing_slots[path_pos.row][path_pos.col].insert(i);
                                    }
                                    operations.iter().for_each(|operation| {
                                        game.add_operation(small_crane_id, operation.clone())
                                            .unwrap();
                                    });
                                }
                            }
                        }
                    }
                }
            });

            let res = game.tick();
            if res.is_err() {
                // エラーが発生したクレーンのIDを取得
                let err = res.err().unwrap();
                match err {
                    TickError::PathBlocked(crane_id) => {
                        eprintln!("PathBlocked: {:?}", crane_id);
                        snapshot.clear_operations(crane_id);
                        snapshot
                            .add_operation(crane_id, Operation::Release)
                            .unwrap();
                        game = snapshot;
                    }
                    _ => {
                        panic!("unexpected error: {:?}", err);
                    }
                }
            }
            eprintln!("{:?}", game);
            // game.debug_lock();
            // game.debug_timing();
        }

        // println!("{}", game.answer());
        answers.push(game.answer());
    });

    let mut best_score = std::i64::MAX;
    let mut best_answer = String::new();
    for (i, answer) in answers.iter().enumerate() {
        // convert input to ToolInput
        let input = ToolInput {
            n,
            A: a.iter()
                .map(|row| row.iter().map(|value| value.0 as i32).collect())
                .collect(),
        };
        // convert answer to ToolOutput
        let out = ToolOutput {
            out: answer.split('\n').map(|s| s.chars().collect()).collect(),
        };
        let (score, err) = compute_score(&input, &out);
        eprintln!("small: {} score: {} err: {}", i + 1, score, err);
        if score < best_score && err.is_empty() {
            best_score = score;
            best_answer = answer.to_string();
        }
    }

    println!("{}", best_answer);
}
