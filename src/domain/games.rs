//! Pure game rules for the browser game collection.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player { Human, Computer }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DuelMode { Human, Computer }

pub fn ttt_winner(board: &[char; 9]) -> Option<char> {
    const LINES: [[usize; 3]; 8] = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
    for [a,b,c] in LINES { if board[a] != ' ' && board[a] == board[b] && board[b] == board[c] { return Some(board[a]); } }
    None
}
pub fn ttt_is_draw(board: &[char; 9]) -> bool { ttt_winner(board).is_none() && board.iter().all(|c| *c != ' ') }
pub fn ttt_best_move(board: &[char; 9]) -> Option<usize> {
    if ttt_winner(board).is_some() || ttt_is_draw(board) { return None; }
    let mut best=None; let mut best_score=i32::MIN;
    for i in 0..9 { if board[i]!=' ' {continue;} let mut next=*board; next[i]='O'; let score=minimax_ttt(&mut next,false); if score>best_score {best_score=score;best=Some(i);} }
    best
}
fn minimax_ttt(board:&mut [char;9], computer_turn:bool)->i32 {
    if let Some(w)=ttt_winner(board){return if w=='O'{10}else{-10};} if ttt_is_draw(board){return 0;}
    let mut value=if computer_turn{i32::MIN}else{i32::MAX};
    for i in 0..9 { if board[i]!=' '{continue;} board[i]=if computer_turn{'O'}else{'X'}; let s=minimax_ttt(board,!computer_turn); board[i]=' '; value=if computer_turn{value.max(s)}else{value.min(s)}; }
    value
}

pub fn connect_four_drop(board:&mut [u8;42],column:usize,piece:u8)->Option<usize>{
    if column>=7{return None;} for row in (0..6).rev(){let i=row*7+column;if board[i]==0{board[i]=piece;return Some(i);}} None
}
pub fn connect_four_winner(board:&[u8;42])->Option<u8>{
    for row in 0..6 {for col in 0..7 {let piece=board[row*7+col];if piece==0{continue;}for(dr,dc)in[(1i32,0i32),(0,1),(1,1),(1,-1)]{let mut ok=true;for step in 1..4{let r=row as i32+dr*step;let c=col as i32+dc*step;if r<0||r>=6||c<0||c>=7||board[r as usize*7+c as usize]!=piece{ok=false;break;}}if ok{return Some(piece);}}}}} None
}
pub fn connect_four_ai_column(board:&[u8;42])->Option<usize>{
    if connect_four_winner(board).is_some(){return None;}
    for col in 0..7{let mut n=*board;if connect_four_drop(&mut n,col,2).is_some()&&connect_four_winner(&n)==Some(2){return Some(col);}}
    for col in 0..7{let mut n=*board;if connect_four_drop(&mut n,col,1).is_some()&&connect_four_winner(&n)==Some(1){return Some(col);}}
    if board[3]==0{Some(3)}else{(0..7).find(|c|board[*c]==0)}
}

pub fn blackjack_score(cards:&[u8])->u8{let(mut total,mut aces)=(0,0);for card in cards{if *card==1{total+=11;aces+=1}else{total+=(*card).min(10)}}while total>21&&aces>0{total-=10;aces-=1}total}
pub fn blackjack_should_hit(cards:&[u8])->bool{blackjack_score(cards)<17}

pub fn sudoku_valid(board:&[u8;81],index:usize,value:u8)->bool{
    if !(1..=9).contains(&value)||index>=81{return false;} let row=index/9;let col=index%9;
    for i in 0..9{if board[row*9+i]==value&&row*9+i!=index{return false;}if board[i*9+col]==value&&i*9+col!=index{return false;}}
    let br=(row/3)*3;let bc=(col/3)*3;for r in br..br+3{for c in bc..bc+3{if r*9+c!=index&&board[r*9+c]==value{return false;}}}true
}

pub fn lights_toggle(board:&mut [bool;25],index:usize){if index>=25{return;}let row=index/5;let col=index%5;for(r,c)in[(row,col),(row.wrapping_sub(1),col),(row+1,col),(row,col.wrapping_sub(1)),(row,col+1)]{if r<5&&c<5{let i=r*5+c;board[i]=!board[i];}}}
pub fn memory_pair(index:usize)->usize{index%8}
pub fn hangman_word()->&'static str{"rustacean"}
pub fn wordle_word()->&'static str{"lepto"}
pub fn typing_words()->[&'static str;10]{["rust","wasm","leptos","browser","system","design","clean","domain","service","game"]}

pub fn puzzle_move(board:&[u8;16],index:usize)->Option<[u8;16]>{if index>=16||board[index]==0{return None;}let empty=board.iter().position(|v|*v==0)?;let r=index/4;let c=index%4;let er=empty/4;let ec=empty%4;if(r as i32-er as i32).abs()+(c as i32-ec as i32).abs()!=1{return None;}let mut n=*board;n.swap(index,empty);Some(n)}
pub fn snake_step(head:(i32,i32),direction:(i32,i32),width:i32,height:i32)->Option<(i32,i32)>{let n=(head.0+direction.0,head.1+direction.1);if n.0<0||n.1<0||n.0>=width||n.1>=height{None}else{Some(n)}}
pub fn minesweeper_adjacent_mines(mines:&[bool;25],index:usize)->u8{if index>=25{return 0;}let row=index/5;let col=index%5;let mut count=0;for dr in -1i32..=1{for dc in -1i32..=1{if dr==0&&dc==0{continue;}let r=row as i32+dr;let c=col as i32+dc;if r>=0&&r<5&&c>=0&&c<5&&mines[r as usize*5+c as usize]{count+=1;}}}count}
pub fn tetris_clear_lines(board:&mut Vec<bool>,width:usize)->usize{if width==0{return 0;}let mut cleared=0;let mut row=0;while row<board.len()/width{if(0..width).all(|c|board[row*width+c]){board.drain(row*width..(row+1)*width);for _ in 0..width{board.insert(0,false);}cleared+=1}else{row+=1;}}cleared}
pub fn pong_ai_y(paddle_y:i32,ball_y:i32,max_y:i32)->i32{(paddle_y+(ball_y-paddle_y)/2).clamp(0,max_y)}

#[cfg(test)]
mod tests{use super::*;
#[test]fn ttt_ai_wins(){let b=['O','O',' ','X','X',' ',' ',' ',' '];assert_eq!(ttt_best_move(&b),Some(2));}
#[test]fn connect_four_stacks_from_bottom(){let mut b=[0u8;42];assert_eq!(connect_four_drop(&mut b,0,1),Some(35));}
#[test]fn blackjack_handles_soft_ace(){assert_eq!(blackjack_score(&[1,10,5]),16);}
#[test]fn sudoku_rejects_duplicate(){let mut b=[0u8;81];b[0]=5;assert!(!sudoku_valid(&b,1,5));}
#[test]fn lights_out_toggles_five(){let mut b=[false;25];lights_toggle(&mut b,12);assert_eq!(b.iter().filter(|v|**v).count(),5);}
#[test]fn puzzle_moves_adjacent_tile(){let b=[1,2,3,4,5,6,7,8,9,10,11,12,13,14,0,15];assert_eq!(puzzle_move(&b,15).unwrap()[14],15);}
#[test]fn snake_rejects_wall(){assert_eq!(snake_step((0,0),(-1,0),5,5),None);}
#[test]fn minesweeper_counts_neighbours(){let mut m=[false;25];m[6]=true;assert_eq!(minesweeper_adjacent_mines(&m,0),1);}
#[test]fn tetris_clears_full_row(){let mut b=vec![true;10];assert_eq!(tetris_clear_lines(&mut b,10),1);assert!(b.iter().all(|v|!*v));}
}