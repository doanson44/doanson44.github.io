//! Pure game rules and AI helpers for the browser game collection.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Human,
    Computer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TicTacToeResult {
    InProgress,
    HumanWon,
    ComputerWon,
    Draw,
}

pub fn ttt_winner(board: &[char; 9]) -> TicTacToeResult {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];
    for line in LINES {
        let [a, b, c] = line;
        if board[a] != ' ' && board[a] == board[b] && board[b] == board[c] {
            return if board[a] == 'X' { TicTacToeResult::HumanWon } else { TicTacToeResult::ComputerWon };
        }
    }
    if board.iter().all(|cell| *cell != ' ') { TicTacToeResult::Draw } else { TicTacToeResult::InProgress }
}

pub fn ttt_best_move(board: &[char; 9]) -> Option<usize> {
    if ttt_winner(board) != TicTacToeResult::InProgress { return None; }
    let mut best = None;
    let mut best_score = i32::MIN;
    for index in 0..9 {
        if board[index] != ' ' { continue; }
        let mut next = *board;
        next[index] = 'O';
        let score = minimax_ttt(&mut next, false);
        if score > best_score { best_score = score; best = Some(index); }
    }
    best
}

fn minimax_ttt(board: &mut [char; 9], computer_turn: bool) -> i32 {
    match ttt_winner(board) {
        TicTacToeResult::ComputerWon => return 10,
        TicTacToeResult::HumanWon => return -10,
        TicTacToeResult::Draw => return 0,
        TicTacToeResult::InProgress => {}
    }
    let mut value = if computer_turn { i32::MIN } else { i32::MAX };
    for index in 0..9 {
        if board[index] != ' ' { continue; }
        board[index] = if computer_turn { 'O' } else { 'X' };
        let score = minimax_ttt(board, !computer_turn);
        board[index] = ' ';
        if computer_turn { value = value.max(score); } else { value = value.min(score); }
    }
    value
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectFourResult { InProgress, HumanWon, ComputerWon, Draw }

pub fn connect_four_drop(board: &mut [u8; 42], column: usize, piece: u8) -> bool {
    if column >= 7 { return false; }
    for row in (0..6).rev() {
        let index = row * 7 + column;
        if board[index] == 0 { board[index] = piece; return true; }
    }
    false
}

pub fn connect_four_result(board: &[u8; 42]) -> ConnectFourResult {
    for row in 0..6 { for col in 0..7 {
        let piece = board[row * 7 + col];
        if piece == 0 { continue; }
        for (dr, dc) in [(1i32,0i32),(0,1),(1,1),(1,-1)] {
            let mut count = 1;
            for step in 1..4 {
                let r = row as i32 + dr * step;
                let c = col as i32 + dc * step;
                if r < 0 || r >= 6 || c < 0 || c >= 7 || board[r as usize * 7 + c as usize] != piece { break; }
                count += 1;
            }
            if count == 4 { return if piece == 1 { ConnectFourResult::HumanWon } else { ConnectFourResult::ComputerWon }; }
        }
    }}
    if board.iter().all(|cell| *cell != 0) { ConnectFourResult::Draw } else { ConnectFourResult::InProgress }
}

pub fn connect_four_ai_column(board: &[u8; 42]) -> Option<usize> {
    if connect_four_result(board) != ConnectFourResult::InProgress { return None; }
    for column in 0..7 {
        let mut next = *board;
        if connect_four_drop(&mut next, column, 2) && connect_four_result(&next) == ConnectFourResult::ComputerWon { return Some(column); }
    }
    for column in 0..7 {
        let mut next = *board;
        if connect_four_drop(&mut next, column, 1) && connect_four_result(&next) == ConnectFourResult::HumanWon { return Some(column); }
    }
    if board[3] == 0 { Some(3) } else { (0..7).find(|column| board[*column] == 0) }
}

pub fn blackjack_score(cards: &[u8]) -> u8 {
    let mut total = 0u8;
    let mut aces = 0u8;
    for card in cards {
        if *card == 1 { total += 11; aces += 1; } else { total += (*card).min(10); }
    }
    while total > 21 && aces > 0 { total -= 10; aces -= 1; }
    total
}

pub fn blackjack_dealer_should_hit(cards: &[u8]) -> bool { blackjack_score(cards) < 17 }

pub fn checkers_moves(board: &[u8; 32], player: u8) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    for from in 0..32 {
        if board[from] != player { continue; }
        let row = from / 4;
        let col = from % 4;
        for delta in [-1i32, 1] {
            let target_row = row as i32 + if player == 1 { -1 } else { 1 };
            let target_col = col as i32 + delta;
            if target_row >= 0 && target_row < 8 && target_col >= 0 && target_col < 4 {
                let to = target_row as usize * 4 + target_col as usize;
                if board[to] == 0 { moves.push((from, to)); }
            }
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tic_tac_toe_ai_takes_winning_move() {
        let board = ['O','O',' ','X','X',' ',' ',' ',' '];
        assert_eq!(ttt_best_move(&board), Some(2));
    }

    #[test]
    fn connect_four_drop_uses_lowest_free_row() {
        let mut board = [0u8; 42];
        assert!(connect_four_drop(&mut board, 0, 1));
        assert_eq!(board[35], 1);
    }

    #[test]
    fn blackjack_aces_adjust_down() {
        assert_eq!(blackjack_score(&[1, 10, 5]), 16);
    }
}