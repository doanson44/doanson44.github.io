use leptos::prelude::*;

use crate::domain::futures::TrackedFuturesTicker;
use crate::features::socket::state::{SocketState, SocketViewMode};
use crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus;

/// Realtime MEXC Futures ticker monitor page.
#[component]
pub fn SocketPage() -> impl IntoView {
    let state = SocketState::new();
    let visible = Memo::new({
        let tickers = state.tickers;
        let view_mode = state.view_mode;
        let ticker_limit = state.ticker_limit;
        let pinned_slots = state.pinned_slots;
        move |_| build_visible(tickers.get(), view_mode.get(), ticker_limit.get(), pinned_slots.get())
    });

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-hidden socket-page">
            <div class="container-fluid py-3 d-flex flex-column flex-grow-1 overflow-hidden">
                <header class="d-flex flex-wrap justify-content-between align-items-center gap-2 mb-3 flex-shrink-0">
                    <div>
                        <h2 class="mb-1">
                            <i class="bi bi-broadcast me-2 text-primary"></i>
                            "MEXC Futures"
                        </h2>
                        <div class="small text-body-secondary">
                            "Realtime market momentum from the moment this page opens"
                        </div>
                    </div>
                    <div class="small">
                        {move || status_badge(state.connection_status.get())}
                    </div>
                </header>

                <div class="d-flex flex-wrap justify-content-between align-items-center gap-2 mb-3 flex-shrink-0">
                    <div class="btn-group" role="group" aria-label="Ticker view">
                        <button
                            class=move || view_button_class(state.view_mode.get() == SocketViewMode::All)
                            type="button"
                            aria-pressed=move || (state.view_mode.get() == SocketViewMode::All).to_string()
                            on:click=move |_| state.view_mode.set(SocketViewMode::All)
                        >
                            <i class="bi bi-grid-3x3-gap me-1"></i>
                            "All"
                        </button>
                        <button
                            class=move || view_button_class(state.view_mode.get() == SocketViewMode::PinnedOnly)
                            type="button"
                            aria-pressed=move || (state.view_mode.get() == SocketViewMode::PinnedOnly).to_string()
                            on:click=move |_| state.view_mode.set(SocketViewMode::PinnedOnly)
                        >
                            <i class="bi bi-pin-angle me-1"></i>
                            "Pinned only"
                        </button>
                    </div>

                    <div class="d-flex align-items-center gap-2">
                        <label class="small text-body-secondary" for="socket-ticker-limit">"Show"</label>
                        <select
                            id="socket-ticker-limit"
                            class="form-select form-select-sm socket-limit-select"
                            aria-label="Number of dynamic tickers to show"
                            prop:value=move || state.ticker_limit.get().to_string()
                            on:change=move |ev| {
                                let value = event_target_value(&ev).parse::<usize>().unwrap_or(10);
                                state.set_ticker_limit(value);
                            }
                        >
                            {SocketState::limit_options().iter().map(|value| view! {
                                <option value=value.to_string()>{value}</option>
                            }).collect_view()}
                        </select>
                        <span class="small text-body-secondary">"dynamic tickers"</span>
                    </div>
                </div>

                <div class="socket-grid flex-grow-1 overflow-auto pe-1" aria-live="polite">
                    <Show
                        when=move || !visible.get().is_empty()
                        fallback=move || empty_state(state.view_mode.get())
                    >
                        <For
                            each=move || visible.get().iter().map(|ticker| ticker.ticker.symbol.clone()).collect::<Vec<_>>()
                            key=|symbol| symbol.clone()
                            children=move |symbol| {
                                let ticker = Memo::new({
                                    let tickers = state.tickers;
                                    let symbol = symbol.clone();
                                    move |_| tickers.get().into_iter().find(|item| item.ticker.symbol == symbol)
                                });
                                let index = Memo::new({
                                    let visible = visible;
                                    let symbol = symbol.clone();
                                    move |_| visible.get().iter().position(|item| item.ticker.symbol == symbol).unwrap_or(0)
                                });
                                view! {
                                    <TickerCard ticker=ticker index=index state=state />
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TickerCard(
    ticker: Memo<Option<TrackedFuturesTicker>>,
    index: Memo<usize>,
    state: SocketState,
) -> impl IntoView {
    let symbol = Memo::new({
        let ticker = ticker;
        move |_| ticker.get().map(|item| item.ticker.symbol).unwrap_or_default()
    });
    let is_pinned = Memo::new({
        let symbol = symbol;
        let pinned_slots = state.pinned_slots;
        move |_| pinned_slots.get().iter().any(|slot| slot.as_deref() == Some(symbol.get().as_str()))
    });

    view! {
        <button
            type="button"
            class=move || if is_pinned.get() {
                "socket-ticker-card card bg-body-tertiary border-primary-subtle socket-ticker-card-pinned"
            } else {
                "socket-ticker-card card bg-body-tertiary border-secondary"
            }
            title=move || if is_pinned.get() { format!("Unpin {}", symbol.get()) } else { format!("Pin {}", symbol.get()) }
            aria-label=move || card_aria_label(ticker.get(), is_pinned.get())
            on:click=move |_| state.toggle_pin(&symbol.get_untracked(), index.get_untracked())
        >
            <div class="card-body p-2 d-flex flex-column min-h-0">
                <div class="d-flex align-items-start justify-content-between gap-2">
                    <span class="font-monospace fw-semibold text-truncate">{move || symbol.get()}</span>
                    <i class=move || if is_pinned.get() { "bi bi-pin-angle-fill text-primary" } else { "bi bi-pin-angle text-body-tertiary" } aria-hidden="true"></i>
                </div>

                <div class="socket-ticker-price font-monospace mt-1 text-truncate">
                    {move || format_number(ticker.get().and_then(|item| item.ticker.last_price))}
                </div>

                <div class="d-flex justify-content-between align-items-center gap-2 mt-1">
                    <span class=move || change_class(ticker.get().and_then(|item| item.ticker.change_24h))>
                        {move || format_percent(ticker.get().and_then(|item| item.ticker.change_24h))}
                    </span>
                    <span class="small text-body-secondary font-monospace">
                        {move || format!("{}%", ticker.get().map(|item| item.momentum.progress()).unwrap_or(0))}
                    </span>
                </div>

                <progress
                    class="socket-ticker-progress mt-2"
                    max="100"
                    value=move || ticker.get().map(|item| item.momentum.progress()).unwrap_or(0).to_string()
                    aria-label="Directional progress"
                ></progress>

                <div class="d-flex justify-content-between gap-2 mt-auto pt-2 small font-monospace">
                    <span class="text-success-emphasis">{move || format!("↑ {}", ticker.get().map(|item| item.momentum.up_ticks).unwrap_or(0))}</span>
                    <span class="text-danger-emphasis">{move || format!("↓ {}", ticker.get().map(|item| item.momentum.down_ticks).unwrap_or(0))}</span>
                </div>
            </div>
        </button>
    }
}

fn build_visible(
    all: Vec<TrackedFuturesTicker>,
    mode: SocketViewMode,
    limit: usize,
    slots: Vec<Option<String>>,
) -> Vec<TrackedFuturesTicker> {
    let find_ticker = |symbol: &str| all.iter().find(|item| item.ticker.symbol == symbol).cloned();

    if mode == SocketViewMode::PinnedOnly {
        return slots.iter().filter_map(|slot| slot.as_deref().and_then(find_ticker)).collect();
    }

    let pinned_symbols = slots.iter().filter_map(|slot| slot.as_deref()).collect::<Vec<_>>();
    let mut dynamic = all
        .iter()
        .filter(|item| !pinned_symbols.contains(&item.ticker.symbol.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    dynamic.sort_unstable_by(|left, right| right.momentum.progress().cmp(&left.momentum.progress()));
    dynamic.truncate(limit);

    let pinned_count = slots.iter().filter(|slot| slot.is_some()).count();
    let output_len = slots.len().max(dynamic.len() + pinned_count);
    let mut output = vec![None; output_len];
    let mut dynamic_index = 0;

    for index in 0..output_len {
        if let Some(symbol) = slots.get(index).and_then(|slot| slot.as_deref()) {
            output[index] = find_ticker(symbol);
        } else if let Some(ticker) = dynamic.get(dynamic_index) {
            output[index] = Some(ticker.clone());
            dynamic_index += 1;
        }
    }

    output.into_iter().flatten().collect()
}

fn status_badge(status: MexcFuturesConnectionStatus) -> impl IntoView {
    match status {
        MexcFuturesConnectionStatus::Connected => view! {
            <span class="badge bg-success-subtle text-success-emphasis border border-success-subtle">
                <i class="bi bi-wifi me-1"></i>"Connected"
            </span>
        }.into_any(),
        MexcFuturesConnectionStatus::Connecting => view! {
            <span class="badge bg-warning-subtle text-warning-emphasis border border-warning-subtle">
                <i class="bi bi-arrow-repeat me-1"></i>"Connecting"
            </span>
        }.into_any(),
        MexcFuturesConnectionStatus::Reconnecting => view! {
            <span class="badge bg-warning-subtle text-warning-emphasis border border-warning-subtle">
                <i class="bi bi-arrow-repeat me-1"></i>"Reconnecting"
            </span>
        }.into_any(),
        MexcFuturesConnectionStatus::Disconnected => view! {
            <span class="badge bg-secondary-subtle text-secondary-emphasis border border-secondary-subtle">
                "Disconnected"
            </span>
        }.into_any(),
        MexcFuturesConnectionStatus::Error(message) => view! {
            <span class="badge bg-danger-subtle text-danger-emphasis border border-danger-subtle" title=message>
                <i class="bi bi-exclamation-triangle me-1"></i>"Connection error"
            </span>
        }.into_any(),
    }
}

fn empty_state(mode: SocketViewMode) -> impl IntoView {
    let text = match mode {
        SocketViewMode::All => "Waiting for market data...",
        SocketViewMode::PinnedOnly => "No pinned tickers",
    };
    view! {
        <div class="d-flex flex-column align-items-center justify-content-center h-100 text-body-secondary py-5">
            <i class="bi bi-grid-3x3-gap fs-2 mb-2" aria-hidden="true"></i>
            <span>{text}</span>
        </div>
    }
}

fn view_button_class(active: bool) -> &'static str {
    if active { "btn btn-primary btn-sm" } else { "btn btn-outline-secondary btn-sm" }
}

fn card_aria_label(ticker: Option<TrackedFuturesTicker>, pinned: bool) -> String {
    let Some(ticker) = ticker else { return "Ticker unavailable".into() };
    format!(
        "{}, 24 hour change {}, price {}, {} up ticks, {} down ticks, {} percent progress, {}",
        ticker.ticker.symbol,
        format_percent(ticker.ticker.change_24h),
        format_number(ticker.ticker.last_price),
        ticker.momentum.up_ticks,
        ticker.momentum.down_ticks,
        ticker.momentum.progress(),
        if pinned { "pinned" } else { "not pinned" }
    )
}

fn change_class(value: Option<f64>) -> &'static str {
    match value {
        Some(value) if value > 0.0 => "text-success-emphasis",
        Some(value) if value < 0.0 => "text-danger-emphasis",
        _ => "text-body",
    }
}

fn format_number(value: Option<f64>) -> String {
    value
        .map(|number| {
            if number.abs() >= 1.0 {
                format!("{number:.4}")
            } else {
                format!("{number:.8}")
            }
        })
        .unwrap_or_else(|| "—".into())
}

fn format_percent(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:+.2}%", number = number * 100.0))
        .unwrap_or_else(|| "—".into())
}
