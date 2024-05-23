use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt;

use proconio::input;

struct Input {
    n: usize,
    a: Vec<Vec<usize>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
    (p1.row as isize - p2.row as isize).unsigned_abs()
        + (p1.col as isize - p2.col as isize).unsigned_abs()
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
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
enum Operation {
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

fn get_direct_path(p1: &Position, p2: &Position) -> Vec<Direction> {
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

fn simulate_operations(p1: &Position, operations: Vec<Operation>) -> Vec<Position> {
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

fn simulate_path(p1: &Position, path: Vec<Direction>) -> Vec<Position> {
    let mut positions = Vec::new();
    let mut current = *p1;
    positions.push(current);
    for direction in path {
        match direction {
            Direction::Up => current.row -= 1,
            Direction::Down => current.row += 1,
            Direction::Left => current.col -= 1,
            Direction::Right => current.col += 1,
        }
        positions.push(current);
    }
    positions
}

#[derive(Clone)]
struct BoardCell {
    value: Option<usize>,
}

impl fmt::Debug for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "-1"),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Crane {
    pos: Position,
    holding: Option<usize>,
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

struct Game {
    n: usize,
    board: Vec<Vec<BoardCell>>,
    input_queues: Vec<VecDeque<usize>>,
    output_stacks: Vec<Vec<usize>>,
    big_crane: Option<Crane>,
    small_crane: HashMap<usize, Crane>,
    requests: Vec<Option<usize>>,
    history: Vec<Vec<Operation>>,
    timing_slots: Vec<Vec<BTreeSet<usize>>>,
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Game {{")?;
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

impl Game {
    fn new(input: &Input) -> Self {
        let n = input.n;
        let board = vec![vec![BoardCell { value: None }; n]; n];
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
                row,
                Crane {
                    pos: Position::new(row, 0),
                    holding: None,
                    operations: Vec::new(),
                },
            );
        });
        let requests = (0..n).map(|i| Some(i * n)).collect();
        let history = vec![vec![]; n];
        let timing_slots = vec![vec![BTreeSet::new(); n]; n];
        Self {
            n,
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

    fn get_crane(&self, crane_id: usize) -> Option<&Crane> {
        if crane_id == 0 {
            self.big_crane.as_ref()
        } else {
            self.small_crane.get(&crane_id)
        }
    }

    fn get_crane_mut(&mut self, crane_id: usize) -> Option<&mut Crane> {
        if crane_id == 0 {
            self.big_crane.as_mut()
        } else {
            self.small_crane.get_mut(&crane_id)
        }
    }

    fn move_crane(&mut self, crane_id: usize, direction: Direction) -> Result<(), &str> {
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
        if crane_id == 0 {
            self.big_crane.as_mut().unwrap().pos = new_pos;
        } else {
            self.small_crane.get_mut(&crane_id).unwrap().pos = new_pos;
        }
        self.history[crane_id].push(Operation::Move(direction));
        Ok(())
    }

    fn hold(&mut self, crane_id: usize) -> Result<(), &str> {
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
        if crane_id == 0 {
            self.big_crane.as_mut().unwrap().holding = Some(value);
        } else {
            self.small_crane.get_mut(&crane_id).unwrap().holding = Some(value);
        }
        self.board[pos.row][pos.col].value = None;
        self.history[crane_id].push(Operation::Hold);
        Ok(())
    }

    fn release(&mut self, crane_id: usize) -> Result<(), &str> {
        let crane = self.get_crane(crane_id).ok_or("Invalid crane ID")?;
        let pos = crane.pos;
        let value = if crane_id == 0 {
            self.big_crane.as_mut().unwrap().holding
        } else {
            self.small_crane.get_mut(&crane_id).unwrap().holding
        }
        .ok_or("No value to release")?;
        if self.board[pos.row][pos.col].value.is_some() {
            return Err("Cell is not empty");
        }
        self.board[pos.row][pos.col].value = Some(value);
        if crane_id == 0 {
            self.big_crane.as_mut().unwrap().holding = None;
        } else {
            self.small_crane.get_mut(&crane_id).unwrap().holding = None;
        }
        self.history[crane_id].push(Operation::Release);
        Ok(())
    }

    fn crush(&mut self, crane_id: usize) -> Result<(), &str> {
        if crane_id == 0 {
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
        self.history[crane_id].push(Operation::Crush);
        Ok(())
    }

    fn stay(&mut self, crane_id: usize) {
        self.history[crane_id].push(Operation::Stay);
    }

    fn get_crane_ids(&self) -> Vec<usize> {
        let mut ids = Vec::new();
        if self.big_crane.is_some() {
            ids.push(0);
        }
        ids.extend(self.small_crane.keys().copied());
        ids.sort();
        ids
    }

    fn add_operation(&mut self, crane_id: usize, operation: Operation) -> Result<(), &str> {
        if let Some(crane) = self.get_crane_mut(crane_id) {
            crane.operations.push(operation);
            Ok(())
        } else {
            Err("Invalid crane ID")
        }
    }

    fn find_value(&self, value: usize) -> Option<Position> {
        for row in 0..self.n {
            for col in 0..self.n {
                if self.board[row][col].value == Some(value) {
                    return Some(Position::new(row, col));
                }
            }
        }
        None
    }

    fn is_request_completed(&self) -> bool {
        self.requests.iter().all(|request| request.is_none())
    }

    fn is_crane_operations_empty(&self, crane_id: usize) -> bool {
        self.get_crane(crane_id)
            .map(|crane| crane.operations.is_empty())
            .unwrap_or(true)
    }

    // col+1が空いているセルをfloating_positionとして探す
    fn get_floating_positions(&self) -> Vec<Position> {
        let mut floating_positions = Vec::new();
        for row in 0..self.n {
            for col in 0..self.n - 2 {
                if self.board[row][col].value.is_some() && self.board[row][col + 1].value.is_none()
                {
                    floating_positions.push(Position::new(row, col));
                }
            }
        }
        floating_positions
    }

    fn find_empty_cells(&self, pos: &Position) -> Vec<Position> {
        let mut empty_cells = Vec::new();
        for row in 0..self.n {
            for col in 0..self.n - 1 {
                if self.board[row][col].value.is_none() {
                    empty_cells.push(Position::new(row, col));
                }
            }
        }
        empty_cells.sort_by_key(|empty_pos| manhattan_distance(pos, empty_pos));
        empty_cells
    }

    // timing_slotsを使って衝突を避けるようなpathを生成して返す
    fn get_escape_path(&self, from: &Position, to: &Position) -> Vec<Direction> {
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
            dist: 0,
        });
        visited[from.row][from.col] = true;
        dist[from.row][from.col] = 0;
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
        while current != *from {
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
        path
    }

    fn tick(&mut self) {
        let crane_ids = self.get_crane_ids();
        let all_operations_empty = crane_ids
            .iter()
            .all(|&id| self.get_crane(id).unwrap().operations.is_empty());
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
                        self.move_crane(id, direction.clone()).unwrap();
                    }
                    Operation::Hold => {
                        self.hold(id).unwrap();
                    }
                    Operation::Release => {
                        self.release(id).unwrap();
                    }
                    Operation::Crush => {
                        self.crush(id).unwrap();
                    }
                }
            });
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
                    if self.requests[row] == Some(row * self.n + self.n - 1) {
                        self.requests[row] = None;
                    } else {
                        self.requests[row] = Some(self.requests[row].unwrap() + 1);
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
    }

    fn answer(&self) -> String {
        let mut answer = String::new();
        for operations in &self.history {
            for operation in operations {
                answer.push_str(&format!("{}", operation));
            }
            answer.push('\n');
        }
        answer.pop();
        answer
    }
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let input = Input { n, a };
    let mut game = Game::new(&input);
    game.tick();

    let mut times = 0;
    for col in (1..input.n - 1).rev() {
        for row in 0..input.n {
            let mut operations = Vec::new();
            operations.push(Operation::Hold);
            let start_pos = game.get_crane(row).unwrap().pos;
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
                game.add_operation(row, operation.clone()).unwrap();
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
    }

    for i in 2..input.n {
        game.add_operation(i, Operation::Crush).unwrap();
    }

    while !game.is_request_completed() {
        if game.is_crane_operations_empty(1) {
            let small_crane = game.small_crane.get(&1).unwrap();
            let floating_positions = game.get_floating_positions();
            let mut min_distance = std::usize::MAX;
            let mut min_hold_pos = None;
            let mut min_release_pos = None;
            for floating_pos in floating_positions {
                let mut release_pos = floating_pos;
                // マスが空いている限り右にrelease_posを移動
                while game.board[release_pos.row][release_pos.col + 1]
                    .value
                    .is_none()
                    && release_pos.col + 1 < input.n - 1
                {
                    release_pos.col += 1;
                }
                eprintln!(
                    "floating_pos: {:?}, release_pos: {:?}",
                    floating_pos, release_pos
                );
                let distance = manhattan_distance(&small_crane.pos, &floating_pos)
                    + manhattan_distance(&floating_pos, &release_pos);
                if distance < min_distance {
                    min_distance = distance;
                    min_hold_pos = Some(floating_pos);
                    min_release_pos = Some(release_pos);
                }
            }
            if let (Some(min_hold_pos), Some(min_release_pos)) = (min_hold_pos, min_release_pos) {
                let hold_path = game.get_escape_path(&small_crane.pos, &min_hold_pos);
                let release_path = game.get_escape_path(&min_hold_pos, &min_release_pos);
                let mut operations = Vec::new();
                for direction in hold_path {
                    operations.push(Operation::Move(direction));
                }
                operations.push(Operation::Hold);
                for direction in release_path {
                    operations.push(Operation::Move(direction));
                }
                operations.push(Operation::Release);
                let path_positions = simulate_operations(&small_crane.pos, operations.clone());
                operations.iter().for_each(|operation| {
                    game.add_operation(1, operation.clone()).unwrap();
                });
                assert_eq!(path_positions.len(), operations.len() + 1);
                for (i, path_pos) in path_positions.iter().enumerate() {
                    game.timing_slots[path_pos.row][path_pos.col].insert(i);
                }
            }
        }
        
        if game.is_crane_operations_empty(0) {
            let big_crane = game.big_crane.as_ref().unwrap();
            // game.requestsの値の中で、盤面に存在する値を持っているクレーンを探し、一番近いものを探す
            let mut min_distance = std::usize::MAX;
            let mut min_hold_pos = None;
            let mut min_release_pos = None;
            for (row, request) in game.requests.iter().enumerate() {
                let release_pos = Position::new(row, input.n - 1);
                if let Some(request) = request {
                    if let Some(hold_pos) = game.find_value(*request) {
                        let distance = manhattan_distance(&big_crane.pos, &hold_pos)
                            + manhattan_distance(&hold_pos, &release_pos);
                        if distance < min_distance {
                            min_distance = distance;
                            min_hold_pos = Some(hold_pos);
                            min_release_pos = Some(release_pos);
                        }
                    }
                }
            }
            if let (Some(min_hold_pos), Some(min_release_pos)) = (min_hold_pos, min_release_pos) {
                let hold_path = game.get_escape_path(&big_crane.pos, &min_hold_pos);
                let release_path = game.get_escape_path(&min_hold_pos, &min_release_pos);
                let mut operations = Vec::new();
                for direction in hold_path {
                    operations.push(Operation::Move(direction));
                }
                operations.push(Operation::Hold);
                for direction in release_path {
                    operations.push(Operation::Move(direction));
                }
                operations.push(Operation::Release);
                let path_positions = simulate_operations(&big_crane.pos, operations.clone());
                operations.iter().for_each(|operation| {
                    game.add_operation(0, operation.clone()).unwrap();
                });
                assert_eq!(path_positions.len(), operations.len() + 1);
                for (i, path_pos) in path_positions.iter().enumerate() {
                    game.timing_slots[path_pos.row][path_pos.col].insert(i);
                }
            }
        }


        game.tick();
        eprintln!("{:?}", game);
    }

    println!("{}", game.answer());
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
    fn test_simulate_path() {
        let p1 = Position::new(0, 0);
        let path = vec![Direction::Right];
        assert_eq!(
            simulate_path(&p1, path),
            vec![Position::new(0, 0), Position::new(0, 1)]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down];
        assert_eq!(
            simulate_path(&p1, path),
            vec![Position::new(0, 0), Position::new(1, 0)]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down, Direction::Right];
        assert_eq!(
            simulate_path(&p1, path),
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1)
            ]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down, Direction::Right, Direction::Right];
        assert_eq!(
            simulate_path(&p1, path),
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2)
            ]
        );
    }

    #[test]
    fn test_new_game() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let game = Game::new(&input);
        assert_eq!(game.n, 3);
        assert_eq!(game.board.len(), 3);
        assert_eq!(game.board[0].len(), 3);
        assert_eq!(game.input_queues.len(), 3);
        assert_eq!(game.input_queues[0].len(), 3);
        assert_eq!(game.input_queues[0][0], 1);
        assert_eq!(game.input_queues[0][1], 2);
        assert_eq!(game.input_queues[0][2], 3);
        assert_eq!(game.output_stacks.len(), 3);
        assert_eq!(game.output_stacks[0].len(), 0);
        assert_eq!(game.big_crane.as_ref().unwrap().pos, Position::new(0, 0));
        assert_eq!(game.big_crane.as_ref().unwrap().holding, None);
        assert_eq!(game.big_crane.as_ref().unwrap().operations.len(), 0);
        assert_eq!(game.small_crane.len(), 2);
        assert_eq!(game.small_crane[&1].pos, Position::new(1, 0));
        assert_eq!(game.small_crane[&1].holding, None);
        assert_eq!(game.small_crane[&1].operations.len(), 0);
        assert_eq!(game.small_crane[&2].pos, Position::new(2, 0));
        assert_eq!(game.small_crane[&2].holding, None);
        assert_eq!(game.small_crane[&2].operations.len(), 0);
        assert_eq!(game.requests.len(), 3);
        assert_eq!(game.requests[0], Some(0));
        assert_eq!(game.requests[1], Some(3));
        assert_eq!(game.requests[2], Some(6));
        assert_eq!(game.history.len(), 3);
    }

    #[test]
    fn test_get_crane() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let game = Game::new(&input);

        // 0, 1, 2のクレーンが存在する
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 0));
        assert_eq!(game.get_crane(1).unwrap().pos, Position::new(1, 0));
        assert_eq!(game.get_crane(2).unwrap().pos, Position::new(2, 0));

        // 3は存在しない
        assert_eq!(game.get_crane(3), None);
    }

    #[test]
    fn test_move_crane() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        // 0は下右上左に続けて移動することができる
        let res = game.move_crane(0, Direction::Down);
        assert!(res.is_ok());
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(1, 0));
        let res = game.move_crane(0, Direction::Right);
        assert!(res.is_ok());
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(1, 1));
        let res = game.move_crane(0, Direction::Up);
        assert!(res.is_ok());
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 1));
        let res = game.move_crane(0, Direction::Left);
        assert!(res.is_ok());
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 0));
        assert_eq!(
            game.history[0],
            vec![
                Operation::Move(Direction::Down),
                Operation::Move(Direction::Right),
                Operation::Move(Direction::Up),
                Operation::Move(Direction::Left)
            ]
        );

        // 2は下には移動できない
        let res = game.move_crane(2, Direction::Down);
        assert_eq!(res, Err("Invalid move"));
        assert_eq!(game.get_crane(2).unwrap().pos, Position::new(2, 0));
        assert_eq!(game.history[2].len(), 0);
    }

    #[test]
    fn test_hold() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        // クレーン0の位置には値がないため持ち上げられない
        let res = game.hold(0);
        assert_eq!(res, Err("No value to hold"));
        assert_eq!(game.get_crane(0).unwrap().holding, None);

        // クレーン0の位置に値を置く
        game.board[0][0].value = Some(1);

        // クレーン0が値を持ち上げる
        let res = game.hold(0);
        assert!(res.is_ok());
        assert_eq!(game.get_crane(0).unwrap().holding, Some(1));
        assert_eq!(game.board[0][0].value, None);
        assert_eq!(game.history[0], vec![Operation::Hold]);

        // クレーン0が値を持ち上げている状態にする
        game.big_crane.as_mut().unwrap().holding = Some(2);
        // クレーン0の位置に値を置く
        game.board[0][0].value = Some(1);

        // クレーン0が値を持ち上げる
        let res = game.hold(0);
        assert_eq!(res, Err("Already holding a value"));
        assert_eq!(game.get_crane(0).unwrap().holding, Some(2));
        assert_eq!(game.board[0][0].value, Some(1));
    }

    #[test]
    fn test_release() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        // クレーン0は値を持ち上げていないため置けない
        let res = game.release(0);
        assert_eq!(res, Err("No value to release"));
        assert_eq!(game.board[0][0].value, None);
        assert_eq!(game.big_crane.as_ref().unwrap().holding, None);

        // クレーン0が値を持ち上げている状態にする
        game.big_crane.as_mut().unwrap().holding = Some(1);

        // クレーン0が値を置く
        let res = game.release(0);
        assert!(res.is_ok());
        assert_eq!(game.board[0][0].value, Some(1));
        assert_eq!(game.big_crane.as_ref().unwrap().holding, None);
        assert_eq!(game.history[0], vec![Operation::Release]);

        // クレーン0が値を持ち上げている状態にする
        game.big_crane.as_mut().unwrap().holding = Some(2);

        // クレーン0が値を置く
        let res = game.release(0);
        assert_eq!(res, Err("Cell is not empty"));
        assert_eq!(game.board[0][0].value, Some(1));
        assert_eq!(game.big_crane.as_ref().unwrap().holding, Some(2));
    }

    #[test]
    fn test_crush() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        // クレーン0を破壊する
        let res = game.crush(0);
        assert!(res.is_ok());
        assert_eq!(game.big_crane, None);
        assert_eq!(game.get_crane(0), None);
        assert_eq!(game.history[0], vec![Operation::Crush]);

        // クレーン0を破壊する
        let res = game.crush(0);
        assert_eq!(res, Err("Already crushed"));

        let mut game = Game::new(&input);

        // クレーン0が値を持ち上げている状態にする
        game.big_crane.as_mut().unwrap().holding = Some(1);

        // クレーン0を破壊する
        let res = game.crush(0);
        assert_eq!(res, Err("Cannot crush while holding a value"));
        assert_eq!(game.big_crane.as_ref().unwrap().holding, Some(1));
    }

    #[test]
    fn test_stay() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        // クレーン0の位置には値がないため置けない
        game.stay(0);
        assert_eq!(game.history[0], vec![Operation::Stay]);
    }

    #[test]
    fn test_get_crane_ids() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let game = Game::new(&input);

        // クレーン0, 1, 2が存在する
        assert_eq!(game.get_crane_ids(), vec![0, 1, 2]);

        // クレーン0を破壊する
        let mut game = Game::new(&input);
        game.big_crane = None;
        assert_eq!(game.get_crane_ids(), vec![1, 2]);

        // クレーン1を破壊する
        let mut game = Game::new(&input);
        game.small_crane.remove(&1);
        assert_eq!(game.get_crane_ids(), vec![0, 2]);
    }

    #[test]
    fn test_tick_board_update() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        assert_eq!(game.board[0][0].value, None);
        assert_eq!(game.board[1][0].value, None);
        assert_eq!(game.board[2][0].value, None);

        // Tickすると入力キューの先頭が各行の先頭に移動する
        game.tick();

        assert_eq!(game.board[0][0].value, Some(1));
        assert_eq!(game.board[1][0].value, Some(4));
        assert_eq!(game.board[2][0].value, Some(7));
        assert_eq!(game.input_queues[0][0], 2);
        assert_eq!(game.input_queues[1][0], 5);
        assert_eq!(game.input_queues[2][0], 8);

        // Tickすると各行の最後の値が出力スタックに移動する
        game.board[0][2].value = game.board[0][0].value;
        game.board[0][0].value = None;
        game.board[1][2].value = game.board[1][0].value;
        game.board[1][0].value = None;
        game.board[2][2].value = game.board[2][0].value;
        game.board[2][0].value = None;

        game.tick();

        assert_eq!(game.board[0][2].value, None);
        assert_eq!(game.board[1][2].value, None);
        assert_eq!(game.board[2][2].value, None);
        assert_eq!(game.output_stacks[0][0], 1);
        assert_eq!(game.output_stacks[1][0], 4);
        assert_eq!(game.output_stacks[2][0], 7);
    }

    #[test]
    fn test_tick_crane_operations() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        assert_eq!(game.board[0][0].value, None);
        game.tick();

        // クレーン0が今いるマスの値を持ち上げる
        game.add_operation(0, Operation::Hold).unwrap();
        // クレーン0が持ち上げた値を右に移動する
        game.add_operation(0, Operation::Move(Direction::Right))
            .unwrap();
        // クレーン0が持ち上げた値を置く
        game.add_operation(0, Operation::Release).unwrap();

        assert_eq!(game.board[0][0].value, Some(1));
        game.tick();
        // 次のinputが入る
        assert_eq!(game.board[0][0].value, Some(2));
        assert_eq!(game.get_crane(0).unwrap().holding, Some(1));
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 0));
        game.tick();
        assert_eq!(game.board[0][1].value, None);
        assert_eq!(game.get_crane(0).unwrap().holding, Some(1));
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 1));
        game.tick();
        assert_eq!(game.board[0][1].value, Some(1));
        assert_eq!(game.get_crane(0).unwrap().holding, None);
        assert_eq!(game.get_crane(0).unwrap().pos, Position::new(0, 1));
    }

    #[test]
    fn test_find_value() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);
        for row in 0..3 {
            for col in 0..3 {
                game.board[row][col].value = Some(row * 3 + col + 1);
            }
        }

        assert_eq!(game.find_value(1), Some(Position::new(0, 0)));
        assert_eq!(game.find_value(5), Some(Position::new(1, 1)));
        assert_eq!(game.find_value(9), Some(Position::new(2, 2)));
        assert_eq!(game.find_value(10), None);
    }

    #[test]
    fn test_is_request_completed() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
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
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        assert!(game.is_crane_operations_empty(0));

        game.add_operation(0, Operation::Move(Direction::Down))
            .unwrap();

        assert!(!game.is_crane_operations_empty(0));
    }

    #[test]
    fn test_find_empty_cells() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        for row in 0..3 {
            for col in 0..3 {
                if row == col {
                    continue;
                }
                game.board[row][col].value = Some(row * 3 + col + 1);
            }
        }

        let empty_cells = game.find_empty_cells(&Position::new(0, 0));
        assert_eq!(empty_cells.len(), 2);
        assert_eq!(empty_cells[0], Position::new(0, 0));
        assert_eq!(empty_cells[1], Position::new(1, 1));
        // 右端は空いていても無視される
    }

    #[test]
    fn test_get_floating_positions() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let game = Game::new(&input);

        let floating_positions = game.get_floating_positions();
        assert_eq!(floating_positions.len(), 0);

        let mut game = Game::new(&input);
        game.board[0][0].value = Some(1);
        game.board[0][1].value = Some(2);
        game.board[0][2].value = None;
        game.board[1][0].value = Some(4);
        game.board[1][1].value = Some(5);
        game.board[1][2].value = None;
        game.board[2][0].value = Some(7);
        game.board[2][1].value = Some(8);
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

        game.board[0][0].value = Some(1);
        game.board[0][1].value = None;
        game.board[0][2].value = Some(3);
        game.board[1][0].value = None;
        game.board[1][1].value = Some(5);
        game.board[1][2].value = None;
        game.board[2][0].value = Some(7);
        game.board[2][1].value = None;
        game.board[2][2].value = Some(9);

        let floating_positions = game.get_floating_positions();
        assert_eq!(floating_positions.len(), 2);
        assert_eq!(floating_positions[0], Position::new(0, 0));
        assert_eq!(floating_positions[1], Position::new(2, 0));
    }

    #[test]
    fn test_get_escape_path() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };

        let from = Position::new(0, 0);
        let to = Position::new(0, 2);

        // timing_slotsが空の場合は直線的なpathが返る
        let game = Game::new(&input);
        let path = game.get_escape_path(&from, &to);
        assert_eq!(path, vec![Direction::Right, Direction::Right]);

        // timing_slotsが埋まっている場合は避けるようなpathが返る
        let mut game = Game::new(&input);
        game.timing_slots[0][1].insert(1);
        let path = game.get_escape_path(&from, &to);
        assert_eq!(
            path,
            vec![
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Up
            ]
        );

        // 複雑なケース
        let input = Input {
            n: 5,
            a: vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ],
        };
        let mut game = Game::new(&input);
        let dummy_from = Position::new(0, 4);
        let dummy_to = Position::new(0, 0);
        let path = get_direct_path(&dummy_from, &dummy_to);
        let path_positions = simulate_path(&dummy_from, path);
        for (i, path_pos) in path_positions.iter().enumerate() {
            game.timing_slots[path_pos.row][path_pos.col].insert(i);
        }
        let from = Position::new(0, 0);
        let to = Position::new(4, 4);
        let path = game.get_escape_path(&from, &to);
        assert_eq!(path, vec![]);
    }
}
