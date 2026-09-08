#![allow(clippy::possible_missing_else)]
use crate::domain::games::{
    blackjack_score, blackjack_should_hit, checkers_moves, chess_ai_move, chess_apply_move,
    chess_glyph, chess_has_move, chess_is_check, chess_legal_moves, chess_start,
    connect_four_ai_column, connect_four_drop, connect_four_winner, hangman_word, has_move_2048,
    lights_toggle, minesweeper_adjacent_mines, minesweeper_flood_reveal, puzzle_is_solved,
    puzzle_move, shuffle_deck, slide_2048, snake_step, sudoku_given, sudoku_puzzle, sudoku_valid,
    tetris_clear_filled, tetris_rotate_cw, tower_wave_countdown, tower_wave_damage, ttt_best_move,
    ttt_is_draw, ttt_winner, typing_words, wordle_check, wordle_word,
};
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

fn bind_keys(handler: impl Fn(web_sys::KeyboardEvent) + 'static) {
    let handle = window_event_listener(ev::keydown, handler);
    on_cleanup(move || handle.remove());
}

fn is_text_input(e: &web_sys::KeyboardEvent) -> bool {
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .is_some_and(|el| {
            matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
                || el.is_content_editable()
        })
}

fn dpad(
    on_up: impl Fn() + Copy + 'static,
    on_left: impl Fn() + Copy + 'static,
    on_down: impl Fn() + Copy + 'static,
    on_right: impl Fn() + Copy + 'static,
) -> AnyView {
    let btn = "flex h-12 w-12 items-center justify-center rounded-lg border border-[var(--border-color)] text-lg font-bold text-[var(--text-primary)] hover:bg-[var(--surface-hover)] active:scale-95 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";
    view! {
        <div class="mx-auto grid w-max grid-cols-3 gap-1">
            <div></div>
            <button type="button" class=btn on:click=move |_| on_up()>"↑"</button>
            <div></div>
            <button type="button" class=btn on:click=move |_| on_left()>"←"</button>
            <button type="button" class=btn on:click=move |_| on_down()>"↓"</button>
            <button type="button" class=btn on:click=move |_| on_right()>"→"</button>
        </div>
    }
    .into_any()
}

fn rand_f64() -> f64 {
    js_sys::Math::random()
}

fn rand_usize(n: usize) -> usize {
    (rand_f64() * n as f64) as usize
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameKind {
    TwentyFortyEight,
    TicTacToe,
    Minesweeper,
    Snake,
    Sudoku,
    ConnectFour,
    Memory,
    Typing,
    Wordle,
    Hangman,
    FifteenPuzzle,
    LightsOut,
    TowerDefense,
    Breakout,
    Pong,
    Flappy,
    Tetris,
    Chess,
    Checkers,
    Blackjack,
}
impl GameKind {
    fn all() -> [Self; 20] {
        [
            Self::TwentyFortyEight,
            Self::TicTacToe,
            Self::Minesweeper,
            Self::Snake,
            Self::Sudoku,
            Self::ConnectFour,
            Self::Memory,
            Self::Typing,
            Self::Wordle,
            Self::Hangman,
            Self::FifteenPuzzle,
            Self::LightsOut,
            Self::TowerDefense,
            Self::Breakout,
            Self::Pong,
            Self::Flappy,
            Self::Tetris,
            Self::Chess,
            Self::Checkers,
            Self::Blackjack,
        ]
    }
    fn title(self) -> &'static str {
        match self {
            Self::TwentyFortyEight => "2048",
            Self::TicTacToe => "Tic-Tac-Toe",
            Self::Minesweeper => "Minesweeper",
            Self::Snake => "Snake",
            Self::Sudoku => "Sudoku",
            Self::ConnectFour => "Connect Four",
            Self::Memory => "Memory Cards",
            Self::Typing => "Typing Speed",
            Self::Wordle => "Wordle",
            Self::Hangman => "Hangman",
            Self::FifteenPuzzle => "15 Puzzle",
            Self::LightsOut => "Lights Out",
            Self::TowerDefense => "Mini Tower Defense",
            Self::Breakout => "Breakout",
            Self::Pong => "Pong",
            Self::Flappy => "Flappy",
            Self::Tetris => "Tetris",
            Self::Chess => "Chess",
            Self::Checkers => "Checkers",
            Self::Blackjack => "Blackjack",
        }
    }
    fn description(self) -> &'static str {
        match self {
            Self::TwentyFortyEight => "Slide tiles to merge and reach 2048.",
            Self::TicTacToe => "Play against an AI on a classic 3×3 grid.",
            Self::Minesweeper => "Reveal safe cells without hitting mines.",
            Self::Snake => "Grow the snake and avoid the walls.",
            Self::Sudoku => "Complete the logic grid without repeating numbers.",
            Self::ConnectFour => "Drop four in a row before the AI does.",
            Self::Memory => "Find all matching pairs to win.",
            Self::Typing => "Type words as fast as you can.",
            Self::Wordle => "Guess the 5-letter word in 6 tries.",
            Self::Hangman => "Guess the word before the figure is complete.",
            Self::FifteenPuzzle => "Slide tiles into numerical order.",
            Self::LightsOut => "Toggle lights until all are off.",
            Self::TowerDefense => "Survive waves with a tiny defense line.",
            Self::Breakout => "Break all blocks with the ball.",
            Self::Pong => "Keep the ball away from your side.",
            Self::Flappy => "Navigate gaps with timed jumps.",
            Self::Tetris => "Clear lines with falling blocks.",
            Self::Chess => "Play a lightweight local chess board.",
            Self::Checkers => "Capture pieces on a checkers board.",
            Self::Blackjack => "Beat the dealer without going over 21.",
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Self::TwentyFortyEight => "2048",
            Self::TicTacToe => "XO",
            Self::Minesweeper => "💣",
            Self::Snake => "🐍",
            Self::Sudoku => "9",
            Self::ConnectFour => "4",
            Self::Memory => "🃏",
            Self::Typing => "⌨",
            Self::Wordle => "W",
            Self::Hangman => "H",
            Self::FifteenPuzzle => "15",
            Self::LightsOut => "💡",
            Self::TowerDefense => "🏰",
            Self::Breakout => "🧱",
            Self::Pong => "🏓",
            Self::Flappy => "🐦",
            Self::Tetris => "T",
            Self::Chess => "♟",
            Self::Checkers => "⚫",
            Self::Blackjack => "21",
        }
    }
}

#[component]
pub fn GamesPage() -> impl IntoView {
    let selected = RwSignal::new(None::<GameKind>);
    view! { <main class="flex flex-1 flex-col px-4 py-8 sm:px-6 lg:px-8"><div class="mx-auto w-full max-w-7xl">
        <div class="mb-8 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between"><div><p class="text-xs font-semibold uppercase tracking-widest text-[var(--accent)]">"Arcade"</p><h1 class="mt-1 text-3xl font-bold text-[var(--text-primary)]">"Games"</h1><p class="mt-2 max-w-2xl text-sm text-[var(--text-secondary)]">"Twenty compact browser games — all client-side Rust/WASM, no server needed."</p></div><span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs text-[var(--text-tertiary)]">"20 games"</span></div>
        {move || match selected.get(){Some(game)=>view!{<GameView game on_back=move||selected.set(None) />}.into_any(),None=>view!{<GameGrid on_select=move|g|selected.set(Some(g))/>}.into_any()}}
    </div></main> }
}

#[component]
fn GameGrid(on_select: impl Fn(GameKind) + Copy + 'static) -> impl IntoView {
    view! { <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">{GameKind::all().into_iter().map(|game|view!{
    <button type="button" class="group flex min-h-40 flex-col rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 text-left transition hover:-translate-y-0.5 hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" on:click=move |_|on_select(game)><span class="flex h-10 w-10 items-center justify-center rounded-lg border border-[var(--border-color)] text-xs font-bold text-[var(--accent)]">{game.icon()}</span><span class="mt-4 text-base font-semibold text-[var(--text-primary)]">{game.title()}</span><span class="mt-1 text-sm text-[var(--text-secondary)]">{game.description()}</span><span class="mt-auto pt-4 text-xs font-medium text-[var(--accent)]">"Play →"</span></button>
    }).collect_view()}</div> }
}

#[component]
fn GameView(game: GameKind, on_back: impl Fn() + Copy + 'static) -> impl IntoView {
    let score = RwSignal::new(0u32);
    let status = RwSignal::new(String::from("Ready"));
    view! { <section class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-4 sm:p-6"><div class="mb-5 flex flex-wrap items-center gap-3"><button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" on:click=move |_|on_back()>"← All games"</button><div class="min-w-0 flex-1"><h2 class="text-xl font-bold text-[var(--text-primary)]">{game.title()}</h2><p class="text-sm text-[var(--text-secondary)]">{game.description()}</p></div><span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs text-[var(--text-tertiary)]">{move||status.get()}</span></div>
    {match game {
        GameKind::TwentyFortyEight=>board_2048(score,status),
        GameKind::TicTacToe=>board_ttt(score,status),
        GameKind::Minesweeper=>board_mines(score,status),
        GameKind::Snake=>board_snake(score,status),
        GameKind::Sudoku=>board_sudoku(score,status),
        GameKind::ConnectFour=>board_connect_four(score,status),
        GameKind::Memory=>board_memory(score,status),
        GameKind::Typing=>board_typing(score,status),
        GameKind::Wordle=>board_wordle(score,status),
        GameKind::Hangman=>board_hangman(score,status),
        GameKind::FifteenPuzzle=>board_puzzle(score,status),
        GameKind::LightsOut=>board_lights(score,status),
        GameKind::TowerDefense=>board_tower(score,status),
        GameKind::Breakout=>board_breakout(score,status),
        GameKind::Pong=>board_pong(score,status),
        GameKind::Flappy=>board_flappy(score,status),
        GameKind::Tetris=>board_tetris(score,status),
        GameKind::Chess=>board_chess(score,status),
        GameKind::Checkers=>board_checkers(score,status),
        GameKind::Blackjack=>board_blackjack(score,status),
    }}
    </section> }
}

// ── 2048 ──────────────────────────────────────────────────────────────────────

fn spawn_tile(board: &mut [u32; 16]) {
    let empties: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, _)| i)
        .collect();
    if let Some(&idx) = empties.get(rand_usize(empties.len())) {
        board[idx] = if rand_f64() < 0.9 { 2 } else { 4 };
    }
}

fn board_2048(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let mut init = [0u32; 16];
    spawn_tile(&mut init);
    spawn_tile(&mut init);
    let board = RwSignal::new(init);
    let game_over = RwSignal::new(false);

    let slide = move |dir: u8| {
        if game_over.get() {
            return;
        }
        let old = board.get();
        let (new_board, pts) = slide_2048(old, dir);
        if new_board == old {
            return;
        }
        let mut b = new_board;
        spawn_tile(&mut b);
        board.set(b);
        score.update(|s| *s += pts);
        if b.contains(&2048) {
            status.set("🎉 You reached 2048!".into());
            game_over.set(true);
        } else if !has_move_2048(&b) {
            status.set("Game over — no moves left".into());
            game_over.set(true);
        } else {
            status.set(format!("Score: {}", score.get()));
        }
    };

    let reset = move || {
        let mut b = [0u32; 16];
        spawn_tile(&mut b);
        spawn_tile(&mut b);
        board.set(b);
        score.set(0);
        game_over.set(false);
        status.set("Ready".into());
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        let dir = match e.key().as_str() {
            "ArrowUp" | "w" | "W" => Some(0u8),
            "ArrowRight" | "d" | "D" => Some(1),
            "ArrowDown" | "s" | "S" => Some(2),
            "ArrowLeft" | "a" | "A" => Some(3),
            _ => None,
        };
        if let Some(d) = dir {
            e.prevent_default();
            slide(d);
        }
    });

    view! {
        <div class="mx-auto max-w-sm space-y-3">
            {dpad(
                move || slide(0),
                move || slide(3),
                move || slide(2),
                move || slide(1),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Arrows / WASD to slide"</p>
            <div class="grid grid-cols-4 gap-2 rounded-lg border border-[var(--border-color)] p-2">
                {(0..16).map(|i| view! {
                    <div class=move || {
                        let v = board.get()[i];
                        let bg = match v {
                            0 => "bg-[var(--surface-hover)]",
                            2 => "bg-amber-100 dark:bg-amber-900",
                            4 => "bg-amber-200 dark:bg-amber-800",
                            8 => "bg-orange-300 dark:bg-orange-700",
                            16 => "bg-orange-400 dark:bg-orange-600",
                            32 => "bg-red-400 dark:bg-red-600",
                            64 => "bg-red-500 dark:bg-red-500",
                            _ => "bg-yellow-400 dark:bg-yellow-500",
                        };
                        format!("flex aspect-square items-center justify-center rounded text-sm font-bold text-[var(--text-primary)] {bg}")
                    }>
                        {move || { let v = board.get()[i]; if v == 0 { String::new() } else { v.to_string() } }}
                    </div>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Tic-Tac-Toe ───────────────────────────────────────────────────────────────

fn board_ttt(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let board = RwSignal::new([' '; 9]);
    let player_turn = RwSignal::new(true);
    let game_over = RwSignal::new(false);

    let reset = move || {
        board.set([' '; 9]);
        player_turn.set(true);
        game_over.set(false);
        status.set("Your turn (X)".into());
    };

    status.set("Your turn (X)".into());

    let click = move |i: usize| {
        if game_over.get() || !player_turn.get() {
            return;
        }
        let mut b = board.get();
        if b[i] != ' ' {
            return;
        }
        b[i] = 'X';
        board.set(b);

        if let Some(w) = ttt_winner(&b) {
            status.set(format!("{w} wins! 🎉"));
            game_over.set(true);
            if w == 'X' {
                score.update(|s| *s += 10);
            }
            return;
        }
        if ttt_is_draw(&b) {
            status.set("Draw!".into());
            game_over.set(true);
            return;
        }
        player_turn.set(false);
        status.set("AI thinking…".into());

        let b_copy = b;
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            if let Some(ai_idx) = ttt_best_move(&b_copy) {
                let mut b2 = b_copy;
                b2[ai_idx] = 'O';
                board.set(b2);
                if let Some(w) = ttt_winner(&b2) {
                    status.set(format!("{w} wins!"));
                    game_over.set(true);
                } else if ttt_is_draw(&b2) {
                    status.set("Draw!".into());
                    game_over.set(true);
                } else {
                    status.set("Your turn (X)".into());
                    player_turn.set(true);
                }
            }
        });
    };

    view! {
        <div class="mx-auto max-w-xs space-y-3">
            <div class="grid grid-cols-3 gap-2">
                {(0..9).map(|i| view! {
                    <button type="button"
                        class=move || format!("aspect-square rounded-lg border border-[var(--border-color)] text-3xl font-bold hover:bg-[var(--surface-hover)] {}",
                            match board.get()[i] { 'X' => "text-[var(--accent)]", 'O' => "text-red-500", _ => "text-[var(--text-primary)]" })
                        on:click=move |_| click(i)>
                        {move || board.get()[i].to_string()}
                    </button>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Minesweeper ───────────────────────────────────────────────────────────────

fn board_mines(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let mines: RwSignal<[bool; 25]> = RwSignal::new([false; 25]);
    let revealed: RwSignal<[bool; 25]> = RwSignal::new([false; 25]);
    let flagged: RwSignal<[bool; 25]> = RwSignal::new([false; 25]);
    let first_click = RwSignal::new(true);
    let game_over = RwSignal::new(false);

    let reset = move || {
        mines.set([false; 25]);
        revealed.set([false; 25]);
        flagged.set([false; 25]);
        first_click.set(true);
        game_over.set(false);
        score.set(0);
        status.set("Ready — click to reveal".into());
    };

    let reveal = move |i: usize| {
        if game_over.get() || revealed.get()[i] || flagged.get()[i] {
            return;
        }

        let mut m = mines.get();
        if first_click.get() {
            first_click.set(false);
            let mut placed = 0;
            while placed < 5 {
                let idx = rand_usize(25);
                if idx != i && !m[idx] {
                    m[idx] = true;
                    placed += 1;
                }
            }
            mines.set(m);
        }
        let m = mines.get();
        if m[i] {
            let mut r = revealed.get();
            r[i] = true;
            revealed.set(r);
            game_over.set(true);
            status.set("💥 Mine! Game over.".into());
            return;
        }
        let mut r = revealed.get();
        let newly = minesweeper_flood_reveal(&m, &r, i);
        for idx in &newly {
            r[*idx] = true;
        }
        revealed.set(r);
        let safe_count = r.iter().filter(|&&v| v).count();
        score.set(safe_count as u32);
        if safe_count == 20 {
            game_over.set(true);
            status.set("🎉 You cleared the field!".into());
        } else {
            status.set(format!("{} safe cells revealed", safe_count));
        }
    };

    let flag = move |i: usize| {
        if game_over.get() || revealed.get()[i] {
            return;
        }
        flagged.update(|f| f[i] = !f[i]);
    };

    view! {
        <div class="mx-auto max-w-sm space-y-3">
            <div class="grid grid-cols-5 gap-1">
                {(0..25).map(|i| view! {
                    <button type="button"
                        class=move || {
                            let m = mines.get();
                            let r = revealed.get();
                            let f = flagged.get();
                            if r[i] && m[i] { "aspect-square rounded border bg-red-500 text-white text-xs font-bold".into() }
                            else if r[i] {
                                let n = minesweeper_adjacent_mines(&m, i);
                                let color = match n { 1 => "text-blue-500", 2 => "text-green-500", 3 => "text-red-500", _ => "text-[var(--text-primary)]" };
                                format!("aspect-square rounded border border-[var(--border-color)] bg-[var(--surface-hover)] text-xs font-bold {color}")
                            } else if f[i] { "aspect-square rounded border border-[var(--border-color)] bg-yellow-400 text-xs".into() }
                            else { "aspect-square rounded border border-[var(--border-color)] text-xs hover:bg-[var(--surface-hover)]".into() }
                        }
                        on:click=move |_| reveal(i)
                        on:contextmenu=move |e| { e.prevent_default(); flag(i); }>
                        {move || {
                            let m = mines.get();
                            let r = revealed.get();
                            let f = flagged.get();
                            if f[i] && !r[i] { "🚩".to_string() }
                            else if r[i] && m[i] { "💣".to_string() }
                            else if r[i] {
                                let n = minesweeper_adjacent_mines(&m, i);
                                if n == 0 { String::new() } else { n.to_string() }
                            } else { String::new() }
                        }}
                    </button>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Snake ─────────────────────────────────────────────────────────────────────

fn board_snake(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let cols: i32 = 10;
    let rows: i32 = 10;
    let body: RwSignal<Vec<(i32, i32)>> = RwSignal::new(vec![(5, 5), (5, 4), (5, 3)]);
    let dir: RwSignal<(i32, i32)> = RwSignal::new((0, 1));
    let food: RwSignal<(i32, i32)> = RwSignal::new((3, 7));
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);

    let spawn_food = move |b: &[(i32, i32)]| -> (i32, i32) {
        loop {
            let fx = rand_usize(cols as usize) as i32;
            let fy = rand_usize(rows as usize) as i32;
            if !b.contains(&(fx, fy)) {
                return (fx, fy);
            }
        }
    };

    let step = move || {
        if game_over.get() {
            return;
        }
        let mut b = body.get();
        let (dx, dy) = dir.get();
        let head = b[0];
        let Some(new_head) = snake_step(head, (dx, dy), cols, rows) else {
            game_over.set(true);
            running.set(false);
            status.set(format!("Game over! Score: {}", score.get()));
            return;
        };
        if b.contains(&new_head) {
            game_over.set(true);
            running.set(false);
            status.set(format!("Game over! Score: {}", score.get()));
            return;
        }
        let ate = new_head == food.get();
        b.insert(0, new_head);
        if !ate {
            b.pop();
        }
        body.set(b.clone());
        if ate {
            score.update(|s| *s += 1);
            food.set(spawn_food(&b));
            status.set(format!("Length: {}", b.len()));
        }
    };

    let start = move || {
        if running.get() {
            return;
        }
        if game_over.get() {
            body.set(vec![(5, 5), (5, 4), (5, 3)]);
            dir.set((0, 1));
            food.set((3, 7));
            score.set(0);
            game_over.set(false);
        }
        running.set(true);
        status.set("Running…".into());
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(150).await;
                if !running.get() {
                    break;
                }
                step();
            }
        });
    };

    let pause = move || {
        if running.get() && !game_over.get() {
            running.set(false);
            status.set("Paused — Space to resume".into());
        }
    };

    let toggle = move || {
        if running.get() {
            pause();
        } else {
            start();
        }
    };

    let set_dir = move |nd: (i32, i32)| {
        if game_over.get() {
            return;
        }
        let cur = dir.get();
        if nd != (-cur.0, -cur.1) {
            dir.set(nd);
        }
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        match e.key().as_str() {
            " " => {
                e.prevent_default();
                if !e.repeat() {
                    toggle();
                }
            }
            "ArrowUp" | "w" | "W" => {
                e.prevent_default();
                set_dir((0, -1));
            }
            "ArrowDown" | "s" | "S" => {
                e.prevent_default();
                set_dir((0, 1));
            }
            "ArrowLeft" | "a" | "A" => {
                e.prevent_default();
                set_dir((-1, 0));
            }
            "ArrowRight" | "d" | "D" => {
                e.prevent_default();
                set_dir((1, 0));
            }
            _ => {}
        }
    });

    view! {
        <div class="mx-auto max-w-sm space-y-3">
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|toggle()>
                {move || if game_over.get() { "New Game (Space)" } else if running.get() { "⏸ Pause (Space)" } else { "▶ Start (Space)" }}
            </button>
            {dpad(
                move || set_dir((0, -1)),
                move || set_dir((-1, 0)),
                move || set_dir((0, 1)),
                move || set_dir((1, 0)),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · arrows / WASD move"</p>
            <div class="grid gap-0.5" style="grid-template-columns: repeat(10, 1fr)">
                {(0..rows).flat_map(|y| (0..cols).map(move |x| view! {
                    <div class=move || {
                        let b = body.get();
                        let f = food.get();
                        if b.first() == Some(&(x, y)) { "aspect-square rounded bg-[var(--accent)]" }
                        else if b.contains(&(x, y)) { "aspect-square rounded bg-[var(--accent)] opacity-60" }
                        else if f == (x, y) { "aspect-square rounded bg-red-500" }
                        else { "aspect-square rounded border border-[var(--border-color)]" }
                    }></div>
                })).collect_view()}
            </div>
        </div>
    }.into_any()
}

// ── Sudoku ────────────────────────────────────────────────────────────────────

fn board_sudoku(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let puzzle = sudoku_puzzle();
    let given = sudoku_given(&puzzle);
    let board = RwSignal::new(puzzle);
    let conflicts: RwSignal<[bool; 81]> = RwSignal::new([false; 81]);

    let check_complete = move |b: &[u8; 81]| -> bool {
        b.iter().all(|&v| v != 0) && (0..81).all(|i| sudoku_valid(b, i, b[i]))
    };

    let click = move |i: usize| {
        if given[i] {
            return;
        }
        let mut b = board.get();
        b[i] = (b[i] % 9) + 1;
        let mut cf = [false; 81];
        for idx in 0..81 {
            if b[idx] != 0 && !sudoku_valid(&b, idx, b[idx]) {
                cf[idx] = true;
            }
        }
        conflicts.set(cf);
        board.set(b);
        if check_complete(&b) {
            score.update(|s| *s += 50);
            status.set("🎉 Puzzle solved!".into());
        } else {
            let errors = cf.iter().filter(|&&v| v).count();
            if errors > 0 {
                status.set(format!("{errors} conflict(s)"));
            } else {
                status.set("No conflicts".into());
            }
        }
    };

    view! {
        <div class="mx-auto max-w-md space-y-3">
            <div class="grid grid-cols-9 gap-0.5">
                {(0..81).map(|i| view! {
                    <button type="button"
                        class=move || {
                            let is_given = given[i];
                            let conflict = conflicts.get()[i];
                            let border = if i % 9 == 0 || i % 9 == 3 || i % 9 == 6 { "border-l-2" } else { "border-l" };
                            let border_top = if i / 9 == 0 || i / 9 == 3 || i / 9 == 6 { "border-t-2" } else { "border-t" };
                            format!("aspect-square text-xs font-bold border-[var(--border-color)] {border} {border_top} {}",
                                if conflict { "text-red-500 bg-red-50 dark:bg-red-900/20" }
                                else if is_given { "text-[var(--text-primary)] bg-[var(--surface-hover)]" }
                                else { "text-[var(--accent)] hover:bg-[var(--surface-hover)]" })
                        }
                        on:click=move |_| click(i)>
                        {move || { let v = board.get()[i]; if v == 0 { String::new() } else { v.to_string() } }}
                    </button>
                }).collect_view()}
            </div>
        </div>
    }.into_any()
}

// ── Connect Four ──────────────────────────────────────────────────────────────

fn board_connect_four(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let board: RwSignal<[u8; 42]> = RwSignal::new([0u8; 42]);
    let game_over = RwSignal::new(false);
    let player_turn = RwSignal::new(true);

    let reset = move || {
        board.set([0u8; 42]);
        game_over.set(false);
        player_turn.set(true);
        status.set("Your turn (🔴)".into());
    };
    status.set("Your turn (🔴)".into());

    let drop_col = move |col: usize| {
        if game_over.get() || !player_turn.get() {
            return;
        }
        let mut b = board.get();
        if connect_four_drop(&mut b, col, 1).is_none() {
            return;
        }
        board.set(b);
        if connect_four_winner(&b) == Some(1) {
            score.update(|s| *s += 10);
            status.set("🔴 You win!".into());
            game_over.set(true);
            return;
        }
        if b.iter().all(|&v| v != 0) {
            status.set("Draw!".into());
            game_over.set(true);
            return;
        }
        player_turn.set(false);
        status.set("AI thinking…".into());
        let b_copy = b;
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(400).await;
            let mut b2 = b_copy;
            if let Some(ai_col) = connect_four_ai_column(&b2) {
                connect_four_drop(&mut b2, ai_col, 2);
                board.set(b2);
                if connect_four_winner(&b2) == Some(2) {
                    status.set("🟡 AI wins!".into());
                    game_over.set(true);
                } else if b2.iter().all(|&v| v != 0) {
                    status.set("Draw!".into());
                    game_over.set(true);
                } else {
                    status.set("Your turn (🔴)".into());
                    player_turn.set(true);
                }
            }
        });
    };

    view! {
        <div class="mx-auto max-w-md space-y-2">
            <div class="grid grid-cols-7 gap-1">
                {(0..7).map(|col| view! {
                    <button type="button" class="rounded-md border border-[var(--border-color)] py-1 text-xs hover:bg-[var(--surface-hover)]" on:click=move|_|drop_col(col)>"▼"</button>
                }).collect_view()}
            </div>
            <div class="grid grid-cols-7 gap-1 rounded-lg border border-[var(--border-color)] bg-blue-600 p-2">
                {(0..42).map(|i| view! {
                    <div class=move || {
                        let v = board.get()[i];
                        match v {
                            1 => "aspect-square rounded-full bg-red-500 border-2 border-red-700",
                            2 => "aspect-square rounded-full bg-yellow-400 border-2 border-yellow-600",
                            _ => "aspect-square rounded-full bg-blue-900/50 border border-blue-800",
                        }
                    }></div>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Memory Cards ──────────────────────────────────────────────────────────────

fn board_memory(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let emojis = ["🍎", "🍊", "🍋", "🍇", "🍓", "🍒", "🍑", "🥝"];
    let mut deck: Vec<usize> = (0..8).chain(0..8).collect();
    for i in (1..deck.len()).rev() {
        let j = rand_usize(i + 1);
        deck.swap(i, j);
    }
    let cards = RwSignal::new(deck);
    let revealed: RwSignal<Vec<bool>> = RwSignal::new(vec![false; 16]);
    let matched: RwSignal<Vec<bool>> = RwSignal::new(vec![false; 16]);
    let first: RwSignal<Option<usize>> = RwSignal::new(None);
    let locked = RwSignal::new(false);

    let click = move |i: usize| {
        if locked.get() || revealed.get()[i] || matched.get()[i] {
            return;
        }
        let mut r = revealed.get();
        r[i] = true;
        revealed.set(r.clone());

        match first.get() {
            None => first.set(Some(i)),
            Some(j) => {
                first.set(None);
                let c = cards.get();
                if c[i] == c[j] {
                    let mut m = matched.get();
                    m[i] = true;
                    m[j] = true;
                    matched.set(m.clone());
                    score.update(|s| *s += 2);
                    if m.iter().all(|&v| v) {
                        status.set("🎉 All pairs found!".into());
                    } else {
                        status.set(format!(
                            "Match! {} pairs left",
                            (16 - m.iter().filter(|&&v| v).count()) / 2
                        ));
                    }
                } else {
                    locked.set(true);
                    status.set("No match — hiding…".into());
                    leptos::task::spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(900).await;
                        let mut r2 = revealed.get();
                        r2[i] = false;
                        r2[j] = false;
                        revealed.set(r2);
                        locked.set(false);
                        status.set("Keep going!".into());
                    });
                }
            }
        }
    };

    view! {
        <div class="mx-auto grid max-w-sm grid-cols-4 gap-2">
            {(0..16).map(|i| view! {
                <button type="button"
                    class=move || {
                        if matched.get()[i] { "aspect-square rounded-lg border-2 border-green-500 bg-green-100 dark:bg-green-900/30 text-2xl" }
                        else if revealed.get()[i] { "aspect-square rounded-lg border border-[var(--accent)] bg-[var(--surface-hover)] text-2xl" }
                        else { "aspect-square rounded-lg border border-[var(--border-color)] hover:bg-[var(--surface-hover)] text-2xl" }
                    }
                    on:click=move |_| click(i)>
                    {move || {
                        let c = cards.get();
                        if revealed.get()[i] || matched.get()[i] { emojis[c[i]].to_string() }
                        else { "?".to_string() }
                    }}
                </button>
            }).collect_view()}
        </div>
    }.into_any()
}

// ── Typing Speed ──────────────────────────────────────────────────────────────

fn board_typing(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let words = typing_words();
    let idx = RwSignal::new(0usize);
    let input = RwSignal::new(String::new());
    let correct = RwSignal::new(0u32);
    let wrong = RwSignal::new(0u32);
    let started_ms = RwSignal::new(None::<f64>);
    let done = RwSignal::new(false);

    let submit = move || {
        if done.get() {
            return;
        }
        let typed = input.get().trim().to_lowercase();
        let target = words[idx.get()];
        if typed == target {
            correct.update(|c| *c += 1);
            score.update(|s| *s += 10);
        } else {
            wrong.update(|w| *w += 1);
        }
        let next = idx.get() + 1;
        if next >= words.len() {
            done.set(true);
            let elapsed_min = started_ms
                .get()
                .map(|t| ((js_sys::Date::now() - t) / 60_000.0).max(0.001))
                .unwrap_or(0.001);
            let wpm = (correct.get() as f64 / elapsed_min).round() as u32;
            status.set(format!(
                "Done — {wpm} WPM ({} correct, {} wrong)",
                correct.get(),
                wrong.get()
            ));
        } else {
            idx.set(next);
            status.set(format!("{}/{}  ·  type + Enter", next + 1, words.len()));
        }
        input.set(String::new());
    };

    view! {
        <div class="mx-auto max-w-md space-y-4 text-center">
            <p class="text-3xl font-bold tracking-widest text-[var(--accent)]">{move || words[idx.get()]}</p>
            <input
                type="text"
                class="w-full rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-3 text-center text-lg text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                placeholder="Type the word and press Enter"
                prop:value=move || input.get()
                on:input=move |e| {
                    if started_ms.get().is_none() {
                        started_ms.set(Some(js_sys::Date::now()));
                        status.set("Timer started".into());
                    }
                    input.set(event_target_value(&e));
                }
                on:keydown=move |e| { if e.key() == "Enter" { submit(); } }
            />
            <div class="flex justify-center gap-6 text-sm">
                <span class="text-green-500">{move || format!("✅ {}", correct.get())}</span>
                <span class="text-red-500">{move || format!("❌ {}", wrong.get())}</span>
            </div>
        </div>
    }.into_any()
}

// ── Wordle ────────────────────────────────────────────────────────────────────

fn board_wordle(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let answer = wordle_word();
    let guesses: RwSignal<Vec<String>> = RwSignal::new(vec![String::new(); 6]);
    let results: RwSignal<Vec<Option<[u8; 5]>>> = RwSignal::new(vec![None; 6]);
    let current_row = RwSignal::new(0usize);
    let input = RwSignal::new(String::new());
    let won = RwSignal::new(false);

    let submit = move || {
        let row = current_row.get();
        if row >= 6 || won.get() {
            return;
        }
        let guess = input.get().to_lowercase();
        if guess.len() != 5 {
            status.set("Type a 5-letter word".into());
            return;
        }

        let result = wordle_check(&guess, answer);
        let mut gs = guesses.get();
        gs[row] = guess.clone();
        guesses.set(gs);
        let mut rs = results.get();
        rs[row] = Some(result);
        results.set(rs);
        input.set(String::new());

        if result == [2; 5] {
            won.set(true);
            score.update(|s| *s += (6 - row as u32) * 10);
            status.set(format!("🎉 Correct in {} tries!", row + 1));
        } else if row == 5 {
            status.set(format!("Game over — word was '{answer}'"));
        } else {
            current_row.set(row + 1);
            status.set(format!("Try {}/{}", row + 2, 6));
        }
    };

    view! {
        <div class="mx-auto max-w-sm space-y-3">
            <div class="space-y-1">
                {(0..6).map(|row| view! {
                    <div class="grid grid-cols-5 gap-1">
                        {(0..5).map(|col| view! {
                            <div class=move || {
                                let rs = results.get();
                                let gs = guesses.get();
                                if let Some(r) = rs[row] {
                                    let color = match r[col] {
                                        2 => "bg-green-500 text-white border-green-500",
                                        1 => "bg-yellow-400 text-white border-yellow-400",
                                        _ => "bg-[var(--surface-hover)] border-[var(--border-color)]",
                                    };
                                    format!("flex aspect-square items-center justify-center rounded border text-sm font-bold uppercase {color}")
                                } else {
                                    let ch: char = gs[row].chars().nth(col).unwrap_or(' ');
                                    let filled = if ch != ' ' && row == current_row.get() { "border-[var(--accent)]" } else { "border-[var(--border-color)]" };
                                    format!("flex aspect-square items-center justify-center rounded border text-sm font-bold uppercase {filled}")
                                }
                            }>
                                {move || {
                                    let gs = guesses.get();
                                    gs[row].chars().nth(col).filter(|&c| c != ' ').map(|c| c.to_uppercase().to_string()).unwrap_or_default()
                                }}
                            </div>
                        }).collect_view()}
                    </div>
                }).collect_view()}
            </div>
            <div class="flex gap-2">
                <input
                    type="text"
                    maxlength="5"
                    class="flex-1 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-center font-mono text-sm uppercase text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                    placeholder="GUESS"
                    prop:value=move || input.get()
                    on:input=move |e| input.set(event_target_value(&e))
                    on:keydown=move |e| { if e.key() == "Enter" { submit(); } }
                />
                <button type="button" class="rounded-lg border border-[var(--border-color)] px-4 py-2 text-sm hover:bg-[var(--surface-hover)]" on:click=move|_|submit()>"Enter"</button>
            </div>
        </div>
    }.into_any()
}

// ── Hangman ───────────────────────────────────────────────────────────────────

fn board_hangman(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let word = hangman_word();
    let guessed: RwSignal<Vec<char>> = RwSignal::new(vec![]);
    let max_wrong = 6usize;

    let wrong_count = move || {
        let g = guessed.get();
        g.iter().filter(|&&c| !word.contains(c)).count()
    };

    let is_won = move || word.chars().all(|c| guessed.get().contains(&c));
    let is_lost = move || wrong_count() >= max_wrong;

    let guess = move |c: char| {
        if is_won() || is_lost() {
            return;
        }
        let mut g = guessed.get();
        if !g.contains(&c) {
            g.push(c);
            guessed.set(g.clone());
            let wc = g.iter().filter(|&&ch| !word.contains(ch)).count();
            if word.chars().all(|ch| g.contains(&ch)) {
                score.update(|s| *s += 20);
                status.set("🎉 You guessed it!".into());
            } else if wc >= max_wrong {
                status.set(format!("💀 Game over — word was '{word}'"));
            } else {
                status.set(format!("{} wrong ({}/{})", wc, wc, max_wrong));
            }
        }
    };

    let hangman_art = move || {
        let parts = ["😰", "🤕", "😵", "💀", "☠️", "☠️"];
        let wc = wrong_count().min(5);
        let scaffold = "╔═══╗\n║    |\n║";
        let body_parts = [
            "    |",
            " 😀 |",
            " 😀 |\n║  | |",
            " 😀 |\n║ /| |",
            " 😀 |\n║ /|\\|",
            " 😀 |\n║ /|\\|\n║ /  |",
            " 😀 |\n║ /|\\|\n║ / \\|",
        ];
        let _ = (parts, wc, scaffold);
        body_parts[wc.min(6)]
    };

    view! {
        <div class="mx-auto max-w-sm space-y-4">
            <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-4 text-center font-mono text-sm whitespace-pre">{move || hangman_art()}</div>
            <div class="flex justify-center gap-2 text-2xl font-bold tracking-[0.3em] text-[var(--text-primary)]">
                {move || word.chars().map(|c| {
                    let shown = guessed.get().contains(&c) || is_lost();
                    if shown { c.to_string() } else { "_".to_string() }
                }).collect::<Vec<_>>().join(" ")}
            </div>
            <div class="grid grid-cols-7 gap-1">
                {('a'..='z').map(|c| view! {
                    <button type="button"
                        class=move || {
                            let g = guessed.get();
                            let used = g.contains(&c);
                            let correct = word.contains(c);
                            if !used { String::from("rounded border border-[var(--border-color)] py-1 text-xs hover:bg-[var(--surface-hover)]") }
                            else if correct { String::from("rounded border border-green-500 bg-green-100 dark:bg-green-900/30 py-1 text-xs text-green-700 dark:text-green-300") }
                            else { String::from("rounded border border-red-400 bg-red-100 dark:bg-red-900/30 py-1 text-xs text-red-600 line-through opacity-50") }
                        }
                        on:click=move |_| guess(c)>
                        {c.to_string()}
                    </button>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|{
                guessed.set(vec![]);
                status.set("Guess a letter".into());
            }>"New Game"</button>
        </div>
    }.into_any()
}

// ── 15 Puzzle ─────────────────────────────────────────────────────────────────

fn board_puzzle(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let make_board = || {
        let mut b: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        for _ in 0..500 {
            let empty = b.iter().position(|&v| v == 0).unwrap();
            let er = empty / 4;
            let ec = empty % 4;
            let mut neighbors = vec![];
            if er > 0 {
                neighbors.push(empty - 4);
            }
            if er < 3 {
                neighbors.push(empty + 4);
            }
            if ec > 0 {
                neighbors.push(empty - 1);
            }
            if ec < 3 {
                neighbors.push(empty + 1);
            }
            let n = neighbors[rand_usize(neighbors.len())];
            b.swap(empty, n);
        }
        b
    };

    let board: RwSignal<[u8; 16]> = RwSignal::new(make_board());
    let moves = RwSignal::new(0u32);

    let click = move |i: usize| {
        let b = board.get();
        if let Some(new_b) = puzzle_move(&b, i) {
            board.set(new_b);
            moves.update(|m| *m += 1);
            if puzzle_is_solved(&new_b) {
                score.update(|s| *s += 100u32.saturating_sub(moves.get()));
                status.set("🎉 Solved!".into());
            } else {
                status.set(format!("Moves: {}", moves.get()));
            }
        }
    };

    view! {
        <div class="mx-auto max-w-sm space-y-3">
            <div class="grid grid-cols-4 gap-2">
                {(0..16).map(|i| view! {
                    <button type="button"
                        class=move || {
                            let v = board.get()[i];
                            if v == 0 { String::from("aspect-square rounded-lg border border-[var(--border-color)] opacity-0") }
                            else { String::from("aspect-square rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] text-lg font-bold text-[var(--text-primary)] hover:bg-[var(--accent)] hover:text-white transition") }
                        }
                        on:click=move |_| click(i)>
                        {move || { let v = board.get()[i]; if v == 0 { String::new() } else { v.to_string() } }}
                    </button>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|{ board.set(make_board()); moves.set(0); status.set("Ready".into()); }>"Shuffle"</button>
        </div>
    }.into_any()
}

// ── Lights Out ────────────────────────────────────────────────────────────────

fn board_lights(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let make_board = || {
        let mut b = [false; 25];
        for cell in &mut b {
            if rand_f64() < 0.5 {
                *cell = true;
            }
        }
        b
    };
    let lights: RwSignal<[bool; 25]> = RwSignal::new(make_board());
    let taps = RwSignal::new(0u32);

    let toggle = move |i: usize| {
        let mut b = lights.get();
        lights_toggle(&mut b, i);
        lights.set(b);
        taps.update(|t| *t += 1);
        if b.iter().all(|&v| !v) {
            score.update(|s| *s += 100u32.saturating_sub(taps.get()));
            status.set("🎉 All lights off!".into());
        } else {
            let on = b.iter().filter(|&&v| v).count();
            status.set(format!("{on} lights on"));
        }
    };

    view! {
        <div class="mx-auto max-w-xs space-y-3">
            <div class="grid grid-cols-5 gap-2">
                {(0..25).map(|i| view! {
                    <button type="button"
                        class=move || {
                            if lights.get()[i] { "aspect-square rounded-lg border-2 border-yellow-400 bg-yellow-300 shadow-md shadow-yellow-300/50 transition dark:bg-yellow-500" }
                            else { "aspect-square rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] transition hover:bg-[var(--surface)]" }
                        }
                        on:click=move |_| toggle(i)>
                    </button>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|{ lights.set(make_board()); taps.set(0); status.set("Ready".into()); }>"New Puzzle"</button>
        </div>
    }.into_any()
}

// ── Checkers ──────────────────────────────────────────────────────────────────

fn board_checkers(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let mut init_board = [0u8; 32];
    init_board[..12].fill(2);
    init_board[20..32].fill(1);
    let board: RwSignal<[u8; 32]> = RwSignal::new(init_board);
    let selected: RwSignal<Option<usize>> = RwSignal::new(None);
    let game_over = RwSignal::new(false);

    let legal_moves = move |b: &[u8; 32], piece_idx: usize| -> Vec<usize> {
        checkers_moves(b, b[piece_idx])
            .iter()
            .filter(|(from, _)| *from == piece_idx)
            .map(|(_, to)| *to)
            .collect()
    };

    let click_piece = move |logical: usize| {
        if game_over.get() {
            return;
        }
        let b = board.get();
        if b[logical] == 1 {
            selected.set(Some(logical));
            let moves = legal_moves(&b, logical);
            status.set(if moves.is_empty() {
                "No legal moves".into()
            } else {
                format!("{} moves available", moves.len())
            });
            return;
        }
        if let Some(from) = selected.get() {
            let moves = legal_moves(&b, from);
            if moves.contains(&logical) {
                let mut b2 = b;
                b2[logical] = b2[from];
                b2[from] = 0;
                board.set(b2);
                selected.set(None);
                score.update(|s| *s += 1);

                let ai_moves = checkers_moves(&b2, 2);
                if ai_moves.is_empty() {
                    game_over.set(true);
                    status.set("🎉 You win! AI has no moves.".into());
                    return;
                }
                let ai_move = ai_moves[0];
                let mut b3 = b2;
                b3[ai_move.1] = b3[ai_move.0];
                b3[ai_move.0] = 0;
                board.set(b3);

                let player_moves = checkers_moves(&b3, 1);
                if player_moves.is_empty() {
                    game_over.set(true);
                    status.set("AI wins! No moves left.".into());
                } else {
                    status.set("Your turn (light pieces)".into());
                }
                return;
            }
        }
        selected.set(None);
        status.set("Select your piece first".into());
    };

    view! {
        <div class="mx-auto max-w-md space-y-3">
            <div class="grid grid-cols-8 gap-0 rounded-lg overflow-hidden border border-[var(--border-color)]">
                {(0..8).flat_map(|row| (0..8).map(move |col| {
                    let is_dark = (row + col) % 2 == 1;
                    let logical = if is_dark { row / 2 * 4 + if row % 2 == 1 { col / 2 } else { (col - 1) / 2 } } else { usize::MAX };

                    view! {
                        <button type="button"
                            disabled=!is_dark
                            class=move || {
                                let bg = if !is_dark { "bg-amber-100 dark:bg-amber-200" } else { "bg-amber-800 hover:opacity-90" };
                                let highlight = if is_dark && logical != usize::MAX && selected.get().is_some_and(|s| legal_moves(&board.get(), s).contains(&logical)) {
                                    " ring-2 ring-green-400"
                                } else if is_dark && Some(logical) == selected.get() {
                                    " ring-2 ring-[var(--accent)]"
                                } else { "" };
                                format!("aspect-square text-xl flex items-center justify-center {bg}{highlight}")
                            }
                            on:click=move |_| { if is_dark && logical != usize::MAX { click_piece(logical); } }>
                            {move || {
                                if !is_dark || logical == usize::MAX { return String::new(); }
                                match board.get()[logical] {
                                    1 => "⚪".to_string(),
                                    2 => "⚫".to_string(),
                                    _ => String::new(),
                                }
                            }}
                        </button>
                    }
                })).collect_view()}
            </div>
        </div>
    }.into_any()
}

// ── Blackjack ─────────────────────────────────────────────────────────────────

fn card_name(c: u8) -> &'static str {
    match c {
        1 => "A",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => "?",
    }
}

fn draw_card(deck: &mut Vec<u8>) -> u8 {
    if deck.is_empty() {
        *deck = shuffle_deck(rand_usize);
    }
    deck.pop().unwrap_or(10)
}

fn board_blackjack(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let make_deck = || shuffle_deck(rand_usize);
    let deck: RwSignal<Vec<u8>> = RwSignal::new(make_deck());
    let player: RwSignal<Vec<u8>> = RwSignal::new(vec![]);
    let dealer: RwSignal<Vec<u8>> = RwSignal::new(vec![]);
    let dealer_hidden = RwSignal::new(true);
    let game_over = RwSignal::new(false);

    let deal = move || {
        let mut d = deck.get();
        let p = vec![draw_card(&mut d), draw_card(&mut d)];
        let de = vec![draw_card(&mut d), draw_card(&mut d)];
        deck.set(d);
        player.set(p.clone());
        dealer.set(de);
        dealer_hidden.set(true);
        game_over.set(false);
        let ps = blackjack_score(&p);
        if ps == 21 {
            status.set("Blackjack! 🎉".into());
            game_over.set(true);
            dealer_hidden.set(false);
            score.update(|s| *s += 15);
        } else {
            status.set(format!("Your score: {ps}"));
        }
    };

    let hit = move || {
        if game_over.get() {
            return;
        }
        let mut d = deck.get();
        let mut p = player.get();
        p.push(draw_card(&mut d));
        deck.set(d);
        let ps = blackjack_score(&p);
        player.set(p);
        if ps > 21 {
            status.set(format!("Bust! ({ps}) — Dealer wins."));
            game_over.set(true);
            dealer_hidden.set(false);
        } else {
            status.set(format!("Your score: {ps}"));
        }
    };

    let stand = move || {
        if game_over.get() {
            return;
        }
        dealer_hidden.set(false);
        let mut d = deck.get();
        let mut de = dealer.get();
        while blackjack_should_hit(&de) {
            de.push(draw_card(&mut d));
        }
        deck.set(d);
        let ps = blackjack_score(&player.get());
        let ds = blackjack_score(&de);
        dealer.set(de);
        game_over.set(true);
        if ds > 21 || ps > ds {
            status.set(format!("You win! {ps} vs {ds} 🎉"));
            score.update(|s| *s += 10);
        } else if ps == ds {
            status.set(format!("Push! Both {ps}"));
        } else {
            status.set(format!("Dealer wins. {ds} vs {ps}"));
        }
    };

    view! {
        <div class="mx-auto max-w-sm space-y-4">
            <div class="space-y-2">
                <p class="text-sm font-semibold text-[var(--text-secondary)]">"Dealer"</p>
                <div class="flex gap-2">
                    {move || dealer.get().iter().enumerate().map(|(i, &c)| {
                        let hidden = dealer_hidden.get() && i == 1;
                        view! {
                            <div class="flex h-16 w-12 items-center justify-center rounded-lg border-2 border-[var(--border-color)] bg-[var(--surface-hover)] text-lg font-bold text-[var(--text-primary)]">
                                {if hidden { "🂠".to_string() } else { card_name(c).to_string() }}
                            </div>
                        }
                    }).collect_view()}
                </div>
                <p class="text-sm font-semibold text-[var(--text-secondary)]">"You"</p>
                <div class="flex gap-2">
                    {move || player.get().iter().map(|&c| view! {
                        <div class="flex h-16 w-12 items-center justify-center rounded-lg border-2 border-[var(--accent)] bg-[var(--surface-hover)] text-lg font-bold text-[var(--accent)]">
                            {card_name(c)}
                        </div>
                    }).collect_view()}
                </div>
                <p class="text-sm font-bold text-[var(--text-primary)]">{move || {
                    let p = player.get();
                    if p.is_empty() { String::new() } else { format!("Score: {}", blackjack_score(&p)) }
                }}</p>
            </div>
            <div class="flex gap-2">
                <button type="button" class="flex-1 rounded-md border border-[var(--border-color)] py-2 text-sm font-semibold hover:bg-[var(--surface-hover)]" on:click=move|_|deal()>"Deal"</button>
                <button type="button" class="flex-1 rounded-md border border-green-500 bg-green-500/10 py-2 text-sm font-semibold text-green-600 hover:bg-green-500/20 disabled:opacity-40" disabled=move||game_over.get()||player.get().is_empty() on:click=move|_|hit()>"Hit"</button>
                <button type="button" class="flex-1 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 py-2 text-sm font-semibold text-[var(--accent)] hover:bg-[var(--accent)]/20 disabled:opacity-40" disabled=move||game_over.get()||player.get().is_empty() on:click=move|_|stand()>"Stand"</button>
            </div>
        </div>
    }.into_any()
}

// ── Tower Defense (simplified wave game) ─────────────────────────────────────

fn board_tower(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let hp = RwSignal::new(20i32);
    let wave = RwSignal::new(1u32);
    let towers = RwSignal::new(3u32);
    let countdown = RwSignal::new(tower_wave_countdown(1));
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);

    let resolve_wave = move || {
        let t = towers.get();
        let w = wave.get();
        let damage = tower_wave_damage(w, t);
        hp.update(|h| *h = (*h - damage).max(0));
        score.update(|s| *s += t * 5);
        if hp.get() <= 0 {
            game_over.set(true);
            running.set(false);
            status.set(format!("💀 Base destroyed at wave {w}!"));
        } else {
            wave.set(w + 1);
            countdown.set(tower_wave_countdown(w + 1));
            status.set(format!(
                "Wave {w} survived ({damage} dmg). Next in {}s",
                countdown.get()
            ));
        }
    };

    let start = move || {
        if running.get() {
            return;
        }
        if game_over.get() {
            hp.set(20);
            wave.set(1);
            towers.set(3);
            countdown.set(tower_wave_countdown(1));
            score.set(0);
            game_over.set(false);
        }
        running.set(true);
        status.set(format!(
            "Wave {} incoming in {}s — build towers!",
            wave.get(),
            countdown.get()
        ));
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(1000).await;
                if !running.get() || game_over.get() {
                    break;
                }
                let left = countdown.get().saturating_sub(1);
                countdown.set(left);
                if left == 0 {
                    resolve_wave();
                } else {
                    status.set(format!("Wave {} in {}s", wave.get(), left));
                }
            }
        });
    };

    let build = move || {
        if game_over.get() {
            return;
        }
        towers.update(|t| *t += 1);
        status.set(format!("Tower built! ({} towers)", towers.get()));
    };

    view! {
        <div class="mx-auto max-w-sm space-y-4">
            <div class="grid grid-cols-4 gap-2 rounded-lg border border-[var(--border-color)] p-4 text-center">
                <div><p class="text-xs text-[var(--text-secondary)]">"HP"</p><p class="text-2xl font-bold text-red-500">{move || hp.get()}</p></div>
                <div><p class="text-xs text-[var(--text-secondary)]">"Wave"</p><p class="text-2xl font-bold text-[var(--accent)]">{move || wave.get()}</p></div>
                <div><p class="text-xs text-[var(--text-secondary)]">"Towers"</p><p class="text-2xl font-bold text-green-500">{move || towers.get()}</p></div>
                <div><p class="text-xs text-[var(--text-secondary)]">"ETA"</p><p class="text-2xl font-bold text-[var(--text-primary)]">{move || format!("{}s", countdown.get())}</p></div>
            </div>
            <div class="h-4 w-full overflow-hidden rounded-full border border-[var(--border-color)] bg-[var(--surface-hover)]">
                <div class="h-full bg-red-500 transition-all" style=move || format!("width: {}%", (hp.get().max(0) as f32 / 20.0 * 100.0) as u32)></div>
            </div>
            <div class="flex gap-2">
                <button type="button" class="flex-1 rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|start()>
                    {move || if running.get() { "Waves incoming…" } else if game_over.get() { "New Game" } else { "▶ Start waves" }}
                </button>
                <button type="button" class="flex-1 rounded-md border border-green-500 bg-green-500/10 py-2 text-sm font-semibold text-green-600 hover:bg-green-500/20 disabled:opacity-40" disabled=move||game_over.get() on:click=move|_|build()>"🏗️ Build Tower"</button>
            </div>
        </div>
    }.into_any()
}

// ── Breakout ──────────────────────────────────────────────────────────────────

fn board_breakout(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let blocks: RwSignal<[bool; 30]> = RwSignal::new([true; 30]);
    let ball_x = RwSignal::new(5i32);
    let ball_y = RwSignal::new(8i32);
    let ball_dx = RwSignal::new(1i32);
    let ball_dy = RwSignal::new(-1i32);
    let paddle = RwSignal::new(4i32);
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);

    let cols = 10i32;
    let rows = 12i32;

    let step = move || {
        if game_over.get() {
            return;
        }
        let mut bx = ball_x.get() + ball_dx.get();
        let mut by = ball_y.get() + ball_dy.get();
        let mut dx = ball_dx.get();
        let mut dy = ball_dy.get();

        if bx < 0 || bx >= cols {
            dx = -dx;
            bx = ball_x.get();
        }
        if by < 0 {
            dy = -dy;
            by = 0;
        }

        if by >= rows - 1 {
            let p = paddle.get();
            if bx >= p && bx <= p + 2 {
                dy = -dy;
                by = rows - 2;
            } else {
                running.set(false);
                game_over.set(true);
                status.set("💥 Ball lost!".into());
                return;
            }
        }

        if by < 5 {
            let block_idx = (by * cols + bx) as usize;
            let mut b = blocks.get();
            if block_idx < 30 && b[block_idx] {
                b[block_idx] = false;
                blocks.set(b);
                dy = -dy;
                score.update(|s| *s += 1);
                if b.iter().all(|&v| !v) {
                    running.set(false);
                    game_over.set(true);
                    status.set("🎉 All blocks cleared!".into());
                    return;
                }
            }
        }

        ball_x.set(bx);
        ball_y.set(by);
        ball_dx.set(dx);
        ball_dy.set(dy);
    };

    let start = move || {
        if running.get() {
            return;
        }
        if game_over.get() {
            blocks.set([true; 30]);
            ball_x.set(5);
            ball_y.set(8);
            ball_dx.set(1);
            ball_dy.set(-1);
            paddle.set(4);
            score.set(0);
            game_over.set(false);
        }
        running.set(true);
        status.set("Ball in play".into());
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(100).await;
                if !running.get() {
                    break;
                }
                step();
            }
        });
    };

    let pause = move || {
        if running.get() && !game_over.get() {
            running.set(false);
            status.set("Paused — Space to resume".into());
        }
    };

    let toggle = move || {
        if running.get() {
            pause();
        } else {
            start();
        }
    };

    let nudge = move |delta: i32| {
        paddle.update(|p| *p = (*p + delta).clamp(0, cols - 3));
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        match e.key().as_str() {
            " " => {
                e.prevent_default();
                if !e.repeat() {
                    toggle();
                }
            }
            "ArrowLeft" | "a" | "A" => {
                e.prevent_default();
                nudge(-1);
            }
            "ArrowRight" | "d" | "D" => {
                e.prevent_default();
                nudge(1);
            }
            _ => {}
        }
    });

    view! {
        <div class="mx-auto max-w-sm space-y-2">
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|toggle()>
                {move || if game_over.get() { "New Game (Space)" } else if running.get() { "⏸ Pause (Space)" } else { "▶ Start (Space)" }}
            </button>
            {dpad(
                move || {},
                move || nudge(-1),
                move || {},
                move || nudge(1),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · ← → / A D move paddle"</p>
            <div class="relative rounded-lg border border-[var(--border-color)] overflow-hidden bg-[var(--surface-hover)]" style="aspect-ratio: 10/12">
                <div class="grid gap-0.5 p-1" style="grid-template-columns: repeat(10, 1fr)">
                    {(0..30).map(|i| view! {
                        <div class=move || if blocks.get()[i] { "aspect-square rounded bg-[var(--accent)] opacity-80" } else { "aspect-square" }></div>
                    }).collect_view()}
                </div>
                <div class="relative" style="height: calc(100% - 30px)">
                    <div class="absolute w-2 h-2 rounded-full bg-white" style=move || format!("left: calc({}% - 4px); top: calc({}% - 4px)", ball_x.get() * 10, (ball_y.get() - 5) * 100 / 7)></div>
                    <div class="absolute h-2 rounded bg-[var(--accent)]" style=move || format!("left: calc({}% ); bottom: 4px; width: 30%", paddle.get() * 10)></div>
                </div>
            </div>
        </div>
    }.into_any()
}

// ── Pong ──────────────────────────────────────────────────────────────────────

fn board_pong(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let paddle_y = RwSignal::new(5i32);
    let ai_y = RwSignal::new(5i32);
    let ball = RwSignal::new((10i32, 5i32));
    let ball_d = RwSignal::new((1i32, 1i32));
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);
    let max_y = 9i32;

    let step = move || {
        if game_over.get() {
            return;
        }
        let (bx, by) = ball.get();
        let (dx, mut dy) = ball_d.get();
        let mut new_bx = bx + dx;
        let mut new_by = by + dy;
        if new_by < 0 || new_by > max_y {
            dy = -dy;
            new_by = by;
        }

        let ai = crate::domain::games::pong_ai_y(ai_y.get(), new_by, max_y);
        ai_y.set(ai);

        if new_bx <= 1 {
            let p = paddle_y.get();
            if new_by >= p - 1 && new_by <= p + 1 {
                new_bx = 1;
                ball_d.set((-dx, dy));
            } else {
                running.set(false);
                game_over.set(true);
                status.set("AI wins!".into());
                return;
            }
        }
        if new_bx >= 18 {
            let a = ai_y.get();
            if new_by >= a - 1 && new_by <= a + 1 {
                new_bx = 18;
                ball_d.set((-dx, dy));
            } else {
                score.update(|s| *s += 1);
                new_bx = 10;
                status.set(format!("You scored! {}", score.get()));
            }
        }

        ball.set((new_bx, new_by));
        ball_d.set((
            -ball_d.get().0.signum() * if new_bx == 1 || new_bx == 18 { -1 } else { 1 },
            dy,
        ));
    };

    let start = move || {
        if running.get() {
            return;
        }
        if game_over.get() {
            paddle_y.set(5);
            ai_y.set(5);
            ball.set((10, 5));
            ball_d.set((1, 1));
            score.set(0);
            game_over.set(false);
        }
        running.set(true);
        status.set("Rally!".into());
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(80).await;
                if !running.get() {
                    break;
                }
                step();
            }
        });
    };

    let pause = move || {
        if running.get() && !game_over.get() {
            running.set(false);
            status.set("Paused — Space to resume".into());
        }
    };

    let toggle = move || {
        if running.get() {
            pause();
        } else {
            start();
        }
    };

    let nudge = move |delta: i32| {
        paddle_y.update(|p| *p = (*p + delta).clamp(0, max_y));
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        match e.key().as_str() {
            " " => {
                e.prevent_default();
                if !e.repeat() {
                    toggle();
                }
            }
            "ArrowUp" | "w" | "W" => {
                e.prevent_default();
                nudge(-1);
            }
            "ArrowDown" | "s" | "S" => {
                e.prevent_default();
                nudge(1);
            }
            _ => {}
        }
    });

    view! {
        <div class="mx-auto max-w-lg space-y-2">
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|toggle()>
                {move || if game_over.get() { "New Game (Space)" } else if running.get() { "⏸ Pause (Space)" } else { "▶ Start (Space)" }}
            </button>
            {dpad(
                move || nudge(-1),
                move || {},
                move || nudge(1),
                move || {},
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · ↑ ↓ / W S move paddle"</p>
            <div class="relative h-48 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] overflow-hidden">
                <div class="absolute inset-y-0 left-0 w-3 flex items-center">
                    <div class="h-12 w-full rounded-r bg-[var(--accent)] transition-all" style=move || format!("margin-top: {}%", paddle_y.get() * 10)></div>
                </div>
                <div class="absolute inset-y-0 right-0 w-3 flex items-center">
                    <div class="h-12 w-full rounded-l bg-red-400 transition-all" style=move || format!("margin-top: {}%", ai_y.get() * 10)></div>
                </div>
                <div class="absolute w-3 h-3 rounded-full bg-white shadow" style=move || {
                    let (bx, by) = ball.get();
                    format!("left: {}%; top: {}%;", bx * 5, by * 10)
                }></div>
                <div class="absolute inset-0 flex items-center justify-center">
                    <div class="h-full w-px border-dashed border border-[var(--border-color)] opacity-30"></div>
                </div>
            </div>
        </div>
    }.into_any()
}

// ── Flappy ────────────────────────────────────────────────────────────────────

fn board_flappy(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let bird_y: RwSignal<f64> = RwSignal::new(5.0);
    let vel: RwSignal<f64> = RwSignal::new(0.0);
    let pipes: RwSignal<Vec<(f64, f64)>> = RwSignal::new(vec![(20.0, 4.0), (30.0, 6.0)]);
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);
    let bird_x = 3.0f64;

    let flap = move || {
        if game_over.get() {
            bird_y.set(5.0);
            vel.set(0.0);
            pipes.set(vec![(20.0, 4.0), (30.0, 6.0)]);
            running.set(false);
            game_over.set(false);
            score.set(0);
        }
        if !running.get() {
            running.set(true);
            status.set("Flap to fly!".into());
            leptos::task::spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(50).await;
                    if !running.get() {
                        break;
                    }
                    let y = bird_y.get();
                    let v = vel.get() + 0.3;
                    let ny = (y + v).clamp(0.0, 11.0);
                    bird_y.set(ny);
                    vel.set(v);

                    let mut ps = pipes.get();
                    for p in &mut ps {
                        p.0 -= 0.5;
                    }
                    ps.retain(|p| p.0 > -1.0);
                    if ps.last().map(|p| p.0 < 15.0).unwrap_or(true) {
                        ps.push((20.0, 2.0 + rand_f64() * 6.0));
                    }

                    let hit = ps.iter().any(|&(px, gap)| {
                        (bird_x - px).abs() < 1.2 && (ny < gap - 2.0 || ny > gap + 2.0)
                    });
                    if ny >= 11.0 || hit {
                        running.set(false);
                        game_over.set(true);
                        status.set(format!("💥 Crashed! Score: {}", score.get()));
                        break;
                    }

                    let passed = ps
                        .iter()
                        .filter(|&&(px, _)| px < bird_x && px > bird_x - 0.6)
                        .count();
                    if passed > 0 {
                        score.update(|s| *s += 1);
                        status.set(format!("Score: {}", score.get()));
                    }

                    pipes.set(ps);
                }
            });
        }
        vel.set(-2.5);
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        if matches!(e.key().as_str(), " " | "ArrowUp" | "w" | "W") {
            e.prevent_default();
            if !e.repeat() {
                flap();
            }
        }
    });

    let reset = move || {
        bird_y.set(5.0);
        vel.set(0.0);
        pipes.set(vec![(20.0, 4.0), (30.0, 6.0)]);
        running.set(false);
        game_over.set(false);
        score.set(0);
        status.set("Tap to start".into());
    };

    view! {
        <div class="mx-auto max-w-sm space-y-2">
            <div class="relative h-64 cursor-pointer rounded-lg border border-[var(--border-color)] bg-gradient-to-b from-sky-400 to-sky-200 dark:from-sky-900 dark:to-sky-700 overflow-hidden"
                on:click=move|_|flap()>
                <div class="absolute text-2xl transition-all" style=move || format!("left: {}%; top: {}%; transform: translateY(-50%);", bird_x * 5.0, bird_y.get() * 8.0)>"🐦"</div>
                {move || pipes.get().iter().map(|&(px, gap)| view! {
                    <div>
                        <div class="absolute bg-green-500 rounded" style=move || format!("left: calc({}% - 10px); top: 0; width: 20px; height: {}%;", px * 5.0, (gap - 2.0) * 8.0)></div>
                        <div class="absolute bg-green-500 rounded" style=move || format!("left: calc({}% - 10px); bottom: 0; width: 20px; height: {}%;", px * 5.0, (12.0 - gap - 2.0) * 8.0)></div>
                    </div>
                }).collect_view()}
                {move || if game_over.get() || !running.get() { view! { <div class="absolute inset-0 flex items-center justify-center text-white font-bold text-lg drop-shadow">{if game_over.get() { "💥 Click to retry" } else { "👆 Tap to fly!" }}</div> }.into_any() } else { view! { <div></div> }.into_any() }}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"Reset"</button>
        </div>
    }.into_any()
}

// ── Tetris ────────────────────────────────────────────────────────────────────

fn board_tetris(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let cols = 10usize;
    let rows = 20usize;
    let board: RwSignal<Vec<u8>> = RwSignal::new(vec![0; cols * rows]);
    let piece: RwSignal<Vec<(i32, i32)>> = RwSignal::new(vec![]);
    let piece_color = RwSignal::new(1u8);
    let running = RwSignal::new(false);
    let game_over = RwSignal::new(false);

    let tetrominoes: &'static [&'static [(i32, i32)]] = &[
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
        &[(0, 0), (1, 0), (2, 0), (1, 1)],
        &[(0, 0), (1, 0), (2, 0), (2, 1)],
        &[(0, 0), (1, 0), (2, 0), (0, 1)],
        &[(0, 0), (1, 0), (1, 1), (2, 1)],
        &[(1, 0), (2, 0), (0, 1), (1, 1)],
    ];

    let spawn = move || {
        let t = rand_usize(tetrominoes.len());
        let shape: Vec<(i32, i32)> = tetrominoes[t].iter().map(|&(x, y)| (x + 3, y)).collect();
        piece.set(shape);
        piece_color.set(t as u8 + 1);
    };

    let can_place = move |p: &[(i32, i32)], b: &[u8]| -> bool {
        p.iter().all(|&(x, y)| {
            x >= 0
                && x < cols as i32
                && y >= 0
                && y < rows as i32
                && b[y as usize * cols + x as usize] == 0
        })
    };

    let lock_piece = move |p: &[(i32, i32)], color: u8, b: &mut Vec<u8>| {
        for &(x, y) in p {
            if y >= 0 {
                b[y as usize * cols + x as usize] = color;
            }
        }
    };

    let move_piece = move |dx: i32, dy: i32| {
        if game_over.get() {
            return;
        }
        let p = piece.get();
        let moved: Vec<(i32, i32)> = p.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
        if can_place(&moved, &board.get()) {
            piece.set(moved);
        } else if dy > 0 {
            let mut b = board.get();
            lock_piece(&p, piece_color.get(), &mut b);
            let cleared = tetris_clear_filled(&mut b, cols);
            score.update(|s| *s += (cleared * cleared * 10) as u32);
            board.set(b);
            spawn();
            if !can_place(&piece.get(), &board.get()) {
                game_over.set(true);
                running.set(false);
                status.set(format!("Game over! Score: {}", score.get()));
            } else {
                status.set(format!("Score: {}", score.get()));
            }
        }
    };

    let start = move || {
        if running.get() {
            return;
        }
        if game_over.get() {
            board.set(vec![0; cols * rows]);
            score.set(0);
            game_over.set(false);
            spawn();
        } else if piece.get().is_empty() {
            spawn();
        }
        running.set(true);
        status.set("Running!".into());
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(500).await;
                if !running.get() {
                    break;
                }
                move_piece(0, 1);
            }
        });
    };

    let rotate = move || {
        if game_over.get() {
            return;
        }
        let rotated = tetris_rotate_cw(&piece.get());
        if can_place(&rotated, &board.get()) {
            piece.set(rotated);
        }
    };

    let pause = move || {
        if running.get() && !game_over.get() {
            running.set(false);
            status.set("Paused — Space to resume".into());
        }
    };

    let toggle = move || {
        if running.get() {
            pause();
        } else {
            start();
        }
    };

    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        match e.key().as_str() {
            " " => {
                e.prevent_default();
                if !e.repeat() {
                    toggle();
                }
            }
            "ArrowLeft" | "a" | "A" => {
                e.prevent_default();
                move_piece(-1, 0);
            }
            "ArrowRight" | "d" | "D" => {
                e.prevent_default();
                move_piece(1, 0);
            }
            "ArrowDown" | "s" | "S" => {
                e.prevent_default();
                move_piece(0, 1);
            }
            "ArrowUp" | "w" | "W" => {
                e.prevent_default();
                rotate();
            }
            _ => {}
        }
    });

    let colors = [
        "",
        "bg-cyan-400",
        "bg-yellow-400",
        "bg-purple-500",
        "bg-blue-500",
        "bg-orange-500",
        "bg-green-500",
        "bg-red-500",
    ];

    view! {
        <div class="mx-auto max-w-sm space-y-2">
            <div class="flex gap-2">
                <button type="button" class="flex-1 rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|toggle()>
                    {move || if game_over.get() { "New Game (Space)" } else if running.get() { "⏸ Pause (Space)" } else { "▶ Start (Space)" }}
                </button>
            </div>
            {dpad(
                rotate,
                move || move_piece(-1, 0),
                move || move_piece(0, 1),
                move || move_piece(1, 0),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · ↑ rotate · arrows / WASD"</p>
            <div class="grid gap-px rounded border border-[var(--border-color)] bg-[var(--border-color)] overflow-hidden" style="grid-template-columns: repeat(10, 1fr)">
                {(0..rows).flat_map(|row| (0..cols).map(move |col| view! {
                    <div class=move || {
                        let b = board.get();
                        let p = piece.get();
                        let pc = piece_color.get() as usize;
                        let cell = b[row * cols + col];
                        if p.contains(&(col as i32, row as i32)) {
                            format!("aspect-square {}", colors[pc.min(7)])
                        } else if cell != 0 {
                            format!("aspect-square {}", colors[cell as usize % 8])
                        } else {
                            "aspect-square bg-[var(--surface)]".into()
                        }
                    }></div>
                })).collect_view()}
            </div>
        </div>
    }.into_any()
}

// ── Chess (basic board — select & move highlighted squares) ──────────────────

fn board_chess(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let board: RwSignal<[i8; 64]> = RwSignal::new(chess_start());
    let selected: RwSignal<Option<usize>> = RwSignal::new(None);
    let game_over = RwSignal::new(false);
    let busy = RwSignal::new(false);
    status.set("White to move".into());

    let after_white = move |b: [i8; 64]| {
        if !chess_has_move(&b, false) {
            game_over.set(true);
            if chess_is_check(&b, false) {
                score.update(|s| *s += 50);
                status.set("Checkmate — you win!".into());
            } else {
                status.set("Stalemate".into());
            }
            return;
        }
        status.set("Black thinking…".into());
        busy.set(true);
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            let mut b2 = b;
            if let Some((from, to)) = chess_ai_move(&b2) {
                if let Some(n) = chess_apply_move(&b2, from, to) {
                    b2 = n;
                }
            }
            board.set(b2);
            busy.set(false);
            if !chess_has_move(&b2, true) {
                game_over.set(true);
                if chess_is_check(&b2, true) {
                    status.set("Checkmate — Black wins".into());
                } else {
                    status.set("Stalemate".into());
                }
            } else if chess_is_check(&b2, true) {
                status.set("Check! Your move".into());
            } else {
                status.set("White to move".into());
            }
        });
    };

    let click = move |i: usize| {
        if game_over.get() || busy.get() {
            return;
        }
        let b = board.get();
        if let Some(from) = selected.get() {
            if from == i {
                selected.set(None);
                return;
            }
            if let Some(n) = chess_apply_move(&b, from, i) {
                board.set(n);
                selected.set(None);
                score.update(|s| *s += 1);
                after_white(n);
                return;
            }
        }
        if b[i] > 0 {
            selected.set(Some(i));
            let n = chess_legal_moves(&b, i).len();
            status.set(format!("{} legal move(s)", n));
        } else {
            selected.set(None);
        }
    };

    view! {
        <div class="mx-auto max-w-md space-y-3">
            <div class="grid grid-cols-8 gap-0 rounded-lg overflow-hidden border border-[var(--border-color)]">
                {(0..64).map(|i| {
                    let row = i / 8;
                    let col = i % 8;
                    let is_light = (row + col) % 2 == 0;
                    view! {
                        <button type="button"
                            class=move || {
                                let sel = selected.get();
                                let legal = sel.is_some_and(|s| chess_legal_moves(&board.get(), s).contains(&i));
                                let highlight = if sel == Some(i) { " ring-2 ring-inset ring-yellow-400" }
                                    else if legal { " ring-2 ring-inset ring-green-400" }
                                    else { "" };
                                let bg = if is_light { "bg-amber-100 dark:bg-amber-200" } else { "bg-amber-700 dark:bg-amber-800" };
                                format!("aspect-square text-xl flex items-center justify-center {bg}{highlight} hover:opacity-90")
                            }
                            on:click=move |_| click(i)>
                            {move || chess_glyph(board.get()[i])}
                        </button>
                    }
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|{
                board.set(chess_start());
                selected.set(None);
                game_over.set(false);
                busy.set(false);
                score.set(0);
                status.set("White to move".into());
            }>"New Game"</button>
        </div>
    }.into_any()
}
