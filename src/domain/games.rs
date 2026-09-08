//! Pure game rules for the browser game collection.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Human,
    Computer,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DuelMode {
    Human,
    Computer,
}

pub fn ttt_winner(board: &[char; 9]) -> Option<char> {
    const L: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for [a, b, c] in L {
        if board[a] != ' ' && board[a] == board[b] && board[b] == board[c] {
            return Some(board[a]);
        }
    }
    None
}

pub fn ttt_is_draw(board: &[char; 9]) -> bool {
    ttt_winner(board).is_none() && board.iter().all(|c| *c != ' ')
}

pub fn ttt_best_move(board: &[char; 9]) -> Option<usize> {
    if ttt_winner(board).is_some() || ttt_is_draw(board) {
        return None;
    }
    let (mut best, mut bs) = (None, i32::MIN);
    for i in 0..9 {
        if board[i] != ' ' {
            continue;
        }
        let mut n = *board;
        n[i] = 'O';
        let s = minimax_ttt(&mut n, false);
        if s > bs {
            bs = s;
            best = Some(i);
        }
    }
    best
}

fn minimax_ttt(board: &mut [char; 9], computer: bool) -> i32 {
    if let Some(w) = ttt_winner(board) {
        return if w == 'O' { 10 } else { -10 };
    }
    if ttt_is_draw(board) {
        return 0;
    }
    let mut v = if computer { i32::MIN } else { i32::MAX };
    for i in 0..9 {
        if board[i] != ' ' {
            continue;
        }
        board[i] = if computer { 'O' } else { 'X' };
        let s = minimax_ttt(board, !computer);
        board[i] = ' ';
        v = if computer { v.max(s) } else { v.min(s) };
    }
    v
}

pub fn connect_four_drop(board: &mut [u8; 42], column: usize, piece: u8) -> Option<usize> {
    if column >= 7 {
        return None;
    }
    for row in (0..6).rev() {
        let i = row * 7 + column;
        if board[i] == 0 {
            board[i] = piece;
            return Some(i);
        }
    }
    None
}

pub fn connect_four_winner(board: &[u8; 42]) -> Option<u8> {
    for row in 0..6 {
        for col in 0..7 {
            let p = board[row * 7 + col];
            if p == 0 {
                continue;
            }
            for (dr, dc) in [(1i32, 0i32), (0, 1), (1, 1), (1, -1)] {
                let mut ok = true;
                for step in 1..4 {
                    let r = row as i32 + dr * step;
                    let c = col as i32 + dc * step;
                    if !(0..6).contains(&r)
                        || !(0..7).contains(&c)
                        || board[r as usize * 7 + c as usize] != p
                    {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return Some(p);
                }
            }
        }
    }
    None
}

pub fn connect_four_ai_column(board: &[u8; 42]) -> Option<usize> {
    if connect_four_winner(board).is_some() {
        return None;
    }
    for col in 0..7 {
        let mut n = *board;
        if connect_four_drop(&mut n, col, 2).is_some() && connect_four_winner(&n) == Some(2) {
            return Some(col);
        }
    }
    for col in 0..7 {
        let mut n = *board;
        if connect_four_drop(&mut n, col, 1).is_some() && connect_four_winner(&n) == Some(1) {
            return Some(col);
        }
    }
    if board[3] == 0 {
        Some(3)
    } else {
        (0..7).find(|c| board[*c] == 0)
    }
}

pub fn blackjack_score(cards: &[u8]) -> u8 {
    let (mut total, mut aces) = (0, 0);
    for card in cards {
        if *card == 1 {
            total += 11;
            aces += 1;
        } else {
            total += (*card).min(10);
        }
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

pub fn blackjack_should_hit(cards: &[u8]) -> bool {
    blackjack_score(cards) < 17
}

pub fn sudoku_valid(board: &[u8; 81], index: usize, value: u8) -> bool {
    if !(1..=9).contains(&value) || index >= 81 {
        return false;
    }
    let row = index / 9;
    let col = index % 9;
    for i in 0..9 {
        if board[row * 9 + i] == value && row * 9 + i != index {
            return false;
        }
        if board[i * 9 + col] == value && i * 9 + col != index {
            return false;
        }
    }
    let br = row / 3 * 3;
    let bc = col / 3 * 3;
    for r in br..br + 3 {
        for c in bc..bc + 3 {
            if r * 9 + c != index && board[r * 9 + c] == value {
                return false;
            }
        }
    }
    true
}

pub fn lights_toggle(board: &mut [bool; 25], index: usize) {
    if index >= 25 {
        return;
    }
    let row = index / 5;
    let col = index % 5;
    for (r, c) in [
        (row, col),
        (row.wrapping_sub(1), col),
        (row + 1, col),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
    ] {
        if r < 5 && c < 5 {
            let i = r * 5 + c;
            board[i] = !board[i];
        }
    }
}

pub fn memory_pair(index: usize) -> usize {
    index % 8
}

pub fn hangman_word() -> &'static str {
    "rustacean"
}

pub fn wordle_word() -> &'static str {
    "lepto"
}

pub fn typing_words() -> [&'static str; 10] {
    [
        "rust", "wasm", "leptos", "browser", "system", "design", "clean", "domain", "service",
        "game",
    ]
}

pub fn puzzle_move(board: &[u8; 16], index: usize) -> Option<[u8; 16]> {
    if index >= 16 || board[index] == 0 {
        return None;
    }
    let empty = board.iter().position(|v| *v == 0)?;
    let r = index / 4;
    let c = index % 4;
    let er = empty / 4;
    let ec = empty % 4;
    if (r as i32 - er as i32).abs() + (c as i32 - ec as i32).abs() != 1 {
        return None;
    }
    let mut n = *board;
    n.swap(index, empty);
    Some(n)
}

pub fn snake_step(
    head: (i32, i32),
    direction: (i32, i32),
    width: i32,
    height: i32,
) -> Option<(i32, i32)> {
    let n = (head.0 + direction.0, head.1 + direction.1);
    if n.0 < 0 || n.1 < 0 || n.0 >= width || n.1 >= height {
        None
    } else {
        Some(n)
    }
}

pub fn minesweeper_adjacent_mines(mines: &[bool; 25], index: usize) -> u8 {
    if index >= 25 {
        return 0;
    }
    let row = index / 5;
    let col = index % 5;
    let mut count = 0;
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if (0..5).contains(&r) && (0..5).contains(&c) && mines[r as usize * 5 + c as usize] {
                count += 1;
            }
        }
    }
    count
}

pub fn tetris_clear_lines(board: &mut Vec<bool>, width: usize) -> usize {
    if width == 0 {
        return 0;
    }
    let mut cleared = 0;
    let mut row = 0;
    while row < board.len() / width {
        if (0..width).all(|c| board[row * width + c]) {
            board.drain(row * width..(row + 1) * width);
            for _ in 0..width {
                board.insert(0, false);
            }
            cleared += 1;
        } else {
            row += 1;
        }
    }
    cleared
}

pub fn pong_ai_y(paddle_y: i32, ball_y: i32, max_y: i32) -> i32 {
    (paddle_y + (ball_y - paddle_y) / 2).clamp(0, max_y)
}

/// Returns legal non-capturing diagonal moves for a regular checker piece.
pub fn checkers_moves(board: &[u8; 32], player: u8) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for from in 0..32 {
        if board[from] != player {
            continue;
        }
        let row = from / 4;
        let col = from % 4;
        let row_delta = if player == 1 { -1i32 } else { 1 };
        for col_delta in [-1i32, 1] {
            let r = row as i32 + row_delta;
            let c = col as i32 + col_delta;
            if (0..8).contains(&r) && (0..4).contains(&c) {
                let to = r as usize * 4 + c as usize;
                if board[to] == 0 {
                    result.push((from, to));
                }
            }
        }
    }
    result
}

/// Returns `[u8; 5]` colour codes per letter: 2 = correct position, 1 = wrong position, 0 = absent.
pub fn wordle_check(guess: &str, answer: &str) -> [u8; 5] {
    let g: Vec<char> = guess.chars().take(5).collect();
    let a: Vec<char> = answer.chars().take(5).collect();
    let mut result = [0u8; 5];
    let mut answer_used = [false; 5];
    for i in 0..5 {
        if i < g.len() && i < a.len() && g[i] == a[i] {
            result[i] = 2;
            answer_used[i] = true;
        }
    }
    for i in 0..5 {
        if result[i] == 2 || i >= g.len() {
            continue;
        }
        for j in 0..5 {
            if !answer_used[j] && j < a.len() && g[i] == a[j] {
                result[i] = 1;
                answer_used[j] = true;
                break;
            }
        }
    }
    result
}

/// Returns a hardcoded easy Sudoku puzzle (0 = empty cell).
pub fn sudoku_puzzle() -> [u8; 81] {
    [
        5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0,
        0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0,
        2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
    ]
}

/// Returns which cells are pre-filled (given) and must not be edited.
pub fn sudoku_given(puzzle: &[u8; 81]) -> [bool; 81] {
    let mut given = [false; 81];
    for i in 0..81 {
        given[i] = puzzle[i] != 0;
    }
    given
}

/// True when the 15-puzzle board is fully solved: [1, 2, …, 15, 0].
pub fn puzzle_is_solved(board: &[u8; 16]) -> bool {
    board[15] == 0 && (0..15).all(|i| board[i] == i as u8 + 1)
}

/// Slide a 2048 board in direction `dir` (0=up, 1=right, 2=down, 3=left).
/// Returns the new board and the points scored in this move.
pub fn slide_2048(board: [u32; 16], dir: u8) -> ([u32; 16], u32) {
    let mut b = board;
    let mut score = 0u32;

    fn slide_line(line: &mut [u32; 4]) -> u32 {
        let mut pts = 0u32;
        let mut pos = 0usize;
        for i in 0..4 {
            if line[i] != 0 {
                line.swap(i, pos);
                pos += 1;
            }
        }
        for i in 0..3 {
            if line[i] != 0 && line[i] == line[i + 1] {
                line[i] *= 2;
                pts += line[i];
                line[i + 1] = 0;
            }
        }
        let mut pos = 0usize;
        for i in 0..4 {
            if line[i] != 0 {
                if i != pos {
                    line[pos] = line[i];
                    line[i] = 0;
                }
                pos += 1;
            }
        }
        pts
    }

    match dir {
        3 => {
            for row in 0..4 {
                let mut line = [b[row * 4], b[row * 4 + 1], b[row * 4 + 2], b[row * 4 + 3]];
                score += slide_line(&mut line);
                for c in 0..4 {
                    b[row * 4 + c] = line[c];
                }
            }
        }
        1 => {
            for row in 0..4 {
                let mut line = [b[row * 4 + 3], b[row * 4 + 2], b[row * 4 + 1], b[row * 4]];
                score += slide_line(&mut line);
                for c in 0..4 {
                    b[row * 4 + (3 - c)] = line[c];
                }
            }
        }
        0 => {
            for col in 0..4 {
                let mut line = [b[col], b[col + 4], b[col + 8], b[col + 12]];
                score += slide_line(&mut line);
                for r in 0..4 {
                    b[col + r * 4] = line[r];
                }
            }
        }
        2 => {
            for col in 0..4 {
                let mut line = [b[col + 12], b[col + 8], b[col + 4], b[col]];
                score += slide_line(&mut line);
                for r in 0..4 {
                    b[col + (3 - r) * 4] = line[r];
                }
            }
        }
        _ => {}
    }
    (b, score)
}

/// Returns whether any move is still possible on a 2048 board.
pub fn has_move_2048(board: &[u32; 16]) -> bool {
    if board.contains(&0) {
        return true;
    }
    for r in 0..4 {
        for c in 0..4 {
            let v = board[r * 4 + c];
            if c + 1 < 4 && board[r * 4 + c + 1] == v {
                return true;
            }
            if r + 1 < 4 && board[(r + 1) * 4 + c] == v {
                return true;
            }
        }
    }
    false
}

/// Standard 52-card deck (ranks 1..=13 repeated 4 times), shuffled with `rand_index`.
pub fn shuffle_deck(mut rand_index: impl FnMut(usize) -> usize) -> Vec<u8> {
    let mut deck: Vec<u8> = (0..52).map(|i| (i % 13) as u8 + 1).collect();
    for i in (1..deck.len()).rev() {
        let j = rand_index(i + 1).min(i);
        deck.swap(i, j);
    }
    deck
}

/// Damage dealt to the base after a tower-defense wave.
pub fn tower_wave_damage(wave: u32, towers: u32) -> i32 {
    (wave as i32 * 3).saturating_sub(towers as i32 * 2).max(0)
}

/// Seconds before the next wave, shrinking as waves increase (minimum 3).
pub fn tower_wave_countdown(wave: u32) -> u32 {
    (8u32.saturating_sub(wave.saturating_sub(1) / 2)).max(3)
}

/// Rotate a tetromino 90° clockwise, keeping the bounding-box origin.
pub fn tetris_rotate_cw(piece: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if piece.is_empty() {
        return vec![];
    }
    let min_x = piece.iter().map(|p| p.0).min().unwrap_or(0);
    let min_y = piece.iter().map(|p| p.1).min().unwrap_or(0);
    let local: Vec<(i32, i32)> = piece.iter().map(|p| (p.0 - min_x, p.1 - min_y)).collect();
    let height = local.iter().map(|p| p.1).max().unwrap_or(0);
    let rotated: Vec<(i32, i32)> = local.iter().map(|&(x, y)| (height - y, x)).collect();
    rotated
        .into_iter()
        .map(|(x, y)| (x + min_x, y + min_y))
        .collect()
}

/// Clear filled rows on a colour-indexed tetris board.
pub fn tetris_clear_filled(board: &mut Vec<u8>, width: usize) -> usize {
    if width == 0 {
        return 0;
    }
    let mut cleared = 0;
    let mut row = 0;
    while row < board.len() / width {
        if (0..width).all(|c| board[row * width + c] != 0) {
            board.drain(row * width..(row + 1) * width);
            for _ in 0..width {
                board.insert(0, 0);
            }
            cleared += 1;
        } else {
            row += 1;
        }
    }
    cleared
}

// ── Chess (mailbox 8×8, 1=P 2=N 3=B 4=R 5=Q 6=K, negative = black) ──────────

pub fn chess_start() -> [i8; 64] {
    let mut b = [0i8; 64];
    const BACK: [i8; 8] = [4, 2, 3, 5, 6, 3, 2, 4];
    for i in 0..8 {
        b[i] = -BACK[i];
        b[8 + i] = -1;
        b[48 + i] = 1;
        b[56 + i] = BACK[i];
    }
    b
}

pub fn chess_glyph(piece: i8) -> &'static str {
    match piece {
        1 => "♙",
        2 => "♘",
        3 => "♗",
        4 => "♖",
        5 => "♕",
        6 => "♔",
        -1 => "♟",
        -2 => "♞",
        -3 => "♝",
        -4 => "♜",
        -5 => "♛",
        -6 => "♚",
        _ => "",
    }
}

fn chess_rc(i: usize) -> (i32, i32) {
    ((i / 8) as i32, (i % 8) as i32)
}

fn chess_idx(r: i32, c: i32) -> Option<usize> {
    if (0..8).contains(&r) && (0..8).contains(&c) {
        Some((r * 8 + c) as usize)
    } else {
        None
    }
}

fn chess_apply_unchecked(board: &mut [i8; 64], from: usize, to: usize) {
    let p = board[from];
    board[from] = 0;
    let (tr, _) = chess_rc(to);
    board[to] = if p == 1 && tr == 0 {
        5
    } else if p == -1 && tr == 7 {
        -5
    } else {
        p
    };
}

fn chess_king_sq(board: &[i8; 64], white: bool) -> Option<usize> {
    let k = if white { 6 } else { -6 };
    board.iter().position(|&p| p == k)
}

fn chess_piece_attacks(board: &[i8; 64], from: usize, target: usize) -> bool {
    let p = board[from];
    if p == 0 || from == target {
        return false;
    }
    let (fr, fc) = chess_rc(from);
    let (tr, tc) = chess_rc(target);
    let dr = tr - fr;
    let dc = tc - fc;
    match p.abs() {
        1 => {
            let dir = if p > 0 { -1 } else { 1 };
            dr == dir && dc.abs() == 1
        }
        2 => dr.abs() * dc.abs() == 2,
        3 => dr.abs() == dc.abs() && dr != 0 && chess_clear_ray(board, from, target),
        4 => (dr == 0) != (dc == 0) && chess_clear_ray(board, from, target),
        5 => {
            ((dr.abs() == dc.abs() && dr != 0) || ((dr == 0) != (dc == 0)))
                && chess_clear_ray(board, from, target)
        }
        6 => dr.abs() <= 1 && dc.abs() <= 1,
        _ => false,
    }
}

fn chess_clear_ray(board: &[i8; 64], from: usize, to: usize) -> bool {
    let (fr, fc) = chess_rc(from);
    let (tr, tc) = chess_rc(to);
    let sr = (tr - fr).signum();
    let sc = (tc - fc).signum();
    let mut r = fr + sr;
    let mut c = fc + sc;
    while r != tr || c != tc {
        if let Some(i) = chess_idx(r, c) {
            if board[i] != 0 {
                return false;
            }
        } else {
            return false;
        }
        r += sr;
        c += sc;
    }
    true
}

pub fn chess_is_check(board: &[i8; 64], white: bool) -> bool {
    let Some(king) = chess_king_sq(board, white) else {
        return true;
    };
    (0..64).any(|from| {
        let p = board[from];
        p != 0 && (p > 0) != white && chess_piece_attacks(board, from, king)
    })
}

fn chess_pseudo_moves(board: &[i8; 64], from: usize) -> Vec<usize> {
    let p = board[from];
    if p == 0 {
        return vec![];
    }
    let white = p > 0;
    let (r, c) = chess_rc(from);
    let mut dests = Vec::new();
    let mut try_to = |nr: i32, nc: i32| {
        if let Some(i) = chess_idx(nr, nc) {
            let t = board[i];
            if t == 0 || (t > 0) != white {
                dests.push(i);
            }
        }
    };
    match p.abs() {
        1 => {
            let dir = if white { -1 } else { 1 };
            let start = if white { 6 } else { 1 };
            if let Some(i) = chess_idx(r + dir, c) {
                if board[i] == 0 {
                    dests.push(i);
                    if r == start {
                        if let Some(i2) = chess_idx(r + dir * 2, c) {
                            if board[i2] == 0 {
                                dests.push(i2);
                            }
                        }
                    }
                }
            }
            for dc in [-1, 1] {
                if let Some(i) = chess_idx(r + dir, c + dc) {
                    let t = board[i];
                    if t != 0 && (t > 0) != white {
                        dests.push(i);
                    }
                }
            }
        }
        2 => {
            for (dr, dc) in [
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
            ] {
                try_to(r + dr, c + dc);
            }
        }
        3..=5 => {
            let dirs: &[(i32, i32)] = match p.abs() {
                3 => &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
                4 => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
                _ => &[
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (1, 1),
                    (-1, 0),
                    (1, 0),
                    (0, -1),
                    (0, 1),
                ],
            };
            for &(dr, dc) in dirs {
                let mut nr = r + dr;
                let mut nc = c + dc;
                while let Some(i) = chess_idx(nr, nc) {
                    let t = board[i];
                    if t == 0 {
                        dests.push(i);
                    } else {
                        if (t > 0) != white {
                            dests.push(i);
                        }
                        break;
                    }
                    nr += dr;
                    nc += dc;
                }
            }
        }
        6 => {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr != 0 || dc != 0 {
                        try_to(r + dr, c + dc);
                    }
                }
            }
        }
        _ => {}
    }
    dests
}

pub fn chess_legal_moves(board: &[i8; 64], from: usize) -> Vec<usize> {
    let p = board[from];
    if p == 0 {
        return vec![];
    }
    let white = p > 0;
    chess_pseudo_moves(board, from)
        .into_iter()
        .filter(|&to| {
            let mut n = *board;
            chess_apply_unchecked(&mut n, from, to);
            !chess_is_check(&n, white)
        })
        .collect()
}

pub fn chess_apply_move(board: &[i8; 64], from: usize, to: usize) -> Option<[i8; 64]> {
    if !chess_legal_moves(board, from).contains(&to) {
        return None;
    }
    let mut n = *board;
    chess_apply_unchecked(&mut n, from, to);
    Some(n)
}

pub fn chess_has_move(board: &[i8; 64], white: bool) -> bool {
    (0..64).any(|from| {
        let p = board[from];
        p != 0 && (p > 0) == white && !chess_legal_moves(board, from).is_empty()
    })
}

/// Picks a capture if available, otherwise the first quiet legal move (black).
pub fn chess_ai_move(board: &[i8; 64]) -> Option<(usize, usize)> {
    let mut capture = None;
    let mut quiet = None;
    for from in 0..64 {
        if board[from] >= 0 {
            continue;
        }
        for to in chess_legal_moves(board, from) {
            if board[to] > 0 {
                capture = Some((from, to));
            } else if quiet.is_none() {
                quiet = Some((from, to));
            }
        }
    }
    capture.or(quiet)
}

/// Flood-fill reveal for Minesweeper: reveals all connected safe cells from `index`.
/// Returns the list of newly revealed indices.
pub fn minesweeper_flood_reveal(
    mines: &[bool; 25],
    revealed: &[bool; 25],
    index: usize,
) -> Vec<usize> {
    if index >= 25 || mines[index] {
        return vec![];
    }
    let mut visited = *revealed;
    let mut queue = vec![index];
    let mut result = vec![];
    while let Some(idx) = queue.pop() {
        if visited[idx] {
            continue;
        }
        visited[idx] = true;
        result.push(idx);
        if minesweeper_adjacent_mines(mines, idx) == 0 {
            let row = idx / 5;
            let col = idx % 5;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let r = row as i32 + dr;
                    let c = col as i32 + dc;
                    if (0..5).contains(&r) && (0..5).contains(&c) {
                        let ni = r as usize * 5 + c as usize;
                        if !visited[ni] {
                            queue.push(ni);
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttt_ai_wins() {
        let b = ['O', 'O', ' ', 'X', 'X', ' ', ' ', ' ', ' '];
        assert_eq!(ttt_best_move(&b), Some(2));
    }

    #[test]
    fn connect_four_stacks_from_bottom() {
        let mut b = [0u8; 42];
        assert_eq!(connect_four_drop(&mut b, 0, 1), Some(35));
    }

    #[test]
    fn blackjack_handles_soft_ace() {
        assert_eq!(blackjack_score(&[1, 10, 5]), 16);
    }

    #[test]
    fn sudoku_rejects_duplicate() {
        let mut b = [0u8; 81];
        b[0] = 5;
        assert!(!sudoku_valid(&b, 1, 5));
    }

    #[test]
    fn lights_out_toggles_five() {
        let mut b = [false; 25];
        lights_toggle(&mut b, 12);
        assert_eq!(b.iter().filter(|v| **v).count(), 5);
    }

    #[test]
    fn puzzle_moves_adjacent_tile() {
        let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];
        assert_eq!(puzzle_move(&b, 15).unwrap()[14], 15);
    }

    #[test]
    fn snake_rejects_wall() {
        assert_eq!(snake_step((0, 0), (-1, 0), 5, 5), None);
    }

    #[test]
    fn minesweeper_counts_neighbours() {
        let mut m = [false; 25];
        m[6] = true;
        assert_eq!(minesweeper_adjacent_mines(&m, 0), 1);
    }

    #[test]
    fn tetris_clears_full_row() {
        let mut b = vec![true; 10];
        assert_eq!(tetris_clear_lines(&mut b, 10), 1);
        assert!(b.iter().all(|v| !*v));
    }

    #[test]
    fn checkers_has_forward_move() {
        let mut b = [0u8; 32];
        b[20] = 1;
        assert!(!checkers_moves(&b, 1).is_empty());
    }

    #[test]
    fn wordle_check_correct_word() {
        assert_eq!(wordle_check("lepto", "lepto"), [2, 2, 2, 2, 2]);
    }

    #[test]
    fn wordle_check_wrong_position() {
        let r = wordle_check("elpto", "lepto");
        assert_eq!(r[0], 1);
        assert_eq!(r[1], 1);
        assert_eq!(r[2], 2);
    }

    #[test]
    fn puzzle_solved_detection() {
        let b: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        assert!(puzzle_is_solved(&b));
    }

    #[test]
    fn slide_2048_merges_left() {
        let mut board = [0u32; 16];
        board[0] = 2;
        board[1] = 2;
        let (b, score) = slide_2048(board, 3);
        assert_eq!(b[0], 4);
        assert_eq!(b[1], 0);
        assert_eq!(score, 4);
    }

    #[test]
    fn minesweeper_flood_reveals_safe_region() {
        let mines = [false; 25];
        let revealed = [false; 25];
        let cells = minesweeper_flood_reveal(&mines, &revealed, 0);
        assert_eq!(cells.len(), 25);
    }

    #[test]
    fn shuffle_deck_has_52_cards() {
        let mut i = 0usize;
        let d = shuffle_deck(|n| {
            i = (i + 3) % n.max(1);
            i
        });
        assert_eq!(d.len(), 52);
        assert_eq!(d.iter().filter(|&&c| c == 1).count(), 4);
    }

    #[test]
    fn tower_damage_scales_with_wave() {
        assert_eq!(tower_wave_damage(1, 3), 0);
        assert!(tower_wave_damage(8, 3) > 0);
        assert_eq!(tower_wave_countdown(1), 8);
        assert_eq!(tower_wave_countdown(20), 3);
    }

    #[test]
    fn tetris_rotate_turns_i_piece() {
        let p = [(0, 0), (1, 0), (2, 0), (3, 0)];
        let r = tetris_rotate_cw(&p);
        assert_eq!(r.len(), 4);
        assert_ne!(r, p);
    }

    #[test]
    fn tetris_clears_filled_u8_row() {
        let mut b = vec![1u8; 10];
        assert_eq!(tetris_clear_filled(&mut b, 10), 1);
        assert!(b.iter().all(|&v| v == 0));
    }

    #[test]
    fn chess_white_pawn_can_double_step() {
        let b = chess_start();
        let moves = chess_legal_moves(&b, 52);
        assert!(moves.contains(&44));
        assert!(moves.contains(&36));
    }

    #[test]
    fn chess_knight_from_b1() {
        let b = chess_start();
        let moves = chess_legal_moves(&b, 57);
        assert!(moves.contains(&40));
        assert!(moves.contains(&42));
    }

    #[test]
    fn chess_rejects_illegal_jump() {
        let b = chess_start();
        assert!(chess_apply_move(&b, 56, 40).is_none());
    }

    #[test]
    fn chess_king_cannot_move_into_check() {
        let mut b = [0i8; 64];
        b[60] = 6;
        b[4] = -4;
        let moves = chess_legal_moves(&b, 60);
        assert!(!moves.contains(&52));
    }
}
