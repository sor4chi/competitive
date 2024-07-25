use crate::io::{Input, IO};
use rand::prelude::SliceRandom;
use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
};

use super::Solver;

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<usize>>,
}

impl Board {
    fn new(input: &Input) -> Self {
        Board {
            cells: vec![vec![0; input.n]; input.n],
        }
    }
}

const DIRECTION: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub struct GreedySolver {
    input: Input,
    io: IO,
}

impl GreedySolver {
    pub fn new(io: IO, input: Input) -> Self {
        GreedySolver { input, io }
    }

    fn solve_all(&mut self) {
        // あり得る盤面を全て作る
        let mut q = VecDeque::new();
        let board = Board::new(&self.input);
        q.push_back(board);
        for mino in &self.input.minos {
            let mut next_q = VecDeque::new();
            while let Some(b) = q.pop_front() {
                for x in 0..self.input.n - mino.height + 1 {
                    for y in 0..self.input.n - mino.width + 1 {
                        let mut new_board = b.clone();
                        for i in 0..mino.height {
                            for j in 0..mino.width {
                                if mino.shape[i][j] {
                                    new_board.cells[x + i][y + j] += 1;
                                }
                            }
                        }
                        next_q.push_back(new_board);
                    }
                }
            }
            q = next_q;
        }

        let mut board_cands = Vec::new();
        while let Some(b) = q.pop_front() {
            board_cands.push(b);
        }

        let mut cells = Vec::new();
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                cells.push((i, j));
            }
        }

        // 情報量が多いセルからdigする
        let mut mis = Vec::new();

        for (x, y) in &cells {
            let mut cnt = vec![0; 100];
            for b in &board_cands {
                cnt[b.cells[*x][*y]] += 1;
            }
            let mut mi = 0.0;
            for i in 0..10 {
                if cnt[i] == 0 {
                    continue;
                }
                let p = cnt[i] as f64 / board_cands.len() as f64;
                mi -= p * p.log2();
            }
            mis.push((mi, (*x, *y)));
        }

        mis.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        while !cells.is_empty() {
            let (x, y) = mis[0].1;
            let res = self.io.query_dig(x, y);
            cells.retain(|&(i, j)| i != x || j != y);
            board_cands.retain(|b| b.cells[x][y] == res);
            if board_cands.len() <= 1 {
                break;
            }
            mis.clear();
            for (x, y) in &cells {
                let mut cnt = vec![0; 100];
                for b in &board_cands {
                    cnt[b.cells[*x][*y]] += 1;
                }
                let mut mi = 0.0;
                for i in 0..10 {
                    if cnt[i] == 0 {
                        continue;
                    }
                    let p = cnt[i] as f64 / board_cands.len() as f64;
                    mi -= p * p.log2();
                }
                mis.push((mi, (*x, *y)));
            }
            mis.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        }

        let mut board_info = Vec::new();
        for x in 0..self.input.n {
            for y in 0..self.input.n {
                if board_cands[0].cells[x][y] != 0 {
                    board_info.push((x, y));
                }
            }
        }

        self.io.answer(board_info);
    }

    fn solve_random_bfs(&mut self) {
        let mut rng = rand::thread_rng();
        let mut d_sum = 0;
        for mino in &self.input.minos {
            d_sum += mino.d;
        }

        let mut known_board = vec![vec![-1; self.input.n]; self.input.n];
        let mut sum = 0;
        let mut cells = Vec::new();
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                cells.push((i, j));
            }
        }

        while !cells.is_empty() {
            let candidate_size = {
                // mが5以下なら100
                if self.input.m <= 5 {
                    100
                }
                // mが10以下なら30
                else if self.input.m <= 10 {
                    30
                }
                // mが15以下なら10
                else if self.input.m <= 15 {
                    10
                }
                // それ以外は5
                else {
                    5
                }
            };
            // あり得る盤面を最大candidate_size個作る
            let mut q = VecDeque::new();
            let mut board = Board::new(&self.input);
            q.push_back(board);
            for (mino_id, mino) in self.input.minos.iter().enumerate() {
                let mut next_v = Vec::new();
                while let Some(b) = q.pop_front() {
                    for x in 0..self.input.n - mino.height + 1 {
                        for y in 0..self.input.n - mino.width + 1 {
                            let mut new_board = b.clone();
                            let mut is_valid = true;
                            'generate: for i in 0..mino.height {
                                for j in 0..mino.width {
                                    // 既知の盤面と矛盾する場合はスキップ
                                    // 既知の盤面が0なのにminoがある場合
                                    if known_board[x + i][y + j] == 0 && mino.shape[i][j] {
                                        is_valid = false;
                                        break 'generate;
                                    }
                                    if mino.shape[i][j] {
                                        new_board.cells[x + i][y + j] += 1;
                                        if mino_id == self.input.m - 1
                                            && known_board[x + i][y + j] != -1
                                            && known_board[x + i][y + j]
                                                != new_board.cells[x + i][y + j] as i32
                                        {
                                            is_valid = false;
                                            break 'generate;
                                        }
                                    }
                                }
                            }

                            if is_valid {
                                next_v.push(new_board);
                            }
                        }
                    }
                }
                // ランダムにcandidate_size個選んであとは捨てる
                next_v.shuffle(&mut rng);
                q.clear();
                for b in next_v.iter().take(min(candidate_size, next_v.len())) {
                    q.push_back(b.clone());
                }
            }

            let mut board_cands = Vec::new();
            while let Some(b) = q.pop_front() {
                board_cands.push(b);
            }

            // 情報量が多いセルからdigする
            let mut mis: Vec<(f64, (usize, usize))> = Vec::new();

            for (x, y) in &cells {
                if known_board[*x][*y] != -1 {
                    continue;
                }
                let mut cnt = vec![0; 100];
                for b in &board_cands {
                    cnt[b.cells[*x][*y]] += 1;
                }
                let mut mi = 0.0;
                for i in 0..10 {
                    if cnt[i] == 0 {
                        continue;
                    }
                    let p = cnt[i] as f64 / board_cands.len() as f64;
                    mi -= p * p.log2();
                }
                mis.push((mi, (*x, *y)));
            }

            let (_, (x, y)) = mis.choose(&mut rng).unwrap();
            // 選んだらcellsから削除
            cells.retain(|&(i, j)| i != *x || j != *y);
            let res = self.io.query_dig(*x, *y);
            known_board[*x][*y] = res as i32;
            sum += res;
            if sum == d_sum {
                break;
            }
            if res != 0 {
                // BFSをする。0がでるまで探索する
                let mut q = VecDeque::new();
                q.push_back((*x, *y));
                while let Some((x, y)) = q.pop_front() {
                    for (dx, dy) in DIRECTION.iter() {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx < 0
                            || nx >= self.input.n as i32
                            || ny < 0
                            || ny >= self.input.n as i32
                        {
                            continue;
                        }
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if known_board[nx][ny] != -1 {
                            continue;
                        }
                        let res = self.io.query_dig(nx, ny);
                        cells.retain(|&(i, j)| i != nx || j != ny);
                        known_board[nx][ny] = res as i32;
                        sum += res;
                        if sum == d_sum {
                            break;
                        }
                        if res != 0 {
                            q.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        let mut board_info = Vec::new();
        for x in 0..self.input.n {
            for y in 0..self.input.n {
                if known_board[x][y] > 0 {
                    board_info.push((x, y));
                }
            }
        }

        self.io.answer(board_info);
    }

    fn solve_almost(&mut self) {
        let mut rng = rand::thread_rng();
        let mut d_sum = 0;
        for mino in &self.input.minos {
            d_sum += mino.d;
        }

        let mut known_board = vec![vec![-1; self.input.n]; self.input.n];
        let mut sum = 0;
        let mut cells = Vec::new();
        for i in 0..self.input.n {
            for j in 0..self.input.n {
                cells.push((i, j));
            }
        }

        let mut reserved_minos: Vec<usize> = Vec::new();
        let mut reserved_cells: HashSet<(usize, usize)> = HashSet::new();

        'end: loop {
            let candidate_size = {
                // mが5以下なら100
                if self.input.m <= 5 {
                    100
                }
                // mが10以下なら30
                else if self.input.m <= 10 {
                    30
                }
                // mが15以下なら10
                else if self.input.m <= 15 {
                    10
                }
                // それ以外は5
                else {
                    5
                }
            };
            // あり得る盤面を最大candidate_size個作る
            let mut q = VecDeque::new();
            let mut board = Board::new(&self.input);
            for (x, y) in &reserved_cells {
                board.cells[*x][*y] = 1;
            }
            q.push_back(board);
            for (mino_id, mino) in self.input.minos.iter().enumerate() {
                if reserved_minos.contains(&mino_id) {
                    continue;
                }
                let mut next_v = Vec::new();
                while let Some(b) = q.pop_front() {
                    for x in 0..self.input.n - mino.height + 1 {
                        for y in 0..self.input.n - mino.width + 1 {
                            let mut new_board = b.clone();
                            let mut is_valid = true;
                            'generate: for i in 0..mino.height {
                                for j in 0..mino.width {
                                    // 既知の盤面と矛盾する場合はスキップ
                                    // 既知の盤面が0なのにminoがある場合
                                    if known_board[x + i][y + j] == 0 && mino.shape[i][j] {
                                        is_valid = false;
                                        break 'generate;
                                    }
                                    if mino.shape[i][j] {
                                        new_board.cells[x + i][y + j] += 1;
                                        if mino_id == self.input.m - 1
                                            && known_board[x + i][y + j] != -1
                                            && known_board[x + i][y + j]
                                                != new_board.cells[x + i][y + j] as i32
                                        {
                                            is_valid = false;
                                            break 'generate;
                                        }
                                    }
                                }
                            }

                            if is_valid {
                                next_v.push(new_board);
                            }
                        }
                    }
                }
                // ランダムにcandidate_size個選んであとは捨てる
                eprintln!("next_v.len() = {}", next_v.len());
                next_v.shuffle(&mut rng);
                q.clear();
                for b in next_v.iter().take(min(candidate_size, next_v.len())) {
                    q.push_back(b.clone());
                }
            }

            let mut board_cands = Vec::new();
            while let Some(b) = q.pop_front() {
                board_cands.push(b);
            }

            // 情報量が多いセルからdigする
            let mut mis: Vec<(f64, (usize, usize))> = Vec::new();
            let mut is_same = true;

            for (x, y) in &cells {
                if known_board[*x][*y] != -1 {
                    continue;
                }
                let mut cnt = vec![0; 100];
                for b in &board_cands {
                    cnt[b.cells[*x][*y]] += 1;
                }
                let mut mi = 0.0;
                for i in 0..10 {
                    if cnt[i] == 0 {
                        continue;
                    }
                    let p = cnt[i] as f64 / board_cands.len() as f64;
                    mi -= p * p.log2();
                }
                if let Some((last_mi, _)) = mis.last() {
                    if *last_mi != mi {
                        is_same = false;
                    }
                }
                mis.push((mi, (*x, *y)));
            }

            if is_same {
                mis.shuffle(&mut rng);
            } else {
                mis.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            }

            if mis.len() > 1 {
                // 残ってるますの1/4もしくは2の最大値だけdigする
                for i in 0..min(self.input.m / 2, mis.len()) {
                    let (x, y) = mis[i].1;
                    let res = self.io.query_dig(x, y);
                    known_board[x][y] = res as i32;
                    sum += res;
                    if sum == d_sum {
                        break;
                    }
                    cells.retain(|&(i, j)| i != x || j != y);
                    board_cands.retain(|b| b.cells[x][y] == res);
                }
            }

            // let (x, y) = max_mi_cell;
            // let res = self.io.query_dig(x, y);
            // known_board[x][y] = res as i32;
            // cells.retain(|&(i, j)| i != x || j != y);
            // board_cands.retain(|b| b.cells[x][y] == res);

            if cells.is_empty() || sum == d_sum {
                // セルが全て割れてれば回答して終了
                let mut board_info = Vec::new();
                for x in 0..self.input.n {
                    for y in 0..self.input.n {
                        if known_board[x][y] > 0 {
                            board_info.push((x, y));
                        }
                    }
                }
                self.io.debug_clear(&self.input);
                self.io.answer(board_info);
                break 'end;
            } else if board_cands.len() == 1
                || (cells.len() <= self.input.n * self.input.n / 2 && board_cands.len() <= 3)
            {
                // board_candsが一つに絞れてるかもしくは半分以上空いてて3個いないに絞れてる
                eprintln!("board_cands.len() = {}", board_cands.len());
                // もし3つ以内に絞れたら答えを出す
                for b in &board_cands {
                    let mut board_info = Vec::new();
                    let mut is_valid = true;
                    for x in 0..self.input.n {
                        for y in 0..self.input.n {
                            if known_board[x][y] > 0 && b.cells[x][y] == 0 {
                                is_valid = false;
                                break;
                            }
                            if b.cells[x][y] != 0 {
                                board_info.push((x, y));
                            }
                        }
                    }

                    if !is_valid {
                        continue;
                    }

                    self.io.debug_clear(&self.input);
                    let ok = self.io.answer(board_info);
                    if ok {
                        break 'end;
                    }
                }
            } else {
                eprintln!("board_cands.len() = {}", board_cands.len());
                if board_cands.len() > 1 {
                    // board_cands[0]をデバッグ出力
                    self.io.debug_clear(&self.input);
                    for x in 0..self.input.n {
                        for y in 0..self.input.n {
                            if board_cands[0].cells[x][y] != 0 {
                                self.io.debug_colorize(x, y, "#ff8888");
                            }
                        }
                    }
                }
            }

            // known_boardの情報ですでに確定するセルを探す
            // BFSをして!0かつ壁と0で囲まれたエリアを探す
            // 見つかったエリアがミノの組み合わせで作れるかどうかを確認
            // 作れるならreservedに追加
            let mut visited = vec![vec![false; self.input.n]; self.input.n];
            let mut areas = Vec::new();
            for i in 0..self.input.n {
                for j in 0..self.input.n {
                    if known_board[i][j] == -1 || visited[i][j] || reserved_cells.contains(&(i, j))
                    {
                        continue;
                    }
                    let mut area = Vec::new();
                    let mut q = VecDeque::new();
                    q.push_back((i, j));
                    visited[i][j] = true;
                    let mut ok = true;
                    while let Some((x, y)) = q.pop_front() {
                        if known_board[x][y] > 0 {
                            area.push((x, y));
                        }

                        for (dx, dy) in DIRECTION.iter() {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx < 0
                                || nx >= self.input.n as i32
                                || ny < 0
                                || ny >= self.input.n as i32
                            {
                                continue;
                            }
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if visited[nx][ny] {
                                continue;
                            }
                            if known_board[nx][ny] == -1 {
                                ok = false;
                                break;
                            }
                            if known_board[nx][ny] == 0 {
                                continue;
                            }
                            if reserved_cells.contains(&(nx, ny)) {
                                continue;
                            }
                            visited[nx][ny] = true;
                            q.push_back((nx, ny));
                        }
                    }
                    if !area.is_empty() && ok {
                        areas.push(area);
                    }
                }
            }

            // areasの中でminoの組み合わせで作れるものをreservedに追加
            for area in &areas {
                let mut might_be_minos = Vec::new();
                // areaのwidth, heightを求める
                let mut min_x = self.input.n;
                let mut max_x = 0;
                let mut min_y = self.input.n;
                let mut max_y = 0;
                for (x, y) in area {
                    min_x = min(min_x, *x);
                    max_x = max(max_x, *x);
                    min_y = min(min_y, *y);
                    max_y = max(max_y, *y);
                }
                let height = max_x - min_x + 1;
                let width = max_y - min_y + 1;
                for (mino_id, mino) in self.input.minos.iter().enumerate() {
                    if mino.width == width && mino.height == height {
                        might_be_minos.push(mino_id);
                    }
                }

                // might_be_minosから一つ選んでそれがareaを作れるか確認
                for mino_id in might_be_minos {
                    let mino = &self.input.minos[mino_id];
                    // 一つならすっぽり入るはず
                    let area_shape = area
                        .iter()
                        .map(|(x, y)| (x - min_x, y - min_y))
                        .collect::<HashSet<_>>();
                    // 全ての要素がmino.shapeと完全に一致するか確認
                    let mut is_same = true;
                    for i in 0..mino.height {
                        for j in 0..mino.width {
                            if mino.shape[i][j] != area_shape.contains(&(i, j)) {
                                is_same = false;
                                break;
                            }
                        }
                    }
                    if is_same {
                        reserved_minos.push(mino_id);
                        for (x, y) in area {
                            reserved_cells.insert((*x, *y));
                        }
                        break;
                    }
                }
            }
        }
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self) {
        if self.input.m == 2 || (self.input.m == 3 && self.input.n <= 10) {
            self.solve_all();
        } else if self.input.m > 5 {
            self.solve_random_bfs();
        } else {
            self.solve_almost();
        }
    }
}
