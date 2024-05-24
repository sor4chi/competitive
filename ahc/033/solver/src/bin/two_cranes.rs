use proconio::input;

use solver::game::{
    get_direct_path, manhattan_distance, simulate_operations, CraneId, EscapeMode, Game, Input,
    Operation, Position, Value,
};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let a = a
        .iter()
        .map(|row| row.iter().map(|&value| Value(value)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let input = Input { n, a };
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

    for i in 2..input.n {
        game.add_operation(CraneId(i), Operation::Crush).unwrap();
    }

    while !game.is_request_completed() {
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
                        eprintln!("TRY HOLDING: {} {:?} {:?}", request, hold_pos, release_pos);
                        if game.board[hold_pos.row][hold_pos.col].lock.is_some() {
                            continue;
                        }
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
                eprintln!(
                    "CRANE 0 current: {:?} hold: {:?} release: {:?}",
                    big_crane.pos, min_hold_pos, min_release_pos
                );
                let hold_path = game
                    .get_escape_path(&big_crane.pos, &min_hold_pos, EscapeMode::Flying, 0)
                    .unwrap();
                let release_path = game
                    .get_escape_path(
                        &min_hold_pos,
                        &min_release_pos,
                        EscapeMode::Flying,
                        hold_path.len(),
                    )
                    .unwrap();
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
                assert_eq!(path_positions.len(), operations.len() + 1);
                let mut is_conflicted = false;
                for (i, path_pos) in path_positions.iter().enumerate() {
                    if game.timing_slots[path_pos.row][path_pos.col].contains(&i) && i != 0 {
                        is_conflicted = true;
                        break;
                    }
                }
                if !is_conflicted {
                    for (i, path_pos) in path_positions.iter().enumerate() {
                        game.timing_slots[path_pos.row][path_pos.col].insert(i);
                    }
                    game.board[min_hold_pos.row][min_hold_pos.col].lock = Some(CraneId(0));
                    operations.iter().for_each(|operation| {
                        game.add_operation(CraneId(0), operation.clone()).unwrap();
                    });
                }
            } else {
                // 自分のいる位置にtiming_slotがある場合にない場所へ移動
                let current_pos = big_crane.pos;
                let no_timing_slot_cells = game.find_no_timing_slot_cells(&current_pos);
                if let Some(no_timing_slot_pos) = no_timing_slot_cells.first() {
                    let path = game
                        .get_escape_path(&current_pos, no_timing_slot_pos, EscapeMode::Flying, 0)
                        .unwrap();
                    let mut operations = Vec::new();
                    for direction in path {
                        operations.push(Operation::Move(direction));
                    }
                    let path_positions = simulate_operations(&big_crane.pos, operations.clone());
                    assert_eq!(path_positions.len(), operations.len() + 1);
                    let mut is_conflicted = false;
                    for (i, path_pos) in path_positions.iter().enumerate() {
                        if game.timing_slots[path_pos.row][path_pos.col].contains(&i) && i != 0 {
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

        if game.is_crane_operations_empty(CraneId(1)) {
            let floating_positions = game.get_floating_positions();

            // 複数試すようにする
            struct FloatingJob {
                distance: usize,
                hold_pos: Position,
                release_pos: Position,
            }

            let mut escapes = Vec::new();
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
                let distance = manhattan_distance(
                    &game.small_crane.get(&CraneId(1)).unwrap().pos,
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

            if !escapes.is_empty() {
                for escape in escapes {
                    let hold_path = game.get_escape_path(
                        &game.small_crane.get(&CraneId(1)).unwrap().pos,
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
                                &game.small_crane.get(&CraneId(1)).unwrap().pos,
                                operations.clone(),
                            );
                            let mut is_conflicted = false;
                            for (i, path_pos) in path_positions.iter().enumerate() {
                                if game.timing_slots[path_pos.row][path_pos.col].contains(&i)
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
                                    Some(CraneId(1));
                                operations.iter().for_each(|operation| {
                                    game.add_operation(CraneId(1), operation.clone()).unwrap();
                                });
                                is_stacked = false;
                                break;
                            }
                        }
                    }
                }
            }
            if is_stacked {
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
                            "TRY HOLDING: {} {:?}",
                            game.requests[row].unwrap(),
                            hold_pos
                        );
                        let release_pos = Position::new(row, input.n - 1);
                        let hold_path = game.get_escape_path(
                            &game.small_crane.get(&CraneId(1)).unwrap().pos,
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
                            &game.small_crane.get(&CraneId(1)).unwrap().pos,
                            operations.clone(),
                        );
                        assert_eq!(path_positions.len(), operations.len() + 1);
                        let mut is_conflicted = false;
                        for (i, path_pos) in path_positions.iter().enumerate() {
                            if game.timing_slots[path_pos.row][path_pos.col].contains(&i) && i != 0
                            {
                                is_conflicted = true;
                                break;
                            }
                        }
                        if !is_conflicted {
                            for (i, path_pos) in path_positions.iter().enumerate() {
                                game.timing_slots[path_pos.row][path_pos.col].insert(i);
                            }
                            game.board[hold_pos.row][hold_pos.col].lock = Some(CraneId(1));
                            operations.iter().for_each(|operation| {
                                game.add_operation(CraneId(1), operation.clone()).unwrap();
                            });
                            is_job_found = true;
                            break;
                        }
                    }
                }
                if !is_job_found {
                    // 自分のいる位置にtiming_slotがある場合にない場所へ移動
                    let current_pos = game.small_crane.get(&CraneId(1)).unwrap().pos;
                    let no_timing_slot_cells = game.find_no_timing_slot_cells(&current_pos);
                    if let Some(no_timing_slot_pos) = no_timing_slot_cells.first() {
                        eprintln!("TRY ESCAPE: {:?} {:?}", current_pos, no_timing_slot_pos);
                        let path = game
                            .get_escape_path(
                                &current_pos,
                                no_timing_slot_pos,
                                EscapeMode::Flying,
                                0,
                            )
                            .unwrap();
                        let mut operations = Vec::new();
                        for direction in path {
                            operations.push(Operation::Move(direction));
                        }
                        let path_positions = simulate_operations(
                            &game.small_crane.get(&CraneId(1)).unwrap().pos,
                            operations.clone(),
                        );
                        assert_eq!(path_positions.len(), operations.len() + 1);
                        let mut is_conflicted = false;
                        // path_positionsの0番目は自分の位置なので1から
                        for (i, path_pos) in path_positions.iter().enumerate() {
                            if game.timing_slots[path_pos.row][path_pos.col].contains(&i) && i != 0
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
                                game.add_operation(CraneId(1), operation.clone()).unwrap();
                            });
                        }
                    }
                }
            }
        }

        game.tick();
        // eprintln!("{:?}", game);
        // game.debug_lock();
        // game.debug_timing();
    }

    println!("{}", game.answer());
}
