use std::collections::{HashMap, VecDeque};
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
    let mut current = p1.clone();
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

fn simulate_path(p1: &Position, path: &[Direction]) -> Vec<Position> {
    let mut positions = Vec::new();
    let mut current = p1.clone();
    positions.push(current.clone());
    for direction in path {
        match direction {
            Direction::Up => current.row -= 1,
            Direction::Down => current.row += 1,
            Direction::Left => current.col -= 1,
            Direction::Right => current.col += 1,
        }
        positions.push(current.clone());
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
        Self {
            n,
            board,
            input_queues,
            output_stacks,
            big_crane,
            small_crane,
            requests,
            history,
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
        let pos = crane.pos.clone();

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
            get_direct_path(&game.get_crane(row).unwrap().pos, &Position::new(row, col))
                .iter()
                .for_each(|direction| {
                    operations.push(Operation::Move(direction.clone()));
                });
            operations.push(Operation::Release);
            let start_col = if col == 1 { 1 } else { 0 };
            get_direct_path(&Position::new(row, col), &Position::new(row, start_col))
                .iter()
                .for_each(|direction| {
                    operations.push(Operation::Move(direction.clone()));
                });
            operations.iter().for_each(|operation| {
                game.add_operation(row, operation.clone()).unwrap();
            });
            if row == 0 {
                times += operations.len();
            }
        }
    }

    for _ in 0..times {
        game.tick();
    }

    for i in 1..input.n {
        game.add_operation(i, Operation::Crush).unwrap();
    }

    while !game.is_request_completed() {
        let mut help_needed = Vec::new();
        for row in 0..input.n {
            if !game.input_queues[row].is_empty() && game.board[row][0].value.is_some() {
                help_needed.push(row);
            }
        }

        if game.is_crane_operations_empty(0) && !help_needed.is_empty() {
            let big_crane = game.big_crane.as_ref().unwrap();
            let mut min_distance = std::usize::MAX;
            let mut min_holding_pos = None;
            let mut min_release_pos = None;
            for row in help_needed {
                let holding_pos = Position::new(row, 0);
                let empty_cells = game.find_empty_cells(&holding_pos);
                if empty_cells.is_empty() {
                    continue;
                }
                let release_pos = empty_cells[0];
                let distance = manhattan_distance(&big_crane.pos, &holding_pos)
                    + manhattan_distance(&holding_pos, &release_pos);
                if distance < min_distance {
                    min_distance = distance;
                    min_holding_pos = Some(holding_pos);
                    min_release_pos = Some(release_pos);
                }
            }
            if let (Some(min_holding_pos), Some(min_release_pos)) =
                (min_holding_pos, min_release_pos)
            {
                let holding_path = get_direct_path(&big_crane.pos, &min_holding_pos);
                let release_path = get_direct_path(&min_holding_pos, &min_release_pos);
                for direction in holding_path {
                    game.add_operation(0, Operation::Move(direction)).unwrap();
                }
                game.add_operation(0, Operation::Hold).unwrap();
                for direction in release_path {
                    game.add_operation(0, Operation::Move(direction)).unwrap();
                }
                game.add_operation(0, Operation::Release).unwrap();
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
                let hold_path = get_direct_path(&big_crane.pos, &min_hold_pos);
                let release_path = get_direct_path(&min_hold_pos, &min_release_pos);
                for direction in hold_path {
                    game.add_operation(0, Operation::Move(direction)).unwrap();
                }
                game.add_operation(0, Operation::Hold).unwrap();
                for direction in release_path {
                    game.add_operation(0, Operation::Move(direction)).unwrap();
                }
                game.add_operation(0, Operation::Release).unwrap();
            }
        }

        game.tick();
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
            simulate_path(&p1, &path),
            vec![Position::new(0, 0), Position::new(0, 1)]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down];
        assert_eq!(
            simulate_path(&p1, &path),
            vec![Position::new(0, 0), Position::new(1, 0)]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down, Direction::Right];
        assert_eq!(
            simulate_path(&p1, &path),
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1)
            ]
        );

        let p1 = Position::new(0, 0);
        let path = vec![Direction::Down, Direction::Right, Direction::Right];
        assert_eq!(
            simulate_path(&p1, &path),
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
    fn test_tick() {
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

        assert_eq!(game.is_request_completed(), false);

        game.requests[0] = None;
        game.requests[1] = None;
        game.requests[2] = None;

        assert_eq!(game.is_request_completed(), true);
    }

    #[test]
    fn test_is_crane_operations_empty() {
        let input = Input {
            n: 3,
            a: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut game = Game::new(&input);

        assert_eq!(game.is_crane_operations_empty(0), true);

        game.add_operation(0, Operation::Move(Direction::Down))
            .unwrap();

        assert_eq!(game.is_crane_operations_empty(0), false);
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
}
