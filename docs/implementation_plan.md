# Implementation Plan — All 20 Games Fully Playable

## Goal
Make all 20 games in `src/features/games/page.rs` actually playable client-side (solo / vs AI). No server needed.

## Architecture

All game logic stays in `src/domain/games.rs` (pure Rust, zero framework deps).
Feature layer (`page.rs`) wires signals → domain functions → reactive UI.
Real-time games use `leptos::task::spawn_local` + `gloo_timers` or `web_sys::set_interval`.

---

## Phase 1 — Wire existing domain logic (Quick wins)

### 1. Tic-Tac-Toe (`board_ttt`)
- Player = X, AI = O using `ttt_best_move`
- After each X move → `spawn_local` → AI move after 300ms delay
- `ttt_winner` after every move → show "X wins / O wins / Draw"
- Reset button

### 2. Connect Four (`board_connect_four`)
- Board = `[u8; 42]` (1=player, 2=AI), display 7 column-drop buttons + 6×7 grid
- Click column → `connect_four_drop(col, 1)` → check winner → AI: `connect_four_ai_column` → drop → check winner
- Color: player=accent, AI=red, empty=surface

### 3. Blackjack (`board_blackjack`)
- Deck = `Vec<u8>` shuffled with `js_sys::Math::random()`
- Player hand + dealer hidden card shown as `🂠`
- Hit: draw card, `blackjack_score` → bust check
- Stand: reveal dealer, dealer hits while `blackjack_should_hit` → compare scores

### 4. 15 Puzzle (`board_puzzle`)
- Board = `[u8; 16]`, use `puzzle_move` to validate each click
- Detect solved: `board == [1,2,...,15,0]`
- Show move counter
- Shuffle on start (random valid moves from solved state)

### 5. Hangman (`board_hangman`)
- Word from `hangman_word()` = "rustacean"
- `guessed: Vec<char>` signal
- Click letter → add to guessed
- Reveal: `word.chars().map(|c| if guessed.contains(c) { c } else { '_' })`
- Wrong guesses ≤ 6 → ASCII art hangman progression
- Win if all revealed, lose if 6 wrong

### 6. Wordle (`board_wordle`)
- Word from `wordle_word()` = "lepto"
- Text `<input>` for each row, submit on Enter or button
- Coloring: 🟩 correct pos, 🟨 wrong pos, ⬛ absent
- 6 attempts max

### 7. Sudoku (`board_sudoku`)
- Pre-filled puzzle (hardcoded easy puzzle)  
- Click cell → cycle 1-9 → `sudoku_valid` → highlight red if conflict
- Detect board complete

### 8. Checkers (`board_checkers`)
- Board = `[u8; 32]` (player=1, AI=2), render 8×8 grid
- Click piece → highlight `checkers_moves` legal destinations
- Click destination → move piece
- AI: pick first legal move from `checkers_moves(board, 2)`

### 9. Memory Cards (`board_memory`)
- 8 pairs shuffled → 16 cards face-down
- Click 1st card → reveal; click 2nd card → reveal
- If match → stay revealed + score; if no match → hide both after 1s delay (`spawn_local` + sleep)
- Win when all matched

### 10. Minesweeper (`board_minesweeper`)
- Random mine placement on first click (never mine on first click)
- `minesweeper_adjacent_mines` for number cells
- Flood-fill reveal for empty cells
- Flag with right-click (or toggle button)

---

## Phase 2 — New logic needed (timer-based)

### 11. Typing Speed (`board_typing`)
- `<input>` field, word from `typing_words()`
- Start timer on first keypress, compare on Enter
- WPM = words typed / elapsed minutes
- Cycle through 10 words

### 12. Snake (`board_snake`)
- `gloo_timers::callback::Interval` for game loop (150ms)
- Arrow key / WASD via `window().add_event_listener_with_callback`
- Head, body Vec, food position
- Collision detection with `snake_step`
- Score = body length

### 13. 2048 (`board_2048`)
- Listen to keyboard arrow keys
- Slide + merge logic (pure Rust in domain)
- Spawn random 2/4 tile after each move
- Detect 2048 win + no-move-possible loss

---

## Phase 3 — Real-time / canvas (game loop)

### 14. Tetris — falling pieces with `Interval`
### 15. Pong — ball physics with `Interval`, AI paddle using `pong_ai_y`
### 16. Breakout — ball + paddle + block collision
### 17. Flappy — gravity + jump with `Interval`
### 18. Tower Defense — wave system with `Interval`

### 19. Chess — full move validation (domain logic to be written)
### 20. Tower Defense — wave countdown

---

## Proposed Changes

### `src/domain/games.rs`
- Add `shuffle_deck() -> Vec<u8>` for Blackjack
- Add `puzzle_is_solved(board: &[u8;16]) -> bool`
- Add Wordle coloring: `wordle_check(guess: &str, answer: &str) -> [u8; 5]` (0=absent,1=wrong-pos,2=correct)
- Add `sudoku_puzzle() -> [u8; 81]` (a hardcoded easy starter)
- Add 2048 slide logic: `slide_2048(board: [u32;16], dir: Direction) -> ([u32;16], u32)`

### `src/features/games/page.rs`
- Rewrite each `board_*` function with real game logic
- Add `use crate::domain::games::*` import

### `Cargo.toml`
- Add `gloo-timers = { version = "0.3", features = ["futures"] }` for Snake/Tetris/Pong intervals
- Add `js-sys = "0.3"` for Math.random (already likely present)

---

## Verification Plan

### Automated Tests
- `cargo test` — existing 86 tests + new domain tests for new functions
- `cargo clippy --target wasm32-unknown-unknown -- -D warnings`

### Manual Verification
- `trunk serve` and play each game in browser
- Verify: win/lose detection, AI responses, score tracking, reset functionality

> [!NOTE]
> Phase 1 (10 games) is the focus of this implementation. Phase 2-3 can follow as separate PRs.
