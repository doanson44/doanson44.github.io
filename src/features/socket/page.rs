use std::{collections::HashMap, rc::Rc};

use leptos::prelude::*;

use crate::application::ports::{FuturesConnectionStatus, FuturesMarketStream};
use crate::domain::futures::TrackedFuturesTicker;
use crate::features::socket::state::{
    SocketChangeFilter, SocketFairPriceFilter, SocketMomentumFilter, SocketSortMode,
    SocketState, SocketViewMode, SocketVolumeFilter,
};

/// Realtime MEXC Futures ticker monitor page.
#[component]
pub fn SocketPage(stream: Rc<dyn FuturesMarketStream>) -> impl IntoView {
    let state = SocketState::new(stream);
    let visible = Memo::new({
        let tickers = state.tickers;
        let view_mode = state.view_mode;
        let sort_mode = state.sort_mode;
        let change_filter = state.change_filter;
        let momentum_filter = state.momentum_filter;
        let fair_price_filter = state.fair_price_filter;
        let volume_filter = state.volume_filter;
        let ticker_limit = state.ticker_limit;
        let pinned_slots = state.pinned_slots;
        move |_| {
            build_visible(
                tickers.get(),
                view_mode.get(),
                sort_mode.get(),
                change_filter.get(),
                momentum_filter.get(),
                fair_price_filter.get(),
                volume_filter.get(),
                ticker_limit.get(),
                pinned_slots.get(),
            )
        }
    });

    let clear_filters = move |_| {
        state.change_filter.set(SocketChangeFilter::All);
        state.momentum_filter.set(SocketMomentumFilter::All);
        state.fair_price_filter.set(SocketFairPriceFilter::All);
        state.volume_filter.set(SocketVolumeFilter::All);
    };

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

                <div class="d-flex flex-column gap-2 mb-3 flex-shrink-0">
                    <div class="d-flex flex-wrap align-items-center gap-2">
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

                        <div class="d-flex align-items-center gap-2 ms-md-auto">
                            <label class="small text-body-secondary" for="socket-sort-mode">"Sort by"</label>
                            <select
                                id="socket-sort-mode"
                                class="form-select form-select-sm socket-sort-select"
                                aria-label="Sort tickers by"
                                prop:value=move || match state.sort_mode.get() {
                                    SocketSortMode::Momentum => "momentum",
                                    SocketSortMode::TotalTicks => "activity",
                                }
                                on:change=move |ev| {
                                    if event_target_value(&ev) == "activity" {
                                        state.sort_mode.set(SocketSortMode::TotalTicks);
                                    } else {
                                        state.sort_mode.set(SocketSortMode::Momentum);
                                    }
                                }
                            >
                                <option value="momentum">"Momentum"</option>
                                <option value="activity">"Total Ticks"</option>
                            </select>
                        </div>

                        <div class="d-flex align-items-center gap-2">
                            <label class="small text-body-secondary" for="socket-ticker-limit">"Show"</label>
                            <select
                                id="socket-ticker-limit"
                                class="form-select form-select-sm socket-limit-select"
                                aria-label="Number of dynamic tickers to show"
                                prop:value=move || state.ticker_limit.get().to_string()
                                on:change=move |ev| {
                                    let value = event_target_value(&ev).parse::<usize>().unwrap_or(DEFAULT_LIMIT);
                                    state.set_ticker_limit(value);
                                }
                            >
                                {SocketState::limit_options().iter().map(|value| {
                                    let label = if *value == usize::MAX { "All".to_string() } else { value.to_string() };
                                    view! { <option value=value.to_string()>{label}</option> }
                                }).collect_view()}
                            </select>
                            <span class="small text-body-secondary">"dynamic"</span>
                        </div>
                    </div>

                    <div class="card bg-body-tertiary border-secondary">
                        <div class="card-body p-2">
                            <div class="d-flex flex-wrap align-items-end gap-2">
                                <div class="small fw-semibold text-body-secondary align-self-center me-1">
                                    <i class="bi bi-funnel me-1"></i>
                                    "Filters"
                                </div>

                                <div>
                                    <label class="form-label small text-body-secondary mb-1" for="socket-change-filter">"24h Change"</label>
                                    <select
                                        id="socket-change-filter"
                                        class="form-select form-select-sm"
                                        aria-label="Filter by 24 hour price change"
                                        prop:value=move || match state.change_filter.get() {
                                            SocketChangeFilter::All => "all",
                                            SocketChangeFilter::Positive => "positive",
                                            SocketChangeFilter::Negative => "negative",
                                        }
                                        on:change=move |ev| state.change_filter.set(match event_target_value(&ev).as_str() {
                                            "positive" => SocketChangeFilter::Positive,
                                            "negative" => SocketChangeFilter::Negative,
                                            _ => SocketChangeFilter::All,
                                        })
                                    >
                                        <option value="all">"All"</option>
                                        <option value="positive">"Positive"</option>
                                        <option value="negative">"Negative"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="form-label small text-body-secondary mb-1" for="socket-momentum-filter">"Momentum"</label>
                                    <select
                                        id="socket-momentum-filter"
                                        class="form-select form-select-sm"
                                        aria-label="Filter by momentum direction"
                                        prop:value=move || match state.momentum_filter.get() {
                                            SocketMomentumFilter::All => "all",
                                            SocketMomentumFilter::Bullish => "bullish",
                                            SocketMomentumFilter::Bearish => "bearish",
                                        }
                                        on:change=move |ev| state.momentum_filter.set(match event_target_value(&ev).as_str() {
                                            "bullish" => SocketMomentumFilter::Bullish,
                                            "bearish" => SocketMomentumFilter::Bearish,
                                            _ => SocketMomentumFilter::All,
                                        })
                                    >
                                        <option value="all">"All"</option>
                                        <option value="bullish">"Bullish"</option>
                                        <option value="bearish">"Bearish"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="form-label small text-body-secondary mb-1" for="socket-fair-price-filter">"Fair Price"</label>
                                    <select
                                        id="socket-fair-price-filter"
                                        class="form-select form-select-sm"
                                        aria-label="Filter by last price relative to fair price"
                                        prop:value=move || match state.fair_price_filter.get() {
                                            SocketFairPriceFilter::All => "all",
                                            SocketFairPriceFilter::Above => "above",
                                            SocketFairPriceFilter::Below => "below",
                                        }
                                        on:change=move |ev| state.fair_price_filter.set(match event_target_value(&ev).as_str() {
                                            "above" => SocketFairPriceFilter::Above,
                                            "below" => SocketFairPriceFilter::Below,
                                            _ => SocketFairPriceFilter::All,
                                        })
                                    >
                                        <option value="all">"All"</option>
                                        <option value="above">"Above fair"</option>
                                        <option value="below">"Below fair"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="form-label small text-body-secondary mb-1" for="socket-volume-filter">"24h Volume"</label>
                                    <select
                                        id="socket-volume-filter"
                                        class="form-select form-select-sm"
                                        aria-label="Filter by 24 hour volume rank"
                                        prop:value=move || match state.volume_filter.get() {
                                            SocketVolumeFilter::All => "all",
                                            SocketVolumeFilter::TopHalf => "top-half",
                                            SocketVolumeFilter::TopQuarter => "top-quarter",
                                        }
                                        on:change=move |ev| state.volume_filter.set(match event_target_value(&ev).as_str() {
                                            "top-half" => SocketVolumeFilter::TopHalf,
                                            "top-quarter" => SocketVolumeFilter::TopQuarter,
                                            _ => SocketVolumeFilter::All,
                                        })
                                    >
                                        <option value="all">"All"</option>
                                        <option value="top-half">"Top 50%"</option>
                                        <option value="top-quarter">"Top 25%"</option>
                                    </select>
                                </div>

                                <button
                                    class="btn btn-outline-secondary btn-sm"
                                    type="button"
                                    disabled=move || filter_count(state) == 0
                                    on:click=clear_filters
                                >
                                    "Clear"
                                </button>
                                <span class="small text-body-secondary ms-auto" aria-live="polite">
                                    {move || format!("{} active", filter_count(state))}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="socket-grid flex-grow-1 overflow-auto pe-1" aria-live="polite">
                    <Show
                        when=move || !visible.get().is_empty()
                        fallback=move || empty_state(state.view_mode.get(), filter_count(state) > 0)
                    >
                        <For
                            each=move || visible.get()
                            key=|ticker| ticker.ticker.symbol.clone()
                            children=move |ticker| {
                                view! {
                                    <TickerCard
                                        ticker=ticker
                                        state=state
                                        visible=visible
                                    />
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
        </div>
    }
}

const DEFAULT_LIMIT: usize = 10;

type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

#[component]
fn TickerCard(
    ticker: TrackedFuturesTicker,
    state: SocketState,
    visible: Memo<Vec<TrackedFuturesTicker>>,
) -> impl IntoView {
    let symbol = ticker.ticker.symbol.clone();
    let ticker = Memo::new({
        let tickers = state.tickers;
        let symbol = symbol.clone();
        move |_| tickers.get().get(&symbol).cloned()
    });
    let is_pinned = Memo::new({
        let pinned_slots = state.pinned_slots;
        let symbol = symbol.clone();
        move |_| {
            pinned_slots
                .get()
                .iter()
                .any(|slot| slot.as_deref() == Some(symbol.as_str()))
        }
    });

    view! {
        <button
            type="button"
            class=move || if is_pinned.get() {
                "socket-ticker-card card bg-body-tertiary border-primary-subtle socket-ticker-card-pinned"
            } else {
                "socket-ticker-card card bg-body-tertiary border-secondary"
            }
            title={let symbol_title = symbol.clone(); move || if is_pinned.get() { format!("Unpin {symbol_title}") } else { format!("Pin {symbol_title}") }}
            aria-label={let symbol_aria = symbol.clone(); move || ticker.get().map(|item| card_aria_label(item, is_pinned.get())).unwrap_or_else(|| format!("{symbol_aria}, market data unavailable"))}
            on:click={
                let symbol = symbol.clone();
                move |_| {
                    let index = visible
                        .get_untracked()
                        .iter()
                        .position(|item| item.ticker.symbol == symbol)
                        .unwrap_or(0);
                    state.toggle_pin(&symbol, index);
                }
            }
        >
            <div class="card-body p-2 d-flex flex-column min-h-0">
                <div class="d-flex align-items-start justify-content-between gap-2">
                    <span class="font-monospace fw-semibold text-truncate">{symbol.clone()}</span>
                    <i class=move || if is_pinned.get() { "bi bi-pin-angle-fill text-primary" } else { "bi bi-pin-angle text-body-tertiary" } aria-hidden="true"></i>
                </div>

                <div class="socket-ticker-price font-monospace mt-1 text-truncate">
                    {move || ticker.get().map(|item| format_number(item.ticker.last_price)).unwrap_or_else(|| "—".into())}
                </div>

                <div class="d-flex justify-content-between align-items-center gap-2 mt-1">
                    {move || {
                        ticker
                            .get()
                            .map(|item| view! {
                                <span class=change_class(item.ticker.change_24h)>
                                    {format_percent(item.ticker.change_24h)}
                                </span>
                            })
                            .unwrap_or_else(|| view! {
                                <span class="text-body-secondary">{"—".to_string()}</span>
                            })
                    }}
                    <span class="small text-body-secondary font-monospace">
                        {move || ticker.get().map(|item| format!("{}%", item.momentum.progress())).unwrap_or_else(|| "—".into())}
                    </span>
                </div>

                <progress
                    class="socket-ticker-progress mt-2"
                    max="100"
                    value=move || ticker.get().map(|item| item.momentum.progress().to_string()).unwrap_or_else(|| "0".into())
                    aria-label="Directional progress"
                ></progress>

                <div class="d-flex justify-content-between gap-2 mt-auto pt-2 small font-monospace">
                    <span class="text-success-emphasis">
                        {move || ticker.get().map(|item| format!("↑ {}", item.momentum.up_ticks)).unwrap_or_else(|| "↑ 0".into())}
                    </span>
                    <span class="text-danger-emphasis">
                        {move || ticker.get().map(|item| format!("↓ {}", item.momentum.down_ticks)).unwrap_or_else(|| "↓ 0".into())}
                    </span>
                </div>
            </div>
        </button>
    }
}

fn build_visible(
    all: MarketSnapshot,
    mode: SocketViewMode,
    sort: SocketSortMode,
    change_filter: SocketChangeFilter,
    momentum_filter: SocketMomentumFilter,
    fair_price_filter: SocketFairPriceFilter,
    volume_filter: SocketVolumeFilter,
    limit: usize,
    slots: Vec<Option<String>>,
) -> Vec<TrackedFuturesTicker> {
    let volume_threshold = volume_threshold(&all, volume_filter);
    let pinned_symbols = slots
        .iter()
        .filter_map(|slot| slot.as_deref())
        .collect::<Vec<_>>();

    let matches_filters = |item: &TrackedFuturesTicker| {
        matches_change(item, change_filter)
            && matches_momentum(item, momentum_filter)
            && matches_fair_price(item, fair_price_filter)
            && matches_volume(item, volume_filter, volume_threshold)
    };

    if mode == SocketViewMode::PinnedOnly {
        return slots
            .iter()
            .filter_map(|slot| slot.as_deref().and_then(|symbol| all.get(symbol)))
            .filter(|item| matches_filters(item))
            .cloned()
            .collect();
    }

    let mut dynamic = all
        .values()
        .filter(|item| !pinned_symbols.contains(&item.ticker.symbol.as_str()))
        .filter(|item| matches_filters(item))
        .cloned()
        .collect::<Vec<_>>();

    dynamic.sort_unstable_by(|left, right| match sort {
        SocketSortMode::Momentum => right
            .momentum
            .progress()
            .cmp(&left.momentum.progress())
            .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)),
        SocketSortMode::TotalTicks => {
            let left_total = left.momentum.up_ticks + left.momentum.down_ticks;
            let right_total = right.momentum.up_ticks + right.momentum.down_ticks;
            right_total
                .cmp(&left_total)
                .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
        }
    });
    dynamic.truncate(limit);

    let pinned_count = slots.iter().filter(|slot| slot.is_some()).count();
    let output_len = slots.len().max(dynamic.len() + pinned_count);
    let mut output = Vec::with_capacity(output_len);
    let mut dynamic_index = 0;

    for index in 0..output_len {
        if let Some(symbol) = slots.get(index).and_then(|slot| slot.as_deref()) {
            if let Some(ticker) = all.get(symbol) {
                if matches_filters(ticker) {
                    output.push(ticker.clone());
                }
            }
        } else if let Some(ticker) = dynamic.get(dynamic_index) {
            output.push(ticker.clone());
            dynamic_index += 1;
        }
    }

    output
}

fn matches_change(ticker: &TrackedFuturesTicker, filter: SocketChangeFilter) -> bool {
    match filter {
        SocketChangeFilter::All => true,
        SocketChangeFilter::Positive => ticker.ticker.change_24h.is_some_and(|value| value > 0.0),
        SocketChangeFilter::Negative => ticker.ticker.change_24h.is_some_and(|value| value < 0.0),
    }
}

fn matches_momentum(ticker: &TrackedFuturesTicker, filter: SocketMomentumFilter) -> bool {
    match filter {
        SocketMomentumFilter::All => true,
        SocketMomentumFilter::Bullish => ticker.momentum.net_ticks() > 0,
        SocketMomentumFilter::Bearish => ticker.momentum.net_ticks() < 0,
    }
}

fn matches_fair_price(ticker: &TrackedFuturesTicker, filter: SocketFairPriceFilter) -> bool {
    match filter {
        SocketFairPriceFilter::All => true,
        SocketFairPriceFilter::Above => match (ticker.ticker.last_price, ticker.ticker.fair_price) {
            (Some(last), Some(fair)) => last > fair,
            _ => false,
        },
        SocketFairPriceFilter::Below => match (ticker.ticker.last_price, ticker.ticker.fair_price) {
            (Some(last), Some(fair)) => last < fair,
            _ => false,
        },
    }
}

fn volume_threshold(all: &MarketSnapshot, filter: SocketVolumeFilter) -> Option<f64> {
    if filter == SocketVolumeFilter::All {
        return None;
    }

    let mut volumes = all
        .values()
        .filter_map(|item| item.ticker.volume_24h)
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    if volumes.is_empty() {
        return None;
    }

    volumes.sort_unstable_by(f64::total_cmp);
    let rank = match filter {
        SocketVolumeFilter::All => return None,
        SocketVolumeFilter::TopHalf => 0.5,
        SocketVolumeFilter::TopQuarter => 0.75,
    };
    let index = ((volumes.len() - 1) as f64 * rank).floor() as usize;
    volumes.get(index).copied()
}

fn matches_volume(
    ticker: &TrackedFuturesTicker,
    filter: SocketVolumeFilter,
    threshold: Option<f64>,
) -> bool {
    match filter {
        SocketVolumeFilter::All => true,
        SocketVolumeFilter::TopHalf | SocketVolumeFilter::TopQuarter => ticker
            .ticker
            .volume_24h
            .zip(threshold)
            .is_some_and(|(volume, threshold)| volume >= threshold),
    }
}

fn filter_count(state: SocketState) -> usize {
    (state.change_filter.get() != SocketChangeFilter::All) as usize
        + (state.momentum_filter.get() != SocketMomentumFilter::All) as usize
        + (state.fair_price_filter.get() != SocketFairPriceFilter::All) as usize
        + (state.volume_filter.get() != SocketVolumeFilter::All) as usize
}

fn status_badge(status: FuturesConnectionStatus) -> impl IntoView {
    match status {
        FuturesConnectionStatus::Connected => view! {
            <span class="badge bg-success-subtle text-success-emphasis border border-success-subtle">
                <i class="bi bi-wifi me-1"></i>"Connected"
            </span>
        }.into_any(),
        FuturesConnectionStatus::Connecting => view! {
            <span class="badge bg-warning-subtle text-warning-emphasis border border-warning-subtle">
                <i class="bi bi-arrow-repeat me-1"></i>"Connecting"
            </span>
        }.into_any(),
        FuturesConnectionStatus::Reconnecting => view! {
            <span class="badge bg-warning-subtle text-warning-emphasis border border-warning-subtle">
                <i class="bi bi-arrow-repeat me-1"></i>"Reconnecting"
            </span>
        }.into_any(),
        FuturesConnectionStatus::Disconnected => view! {
            <span class="badge bg-secondary-subtle text-secondary-emphasis border border-secondary-subtle">
                "Disconnected"
            </span>
        }.into_any(),
        FuturesConnectionStatus::Error(message) => view! {
            <span class="badge bg-danger-subtle text-danger-emphasis border border-danger-subtle" title=message>
                <i class="bi bi-exclamation-triangle me-1"></i>"Connection error"
            </span>
        }.into_any(),
    }
}

fn empty_state(mode: SocketViewMode, filters_active: bool) -> impl IntoView {
    let text = match (mode, filters_active) {
        (SocketViewMode::All, true) => "No contracts match the active filters",
        (SocketViewMode::All, false) => "Waiting for market data...",
        (SocketViewMode::PinnedOnly, true) => "No pinned tickers match the active filters",
        (SocketViewMode::PinnedOnly, false) => "No pinned tickers",
    };
    view! {
        <div class="d-flex flex-column align-items-center justify-content-center h-100 text-body-secondary py-5">
            <i class="bi bi-funnel fs-2 mb-2" aria-hidden="true"></i>
            <span>{text}</span>
        </div>
    }
}

fn view_button_class(active: bool) -> &'static str {
    if active {
        "btn btn-primary btn-sm"
    } else {
        "btn btn-outline-secondary btn-sm"
    }
}

fn card_aria_label(ticker: TrackedFuturesTicker, pinned: bool) -> String {
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
