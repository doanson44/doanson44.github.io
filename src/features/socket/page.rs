use std::{collections::HashMap, rc::Rc};

use leptos::prelude::*;

use crate::i18n::*;

use crate::application::ports::{
    FundingRateProvider, FuturesConnectionStatus, FuturesMarketStream,
};
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::TrackedFuturesTicker;
use crate::features::socket::state::{
    SocketFilter, SocketSortDirection, SocketSortMode, SocketState, SocketViewMode,
};

/// Realtime Futures market ticker monitor page.
#[component]
pub fn SocketPage(
    stream: Rc<dyn FuturesMarketStream>,
    funding_provider: Rc<dyn FundingRateProvider>,
) -> impl IntoView {
    let state = SocketState::new(stream, funding_provider);
    let i18n = use_i18n();
    let visible = Memo::new({
        let tickers = state.tickers;
        let view_mode = state.view_mode;
        let filter = state.filter;
        let sort_mode = state.sort_mode;
        let sort_direction = state.sort_direction;
        let ticker_limit = state.ticker_limit;
        let search_query = state.search_query;
        let pinned_slots = state.pinned_slots;
        let funding_rates = state.funding_rates;
        move |_| {
            build_visible(
                tickers.get(),
                view_mode.get(),
                filter.get(),
                sort_mode.get(),
                sort_direction.get(),
                ticker_limit.get(),
                pinned_slots.get(),
                funding_rates.get(),
                search_query.get(),
            )
        }
    });

    view! {
        <div class="flex flex-grow flex-col overflow-hidden socket-page">
            <div class="flex flex-grow flex-col overflow-hidden px-4 py-3">
                <header class="mb-3 flex shrink-0 flex-wrap items-center justify-between gap-2">
                    <div>
                        <h2 class="mb-1 text-xl font-semibold">{move || if i18n.get_locale() == Locale::vi { "Thị trường Futures" } else { "Futures Market" }}</h2>
                        <div class="text-sm text-[var(--text-secondary)]">{move || if i18n.get_locale() == Locale::vi { "Theo dõi momentum realtime kể từ khi mở trang" } else { "Realtime market momentum from the moment this page opens" }}</div>
                    </div>
                    <div class="text-sm">{move || status_badge(state.connection_status.get())}</div>
                </header>
                <div class="mb-3 flex shrink-0 flex-wrap items-center gap-2">
                    <div class="flex w-full max-w-sm items-center md:mr-auto">
                        <label class="sr-only" for="socket-search">"Search symbol"</label>
                        <input id="socket-search" type="search" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" placeholder={move || t_string!(i18n, socket_search)} prop:value=move || state.search_query.get() on:input=move |ev| state.search_query.set(event_target_value(&ev)) />
                    </div>
                    <div class="flex" role="group" aria-label=move || t_string!(i18n, socket_view)>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::All && state.filter.get() == SocketFilter::All) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::All && state.filter.get() == SocketFilter::All).to_string() on:click=move |_| { state.view_mode.set(SocketViewMode::All); state.filter.set(SocketFilter::All); }>{move || t_string!(i18n, socket_all)}</button>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::All && state.filter.get() == SocketFilter::Burst) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::All && state.filter.get() == SocketFilter::Burst).to_string() on:click=move |_| { state.view_mode.set(SocketViewMode::All); state.filter.set(SocketFilter::Burst); }>{move || t_string!(i18n, socket_burst)}</button>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::PinnedOnly) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::PinnedOnly).to_string() on:click=move |_| { state.view_mode.set(SocketViewMode::PinnedOnly); state.filter.set(SocketFilter::All); }>{move || t_string!(i18n, socket_pinned)}</button>
                    </div>
                    <div class="flex items-center gap-2">
                        <label class="text-sm text-[var(--text-secondary)]" for="socket-sort-mode">{move || t_string!(i18n, socket_sort)}</label>
                        <select id="socket-sort-mode" class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Sort tickers by" prop:value=move || match state.sort_mode.get() { SocketSortMode::Momentum => "momentum", SocketSortMode::Price => "price", SocketSortMode::TotalTicks => "activity", SocketSortMode::Funding => "funding", SocketSortMode::Change24h => "change24h", SocketSortMode::Volume24h => "volume24h" } on:change=move |ev| {
                            let val = event_target_value(&ev);
                            state.sort_mode.set(match val.as_str() { "activity" => SocketSortMode::TotalTicks, "price" => SocketSortMode::Price, "funding" => SocketSortMode::Funding, "change24h" => SocketSortMode::Change24h, "volume24h" => SocketSortMode::Volume24h, _ => SocketSortMode::Momentum });
                        }>
                            <option value="momentum">{move || t_string!(use_i18n(), socket_momentum)}</option><option value="price">{move || t_string!(i18n, socket_price)}</option><option value="activity">{move || t_string!(i18n, socket_activity)}</option><option value="funding">{move || t_string!(use_i18n(), socket_funding)}</option><option value="change24h">{move || t_string!(i18n, socket_change24h)}</option><option value="volume24h">{move || t_string!(i18n, socket_volume24h)}</option>
                        </select>
                        <button type="button" class="rounded-md border border-[var(--border-color)] px-2 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title=move || match state.sort_direction.get() { SocketSortDirection::Ascending => "Sort Ascending", SocketSortDirection::Descending => "Sort Descending" } on:click=move |_| state.sort_direction.update(|d| *d = match d { SocketSortDirection::Ascending => SocketSortDirection::Descending, SocketSortDirection::Descending => SocketSortDirection::Ascending })>
                            {move || match state.sort_direction.get() { SocketSortDirection::Ascending => "↑", SocketSortDirection::Descending => "↓" }}
                        </button>
                    </div>
                    <div class="flex items-center gap-2">
                        <label class="text-sm text-[var(--text-secondary)]" for="socket-ticker-limit">{move || t_string!(i18n, socket_show)}</label>
                        <select id="socket-ticker-limit" class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Number of dynamic tickers to show" prop:value=move || state.ticker_limit.get().to_string() on:change=move |ev| { let value = event_target_value(&ev).parse::<usize>().unwrap_or(DEFAULT_LIMIT); state.set_ticker_limit(value); }>
                            {SocketState::limit_options().iter().map(|value| { let label = if *value == usize::MAX { "All".to_string() } else { value.to_string() }; view! { <option value=value.to_string()>{label}</option> } }).collect_view()}
                        </select>
                        <span class="text-sm text-[var(--text-secondary)]">{move || t_string!(i18n, socket_dynamic)}</span>
                    </div>
                </div>
                <div class="min-h-0 flex-grow overflow-auto" aria-live="polite">
                    <Show when=move || !visible.get().is_empty() fallback=move || empty_state(state.view_mode.get())>
                        <div class="hidden overflow-x-auto rounded-lg border border-[var(--border-color)] bg-[var(--surface)] md:block">
                            <table class="w-full min-w-[900px] border-collapse text-sm">
                                <caption class="sr-only">"Realtime Futures market tickers"</caption>
                                <thead>
                                    <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-left text-[var(--text-secondary)]">
                                        <th class="px-3 py-2 text-center font-medium" scope="col">"Pin"</th>
                                        <th class="px-3 py-2 font-medium" scope="col">"Symbol"</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">"Price"</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">"24h"</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_funding)}</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_momentum)}</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_activity)}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || visible.get().into_iter().map(|ticker| view! {
                                        <TickerTableRow ticker=ticker state=state />
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </div>
                        <div class="flex flex-col gap-2 md:hidden">
                            {move || visible.get().into_iter().map(|ticker| view! {
                                <TickerMobileCard ticker=ticker state=state />
                            }).collect_view()}
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}

const DEFAULT_LIMIT: usize = 10;
type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

#[component]
fn TickerTableRow(ticker: TrackedFuturesTicker, state: SocketState) -> impl IntoView {
    let symbol=ticker.ticker.symbol.clone();
    let is_pinned=Memo::new({let pinned_slots=state.pinned_slots; let symbol=symbol.clone(); move |_| pinned_slots.get().iter().any(|slot| slot.as_deref()==Some(symbol.as_str()))});
    let funding_rate=Memo::new({let funding_rates=state.funding_rates; let symbol=symbol.clone(); move |_| funding_rates.get().and_then(|snapshot| snapshot.get(&symbol))});
    let change_class=change_class(ticker.ticker.change_24h);
    let symbol_title = symbol.clone();
    let symbol_aria = symbol.clone();
    let symbol_click = symbol.clone();
    view! {
        <tr class=move || if is_pinned.get() { "border-b border-[var(--accent)]/50 bg-[var(--accent)]/5 last:border-b-0 hover:bg-[var(--surface-hover)]" } else { "border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]" }>
            <td class="px-3 py-2 text-center">
                <button type="button" class="min-h-11 min-w-11 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                    title=move || if is_pinned.get() { format!("Unpin {}", symbol_title) } else { format!("Pin {}", symbol_title) }
                    aria-label=move || if is_pinned.get() { format!("Unpin {}", symbol_aria) } else { format!("Pin {}", symbol_aria) }
                    on:click=move |_| state.toggle_pin(&symbol_click, 0)>
                    {move || if is_pinned.get() { "★" } else { "☆" }}
                </button>
            </td>
            <th class="px-3 py-2 text-left font-semibold text-[var(--text-primary)]" scope="row">
                <div class="flex items-center gap-2"><span class="font-mono">{symbol.clone()}</span>
                    {if ticker.momentum.is_burst() { view! { <span class="rounded-full border border-[var(--warning)]/50 bg-[var(--warning)]/10 px-1.5 py-0.5 text-[10px] font-semibold text-[var(--warning)]">"BURST"</span> }.into_any() } else { view! { <span></span> }.into_any() }}
                </div>
            </th>
            <td class="px-3 py-2 text-right font-mono font-medium text-[var(--text-primary)]">{format_number(ticker.ticker.last_price)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_percent(ticker.ticker.change_24h)}</td>
            <td class=move || format!("px-3 py-2 text-right {}", funding_rate_class(funding_rate.get()))>{move || format_funding_rate(funding_rate.get())}</td>
            <td class="px-3 py-2 text-right"><div class="flex min-w-32 items-center justify-end gap-2"><progress class="socket-ticker-progress w-24" max="100" value=ticker.momentum.progress().to_string() aria-label="Momentum"></progress><span class="font-mono text-xs text-[var(--text-secondary)]">{format!("{}%",ticker.momentum.progress())}</span></div></td>
            <td class="px-3 py-2 text-right font-mono text-xs"><span class="text-[var(--success)]">{format!("↑ {}",ticker.momentum.up_ticks)}</span><span class="ml-2 text-[var(--danger)]">{format!("↓ {}",ticker.momentum.down_ticks)}</span></td>
        </tr>
    }
}

#[component]
fn TickerMobileCard(ticker: TrackedFuturesTicker, state: SocketState) -> impl IntoView {
    let symbol=ticker.ticker.symbol.clone();
    let is_pinned=Memo::new({let pinned_slots=state.pinned_slots; let symbol=symbol.clone(); move |_| pinned_slots.get().iter().any(|slot| slot.as_deref()==Some(symbol.as_str()))});
    let funding_rate=Memo::new({let funding_rates=state.funding_rates; let symbol=symbol.clone(); move |_| funding_rates.get().and_then(|snapshot| snapshot.get(&symbol))});
    let change_class=change_class(ticker.ticker.change_24h);
    let symbol_title = symbol.clone();
    let symbol_aria = symbol.clone();
    let symbol_click = symbol.clone();
    view! {
        <article class=move || if is_pinned.get() { "rounded-lg border border-[var(--accent)]/60 bg-[var(--accent)]/5 p-3 shadow-sm" } else { "rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3 shadow-sm" }>
            <div class="flex items-start gap-2">
                <div class="min-w-0 flex-1"><div class="flex items-center gap-2"><h3 class="m-0 truncate font-mono text-base font-semibold text-[var(--text-primary)]">{symbol.clone()}</h3>
                    {if ticker.momentum.is_burst() { view! { <span class="shrink-0 rounded-full border border-[var(--warning)]/50 bg-[var(--warning)]/10 px-1.5 py-0.5 text-[10px] font-semibold text-[var(--warning)]">"BURST"</span> }.into_any() } else { view! { <span></span> }.into_any() }}
                </div><div class="mt-1 flex items-center gap-2"><span class="font-mono font-medium text-[var(--text-primary)]">{format_number(ticker.ticker.last_price)}</span><span class=format!("font-medium {change_class}")>{format_percent(ticker.ticker.change_24h)}</span></div></div>
                <button type="button" class="min-h-11 min-w-11 shrink-0 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                    title=move || if is_pinned.get() { format!("Unpin {}", symbol_title) } else { format!("Pin {}", symbol_title) }
                    aria-label=move || if is_pinned.get() { format!("Unpin {}", symbol_aria) } else { format!("Pin {}", symbol_aria) }
                    on:click=move |_| state.toggle_pin(&symbol_click, 0)>
                    {move || if is_pinned.get() { "★" } else { "☆" }}
                </button>
            </div>
            <div class="mt-3 grid grid-cols-2 gap-x-3 gap-y-2 text-sm">
                <div><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_funding)}</span><span class=move || funding_rate_class(funding_rate.get())>{move || format_funding_rate(funding_rate.get())}</span></div>
                <div class="text-right"><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_momentum)}</span><span class="font-mono text-[var(--text-primary)]">{format!("{}%",ticker.momentum.progress())}</span></div>
                <div><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_activity)}</span><span class="font-mono text-xs"><span class="text-[var(--success)]">{format!("↑ {}",ticker.momentum.up_ticks)}</span><span class="ml-2 text-[var(--danger)]">{format!("↓ {}",ticker.momentum.down_ticks)}</span></span></div>
                <div class="text-right"><progress class="socket-ticker-progress mt-1 w-full" max="100" value=ticker.momentum.progress().to_string() aria-label="Momentum"></progress></div>
            </div>
        </article>
    }
}

#[allow(clippy::too_many_arguments)]
fn build_visible(
    all: MarketSnapshot,
    mode: SocketViewMode,
    filter: SocketFilter,
    sort: SocketSortMode,
    direction: SocketSortDirection,
    limit: usize,
    slots: Vec<Option<String>>,
    funding_rates: Option<FundingRateSnapshot>,
    search_query: String,
) -> Vec<TrackedFuturesTicker> {
    let query = search_query.trim().to_uppercase();
    let is_searching = !query.is_empty();
    let sort_fn = |left: &TrackedFuturesTicker, right: &TrackedFuturesTicker| {
        let cmp = if filter == SocketFilter::Burst {
            right
                .momentum
                .burst_streak()
                .cmp(&left.momentum.burst_streak())
                .then_with(|| {
                    right
                        .momentum
                        .burst_score()
                        .cmp(&left.momentum.burst_score())
                })
                .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
        } else {
            match sort {
                SocketSortMode::Momentum => right
                    .momentum
                    .progress()
                    .cmp(&left.momentum.progress())
                    .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)),
                SocketSortMode::Price => {
                    let left_price = left.ticker.last_price.unwrap_or(0.0);
                    let right_price = right.ticker.last_price.unwrap_or(0.0);
                    right_price
                        .partial_cmp(&left_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
                }
                SocketSortMode::TotalTicks => {
                    let left_total = left.momentum.up_ticks + left.momentum.down_ticks;
                    let right_total = right.momentum.up_ticks + right.momentum.down_ticks;
                    right_total
                        .cmp(&left_total)
                        .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
                }
                SocketSortMode::Funding => {
                    let left_funding = funding_rates
                        .as_ref()
                        .and_then(|r| r.get(&left.ticker.symbol))
                        .unwrap_or(0.0);
                    let right_funding = funding_rates
                        .as_ref()
                        .and_then(|r| r.get(&right.ticker.symbol))
                        .unwrap_or(0.0);
                    right_funding
                        .partial_cmp(&left_funding)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
                }
                SocketSortMode::Change24h => {
                    let left_change = left.ticker.change_24h.unwrap_or(0.0);
                    let right_change = right.ticker.change_24h.unwrap_or(0.0);
                    right_change
                        .partial_cmp(&left_change)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
                }
                SocketSortMode::Volume24h => {
                    let left_vol = left.ticker.volume_24h.unwrap_or(0.0);
                    let right_vol = right.ticker.volume_24h.unwrap_or(0.0);
                    right_vol
                        .partial_cmp(&left_vol)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol))
                }
            }
        };

        if filter == SocketFilter::Burst {
            cmp
        } else {
            match direction {
                SocketSortDirection::Descending => cmp,
                SocketSortDirection::Ascending => cmp.reverse(),
            }
        }
    };
    let matches_filter = |item: &TrackedFuturesTicker| match filter {
        SocketFilter::All => true,
        SocketFilter::Burst => item.momentum.is_burst(),
    };

    let pinned_symbols = slots
        .iter()
        .filter_map(|slot| slot.as_deref())
        .collect::<Vec<_>>();

    if mode == SocketViewMode::PinnedOnly {
        let mut pinned = slots
            .iter()
            .filter_map(|slot| slot.as_deref().and_then(|symbol| all.get(symbol)))
            .filter(|item| matches_filter(item))
            .filter(|item| !is_searching || item.ticker.symbol.contains(&query))
            .cloned()
            .collect::<Vec<_>>();

        pinned.sort_unstable_by(sort_fn);
        return pinned;
    }

    if is_searching {
        let mut results = all
            .values()
            .filter(|item| matches_filter(item))
            .filter(|item| item.ticker.symbol.contains(&query))
            .cloned()
            .collect::<Vec<_>>();
        results.sort_unstable_by(sort_fn);
        return results;
    }

    let pinned = slots
        .iter()
        .filter_map(|slot| slot.as_deref().and_then(|symbol| all.get(symbol)))
        .filter(|item| matches_filter(item))
        .filter(|item| !is_searching || item.ticker.symbol.contains(&query))
        .cloned()
        .collect::<Vec<_>>();

    let mut dynamic = all
        .values()
        .filter(|item| !pinned_symbols.contains(&item.ticker.symbol.as_str()))
        .filter(|item| matches_filter(item))
        .filter(|item| !is_searching || item.ticker.symbol.contains(&query))
        .cloned()
        .collect::<Vec<_>>();
    dynamic.sort_unstable_by(sort_fn);

    let dynamic_limit = limit.saturating_sub(pinned.len());
    dynamic.truncate(dynamic_limit);

    let mut output = Vec::with_capacity(pinned.len() + dynamic.len());
    output.extend(pinned);
    output.extend(dynamic);
    output
}

fn status_badge(status: FuturesConnectionStatus) -> impl IntoView {
    let i18n = use_i18n();
    match status {
        FuturesConnectionStatus::Connected => view! { <span class="rounded-full border border-[var(--success)]/40 bg-[var(--success)]/10 px-2 py-1 text-xs text-[var(--success)]">{move || t_string!(i18n, socket_connected)}</span> }.into_any(),
        FuturesConnectionStatus::Connecting => view! { <span class="rounded-full border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-2 py-1 text-xs text-[var(--warning)]">{move || t_string!(i18n, socket_connecting)}</span> }.into_any(),
        FuturesConnectionStatus::Reconnecting => view! { <span class="rounded-full border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-2 py-1 text-xs text-[var(--warning)]">{move || t_string!(i18n, socket_reconnecting)}</span> }.into_any(),
        FuturesConnectionStatus::Disconnected => view! { <span class="rounded-full border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-secondary)]">{move || t_string!(i18n, socket_disconnected)}</span> }.into_any(),
        FuturesConnectionStatus::Error(_) => view! { <span class="rounded-full border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-2 py-1 text-xs text-[var(--danger)]">{move || t_string!(i18n, socket_connection_error)}</span> }.into_any(),
    }
}

fn empty_state(mode: SocketViewMode) -> impl IntoView {
    let i18n = use_i18n();
    let text = move || match mode {
        SocketViewMode::All => t_string!(i18n, socket_waiting),
        SocketViewMode::PinnedOnly => t_string!(i18n, socket_no_pinned),
    };
    view! { <div class="flex h-full flex-col items-center justify-center py-5 text-[var(--text-secondary)]"><span class="mb-2 text-2xl" aria-hidden="true">"◌"</span><span>{text}</span></div> }
}

fn view_button_class(active: bool) -> &'static str {
    if active {
        "rounded-l-md border border-[var(--accent)] bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
    } else {
        "border-y border-r border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
    }
}

fn card_aria_label(
    ticker: TrackedFuturesTicker,
    pinned: bool,
    funding_rate: Option<f64>,
) -> String {
    format!("{}, 24 hour change {}, price {}, funding rate {}, {} up ticks, {} down ticks, {} percent progress, {}", ticker.ticker.symbol, format_percent(ticker.ticker.change_24h), format_number(ticker.ticker.last_price), format_funding_rate(funding_rate), ticker.momentum.up_ticks, ticker.momentum.down_ticks, ticker.momentum.progress(), if pinned { "pinned" } else { "not pinned" })
}

fn change_class(value: Option<f64>) -> &'static str {
    match value {
        Some(value) if value > 0.0 => "text-[var(--success)]",
        Some(value) if value < 0.0 => "text-[var(--danger)]",
        _ => "text-[var(--text-primary)]",
    }
}

fn funding_rate_class(value: Option<f64>) -> &'static str {
    match value {
        Some(value) if value > 0.0 => "font-mono text-[var(--success)]",
        Some(value) if value < 0.0 => "font-mono text-[var(--danger)]",
        Some(_) => "font-mono text-[var(--text-primary)]",
        None => "font-mono text-[var(--text-secondary)]",
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

fn format_funding_rate(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:+.4}%", number = number * 100.0))
        .unwrap_or_else(|| "—".into())
}
