#![allow(clippy::possible_missing_else)]
use crate::application::services::games::{BreakoutService, PongService};
use crate::domain::games::{
    blackjack_score, blackjack_should_hit, checkers_moves, chess_ai_move, chess_apply_move,
    chess_glyph, chess_has_move, chess_is_check, chess_legal_moves, chess_start,
    connect_four_ai_column, connect_four_drop, connect_four_winner, hangman_word, has_move_2048,
    lights_toggle, minesweeper_adjacent_mines_sized, minesweeper_flood_reveal_sized,
    puzzle_is_solved, puzzle_move, shuffle_deck, slide_2048, snake_step, sudoku_given,
    sudoku_puzzle_with_seed, sudoku_valid, tetris_clear_filled, tetris_rotate_cw,
    ttt_best_move_sized, ttt_is_draw_sized, ttt_winner_sized, typing_reactor_tasks, wordle_check,
    wordle_word, BreakoutGame, BreakoutTickResult, FlappyGame, PongGame, TypingReactor,
};
use leptos::ev;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::i18n::*;

fn toggle_browser_fullscreen() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    if document.fullscreen_element().is_some() {
        document.exit_fullscreen();
        return;
    }

    if let Some(root) = document.document_element() {
        let _ = root.request_fullscreen();
    }
}

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
pub enum GameKind {
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
    Breakout,
    Pong,
    Flappy,
    Tetris,
    Chess,
    Checkers,
    Blackjack,
}
impl GameKind {
    fn all() -> [Self; 19] {
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
            Self::Breakout,
            Self::Pong,
            Self::Flappy,
            Self::Tetris,
            Self::Chess,
            Self::Checkers,
            Self::Blackjack,
        ]
    }
    fn slug(self) -> &'static str {
        match self {
            Self::TwentyFortyEight => "2048",
            Self::TicTacToe => "tic-tac-toe",
            Self::Minesweeper => "minesweeper",
            Self::Snake => "snake",
            Self::Sudoku => "sudoku",
            Self::ConnectFour => "connect-four",
            Self::Memory => "memory",
            Self::Typing => "typing",
            Self::Wordle => "wordle",
            Self::Hangman => "hangman",
            Self::FifteenPuzzle => "15-puzzle",
            Self::LightsOut => "lights-out",
            Self::Breakout => "breakout",
            Self::Pong => "pong",
            Self::Flappy => "flappy",
            Self::Tetris => "tetris",
            Self::Chess => "chess",
            Self::Checkers => "checkers",
            Self::Blackjack => "blackjack",
        }
    }

    pub(crate) fn from_slug(slug: &str) -> Option<Self> {
        Self::all().into_iter().find(|game| game.slug() == slug)
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
pub(crate) fn GamesPage(game: Option<GameKind>) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <main class="flex flex-1 flex-col px-4 py-8 sm:px-6 lg:px-8">
            <div class="mx-auto w-full max-w-7xl">
                <div class="mb-8 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
                    <div>
                        <p class="text-xs font-semibold uppercase tracking-widest text-[var(--accent)]">"Arcade"</p>
                        <h1 class="mt-1 text-3xl font-bold text-[var(--text-primary)]">
                            {move || t_string!(i18n, games_title)}
                        </h1>
                        <p class="mt-2 max-w-2xl text-sm text-[var(--text-secondary)]">
                            "Nineteen compact browser games — all client-side Rust/WASM, no server needed."
                        </p>
                    </div>
                    <span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs text-[var(--text-tertiary)]">
                        "19 games"
                    </span>
                </div>
                {match game {
                    Some(game) => view! { <GameView game /> }.into_any(),
                    None => view! { <GameGrid /> }.into_any(),
                }}
            </div>
        </main>
    }
}

#[component]
fn GameGrid() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {GameKind::all()
                .into_iter()
                .map(|game| {
                    let href = format!("#/games/{}", game.slug());
                    view! {
                        <a
                            href=href
                            class="group flex min-h-40 flex-col rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 text-left transition hover:-translate-y-0.5 hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                        >
                            <span class="flex h-10 w-10 items-center justify-center rounded-lg border border-[var(--border-color)] text-xs font-bold text-[var(--accent)]">
                                {game.icon()}
                            </span>
                            <span class="mt-4 text-base font-semibold text-[var(--text-primary)]">
                                {move || localized_game_title(game)}
                            </span>
                            <span class="mt-1 text-sm text-[var(--text-secondary)]">
                                {move || localized_game_description(game)}
                            </span>
                            <span class="mt-auto pt-4 text-xs font-medium text-[var(--accent)]">"Play →"</span>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn GameView(game: GameKind) -> AnyView {
    let score = RwSignal::new(0u32);
    let status = RwSignal::new(String::from("Ready"));

    if game == GameKind::Hangman {
        hangman_game_view(score, status)
    } else {
        standard_game_view(game, score, status)
    }
}

fn hangman_game_view(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    view! {
        <section class="flex min-h-[100dvh] flex-col bg-[var(--surface)] px-4 py-4 sm:px-6 sm:py-6">
            <div class="mx-auto flex w-full max-w-3xl flex-wrap items-center gap-3">
                <a
                    href="#/games"
                    class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                >
                    "← All games"
                </a>
                <div class="min-w-0 flex-1">
                    <h2 class="text-xl font-bold text-[var(--text-primary)]">"Hangman"</h2>
                    <p class="text-sm text-[var(--text-secondary)]">"Guess the word before the figure is complete."</p>
                </div>
                <span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs text-[var(--text-tertiary)]">
                    {move || status.get()}
                </span>
                <button
                    type="button"
                    class="min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                    title="Enter browser fullscreen"
                    aria-label="Enter browser fullscreen"
                    on:click=move |_| toggle_browser_fullscreen()
                >
                    "Fullscreen"
                </button>
            </div>
            <div class="flex min-h-0 flex-1 items-center justify-center py-4 sm:py-6">
                {board_hangman(score, status)}
            </div>
        </section>
    }
    .into_any()
}

fn standard_game_view(game: GameKind, score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    view! {
        <section class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-4 sm:p-6">
            <div class="mb-5 flex flex-wrap items-center gap-3">
                <a
                    href="#/games"
                    class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                >
                    "← All games"
                </a>
                <div class="min-w-0 flex-1">
                    <h2 class="text-xl font-bold text-[var(--text-primary)]">{game.title()}</h2>
                    <p class="text-sm text-[var(--text-secondary)]">{game.description()}</p>
                </div>
                <span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs text-[var(--text-tertiary)]">
                    {move || status.get()}
                </span>
            </div>
            {match game {
                GameKind::TwentyFortyEight => board_2048(score, status),
                GameKind::TicTacToe => board_ttt(score, status),
                GameKind::Minesweeper => board_mines(score, status),
                GameKind::Snake => board_snake(score, status),
                GameKind::Sudoku => board_sudoku(score, status),
                GameKind::ConnectFour => board_connect_four(score, status),
                GameKind::Memory => board_memory(score, status),
                GameKind::Typing => board_typing(score, status),
                GameKind::Wordle => board_wordle(score, status),
                GameKind::FifteenPuzzle => board_puzzle(score, status),
                GameKind::LightsOut => board_lights(score, status),
                GameKind::Breakout => board_breakout(score, status),
                GameKind::Pong => board_pong(score, status),
                GameKind::Flappy => board_flappy(score, status),
                GameKind::Tetris => board_tetris(score, status),
                GameKind::Chess => board_chess(score, status),
                GameKind::Checkers => board_checkers(score, status),
                GameKind::Blackjack => board_blackjack(score, status),
                GameKind::Hangman => unreachable!("Hangman is rendered by hangman_game_view"),
            }}
        </section>
    }
    .into_any()
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
            {dpad(
                move || slide(0),
                move || slide(3),
                move || slide(2),
                move || slide(1),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Arrows / WASD to slide"</p>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Tic-Tac-Toe ───────────────────────────────────────────────────────────────

fn board_ttt(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let i18n = use_i18n();
    let size = RwSignal::new(3usize);
    let win_len = RwSignal::new(3usize);
    let board = RwSignal::new(vec![' '; 9]);
    let player_turn = RwSignal::new(true);
    let game_over = RwSignal::new(false);

    let reset = move || {
        let n = size.get();
        let target = win_len.get();
        board.set(vec![' '; n * n]);
        player_turn.set(true);
        game_over.set(false);
        if i18n.get_locale() == Locale::en {
            status.set(format!(
                "Your turn (X) — {n}×{n}, first to {target} in a row"
            ));
        } else {
            status.set(format!(
                "Lượt của bạn (X) — {n}×{n}, thắng khi có {target} quân liên tiếp"
            ));
        }
    };

    let set_size = move |value: usize| {
        let n = value.clamp(3, 6);
        let target = n.min(5);
        size.set(n);
        win_len.set(target);
        board.set(vec![' '; n * n]);
        player_turn.set(true);
        game_over.set(false);
        if i18n.get_locale() == Locale::en {
            status.set(format!(
                "Your turn (X) — {n}×{n}, first to {target} in a row"
            ));
        } else {
            status.set(format!(
                "Lượt của bạn (X) — {n}×{n}, thắng khi có {target} quân liên tiếp"
            ));
        }
    };

    status.set("Your turn (X) — 3×3, first to 3 in a row".into());

    let click = move |i: usize| {
        if game_over.get() || !player_turn.get() {
            return;
        }
        let n = size.get();
        let target = win_len.get();
        let mut b = board.get();
        if i >= b.len() || b[i] != ' ' {
            return;
        }
        b[i] = 'X';
        board.set(b.clone());

        if let Some(w) = ttt_winner_sized(&b, n, target) {
            if i18n.get_locale() == Locale::en {
                status.set(format!("{w} wins! 🎉"));
            } else {
                status.set(format!("{w} thắng! 🎉"));
            }
            game_over.set(true);
            if w == 'X' {
                score.update(|s| *s += 10);
            }
            return;
        }
        if ttt_is_draw_sized(&b, n, target) {
            status.set(if i18n.get_locale() == Locale::en {
                "Draw!".into()
            } else {
                "Hòa!".into()
            });
            game_over.set(true);
            return;
        }

        player_turn.set(false);
        status.set(if i18n.get_locale() == Locale::en {
            "AI thinking…".into()
        } else {
            "AI đang suy nghĩ…".into()
        });

        let b_copy = b;
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            if let Some(ai_idx) = ttt_best_move_sized(&b_copy, n, target) {
                let mut b2 = b_copy;
                b2[ai_idx] = 'O';
                board.set(b2.clone());
                if let Some(w) = ttt_winner_sized(&b2, n, target) {
                    status.set(if i18n.get_locale() == Locale::en {
                        format!("{w} wins!")
                    } else {
                        format!("{w} thắng!")
                    });
                    game_over.set(true);
                } else if ttt_is_draw_sized(&b2, n, target) {
                    status.set(if i18n.get_locale() == Locale::en {
                        "Draw!".into()
                    } else {
                        "Hòa!".into()
                    });
                    game_over.set(true);
                } else {
                    status.set(if i18n.get_locale() == Locale::en {
                        "Your turn (X)".into()
                    } else {
                        "Lượt của bạn (X)".into()
                    });
                    player_turn.set(true);
                }
            } else {
                player_turn.set(true);
            }
        });
    };

    view! {
        <div class="mx-auto grid w-full max-w-5xl gap-6 lg:grid-cols-[minmax(0,1fr)_18rem] lg:items-start">
            <div class="min-w-0 space-y-4">
                <div class="flex flex-wrap items-center justify-center gap-2">
                    <span class="text-sm font-medium text-[var(--text-secondary)]">
                        {move || if i18n.get_locale() == Locale::en { "Board" } else { "Bàn cờ" }}
                    </span>
                    {[3usize, 4, 5, 6].into_iter().map(|n| {
                        view! {
                            <button
                                type="button"
                                class=move || format!(
                                    "min-h-10 rounded-md border px-3 py-2 text-sm font-semibold {}",
                                    if size.get() == n {
                                        "border-[var(--accent)] bg-[var(--surface-hover)] text-[var(--text-primary)]"
                                    } else {
                                        "border-[var(--border-color)] text-[var(--text-secondary)] hover:bg-[var(--surface-hover)]"
                                    }
                                )
                                aria-pressed=move || size.get() == n
                                on:click=move |_| set_size(n)
                            >
                                {format!("{n}×{n}")}
                            </button>
                        }
                    }).collect_view()}
                </div>

                <div
                    class=move || {
                        let grid_class = match size.get() {
                            3 => "ttt-board--3",
                            4 => "ttt-board--4",
                            5 => "ttt-board--5",
                            _ => "ttt-board--6",
                        };
                        format!("ttt-board mx-auto grid w-full max-w-xl gap-1.5 sm:gap-2 {grid_class}")
                    }
                >
                    {(0..36).map(|i| view! {
                        <button
                            type="button"
                            class=move || {
                                let b = board.get();
                                if i >= b.len() {
                                    "hidden".to_string()
                                } else {
                                    format!(
                                        "aspect-square rounded-lg border border-[var(--border-color)] text-3xl font-bold hover:bg-[var(--surface-hover)] sm:text-4xl {}",
                                        match b[i] {
                                            'X' => "text-[var(--accent)]",
                                            'O' => "text-red-500",
                                            _ => "text-[var(--text-primary)]",
                                        }
                                    )
                                }
                            }
                            on:click=move |_| click(i)
                        >
                            {move || board.get().get(i).copied().unwrap_or(' ').to_string()}
                        </button>
                    }).collect_view()}
                </div>
                <button
                    type="button"
                    class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm"
                    on:click=move|_|reset()
                >
                    {move || if i18n.get_locale() == Locale::en { "New Game" } else { "Ván mới" }}
                </button>
            </div>

            <aside class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                <h3 class="text-base font-semibold text-[var(--text-primary)]">
                    {move || if i18n.get_locale() == Locale::en { "How to play" } else { "Cách chơi" }}
                </h3>
                <div class="mt-3 space-y-3 text-sm text-[var(--text-secondary)]">
                    <p>
                        {move || if i18n.get_locale() == Locale::en {
                            "Choose a board size, then place X before the AI places O."
                        } else {
                            "Chọn kích thước bàn cờ, sau đó đặt X trước khi AI đặt O."
                        }}
                    </p>
                    <p>
                        {move || if i18n.get_locale() == Locale::en {
                            "Win by getting the required number of your symbols in a straight line: horizontal, vertical, or diagonal."
                        } else {
                            "Thắng bằng cách tạo đủ số quân liên tiếp theo hàng ngang, hàng dọc hoặc đường chéo."
                        }}
                    </p>
                    <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3">
                        <p class="font-medium text-[var(--text-primary)]">
                            {move || if i18n.get_locale() == Locale::en { "Win condition" } else { "Điều kiện thắng" }}
                        </p>
                        <ul class="mt-2 space-y-1">
                            <li>"3×3 → 3 " {move || if i18n.get_locale() == Locale::en { "in a row" } else { "quân liên tiếp" }}</li>
                            <li>"4×4 → 4 " {move || if i18n.get_locale() == Locale::en { "in a row" } else { "quân liên tiếp" }}</li>
                            <li>"5×5 → 5 " {move || if i18n.get_locale() == Locale::en { "in a row" } else { "quân liên tiếp" }}</li>
                            <li>"6×6 → 5 " {move || if i18n.get_locale() == Locale::en { "in a row" } else { "quân liên tiếp" }}</li>
                        </ul>
                    </div>
                    <p>
                        {move || if i18n.get_locale() == Locale::en {
                            "If every cell is filled without a winner, the game is a draw."
                        } else {
                            "Nếu tất cả ô đều được đánh mà không có người thắng, ván đấu hòa."
                        }}
                    </p>
                </div>
            </aside>
        </div>
    }.into_any()
}

// ── Minesweeper ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum MinesweeperDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl MinesweeperDifficulty {
    fn all() -> [Self; 3] {
        [Self::Beginner, Self::Intermediate, Self::Expert]
    }

    fn dimensions(self) -> (usize, usize, usize) {
        match self {
            Self::Beginner => (9, 9, 10),
            Self::Intermediate => (16, 16, 40),
            Self::Expert => (30, 16, 99),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Expert => "Expert",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Beginner => "9×9 · 10 mines",
            Self::Intermediate => "16×16 · 40 mines",
            Self::Expert => "30×16 · 99 mines",
        }
    }
}

fn board_mines(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let size = RwSignal::new(MinesweeperDifficulty::Beginner);
    let (width, height, _) = size.get().dimensions();
    let mines = RwSignal::new(vec![false; width * height]);
    let revealed = RwSignal::new(vec![false; width * height]);
    let flagged = RwSignal::new(vec![false; width * height]);
    let first_click = RwSignal::new(true);
    let game_over = RwSignal::new(false);

    let reset = move || {
        let (w, h, _) = size.get().dimensions();
        mines.set(vec![false; w * h]);
        revealed.set(vec![false; w * h]);
        flagged.set(vec![false; w * h]);
        first_click.set(true);
        game_over.set(false);
        score.set(0);
        status.set("Ready — left click to reveal".into());
    };

    let reveal = move |i: usize| {
        if game_over.get() || revealed.get()[i] || flagged.get()[i] {
            return;
        }

        let (w, h, mine_count) = size.get().dimensions();
        let mut m = mines.get();

        if first_click.get() {
            first_click.set(false);
            let mut placed = 0;
            while placed < mine_count {
                let idx = rand_usize(w * h);
                if idx != i && !m[idx] {
                    m[idx] = true;
                    placed += 1;
                }
            }
            mines.set(m.clone());
        }

        if m[i] {
            let mut r = revealed.get();
            r[i] = true;
            revealed.set(r);
            game_over.set(true);
            status.set("💥 Mine! Game over.".into());
            return;
        }

        let mut r = revealed.get();
        let newly = minesweeper_flood_reveal_sized(&m, &r, w, h, i);
        for idx in newly {
            r[idx] = true;
        }
        revealed.set(r.clone());

        let safe_count = r.iter().filter(|&&v| v).count();
        score.set(safe_count as u32);

        if safe_count + mine_count == w * h {
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

    let chord = move |i: usize| {
        if game_over.get() || !revealed.get()[i] {
            return;
        }

        let (w, h, mine_count) = size.get().dimensions();
        let m = mines.get();
        let r = revealed.get();
        let f = flagged.get();
        let adjacent_mines = minesweeper_adjacent_mines_sized(&m, w, h, i);

        let row = i / w;
        let col = i % w;
        let mut adjacent_flags = 0usize;
        let mut neighbours = Vec::new();

        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if (0..h as i32).contains(&nr) && (0..w as i32).contains(&nc) {
                    let ni = nr as usize * w + nc as usize;
                    if f[ni] {
                        adjacent_flags += 1;
                    } else if !r[ni] {
                        neighbours.push(ni);
                    }
                }
            }
        }

        if adjacent_flags != adjacent_mines as usize {
            return;
        }

        // Chord reveals only the eight directly adjacent cells.
        let mut next_revealed = r;
        let mut hit_mine = false;

        for ni in neighbours {
            next_revealed[ni] = true;
            hit_mine |= m[ni];
        }

        revealed.set(next_revealed.clone());

        if hit_mine {
            game_over.set(true);
            status.set("💥 Mine! Game over.".into());
            return;
        }

        let safe_count = next_revealed.iter().filter(|&&v| v).count();
        score.set(safe_count as u32);

        if safe_count + mine_count == w * h {
            game_over.set(true);
            status.set("🎉 You cleared the field!".into());
        } else {
            status.set(format!("{} safe cells revealed", safe_count));
        }
    };

    let reset_to_size = move |new_size: MinesweeperDifficulty| {
        size.set(new_size);
        let (w, h, _) = new_size.dimensions();
        mines.set(vec![false; w * h]);
        revealed.set(vec![false; w * h]);
        flagged.set(vec![false; w * h]);
        first_click.set(true);
        game_over.set(false);
        score.set(0);
        status.set("Ready — left click to reveal".into());
    };

    let handle_mouse_down = move |i: usize, e: web_sys::MouseEvent| {
        e.prevent_default();

        if e.button() == 2 {
            flag(i);
            return;
        }

        if e.button() == 0 {
            if revealed.get()[i] {
                chord(i);
            } else {
                reveal(i);
            }
        }
    };

    let cell_class = move |i: usize| {
        let m = mines.get();
        let r = revealed.get();
        let f = flagged.get();

        if r[i] && m[i] {
            "aspect-square min-w-5 border border-[var(--mines-revealed-border)] bg-[var(--mines-mine)] text-white text-[clamp(0.55rem,1.4vw,0.85rem)] font-bold leading-none".to_string()
        } else if r[i] {
            let number_color = match minesweeper_adjacent_mines_sized(
                &m,
                size.get().dimensions().0,
                size.get().dimensions().1,
                i,
            ) {
                1 => "text-blue-600 dark:text-blue-400",
                2 => "text-green-700 dark:text-green-400",
                3 => "text-red-600 dark:text-red-400",
                4 => "text-purple-700 dark:text-purple-400",
                _ => "text-[var(--text-primary)]",
            };
            format!("aspect-square min-w-5 border border-[var(--mines-revealed-border)] bg-[var(--mines-revealed)] {number_color} text-[clamp(0.55rem,1.4vw,0.85rem)] font-bold leading-none")
        } else if f[i] {
            "aspect-square min-w-5 border-2 border-[var(--mines-raised-border)] bg-[var(--mines-raised)] text-[clamp(0.55rem,1.4vw,0.85rem)] font-bold leading-none".to_string()
        } else {
            "aspect-square min-w-5 border-2 border-[var(--mines-raised-border)] bg-[var(--mines-covered)] text-[clamp(0.55rem,1.4vw,0.85rem)] leading-none hover:bg-[var(--mines-covered-hover)]".to_string()
        }
    };

    let grid_class = move || {
        let (w, _, _) = size.get().dimensions();
        match w {
            9 => "grid grid-cols-[repeat(9,minmax(0,1fr))]",
            16 => "grid grid-cols-[repeat(16,minmax(0,1fr))]",
            _ => "grid grid-cols-[repeat(30,minmax(0,1fr))]",
        }
    };

    view! {
        <div class="mx-auto w-full max-w-5xl space-y-4">
            <div class="flex flex-wrap items-center gap-2">
                <span class="mr-1 text-xs font-semibold uppercase tracking-wide text-[var(--text-tertiary)]">
                    "Difficulty"
                </span>
                {MinesweeperDifficulty::all().into_iter().map(|preset| {
                    let active = move || size.get() == preset;
                    view! {
                        <button
                            type="button"
                            class=move || if active() {
                                "rounded-md border border-[var(--accent)] bg-[var(--accent-soft)] px-3 py-2 text-xs font-semibold text-[var(--text-primary)]"
                            } else {
                                "rounded-md border border-[var(--border-color)] px-3 py-2 text-xs font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)]"
                            }
                            title=preset.description()
                            on:click=move |_| reset_to_size(preset)
                        >
                            {preset.label()}
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="flex flex-wrap items-center justify-between gap-2 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2 text-xs">
                <span class="font-semibold text-[var(--text-primary)]">
                    {move || {
                        let (_, _, mines_count) = size.get().dimensions();
                        format!("Mines: {} · Flags: {}", mines_count, flagged.get().iter().filter(|&&v| v).count())
                    }}
                </span>
                <span class="text-[var(--text-secondary)]">{move || status.get()}</span>
            </div>

            <div class="overflow-x-auto rounded-lg border-4 border-[var(--border-color)] bg-[var(--surface-hover)] p-1 shadow-sm">
                <div class=grid_class()>
                    {move || {
                        let (w, h, _) = size.get().dimensions();
                        (0..w * h).map(|i| view! {
                        <button
                            type="button"
                            class=move || cell_class(i)
                            aria-label=move || {
                                let m = mines.get();
                                let r = revealed.get();
                                let f = flagged.get();
                                if f[i] && !r[i] {
                                    "Flagged cell".to_string()
                                } else if r[i] && m[i] {
                                    "Mine".to_string()
                                } else if r[i] {
                                    let (w, h, _) = size.get().dimensions();
                                    let n = minesweeper_adjacent_mines_sized(&m, w, h, i);
                                    format!("Revealed cell, {} adjacent mines", n)
                                } else {
                                    "Hidden cell".to_string()
                                }
                            }
                            on:mousedown=move |e| handle_mouse_down(i, e)
                            on:contextmenu=move |e| e.prevent_default()
                        >
                            {move || {
                                let m = mines.get();
                                let r = revealed.get();
                                let f = flagged.get();
                                if f[i] && !r[i] {
                                    "🚩".to_string()
                                } else if r[i] && m[i] {
                                    "💣".to_string()
                                } else if r[i] {
                                    let (w, h, _) = size.get().dimensions();
                                    let n = minesweeper_adjacent_mines_sized(&m, w, h, i);
                                    if n == 0 { String::new() } else { n.to_string() }
                                } else {
                                    String::new()
                                }
                            }}
                        </button>
                        }).collect_view()
                    }}
                </div>
            </div>

            <div class="grid gap-2 text-xs text-[var(--text-secondary)] sm:grid-cols-3">
                <div class="rounded-md border border-[var(--border-color)] p-3">
                    <strong class="text-[var(--text-primary)]">"Left click"</strong>
                    <span>" — reveal a cell."</span>
                </div>
                <div class="rounded-md border border-[var(--border-color)] p-3">
                    <strong class="text-[var(--text-primary)]">"Right click"</strong>
                    <span>" — place/remove a flag."</span>
                </div>
                <div class="rounded-md border border-[var(--border-color)] p-3">
                    <strong class="text-[var(--text-primary)]">"Click revealed number"</strong>
                    <span>" — chord its adjacent cells when flags match the number."</span>
                </div>
            </div>

            <button
                type="button"
                class="mx-auto flex h-12 w-12 items-center justify-center rounded-full border-2 border-[var(--border-color)] bg-[var(--surface-hover)] text-xl shadow-sm transition hover:bg-[var(--surface)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                on:click=move |_| reset()
                title="New Game"
                aria-label="New Game"
            >
                {move || if game_over.get() { "😎" } else { "🙂" }}
            </button>
        </div>
    }
    .into_any()
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
            {dpad(
                move || set_dir((0, -1)),
                move || set_dir((-1, 0)),
                move || set_dir((0, 1)),
                move || set_dir((1, 0)),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · arrows / WASD move"</p>
        </div>
    }.into_any()
}

// ── Sudoku ────────────────────────────────────────────────────────────────────

fn board_sudoku(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let initial_puzzle = sudoku_puzzle_with_seed(rand_f64().to_bits());
    let board = RwSignal::new(initial_puzzle);
    let given: RwSignal<[bool; 81]> = RwSignal::new(sudoku_given(&initial_puzzle));
    let conflicts: RwSignal<[bool; 81]> = RwSignal::new([false; 81]);

    let check_complete = move |b: &[u8; 81]| -> bool {
        b.iter().all(|&v| v != 0) && (0..81).all(|i| sudoku_valid(b, i, b[i]))
    };

    let reset = move || {
        let puzzle = sudoku_puzzle_with_seed(rand_f64().to_bits());
        board.set(puzzle);
        given.set(sudoku_given(&puzzle));
        conflicts.set([false; 81]);
        score.set(0);
        status.set("New puzzle".into());
    };

    let click = move |i: usize| {
        if given.get()[i] {
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
            <div class="grid grid-cols-9 gap-0 overflow-hidden rounded-md border-2 border-[var(--text-primary)]">
                {(0..81).map(|i| view! {
                    <button type="button"
                        class=move || {
                            let is_given = given.get()[i];
                            let conflict = conflicts.get()[i];
                            let col = i % 9;
                            let row = i / 9;
                            let right = if col == 2 || col == 5 || col == 8 {
                                "border-r-2 border-r-solid border-r-[var(--border-color)]"
                            } else {
                                "border-r border-r-dashed border-r-[var(--border-color)]"
                            };
                            let bottom = if row == 2 || row == 5 || row == 8 {
                                "border-b-2 border-b-solid border-b-[var(--border-color)]"
                            } else {
                                "border-b border-b-dashed border-b-[var(--border-color)]"
                            };
                            let left = "border-l border-l-dashed border-l-[var(--border-color)]";
                            let top = "border-t border-t-dashed border-t-[var(--border-color)]";
                            format!(
                                "aspect-square text-xs font-bold {left} {right} {top} {bottom} {}",
                                if conflict {
                                    "text-red-500 bg-red-50 dark:bg-red-900/20"
                                } else if is_given {
                                    "text-[var(--text-primary)] bg-[var(--surface-hover)]"
                                } else {
                                    "text-[var(--accent)] hover:bg-[var(--surface-hover)]"
                                }
                            )
                        }
                        on:click=move |_| click(i)>
                        {move || {
                            let v = board.get()[i];
                            if v == 0 { String::new() } else { v.to_string() }
                        }}
                    </button>
                }).collect_view()}
            </div>
            <button
                type="button"
                class="min-h-11 w-full rounded-md border-2 border-[var(--border-color)] bg-[var(--surface)] py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                on:click=move |_| reset()
            >
                "New Game"
            </button>
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
            <div class="connect-four-board grid grid-cols-7 gap-1 rounded-lg border p-2">
                {(0..42).map(|i| view! {
                    <div class=move || {
                        let v = board.get()[i];
                        match v {
                            1 => "connect-four-disc connect-four-disc--red",
                            2 => "connect-four-disc connect-four-disc--yellow",
                            _ => "connect-four-disc connect-four-disc--empty",
                        }
                    }></div>
                }).collect_view()}
            </div>
            <button type="button" class="w-full rounded-md border border-[var(--border-color)] py-2 text-sm" on:click=move|_|reset()>"New Game"</button>
        </div>
    }.into_any()
}

// ── Memory Cards ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum MemorySize {
    Small,
    Medium,
    Large,
}

impl MemorySize {
    fn dimensions(self) -> usize {
        match self {
            Self::Small => 4,
            Self::Medium => 6,
            Self::Large => 8,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Small => "4 × 4",
            Self::Medium => "6 × 6",
            Self::Large => "8 × 8",
        }
    }
}

fn board_memory(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let emojis = [
        "🍎", "🍊", "🍋", "🍇", "🍓", "🍒", "🍑", "🥝", "🥑", "🍉", "🍌", "🍍", "🥭", "🍐", "🍑",
        "🥥", "🥕", "🌽", "🍄", "🥨", "🍪", "🍩", "🍰", "🍫", "⚽", "🏀", "🎸", "🎹", "🚗", "🚲",
        "🚀", "⭐",
    ];

    let size = RwSignal::new(MemorySize::Small);
    let cards: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());
    let revealed: RwSignal<Vec<bool>> = RwSignal::new(Vec::new());
    let matched: RwSignal<Vec<bool>> = RwSignal::new(Vec::new());
    let first: RwSignal<Option<usize>> = RwSignal::new(None);
    let locked = RwSignal::new(false);

    let new_game = move |new_size: MemorySize| {
        let cells = new_size.dimensions() * new_size.dimensions();
        let pairs = cells / 2;
        let mut deck: Vec<usize> = (0..pairs).chain(0..pairs).collect();
        for i in (1..deck.len()).rev() {
            let j = rand_usize(i + 1);
            deck.swap(i, j);
        }

        size.set(new_size);
        cards.set(deck);
        revealed.set(vec![false; cells]);
        matched.set(vec![false; cells]);
        first.set(None);
        locked.set(false);
        score.set(0);
        status.set(format!(
            "{} × {} — find all pairs",
            new_size.dimensions(),
            new_size.dimensions()
        ));
    };

    new_game(MemorySize::Small);

    let click = move |i: usize| {
        if locked.get() || revealed.get()[i] || matched.get()[i] {
            return;
        }

        let mut r = revealed.get();
        r[i] = true;
        revealed.set(r);

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
                            (m.len() - m.iter().filter(|&&v| v).count()) / 2
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
        <div class="mx-auto w-full max-w-2xl space-y-4">
            <div class="flex flex-wrap justify-center gap-2" role="group" aria-label="Memory board size">
                {[MemorySize::Small, MemorySize::Medium, MemorySize::Large]
                    .into_iter()
                    .map(|option| view! {
                        <button
                            type="button"
                            class=move || {
                                if size.get() == option {
                                    "min-h-10 rounded-md border-2 border-[var(--accent)] bg-[var(--surface-hover)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                                } else {
                                    "min-h-10 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                                }
                            }
                            on:click=move |_| new_game(option)
                            aria-pressed=move || size.get() == option
                        >
                            {option.label()}
                        </button>
                    })
                    .collect_view()}
            </div>

            <div
                class=move || {
                    format!(
                        "mx-auto grid w-full gap-2 {}",
                        match size.get() {
                            MemorySize::Small => "max-w-sm grid-cols-4",
                            MemorySize::Medium => "max-w-md grid-cols-6",
                            MemorySize::Large => "max-w-lg grid-cols-8",
                        }
                    )
                }
            >
                {move || {
                    let cells = size.get().dimensions().pow(2);
                    (0..cells)
                        .map(|i| view! {
                            <button
                                type="button"
                                class=move || {
                                    if matched.get()[i] {
                                        "aspect-square rounded-lg border-2 border-green-500 bg-green-100 dark:bg-green-900/30 text-lg sm:text-2xl"
                                    } else if revealed.get()[i] {
                                        "aspect-square rounded-lg border border-[var(--accent)] bg-[var(--surface-hover)] text-lg sm:text-2xl"
                                    } else {
                                        "aspect-square rounded-lg border border-[var(--border-color)] hover:bg-[var(--surface-hover)] text-lg sm:text-2xl"
                                    }
                                }
                                on:click=move |_| click(i)
                            >
                                {move || {
                                    let c = cards.get();
                                    if revealed.get()[i] || matched.get()[i] {
                                        emojis[c[i]].to_string()
                                    } else {
                                        "?".to_string()
                                    }
                                }}
                            </button>
                        })
                        .collect_view()
                }}
            </div>

            <button
                type="button"
                class="min-h-11 w-full rounded-md border-2 border-[var(--border-color)] bg-[var(--surface)] py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                on:click=move |_| new_game(size.get())
            >
                "New Game"
            </button>
        </div>
    }.into_any()
}

// ── Typing Speed ──────────────────────────────────────────────────────────────

fn board_typing(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let tasks = typing_reactor_tasks();
    let reactor = RwSignal::new(TypingReactor::new());
    let task_index = RwSignal::new(0usize);
    let input = RwSignal::new(String::new());
    let started_ms = RwSignal::new(None::<f64>);
    let elapsed_ms = RwSignal::new(0.0f64);
    let running = RwSignal::new(false);
    let finished = RwSignal::new(false);

    let reset = move || {
        reactor.set(TypingReactor::new());
        task_index.set(0);
        input.set(String::new());
        started_ms.set(None);
        elapsed_ms.set(0.0);
        running.set(false);
        finished.set(false);
        score.set(0);
        status.set("Core stable — press Start".into());
    };

    let start = move || {
        if finished.get() {
            reset();
        }
        if running.get() {
            return;
        }
        running.set(true);
        let now = js_sys::Date::now();
        if started_ms.get().is_none() {
            started_ms.set(Some(now));
        }
        status.set("Reactor online — type the command".into());
    };

    let submit = move || {
        if !running.get() || finished.get() {
            return;
        }

        let typed = input.get().trim().to_lowercase();
        if typed.is_empty() {
            return;
        }

        let target = tasks[task_index.get()];
        let correct = typed == target;
        let mut next_reactor = reactor.get();
        next_reactor.submit(correct);
        reactor.set(next_reactor);
        score.set(next_reactor.score());
        input.set(String::new());

        if next_reactor.game_over() {
            running.set(false);
            finished.set(true);
            let elapsed = started_ms
                .get()
                .map(|t| (js_sys::Date::now() - t).max(1.0))
                .unwrap_or(1.0);
            elapsed_ms.set(elapsed);
            status.set(format!(
                "CORE MELTDOWN — {} pts · {} tasks · {}% accuracy",
                next_reactor.score(),
                next_reactor.completed(),
                accuracy_percent(&next_reactor)
            ));
            return;
        }

        let next = (task_index.get() + 1) % tasks.len();
        task_index.set(next);

        if correct {
            if next_reactor
                .combo()
                .is_multiple_of(TypingReactor::CRITICAL_COMBO)
            {
                status.set("CRITICAL HIT — reactor cooled".into());
            } else {
                status.set(format!("GOOD — combo x{}", next_reactor.combo()));
            }
        } else {
            status.set("ERROR — reactor heat spike".into());
        }
    };

    let accuracy = move || accuracy_percent(&reactor.get());

    {
        leptos::task::spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(250).await;
                if finished.get() {
                    break;
                }
                if running.get() {
                    if let Some(started) = started_ms.get() {
                        elapsed_ms.set((js_sys::Date::now() - started).max(0.0));
                    }
                    let mut state = reactor.get();
                    state.tick_heat(1);
                    reactor.set(state);
                    if state.game_over() {
                        running.set(false);
                        finished.set(true);
                        status.set(format!(
                            "CORE MELTDOWN — {} pts · heat reached 100%",
                            state.score()
                        ));
                    }
                }
            }
        });
    }

    view! {
        <div class="mx-auto w-full max-w-2xl space-y-4">
            <div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
                <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-3">
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">"Heat"</p>
                    <p class="mt-1 text-xl font-bold text-[var(--text-primary)]">{move || format!("{}%", reactor.get().heat())}</p>
                </div>
                <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-3">
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">"Combo"</p>
                    <p class="mt-1 text-xl font-bold text-[var(--text-primary)]">{move || format!("x{}", reactor.get().combo())}</p>
                </div>
                <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-3">
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">"Score"</p>
                    <p class="mt-1 text-xl font-bold text-[var(--text-primary)]">{move || reactor.get().score()}</p>
                </div>
                <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-3">
                    <p class="text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">"Accuracy"</p>
                    <p class="mt-1 text-xl font-bold text-[var(--text-primary)]">{move || format!("{}%", accuracy())}</p>
                </div>
            </div>

            <div
                class="h-3 overflow-hidden rounded-full border border-[var(--border-color)] bg-[var(--surface-hover)]"
                role="progressbar"
                aria-label="Reactor heat"
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=move || reactor.get().heat().to_string()
            >
                <div
                    class="h-full rounded-full transition-all duration-200"
                    class=("bg-[var(--accent)]", move || reactor.get().heat() < 70)
                    class=("bg-amber-500", move || {
                        let heat = reactor.get().heat();
                        (70..90).contains(&heat)
                    })
                    class=("bg-red-500", move || reactor.get().heat() >= 90)
                    class=("w-0", move || reactor.get().heat() == 0)
                    class=("w-1/12", move || (1..=10).contains(&reactor.get().heat()))
                    class=("w-2/12", move || (11..=20).contains(&reactor.get().heat()))
                    class=("w-3/12", move || (21..=30).contains(&reactor.get().heat()))
                    class=("w-4/12", move || (31..=40).contains(&reactor.get().heat()))
                    class=("w-5/12", move || (41..=50).contains(&reactor.get().heat()))
                    class=("w-6/12", move || (51..=60).contains(&reactor.get().heat()))
                    class=("w-7/12", move || (61..=70).contains(&reactor.get().heat()))
                    class=("w-8/12", move || (71..=80).contains(&reactor.get().heat()))
                    class=("w-10/12", move || (81..=90).contains(&reactor.get().heat()))
                    class=("w-full", move || reactor.get().heat() >= 91)
                ></div>
            </div>

            <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-5 text-center sm:p-8">
                <p class="text-[10px] font-semibold uppercase tracking-[0.25em] text-[var(--text-tertiary)]">
                    {move || format!("TASK {}/{}", task_index.get() + 1, tasks.len())}
                </p>
                <p class="mt-3 text-2xl font-bold tracking-wide text-[var(--accent)] sm:text-3xl">
                    {move || tasks[task_index.get()]}
                </p>
                <p class="mt-2 text-xs text-[var(--text-tertiary)]">
                    "Keep the reactor below 100%. Every 5-hit combo triggers a critical cooldown."
                </p>
            </div>

            <input
                type="text"
                autocomplete="off"
                spellcheck="false"
                class="w-full rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-4 text-center font-mono text-base text-[var(--text-primary)] outline-none transition focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]"
                placeholder="Type the command and press Enter"
                prop:value=move || input.get()
                disabled=move || finished.get()
                on:input=move |e| {
                    if started_ms.get().is_none() {
                        started_ms.set(Some(js_sys::Date::now()));
                    }
                    input.set(event_target_value(&e));
                    if !running.get() {
                        start();
                    }
                }
                on:keydown=move |e| {
                    if e.key() == "Enter" {
                        e.prevent_default();
                        submit();
                    }
                }
            />

            <div class="flex flex-wrap items-center justify-center gap-2">
                <button
                    type="button"
                    class="min-h-11 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| start()
                >
                    {move || if finished.get() { "Restart Reactor" } else if running.get() { "Reactor Online" } else { "Start Reactor" }}
                </button>
                <button
                    type="button"
                    class="min-h-11 rounded-md border border-[var(--border-color)] px-4 py-2 text-sm font-medium text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| reset()
                >
                    "Reset"
                </button>
            </div>

            <div class="flex flex-wrap justify-center gap-x-6 gap-y-1 text-xs text-[var(--text-tertiary)]" aria-live="polite">
                <span>{move || format!("Correct {}", reactor.get().correct())}</span>
                <span>{move || format!("Wrong {}", reactor.get().wrong())}</span>
                <span>{move || format!("Critical {}", reactor.get().criticals())}</span>
                <span>{move || format!("{:.1}s", elapsed_ms.get() / 1000.0)}</span>
            </div>
        </div>
    }.into_any()
}

fn accuracy_percent(game: &TypingReactor) -> u32 {
    if game.completed() == 0 {
        0
    } else {
        ((game.correct() * 100) / game.completed()).min(100)
    }
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
    let word = hangman_word().to_ascii_lowercase();
    let guessed: RwSignal<Vec<char>> = RwSignal::new(vec![]);
    let max_wrong = 6usize;

    let wrong_word = word.clone();
    let wrong_count = Memo::new(move |_| {
        guessed
            .get()
            .iter()
            .filter(|&&c| !wrong_word.contains(c))
            .count()
    });

    let won_word = word.clone();
    let is_won = Memo::new(move |_| won_word.chars().all(|c| guessed.get().contains(&c)));
    let is_lost = Memo::new(move |_| wrong_count.get() >= max_wrong);

    let guess_word = word.clone();
    let guess_word_reveal = word.clone();
    let guess = move |c: char| {
        if is_won.get() || is_lost.get() || guessed.get().contains(&c) {
            return;
        }

        let mut g = guessed.get();
        g.push(c);
        guessed.set(g.clone());

        let wrong = g.iter().filter(|&&ch| !guess_word.contains(ch)).count();
        if guess_word.chars().all(|ch| g.contains(&ch)) {
            score.update(|s| *s += 20);
            status.set("Mission complete — launch successful!".into());
        } else if wrong >= max_wrong {
            status.set(format!(
                "Launch aborted — the code was {}",
                guess_word_reveal.to_ascii_uppercase()
            ));
        } else {
            status.set(format!(
                "Wrong code fragment — {} attempts remaining",
                max_wrong - wrong
            ));
        }
    };

    let keyboard_guess = guess.clone();
    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) || e.ctrl_key() || e.alt_key() || e.meta_key() {
            return;
        }

        let key = e.key().to_ascii_lowercase();
        if key.len() == 1 {
            if let Some(c) = key.chars().next().filter(|c| c.is_ascii_alphabetic()) {
                e.prevent_default();
                keyboard_guess(c);
            }
        }
    });

    let reset = move || {
        guessed.set(vec![]);
        score.set(0);
        status.set("Mission ready — enter the launch code".into());
    };

    let mission_stage = move || wrong_count.get().min(max_wrong);

    view! {
        <div class="mx-auto w-full max-w-5xl space-y-5">
            <div class="grid gap-4 lg:grid-cols-[320px_minmax(0,1fr)]">
                <section class="relative overflow-hidden rounded-2xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-5 sm:p-6">
                    <div class="flex items-start justify-between gap-3">
                        <div>
                            <p class="text-[11px] font-bold uppercase tracking-[0.18em] text-[var(--accent)]">
                                "Mission Control"
                            </p>
                            <h3 class="mt-1 text-xl font-bold text-[var(--text-primary)]">
                                "Launch Code"
                            </h3>
                        </div>
                        <span class=move || {
                            if is_won.get() {
                                "rounded-full border border-emerald-500/50 bg-emerald-500/10 px-2.5 py-1 text-xs font-bold text-emerald-700 dark:text-emerald-300"
                            } else if is_lost.get() {
                                "rounded-full border border-red-500/50 bg-red-500/10 px-2.5 py-1 text-xs font-bold text-red-700 dark:text-red-300"
                            } else {
                                "rounded-full border border-[var(--border-color)] px-2.5 py-1 text-xs font-bold text-[var(--text-secondary)]"
                            }
                        }>
                            {move || if is_won.get() { "LAUNCHED" } else if is_lost.get() { "ABORTED" } else { "STANDBY" }}
                        </span>
                    </div>

                    <div class="relative mx-auto mt-5 h-72 max-w-[260px] overflow-hidden rounded-xl border border-[var(--border-color)] bg-[var(--surface)]">
                        <div class="absolute inset-x-0 bottom-0 h-16 border-t border-[var(--border-color)] bg-[var(--surface-hover)]"></div>
                        <div class="absolute bottom-12 left-1/2 h-2 w-40 -translate-x-1/2 rounded-full bg-[var(--text-tertiary)]"></div>

                        <div
                            class=move || format!(
                                "absolute bottom-16 left-1/2 h-40 w-24 -translate-x-1/2 rounded-t-[3rem] border-2 border-[var(--text-primary)] bg-[var(--surface-hover)] transition-transform duration-500 {}",
                                if is_won.get() { "-translate-y-8" } else { "" }
                            )
                        >
                            <div class="absolute left-1/2 top-7 h-12 w-12 -translate-x-1/2 rounded-full border border-[var(--border-color)] bg-[var(--surface)]"></div>
                            <div class="absolute bottom-7 left-1/2 h-12 w-8 -translate-x-1/2 rounded-md border border-[var(--border-color)]"></div>
                            <div
                                class="absolute -bottom-2 left-1/2 h-8 w-10 -translate-x-1/2 rounded-b-full bg-[var(--accent)] transition-opacity duration-300"
                                class:opacity-0=move || mission_stage() < 2
                            ></div>
                            <div class="absolute -left-5 bottom-5 h-12 w-8 -skew-x-12 rounded-l-xl border border-[var(--border-color)] bg-[var(--surface)]"></div>
                            <div class="absolute -right-5 bottom-5 h-12 w-8 skew-x-12 rounded-r-xl border border-[var(--border-color)] bg-[var(--surface)]"></div>
                        </div>

                        <div class="absolute left-3 top-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)]/90 px-2 py-1 font-mono text-[10px] font-bold tracking-widest text-[var(--text-tertiary)]">
                            "MISSION 01"
                        </div>
                        <div class="absolute right-3 top-3 text-right font-mono text-[10px] font-bold tracking-widest text-[var(--text-tertiary)]">
                            <div>"STATUS"</div>
                            <div class="mt-1 text-[var(--accent)]">
                                {move || if is_won.get() { "GO" } else if is_lost.get() { "ABORT" } else { "ARMED" }}
                            </div>
                        </div>

                        <div class="absolute bottom-2 left-3 right-3">
                            <div class="flex items-center justify-between font-mono text-[10px] font-bold uppercase tracking-wider text-[var(--text-tertiary)]">
                                <span>"Mission progress"</span>
                                <span>{move || format!("{}/6", mission_stage())}</span>
                            </div>
                            <div class="mt-1 grid grid-cols-6 gap-1">
                                {(0..max_wrong).map(|i| view! {
                                    <div class=move || {
                                        if mission_stage() > i {
                                            "h-1.5 rounded-full bg-[var(--accent)]"
                                        } else {
                                            "h-1.5 rounded-full bg-[var(--border-color)]"
                                        }
                                    }></div>
                                }).collect_view()}
                            </div>
                        </div>
                    </div>

                    <div class="mt-4 grid grid-cols-2 gap-2 text-xs">
                        <div class="rounded-lg border border-[var(--border-color)] p-3">
                            <p class="font-mono uppercase tracking-wider text-[var(--text-tertiary)]">"Engine"</p>
                            <p class=move || {
                                if mission_stage() >= 2 || is_won.get() { "mt-1 font-bold text-[var(--accent)]" } else { "mt-1 font-bold text-[var(--text-tertiary)]" }
                            }>
                                {move || if mission_stage() >= 2 || is_won.get() { "ONLINE" } else { "OFFLINE" }}
                            </p>
                        </div>
                        <div class="rounded-lg border border-[var(--border-color)] p-3">
                            <p class="font-mono uppercase tracking-wider text-[var(--text-tertiary)]">"Navigation"</p>
                            <p class=move || {
                                if mission_stage() >= 4 || is_won.get() { "mt-1 font-bold text-[var(--accent)]" } else { "mt-1 font-bold text-[var(--text-tertiary)]" }
                            }>
                                {move || if mission_stage() >= 4 || is_won.get() { "LOCKED" } else { "PENDING" }}
                            </p>
                        </div>
                    </div>
                </section>

                <section class="flex flex-col rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-7">
                    <div class="flex items-start justify-between gap-4">
                        <div>
                            <p class="text-xs font-bold uppercase tracking-[0.18em] text-[var(--accent)]">
                                "Mission Briefing"
                            </p>
                            <h3 class="mt-1 text-2xl font-bold text-[var(--text-primary)] sm:text-3xl">
                                "Enter the launch code"
                            </h3>
                            <p class="mt-2 max-w-xl text-sm text-[var(--text-secondary)]">
                                "Decode the word before the launch sequence runs out."
                            </p>
                        </div>
                        <div class="text-right">
                            <p class="text-[11px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">"Score"</p>
                            <p class="text-lg font-bold tabular-nums text-[var(--text-primary)]">{move || score.get()}</p>
                        </div>
                    </div>

                    <div class="mt-8 flex min-h-24 flex-wrap items-end justify-center gap-x-2 gap-y-3">
                        {word.chars().map(|c| {
                            let word_char = c;
                            view! {
                                <span class="flex h-12 w-8 items-center justify-center border-b-2 border-[var(--text-primary)] text-2xl font-bold uppercase text-[var(--text-primary)] sm:w-10 sm:text-3xl">
                                    {move || if guessed.get().contains(&word_char) || is_lost.get() {
                                        word_char.to_string()
                                    } else {
                                        String::new()
                                    }}
                                </span>
                            }
                        }).collect_view()}
                    </div>

                    <p class="mt-6 min-h-6 text-center text-sm font-medium text-[var(--text-secondary)]" aria-live="polite">
                        {move || status.get()}
                    </p>

                    <div class="mt-auto pt-7">
                        <div class="grid grid-cols-7 gap-1.5 sm:grid-cols-9" aria-label="Launch code keyboard">
                            {('a'..='z').map(|c| {
                                let button_guess = guess.clone();
                                let button_word = word.clone();
                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let used = guessed.get().contains(&c);
                                            let correct = button_word.contains(c);
                                            if used && correct {
                                                "min-h-10 rounded-lg border border-emerald-500 bg-emerald-500/15 text-sm font-bold uppercase text-emerald-700 dark:text-emerald-300"
                                            } else if used {
                                                "min-h-10 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] text-sm font-bold uppercase text-[var(--text-tertiary)] line-through"
                                            } else {
                                                "min-h-10 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] text-sm font-bold uppercase text-[var(--text-primary)] shadow-sm transition hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] active:scale-95"
                                            }
                                        }
                                        on:click=move |_| button_guess(c)
                                        disabled=move || guessed.get().contains(&c) || is_won.get() || is_lost.get()
                                        aria-label=format!("Enter launch code letter {}", c.to_ascii_uppercase())
                                    >
                                        {c.to_ascii_uppercase().to_string()}
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                        <p class="mt-3 text-center text-xs text-[var(--text-tertiary)]">
                            "Type A–Z or use the launch keyboard."
                        </p>
                    </div>
                </section>
            </div>

            <div class="flex flex-col items-center justify-center gap-3 sm:flex-row">
                <button
                    type="button"
                    class="min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-5 py-2.5 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| reset()
                >
                    "New Mission"
                </button>
                <p class="text-xs text-[var(--text-tertiary)]">
                    "Six failed inputs trigger launch abort."
                </p>
            </div>

            <div class="flex flex-wrap items-center justify-center gap-x-5 gap-y-2 text-xs text-[var(--text-tertiary)]">
                <span>{move || format!("Failed inputs: {}", wrong_count.get())}</span>
                <span>{move || format!("Attempts remaining: {}", max_wrong - wrong_count.get().min(max_wrong))}</span>
            </div>
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

// ── Breakout ──────────────────────────────────────────────────────────────────

fn board_breakout(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let game = RwSignal::new(BreakoutService::new_game());
    let running = RwSignal::new(false);
    let left_pressed = RwSignal::new(false);
    let right_pressed = RwSignal::new(false);
    let animation_frame = RwSignal::new(None::<i32>);

    let start_loop = Rc::new(move || {
        if animation_frame.get_untracked().is_some() {
            return;
        }

        let callback = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
        let callback_ref = Rc::clone(&callback);
        let last_time = Rc::new(RefCell::new(None::<f64>));
        let accumulator = Rc::new(RefCell::new(0.0f64));
        let frame_window = window();
        let callback_window = frame_window.clone();

        let frame = Closure::wrap(Box::new(move |now: f64| {
            if !running.get_untracked() {
                animation_frame.set(None);
                return;
            }

            let previous = last_time.borrow_mut().replace(now).unwrap_or(now);
            let dt = ((now - previous) / 1000.0).clamp(0.0, 0.05);

            let direction = match (left_pressed.get_untracked(), right_pressed.get_untracked()) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            };
            if direction != 0.0 {
                let mut current = game.get_untracked();
                current.move_paddle_by(direction * 7.5 * dt);
                game.set(current);
            }

            {
                let mut accumulated = accumulator.borrow_mut();
                *accumulated += dt;
                while *accumulated >= 1.0 / 60.0 {
                    let mut current = game.get_untracked();
                    let result = BreakoutService::tick(&mut current);
                    let current_score = current.score();
                    let current_lives = current.lives();
                    let finished = current.is_finished();
                    game.set(current);
                    score.set(current_score);

                    match result {
                        BreakoutTickResult::Rally => {}
                        BreakoutTickResult::BrickHit => {
                            status.set(format!("Brick hit · {} points", current_score));
                        }
                        BreakoutTickResult::LifeLost => {
                            status.set(format!("Life lost · {} lives left", current_lives));
                        }
                        BreakoutTickResult::Won => {
                            running.set(false);
                            status.set("All bricks cleared!".into());
                        }
                        BreakoutTickResult::GameOver => {
                            running.set(false);
                            status.set("Game over · press Space to restart".into());
                        }
                    }

                    if finished {
                        running.set(false);
                        break;
                    }
                    *accumulated -= 1.0 / 60.0;
                }
            }

            if !running.get_untracked() {
                animation_frame.set(None);
                return;
            }

            let request_id = {
                let callback_ref = callback_ref.borrow();
                callback_ref.as_ref().and_then(|cb| {
                    callback_window
                        .request_animation_frame(cb.as_ref().unchecked_ref())
                        .ok()
                })
            };
            animation_frame.set(request_id);
        }) as Box<dyn FnMut(f64)>);

        *callback.borrow_mut() = Some(frame);
        let request_id = {
            let callback_ref = callback.borrow();
            callback_ref.as_ref().and_then(|cb| {
                frame_window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .ok()
            })
        };
        animation_frame.set(request_id);
    });

    let stop_loop = Rc::new(move || {
        if let Some(id) = animation_frame.get_untracked() {
            let _ = window().cancel_animation_frame(id);
            animation_frame.set(None);
        }
        left_pressed.set(false);
        right_pressed.set(false);
    });

    on_cleanup(move || {
        if let Some(id) = animation_frame.get_untracked() {
            let _ = window().cancel_animation_frame(id);
        }
        animation_frame.set(None);
        left_pressed.set(false);
        right_pressed.set(false);
    });

    let start_game: Rc<dyn Fn()> = {
        let start_loop = Rc::clone(&start_loop);
        Rc::new(move || {
            if running.get() {
                return;
            }

            if game.get().is_finished() {
                BreakoutService::reset(&mut game.write());
                score.set(0);
            }

            running.set(true);
            status.set("Ball in play".into());
            start_loop();
        })
    };

    let pause: Rc<dyn Fn()> = {
        let stop_loop = Rc::clone(&stop_loop);
        Rc::new(move || {
            if running.get() {
                running.set(false);
                stop_loop();
                status.set("Paused · press Space to resume".into());
            }
        })
    };

    let toggle: Rc<dyn Fn()> = {
        let start_game = Rc::clone(&start_game);
        let pause = Rc::clone(&pause);
        Rc::new(move || {
            if running.get() {
                pause();
            } else {
                start_game();
            }
        })
    };

    let keydown = {
        let toggle = toggle.clone();
        move |e: web_sys::KeyboardEvent| {
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
                    left_pressed.set(true);
                }
                "ArrowRight" | "d" | "D" => {
                    e.prevent_default();
                    right_pressed.set(true);
                }
                _ => {}
            }
        }
    };
    let keyup = move |e: web_sys::KeyboardEvent| match e.key().as_str() {
        "ArrowLeft" | "a" | "A" => left_pressed.set(false),
        "ArrowRight" | "d" | "D" => right_pressed.set(false),
        _ => {}
    };

    let keydown_handle = window_event_listener(ev::keydown, keydown);
    let keyup_handle = window_event_listener(ev::keyup, keyup);
    on_cleanup(move || {
        keydown_handle.remove();
        keyup_handle.remove();
    });

    view! {
        <div class="mx-auto w-full max-w-xl space-y-3">
            <div class="flex flex-wrap justify-center gap-2">
                <span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs font-semibold text-[var(--text-primary)]">
                    {move || format!("Score {}", game.get().score())}
                </span>
                <span class="rounded-full border border-[var(--border-color)] px-3 py-1 text-xs font-semibold text-[var(--text-primary)]">
                    {move || format!("Lives {}", game.get().lives())}
                </span>
            </div>

            <svg
                viewBox="0 0 480 720"
                class="mx-auto block w-full max-w-md rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] shadow-sm"
                role="img"
                aria-label="Breakout game board"
                tabindex="0"
                on:pointerdown=move |_| {
                    if !running.get() {
                        start_game();
                    }
                }
            >
                <rect x="0" y="0" width="480" height="720" fill="currentColor" opacity="0.04"/>
                {(0..BreakoutGame::BRICK_ROWS)
                    .flat_map(|row| {
                        (0..BreakoutGame::BRICK_COLS).map(move |col| {
                            let x = (3 + col as i32) * 40;
                            let y = row as i32 * 40;
                            view! {
                                <rect
                                    x=x
                                    y=y
                                    width="38"
                                    height="34"
                                    rx="5"
                                    class=move || {
                                        if game.get().brick_active(row, col) {
                                            "breakout-brick"
                                        } else {
                                            "opacity-0"
                                        }
                                    }
                                />
                            }
                        })
                    })
                    .collect_view()}
                <rect
                    x=move || game.get().paddle_position() * 40.0
                    y=BreakoutGame::PADDLE_Y * 40
                    width=BreakoutGame::PADDLE_WIDTH * 40
                    height="18"
                    rx="9"
                    class="breakout-paddle"
                />
                <circle
                    cx=move || (game.get().ball_position().0 + 0.5) * 40.0
                    cy=move || (game.get().ball_position().1 + 0.5) * 40.0
                    r="13"
                    class="breakout-ball"
                />
            </svg>

            <div class="flex flex-col gap-2">
                <button
                    type="button"
                    class="min-h-11 w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| toggle()
                >
                    {move || {
                        if game.get().is_finished() {
                            "New Game (Space)"
                        } else if running.get() {
                            "Pause (Space)"
                        } else {
                            "Start (Space)"
                        }
                    }}
                </button>

                {dpad(
                    move || {},
                    move || left_pressed.set(true),
                    move || {},
                    move || right_pressed.set(true),
                )}

                <p class="text-center text-xs text-[var(--text-tertiary)]">
                    "Hold ← → / A D for smooth paddle movement · Space start/pause"
                </p>
            </div>
        </div>
    }
    .into_any()
}

// ── Pong ──────────────────────────────────────────────────────────────────────

fn board_pong(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let game = RwSignal::new(PongService::new_game());
    let running = RwSignal::new(false);
    let up_pressed = RwSignal::new(false);
    let down_pressed = RwSignal::new(false);
    let frame_window = web_sys::window().expect("window should exist");
    let animation_id = Rc::new(std::cell::Cell::new(None::<i32>));
    let last_time = Rc::new(std::cell::Cell::new(None::<f64>));

    let start_loop: Rc<dyn Fn()> = {
        let animation_id = animation_id.clone();
        let last_time = last_time.clone();
        let callback_window = frame_window.clone();

        Rc::new(move || {
            if running.get() {
                return;
            }

            if game.get().is_game_over() {
                PongService::reset(&mut game.write());
                score.set(0);
            }

            running.set(true);
            status.set("Rally!".into());
            last_time.set(None);

            let tick_frame: Rc<RefCell<Option<Box<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
            let tick_frame_clone = tick_frame.clone();
            let last_time_clone = last_time.clone();
            let animation_id_clone = animation_id.clone();
            let callback_window_clone = callback_window.clone();

            *tick_frame.borrow_mut() = Some(Box::new(move |timestamp| {
                if !running.get() {
                    last_time_clone.set(None);
                    animation_id_clone.set(None);
                    return;
                }

                let dt = last_time_clone
                    .get()
                    .map(|previous| (timestamp - previous) / 1000.0)
                    .unwrap_or(0.0);
                last_time_clone.set(Some(timestamp));

                match PongService::tick(&mut game.write(), dt, up_pressed.get(), down_pressed.get())
                {
                    crate::domain::games::PongTickResult::Rally => {}
                    crate::domain::games::PongTickResult::PlayerScored => {
                        score.set(game.get().score());
                        status.set(format!("You scored! {}", game.get().score()));
                    }
                    crate::domain::games::PongTickResult::ComputerScored => {
                        running.set(false);
                        status.set(format!("Computer wins — score {}", game.get().score()));
                        animation_id_clone.set(None);
                        return;
                    }
                }

                if running.get() {
                    let callback_ref = tick_frame_clone.borrow();
                    if let Some(callback) = callback_ref.as_ref() {
                        if let Ok(id) = callback_window_clone
                            .request_animation_frame(callback.as_ref() as &js_sys::Function)
                        {
                            animation_id_clone.set(Some(id));
                        }
                    }
                } else {
                    animation_id_clone.set(None);
                }
            }));

            let callback_ref = tick_frame.borrow();
            if let Some(callback) = callback_ref.as_ref() {
                if let Ok(id) =
                    callback_window.request_animation_frame(callback.as_ref().unchecked_ref())
                {
                    animation_id.set(Some(id));
                }
            }
        })
    };

    let stop_loop: Rc<dyn Fn()> = {
        let animation_id = animation_id.clone();
        let callback_window = frame_window.clone();

        Rc::new(move || {
            running.set(false);
            up_pressed.set(false);
            down_pressed.set(false);
            last_time.set(None);

            if let Some(id) = animation_id.get() {
                let _ = callback_window.cancel_animation_frame(id);
                animation_id.set(None);
            }
        })
    };

    let toggle_loop: Rc<dyn Fn()> = {
        let start_loop = start_loop.clone();
        let stop_loop = stop_loop.clone();

        Rc::new(move || {
            if running.get() {
                stop_loop();
                status.set("Paused — Space to resume".into());
            } else {
                start_loop();
            }
        })
    };

    bind_keys({
        let toggle_loop = toggle_loop.clone();
        move |e: web_sys::KeyboardEvent| {
            if is_text_input(&e) {
                return;
            }

            match e.key().as_str() {
                " " => {
                    e.prevent_default();
                    if !e.repeat() {
                        toggle_loop();
                    }
                }
                "ArrowUp" | "w" | "W" => {
                    e.prevent_default();
                    up_pressed.set(true);
                }
                "ArrowDown" | "s" | "S" => {
                    e.prevent_default();
                    down_pressed.set(true);
                }
                _ => {}
            }
        }
    });

    let keyup_handle = window_event_listener(ev::keyup, move |e: web_sys::KeyboardEvent| {
        match e.key().as_str() {
            "ArrowUp" | "w" | "W" => up_pressed.set(false),
            "ArrowDown" | "s" | "S" => down_pressed.set(false),
            _ => {}
        }
    });
    on_cleanup({
        let animation_id = animation_id.clone();
        let callback_window = frame_window.clone();
        move || {
            if let Some(id) = animation_id.get() {
                let _ = callback_window.cancel_animation_frame(id);
            }
            keyup_handle.remove();
        }
    });

    let board_width = PongGame::WIDTH;
    let board_height = PongGame::HEIGHT;

    view! {
        <div class="mx-auto w-full max-w-5xl space-y-3">
            <div class="flex gap-2">
                <button
                    type="button"
                    class="min-h-11 flex-1 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click={
                        let toggle_loop = toggle_loop.clone();
                        move |_| toggle_loop()
                    }
                >
                    {move || if game.get().is_game_over() {
                        "New Game (Space)"
                    } else if running.get() {
                        "Pause (Space)"
                    } else {
                        "Start (Space)"
                    }}
                </button>
            </div>

            <p class="text-center text-xs text-[var(--text-tertiary)]">
                "One player · Hold ↑ ↓ / W S · Space to start or pause"
            </p>

            <div
                class="mx-auto w-full overflow-hidden rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] shadow-lg"
                role="img"
                aria-label="Single-player Pong game board"
            >
                <svg
                    viewBox="0 0 960 540"
                    class="block aspect-video h-auto w-full select-none"
                    aria-hidden="true"
                >
                    <rect
                        x="0"
                        y="0"
                        width=board_width
                        height=board_height
                        fill="currentColor"
                        opacity="0.03"
                    />
                    <line
                        x1=move || board_width / 2.0
                        y1="0"
                        x2=move || board_width / 2.0
                        y2=board_height
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-dasharray="12 12"
                        opacity="0.22"
                    />
                    <rect
                        x="16"
                        y=move || game.get().player_y() - PongGame::PADDLE_HEIGHT / 2.0
                        width=PongGame::PADDLE_WIDTH
                        height=PongGame::PADDLE_HEIGHT
                        rx="6"
                        fill="var(--accent)"
                    />
                    <rect
                        x=move || board_width - 16.0 - PongGame::PADDLE_WIDTH
                        y=move || game.get().computer_y() - PongGame::PADDLE_HEIGHT / 2.0
                        width=PongGame::PADDLE_WIDTH
                        height=PongGame::PADDLE_HEIGHT
                        rx="6"
                        fill="var(--text-secondary)"
                    />
                    <circle
                        cx=move || game.get().ball_position().0
                        cy=move || game.get().ball_position().1
                        r=PongGame::BALL_RADIUS
                        fill="var(--text-primary)"
                    />
                </svg>
            </div>

            <div class="flex justify-center gap-2">
                <button
                    type="button"
                    class="min-h-11 min-w-20 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] touch-none"
                    aria-label="Move paddle up"
                    on:mousedown=move |_| up_pressed.set(true)
                    on:mouseup=move |_| up_pressed.set(false)
                    on:mouseleave=move |_| up_pressed.set(false)
                    on:touchstart=move |_| up_pressed.set(true)
                    on:touchend=move |_| up_pressed.set(false)
                >
                    "↑"
                </button>
                <button
                    type="button"
                    class="min-h-11 min-w-20 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] touch-none"
                    aria-label="Move paddle down"
                    on:mousedown=move |_| down_pressed.set(true)
                    on:mouseup=move |_| down_pressed.set(false)
                    on:mouseleave=move |_| down_pressed.set(false)
                    on:touchstart=move |_| down_pressed.set(true)
                    on:touchend=move |_| down_pressed.set(false)
                >
                    "↓"
                </button>
            </div>
        </div>
    }
    .into_any()
}

// ── Flappy ────────────────────────────────────────────────────────────────────

fn board_flappy(score: RwSignal<u32>, status: RwSignal<String>) -> AnyView {
    let game = RwSignal::new(FlappyGame::new(300.0));
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let animation_frame = RwSignal::new(None::<i32>);
    let animation_time = RwSignal::new(0.0f64);

    let render = Rc::new(move || {
        let Some(canvas) = canvas_ref.get_untracked() else {
            return;
        };
        let canvas: &HtmlCanvasElement = canvas.as_ref();
        let Some(context) = canvas_context(canvas) else {
            return;
        };
        let dpr = window().device_pixel_ratio().clamp(1.0, 2.5);
        let width = (FlappyGame::WIDTH * dpr).round() as u32;
        let height = (FlappyGame::HEIGHT * dpr).round() as u32;
        if canvas.width() != width || canvas.height() != height {
            canvas.set_width(width);
            canvas.set_height(height);
        }
        let _ = context.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0);
        draw_flappy(
            &context,
            &game.get_untracked(),
            animation_time.get_untracked(),
        );
    });

    let stop_loop = Rc::new(move || {
        if let Some(id) = animation_frame.get_untracked() {
            let _ = window().cancel_animation_frame(id);
            animation_frame.set(None);
        }
    });

    let animation_render = Rc::clone(&render);
    let start_loop = Rc::new(move || {
        if animation_frame.get_untracked().is_some() {
            return;
        }

        let w = window();

        let last_time = Rc::new(RefCell::new(None::<f64>));
        let accumulator = Rc::new(RefCell::new(0.0f64));
        let callback = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
        let callback_ref = Rc::clone(&callback);
        let last_time_ref = Rc::clone(&last_time);
        let accumulator_ref = Rc::clone(&accumulator);

        let frame_window = w.clone();
        let frame_render = Rc::clone(&animation_render);
        let frame = Closure::wrap(Box::new(move |now: f64| {
            let mut last = last_time_ref.borrow_mut();
            let previous = last.replace(now).unwrap_or(now);
            let frame_dt = ((now - previous) / 1000.0).min(0.1);
            drop(last);

            let mut accumulated = accumulator_ref.borrow_mut();
            *accumulated += frame_dt;
            while *accumulated >= 1.0 / 60.0 {
                let mut next = game.get_untracked();
                next.update(
                    1.0 / 60.0,
                    FlappyGame::GAP_MIN_Y
                        + rand_f64() * (FlappyGame::GAP_MAX_Y - FlappyGame::GAP_MIN_Y),
                );
                let next_score = next.score;
                let next_game_over = next.game_over;
                game.set(next);
                score.set(next_score);
                if next_game_over {
                    status.set(format!("Game over — score {}", next_score));
                }
                *accumulated -= 1.0 / 60.0;
            }
            drop(accumulated);

            animation_time.set(now);
            if !game.get_untracked().running {
                animation_frame.set(None);
                frame_render();
                return;
            }

            frame_render();
            let request_id = {
                let callback_ref = callback_ref.borrow();
                callback_ref.as_ref().and_then(|cb| {
                    frame_window
                        .request_animation_frame(cb.as_ref().unchecked_ref())
                        .ok()
                })
            };
            if let Some(id) = request_id {
                animation_frame.set(Some(id));
            }
        }) as Box<dyn FnMut(f64)>);

        *callback.borrow_mut() = Some(frame);
        let request_id = {
            let callback_ref = callback.borrow();
            callback_ref.as_ref().and_then(|cb| {
                window()
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .ok()
            })
        };
        if let Some(id) = request_id {
            animation_frame.set(Some(id));
        }
    });

    on_cleanup(move || {
        if let Some(id) = animation_frame.get_untracked() {
            let _ = window().cancel_animation_frame(id);
            animation_frame.set(None);
        }
    });

    let flap_render = Rc::clone(&render);
    let flap_start = Rc::clone(&start_loop);
    let flap = Rc::new(move || {
        let mut next = game.get_untracked();
        next.flap();
        game.set(next);
        score.set(game.get_untracked().score);
        status.set("Flying".into());
        flap_render();
        flap_start();
    });

    let key_flap = Rc::clone(&flap);
    bind_keys(move |e: web_sys::KeyboardEvent| {
        if is_text_input(&e) {
            return;
        }
        if matches!(e.key().as_str(), " " | "ArrowUp" | "w" | "W") {
            e.prevent_default();
            if !e.repeat() {
                key_flap();
            }
        }
    });

    let reset_stop = Rc::clone(&stop_loop);
    let reset_render = Rc::clone(&render);
    let reset = move || {
        reset_stop();
        game.set(FlappyGame::new(300.0));
        score.set(0);
        status.set("Ready".into());
        animation_time.set(0.0);
        reset_render();
    };

    let initial_render = Rc::clone(&render);
    Effect::new(move |_| {
        let _ = canvas_ref.get();
        initial_render();
    });

    view! {
        <div class="mx-auto w-full max-w-md space-y-3">
            <div class="position-relative mx-auto overflow-hidden rounded-3 border border-secondary shadow-sm flappy-stage">
                <canvas
                    node_ref=canvas_ref
                    width="400"
                    height="600"
                    class="d-block w-100 flappy-canvas"
                    aria-label="Flappy game canvas. Press Space, ArrowUp, or tap the game to flap."
                    role="img"
                    on:pointerdown={
                        let pointer_flap = Rc::clone(&flap);
                        move |ev: web_sys::PointerEvent| {
                            ev.prevent_default();
                            pointer_flap();
                        }
                    }
                >
                    "Flappy game. Use Space, ArrowUp, or tap to flap. Avoid the pipes and ground."
                </canvas>
            </div>
            <div class="d-flex flex-wrap justify-content-center align-items-center gap-2">
                <button
                    type="button"
                    class="btn btn-primary btn-sm px-4"
                    on:click={
                        let button_flap = Rc::clone(&flap);
                        move |_| button_flap()
                    }
                    title="Flap the bird"
                >
                    <span class="mr-1" aria-hidden="true">"↗"</span>
                    "Flap"
                </button>
                <button
                    type="button"
                    class="btn btn-outline-secondary btn-sm"
                    on:click=move |_| reset()
                    title="Reset Flappy"
                >
                    <span class="mr-1" aria-hidden="true">"↻"</span>
                    "Reset"
                </button>
            </div>
            <p class="mb-0 text-center text-body-secondary small">
                "Space / ↑ / W / tap to flap · avoid pipes and the ground"
            </p>
        </div>
    }
    .into_any()
}

fn canvas_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|value| value.dyn_into::<CanvasRenderingContext2d>().ok())
}

fn draw_flappy(context: &CanvasRenderingContext2d, game: &FlappyGame, time: f64) {
    let width = FlappyGame::WIDTH;
    let height = FlappyGame::HEIGHT;

    context.set_fill_style_str("#87CEEB");
    context.fill_rect(0.0, 0.0, width, height);

    context.set_fill_style_str("#BFE8F7");
    for (x, y, radius) in [
        (55.0, 90.0, 28.0),
        (315.0, 125.0, 22.0),
        (235.0, 60.0, 18.0),
    ] {
        context.begin_path();
        let _ = context.arc(x, y, radius, 0.0, std::f64::consts::TAU);
        context.fill();
    }

    for pipe in &game.pipes {
        let gap_top = pipe.gap_y - FlappyGame::PIPE_GAP * 0.5;
        let gap_bottom = pipe.gap_y + FlappyGame::PIPE_GAP * 0.5;
        draw_pipe(context, pipe.x, 0.0, FlappyGame::PIPE_WIDTH, gap_top, true);
        draw_pipe(
            context,
            pipe.x,
            gap_bottom,
            FlappyGame::PIPE_WIDTH,
            height - gap_bottom,
            false,
        );
    }

    context.set_fill_style_str("#D9A441");
    context.fill_rect(0.0, height - 48.0, width, 48.0);
    context.set_fill_style_str("#A8792E");
    for x in (0..width as usize).step_by(24) {
        context.fill_rect(x as f64, height - 48.0, 12.0, 5.0);
    }

    let rotation = (game.bird_velocity / 620.0).clamp(-0.5, 1.0);
    let wing = (time / 90.0).sin() * 5.0;
    context.save();
    let _ = context.translate(FlappyGame::BIRD_X, game.bird_y);
    let _ = context.rotate(rotation);
    context.set_fill_style_str("#F6D365");
    context.begin_path();
    let _ = context.ellipse(0.0, 0.0, 18.0, 14.0, 0.0, 0.0, std::f64::consts::TAU);
    context.fill();

    context.set_fill_style_str("#E9B949");
    context.begin_path();
    let _ = context.ellipse(-3.0, 6.0 + wing, 10.0, 5.0, 0.0, 0.0, std::f64::consts::TAU);
    context.fill();

    context.set_fill_style_str("#F28C28");
    context.begin_path();
    context.move_to(15.0, -2.0);
    context.line_to(28.0, 3.0);
    context.line_to(15.0, 7.0);
    context.close_path();
    context.fill();

    context.set_fill_style_str("#FFFFFF");
    context.begin_path();
    let _ = context.arc(7.0, -6.0, 5.0, 0.0, std::f64::consts::TAU);
    context.fill();
    context.set_fill_style_str("#343A40");
    context.begin_path();
    let _ = context.arc(8.5, -6.0, 2.0, 0.0, std::f64::consts::TAU);
    context.fill();
    context.restore();

    context.set_text_align("center");
    context.set_text_baseline("top");
    context.set_font("700 44px Inter, sans-serif");
    context.set_fill_style_str("#FFFFFF");
    context.set_shadow_color("rgba(52,58,64,0.55)");
    context.set_shadow_blur(4.0);
    let _ = context.fill_text(&game.score.to_string(), width * 0.5, 20.0);
    context.set_shadow_blur(0.0);

    if !game.running || game.game_over {
        context.set_fill_style_str("rgba(33,37,41,0.45)");
        context.fill_rect(0.0, 0.0, width, height - 48.0);
        context.set_font("700 28px Inter, sans-serif");
        context.set_fill_style_str("#FFFFFF");
        context.set_text_baseline("middle");
        let message = if game.game_over {
            "Game over — tap to retry"
        } else {
            "Tap or press Space to start"
        };
        let _ = context.fill_text(message, width * 0.5, height * 0.5);
    }
}

fn draw_pipe(
    context: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    top: bool,
) {
    if height <= 0.0 {
        return;
    }

    context.set_fill_style_str("#5FAF3D");
    context.fill_rect(x, y, width, height);

    context.set_fill_style_str("#7CCB57");
    context.fill_rect(x + 7.0, y, 9.0, height);

    let cap_height = 24.0;
    let cap_y = if top { height - cap_height } else { y };
    context.set_fill_style_str("#4C9631");
    context.fill_rect(x - 5.0, cap_y, width + 10.0, cap_height);

    context.set_stroke_style_str("#376E27");
    context.set_line_width(2.0);
    context.stroke_rect(x, y, width, height);
    context.stroke_rect(x - 5.0, cap_y, width + 10.0, cap_height);
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
            {dpad(
                rotate,
                move || move_piece(-1, 0),
                move || move_piece(0, 1),
                move || move_piece(1, 0),
            )}
            <p class="text-center text-xs text-[var(--text-tertiary)]">"Space start/pause · ↑ rotate · arrows / WASD"</p>
        </div>
    }.into_any()
}

// ── Chess ──────────────────────────────────────────────────────────────────────

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
        <div class="mx-auto w-full max-w-2xl space-y-4">
            <div class="chess-board mx-auto w-[min(92vw,42rem)] max-w-full overflow-hidden rounded-xl border-4 shadow-lg">
                {(0..64).map(|i| {
                    let row = i / 8;
                    let col = i % 8;
                    let is_light = (row + col) % 2 == 0;
                    let square_name = format!(
                        "{}{}",
                        (b'a' + col as u8) as char,
                        8 - row
                    );

                    view! {
                        <button
                            type="button"
                            class=if is_light {
                                "chess-square chess-square--light"
                            } else {
                                "chess-square chess-square--dark"
                            }
                            class=("chess-square--selected", move || selected.get() == Some(i))
                            class=("chess-square--legal", move || {
                                selected.get().is_some_and(|from| {
                                    chess_legal_moves(&board.get(), from).contains(&i)
                                })
                            })
                            on:click=move |_| click(i)
                            aria-pressed=move || selected.get() == Some(i)
                            aria-label=move || format!(
                                "{} {}",
                                square_name,
                                match board.get()[i] {
                                    0 => "empty square",
                                    p if p > 0 => "white piece",
                                    _ => "black piece",
                                }
                            )
                        >
                            <span
                                class="chess-piece"
                                class=("chess-piece--light", move || board.get()[i] > 0)
                                class=("chess-piece--dark", move || board.get()[i] < 0)
                            >
                                {move || chess_glyph(board.get()[i])}
                            </span>
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="flex flex-wrap items-center justify-center gap-2">
                <p
                    class="text-center text-xs text-[var(--text-tertiary)]"
                    aria-live="polite"
                >
                    "Select a white piece, then choose a highlighted square."
                </p>
                <button
                    type="button"
                    class="min-h-11 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| {
                        board.set(chess_start());
                        selected.set(None);
                        game_over.set(false);
                        busy.set(false);
                        score.set(0);
                        status.set("White to move".into());
                    }
                >
                    "New Game"
                </button>
            </div>
        </div>
    }.into_any()
}

fn localized_game_title(game: GameKind) -> String {
    let i18n = use_i18n();
    if i18n.get_locale() == Locale::en {
        return game.title().into();
    }
    match game {
        GameKind::TwentyFortyEight => "2048".into(),
        GameKind::TicTacToe => "Cờ Caro".into(),
        GameKind::Minesweeper => "Dò mìn".into(),
        GameKind::Snake => "Rắn".into(),
        GameKind::Sudoku => "Sudoku".into(),
        GameKind::ConnectFour => "Kết nối 4".into(),
        GameKind::Memory => "Thẻ nhớ".into(),
        GameKind::Typing => "Tốc độ gõ".into(),
        GameKind::Wordle => "Wordle".into(),
        GameKind::Hangman => "Đoán chữ".into(),
        GameKind::FifteenPuzzle => "Xếp hình 15".into(),
        GameKind::LightsOut => "Tắt đèn".into(),
        GameKind::Breakout => "Phá gạch".into(),
        GameKind::Pong => "Pong".into(),
        GameKind::Flappy => "Flappy".into(),
        GameKind::Tetris => "Tetris".into(),
        GameKind::Chess => "Cờ vua".into(),
        GameKind::Checkers => "Cờ đam".into(),
        GameKind::Blackjack => "Blackjack".into(),
    }
}

fn localized_game_description(game: GameKind) -> String {
    let i18n = use_i18n();
    if i18n.get_locale() == Locale::en {
        return game.description().into();
    }
    match game {
        GameKind::TwentyFortyEight => "Ghép các ô để đạt 2048.".into(),
        GameKind::TicTacToe => "Đấu với AI với bàn cờ tùy chọn từ 3×3 đến 6×6.".into(),
        GameKind::Minesweeper => "Mở các ô an toàn và tránh mìn.".into(),
        GameKind::Snake => "Ăn mồi, lớn lên và tránh tường.".into(),
        GameKind::Sudoku => "Hoàn thành bảng logic mà không lặp số.".into(),
        GameKind::ConnectFour => "Xếp bốn quân liên tiếp trước AI.".into(),
        GameKind::Memory => "Tìm tất cả các cặp giống nhau.".into(),
        GameKind::Typing => "Gõ từ nhanh nhất có thể.".into(),
        GameKind::Wordle => "Đoán từ 5 chữ cái trong 6 lượt.".into(),
        GameKind::Hangman => "Đoán từ trước khi hình người hoàn tất.".into(),
        GameKind::FifteenPuzzle => "Trượt các ô về đúng thứ tự số.".into(),
        GameKind::LightsOut => "Tắt tất cả các đèn.".into(),
        GameKind::Breakout => "Phá tất cả các khối bằng quả bóng.".into(),
        GameKind::Pong => "Giữ bóng không đi qua phía của bạn.".into(),
        GameKind::Flappy => "Đi qua các khoảng trống bằng những cú nhảy đúng lúc.".into(),
        GameKind::Tetris => "Xóa các hàng bằng những khối rơi.".into(),
        GameKind::Chess => "Chơi một bàn cờ vua cục bộ nhẹ.".into(),
        GameKind::Checkers => "Ăn quân trên bàn cờ đam.".into(),
        GameKind::Blackjack => "Đánh bại nhà cái mà không vượt quá 21.".into(),
    }
}
