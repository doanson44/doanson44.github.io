use std::{collections::HashMap, rc::Rc};

use leptos::prelude::*;

use crate::application::ports::{FundingRateProvider, FuturesConnectionStatus, FuturesMarketStream};
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::TrackedFuturesTicker;
use crate::features::socket::state::{SocketSortDirection, SocketSortMode, SocketState, SocketViewMode};

/// Realtime Futures market ticker monitor page.
#[component]
pub fn SocketPage(stream: Rc<dyn FuturesMarketStream>, funding_provider: Rc<dyn FundingRateProvider>) -> impl IntoView {
    let state = SocketState::new(stream, funding_provider);
    let visible = Memo::new({
        let tickers = state.tickers;
        let view_mode = state.view_mode;
        let sort_mode = state.sort_mode;
        let sort_direction = state.sort_direction;
        let ticker_limit = state.ticker_limit;
        let search_query = state.search_query;
        let pinned_slots = state.pinned_slots;
        let funding_rates = state.funding_rates;
        move |_| build_visible(tickers.get(), view_mode.get(), sort_mode.get(), sort_direction.get(), ticker_limit.get(), pinned_slots.get(), funding_rates.get(), search_query.get())
    });

    view! {
        <div class="flex flex-grow flex-col overflow-hidden socket-page">
            <div class="flex flex-grow flex-col overflow-hidden px-4 py-3">
                <header class="mb-3 flex shrink-0 flex-wrap items-center justify-between gap-2">
                    <div>
                        <h2 class="mb-1 text-xl font-semibold">"Futures Market"</h2>
                        <div class="text-sm text-[var(--text-secondary)]">"Realtime market momentum from the moment this page opens"</div>
                    </div>
                    <div class="text-sm">{move || status_badge(state.connection_status.get())}</div>
                </header>
                <div class="mb-3 flex shrink-0 flex-wrap items-center gap-2">
                    <div class="flex w-full max-w-sm items-center md:mr-auto">
                        <label class="sr-only" for="socket-search">"Search symbol"</label>
                        <input id="socket-search" type="search" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" placeholder="Search symbol..." prop:value=move || state.search_query.get() on:input=move |ev| state.search_query.set(event_target_value(&ev)) />
                    </div>
                    <div class="flex" role="group" aria-label="Ticker view">
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::All) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::All).to_string() on:click=move |_| state.view_mode.set(SocketViewMode::All)>"All"</button>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::PinnedOnly) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::PinnedOnly).to_string() on:click=move |_| state.view_mode.set(SocketViewMode::PinnedOnly)>"Pinned only"</button>
                    </div>
                    <div class="flex items-center gap-2">
                        <label class="text-sm text-[var(--text-secondary)]" for="socket-sort-mode">"Sort by"</label>
                        <select id="socket-sort-mode" class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Sort tickers by" prop:value=move || match state.sort_mode.get() { SocketSortMode::Momentum => "momentum", SocketSortMode::TotalTicks => "activity", SocketSortMode::Funding => "funding", SocketSortMode::Change24h => "change24h", SocketSortMode::Volume24h => "volume24h" } on:change=move |ev| {
                            let val = event_target_value(&ev);
                            state.sort_mode.set(match val.as_str() { "activity" => SocketSortMode::TotalTicks, "funding" => SocketSortMode::Funding, "change24h" => SocketSortMode::Change24h, "volume24h" => SocketSortMode::Volume24h, _ => SocketSortMode::Momentum });
                        }>
                            <option value="momentum">"Momentum"</option><option value="activity">"Total Ticks"</option><option value="funding">"Funding Rate"</option><option value="change24h">"24h Change"</option><option value="volume24h">"24h Volume"</option>
                        </select>
                        <button type="button" class="rounded-md border border-[var(--border-color)] px-2 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title=move || match state.sort_direction.get() { SocketSortDirection::Ascending => "Sort Ascending", SocketSortDirection::Descending => "Sort Descending" } on:click=move |_| state.sort_direction.update(|d| *d = match d { SocketSortDirection::Ascending => SocketSortDirection::Descending, SocketSortDirection::Descending => SocketSortDirection::Ascending })>
                            {move || match state.sort_direction.get() { SocketSortDirection::Ascending => "↑", SocketSortDirection::Descending => "↓" }}
                        </button>
                    </div>
                    <div class="flex items-center gap-2">
                        <label class="text-sm text-[var(--text-secondary)]" for="socket-ticker-limit">"Show"</label>
                        <select id="socket-ticker-limit" class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Number of dynamic tickers to show" prop:value=move || state.ticker_limit.get().to_string() on:change=move |ev| { let value = event_target_value(&ev).parse::<usize>().unwrap_or(DEFAULT_LIMIT); state.set_ticker_limit(value); }>
                            {SocketState::limit_options().iter().map(|value| { let label = if *value == usize::MAX { "All".to_string() } else { value.to_string() }; view! { <option value=value.to_string()>{label}</option> } }).collect_view()}
                        </select>
                        <span class="text-sm text-[var(--text-secondary)]">"dynamic"</span>
                    </div>
                </div>
                <div class="socket-grid flex-grow overflow-auto" aria-live="polite">
                    <Show when=move || !visible.get().is_empty() fallback=move || empty_state(state.view_mode.get())>
                        <For each=move || visible.get() key=|ticker| ticker.ticker.symbol.clone() children=move |ticker| view! { <TickerCard ticker=ticker state=state visible=visible /> } />
                    </Show>
                </div>
            </div>
        </div>
    }
}

const DEFAULT_LIMIT: usize = 10;
type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

#[component]
fn TickerCard(ticker: TrackedFuturesTicker, state: SocketState, visible: Memo<Vec<TrackedFuturesTicker>>) -> impl IntoView {
    let symbol = ticker.ticker.symbol.clone();
    let ticker = Memo::new({ let tickers = state.tickers; let symbol = symbol.clone(); move |_| tickers.get().get(&symbol).cloned() });
    let is_pinned = Memo::new({ let pinned_slots = state.pinned_slots; let symbol = symbol.clone(); move |_| pinned_slots.get().iter().any(|slot| slot.as_deref() == Some(symbol.as_str())) });
    let funding_rate = Memo::new({ let funding_rates = state.funding_rates; let symbol = symbol.clone(); move |_| funding_rates.get().and_then(|snapshot| snapshot.get(&symbol)) });

    view! {
        <button type="button" class=move || if is_pinned.get() { "socket-ticker-card rounded-lg border border-[var(--accent)] bg-[var(--surface)] text-left shadow-sm socket-ticker-card-pinned" } else { "socket-ticker-card rounded-lg border border-[var(--border-color)] bg-[var(--surface)] text-left shadow-sm" } title={let symbol_title = symbol.clone(); move || if is_pinned.get() { format!("Unpin {symbol_title}") } else { format!("Pin {symbol_title}") }} aria-label={let symbol_aria = symbol.clone(); move || ticker.get().map(|item| card_aria_label(item, is_pinned.get(), funding_rate.get())).unwrap_or_else(|| format!("{symbol_aria}, market data unavailable"))} on:click={let symbol = symbol.clone(); move |_| { let index = visible.get_untracked().iter().position(|item| item.ticker.symbol == symbol).unwrap_or(0); state.toggle_pin(&symbol, index); }}>
            <div class="flex min-h-0 flex-col p-2">
                <div class="flex items-start justify-between gap-2"><span class="truncate font-mono font-semibold">{symbol.clone()}</span><span aria-hidden="true" class="text-sm text-[var(--text-secondary)]">{move || if is_pinned.get() { "●" } else { "○" }}</span></div>
                <div class="socket-ticker-price mt-1 truncate font-mono">{move || ticker.get().map(|item| format_number(item.ticker.last_price)).unwrap_or_else(|| "—".into())}</div>
                <div class="mt-1 flex items-center justify-between gap-2">{move || ticker.get().map(|item| view! { <span class=change_class(item.ticker.change_24h)>{format_percent(item.ticker.change_24h)}</span> }).unwrap_or_else(|| view! { <span class="text-[var(--text-secondary)]">{"—".to_string()}</span> })}<span class="font-mono text-xs text-[var(--text-secondary)]">{move || ticker.get().map(|item| format!("{}%", item.momentum.progress())).unwrap_or_else(|| "—".into())}</span></div>
                <div class="mt-1 flex items-center justify-between gap-2 text-xs"><span class="text-[var(--text-secondary)]">"Funding"</span><span class=move || funding_rate_class(funding_rate.get())>{move || format_funding_rate(funding_rate.get())}</span></div>
                <progress class="socket-ticker-progress mt-2 w-full" max="100" value=move || ticker.get().map(|item| item.momentum.progress().to_string()).unwrap_or_else(|| "0".into()) aria-label="Directional progress"></progress>
                <div class="mt-auto flex justify-between gap-2 pt-2 font-mono text-xs"><span class="text-[var(--success)]">{move || ticker.get().map(|item| format!("↑ {}", item.momentum.up_ticks)).unwrap_or_else(|| "↑ 0".into())}</span><span class="text-[var(--danger)]">{move || ticker.get().map(|item| format!("↓ {}", item.momentum.down_ticks)).unwrap_or_else(|| "↓ 0".into())}</span></div>
            </div>
        </button>
    }
}

#[allow(clippy::too_many_arguments)]
fn build_visible(all: MarketSnapshot, mode: SocketViewMode, sort: SocketSortMode, direction: SocketSortDirection, limit: usize, slots: Vec<Option<String>>, funding_rates: Option<FundingRateSnapshot>, search_query: String) -> Vec<TrackedFuturesTicker> {
    let query = search_query.trim().to_uppercase();
    let is_searching = !query.is_empty();
    let sort_fn = |left: &TrackedFuturesTicker, right: &TrackedFuturesTicker| {
        let cmp = match sort {
            SocketSortMode::Momentum => right.momentum.progress().cmp(&left.momentum.progress()).then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)),
            SocketSortMode::TotalTicks => { let left_total = left.momentum.up_ticks + left.momentum.down_ticks; let right_total = right.momentum.up_ticks + right.momentum.down_ticks; right_total.cmp(&left_total).then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)) }
            SocketSortMode::Funding => { let left_funding = funding_rates.as_ref().and_then(|r| r.get(&left.ticker.symbol)).unwrap_or(0.0); let right_funding = funding_rates.as_ref().and_then(|r| r.get(&right.ticker.symbol)).unwrap_or(0.0); right_funding.partial_cmp(&left_funding).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)) }
            SocketSortMode::Change24h => { let left_change = left.ticker.change_24h.unwrap_or(0.0); let right_change = right.ticker.change_24h.unwrap_or(0.0); right_change.partial_cmp(&left_change).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)) }
            SocketSortMode::Volume24h => { let left_vol = left.ticker.volume_24h.unwrap_or(0.0); let right_vol = right.ticker.volume_24h.unwrap_or(0.0); right_vol.partial_cmp(&left_vol).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)) }
        };
        match direction { SocketSortDirection::Descending => cmp, SocketSortDirection::Ascending => cmp.reverse() }
    };
    if is_searching { let mut results = all.values().filter(|item| item.ticker.symbol.contains(&query)).cloned().collect::<Vec<_>>(); results.sort_unstable_by(sort_fn); return results; }
    let pinned_symbols = slots.iter().filter_map(|slot| slot.as_deref()).collect::<Vec<_>>();
    if mode == SocketViewMode::PinnedOnly { return slots.iter().filter_map(|slot| slot.as_deref().and_then(|symbol| all.get(symbol))).cloned().collect(); }
    let mut dynamic = all.values().filter(|item| !pinned_symbols.contains(&item.ticker.symbol.as_str())).cloned().collect::<Vec<_>>();
    dynamic.sort_unstable_by(sort_fn);
    dynamic.truncate(limit);
    let pinned_count = slots.iter().filter(|slot| slot.is_some()).count();
    let output_len = slots.len().max(dynamic.len() + pinned_count);
    let mut output = Vec::with_capacity(output_len);
    let mut dynamic_index = 0;
    for index in 0..output_len { if let Some(symbol) = slots.get(index).and_then(|slot| slot.as_deref()) { if let Some(ticker) = all.get(symbol) { output.push(ticker.clone()); } } else if let Some(ticker) = dynamic.get(dynamic_index) { output.push(ticker.clone()); dynamic_index += 1; } }
    output
}

fn status_badge(status: FuturesConnectionStatus) -> impl IntoView {
    match status {
        FuturesConnectionStatus::Connected => view! { <span class="rounded-full border border-[var(--success)]/40 bg-[var(--success)]/10 px-2 py-1 text-xs text-[var(--success)]">"Connected"</span> }.into_any(),
        FuturesConnectionStatus::Connecting => view! { <span class="rounded-full border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-2 py-1 text-xs text-[var(--warning)]">"Connecting"</span> }.into_any(),
        FuturesConnectionStatus::Reconnecting => view! { <span class="rounded-full border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-2 py-1 text-xs text-[var(--warning)]">"Reconnecting"</span> }.into_any(),
        FuturesConnectionStatus::Disconnected => view! { <span class="rounded-full border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-secondary)]">"Disconnected"</span> }.into_any(),
        FuturesConnectionStatus::Error(_) => view! { <span class="rounded-full border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-2 py-1 text-xs text-[var(--danger)]">"Connection error"</span> }.into_any(),
    }
}

fn empty_state(mode: SocketViewMode) -> impl IntoView {
    let text = match mode { SocketViewMode::All => "Waiting for market data...", SocketViewMode::PinnedOnly => "No pinned tickers" };
    view! { <div class="flex h-full flex-col items-center justify-center py-5 text-[var(--text-secondary)]"><span class="mb-2 text-2xl" aria-hidden="true">"◌"</span><span>{text}</span></div> }
}

fn view_button_class(active: bool) -> &'static str {
    if active { "rounded-l-md border border-[var(--accent)] bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" } else { "border-y border-r border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" }
}

fn card_aria_label(ticker: TrackedFuturesTicker, pinned: bool, funding_rate: Option<f64>) -> String {
    format!("{}, 24 hour change {}, price {}, funding rate {}, {} up ticks, {} down ticks, {} percent progress, {}", ticker.ticker.symbol, format_percent(ticker.ticker.change_24h), format_number(ticker.ticker.last_price), format_funding_rate(funding_rate), ticker.momentum.up_ticks, ticker.momentum.down_ticks, ticker.momentum.progress(), if pinned { "pinned" } else { "not pinned" })
}

fn change_class(value: Option<f64>) -> &'static str {
    match value { Some(value) if value > 0.0 => "text-[var(--success)]", Some(value) if value < 0.0 => "text-[var(--danger)]", _ => "text-[var(--text-primary)]" }
}

fn funding_rate_class(value: Option<f64>) -> &'static str {
    match value { Some(value) if value > 0.0 => "font-mono text-[var(--success)]", Some(value) if value < 0.0 => "font-mono text-[var(--danger)]", Some(_) => "font-mono text-[var(--text-primary)]", None => "font-mono text-[var(--text-secondary)]" }
}

fn format_number(value: Option<f64>) -> String { value.map(|number| if number.abs() >= 1.0 { format!("{number:.4}") } else { format!("{number:.8}") }).unwrap_or_else(|| "—".into()) }
fn format_percent(value: Option<f64>) -> String { value.map(|number| format!("{number:+.2}%", number = number * 100.0)).unwrap_or_else(|| "—".into()) }
fn format_funding_rate(value: Option<f64>) -> String { value.map(|number| format!("{number:+.4}%", number = number * 100.0)).unwrap_or_else(|| "—".into()) }
