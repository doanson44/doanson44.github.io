use std::{collections::HashMap, rc::Rc};

use leptos::prelude::*;

use crate::i18n::*;

use crate::application::ports::{
    FundingRateProvider, FuturesConnectionStatus, FuturesMarketStream,
};
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::TrackedFuturesTicker;
use crate::features::socket::portfolio::PortfolioPanel;
use crate::features::socket::state::{
    SocketSortDirection, SocketSortMode, SocketState, SocketViewMode,
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
        let sort_mode = state.sort_mode;
        let sort_direction = state.sort_direction;
        let search_query = state.search_query;
        let pinned_symbols = state.pinned_symbols;
        let funding_rates = state.funding_rates;
        move |_| {
            build_visible(
                tickers.get(),
                view_mode.get(),
                sort_mode.get(),
                sort_direction.get(),
                pinned_symbols.get(),
                funding_rates.get(),
                search_query.get(),
            )
        }
    });

    let portfolio_summary = Memo::new({
        let tickers = state.tickers;
        let trading_snapshot = state.trading_snapshot;
        move |_| {
            tickers.get();
            trading_snapshot.get();
            state.portfolio_summary()
        }
    });

    let paginated = Memo::new({
        let page_size = state.page_size;
        let current_page = state.current_page;
        move |_| {
            let items = visible.get();
            let size = page_size.get();
            let total_pages = if size == usize::MAX {
                1
            } else {
                items.len().div_ceil(size).max(1)
            };
            let page = current_page.get().clamp(1, total_pages);
            let start = (page - 1) * size;
            items.into_iter().skip(start).take(size).collect::<Vec<_>>()
        }
    });

    Effect::new({
        let current_page = state.current_page;
        let page_size = state.page_size;
        move |_| {
            let size = page_size.get();
            let total_pages = if size == usize::MAX {
                1
            } else {
                visible.get().len().div_ceil(size).max(1)
            };
            let page = current_page.get();
            if page > total_pages {
                current_page.set(total_pages);
            }
        }
    });

    view! {
        <div class="flex flex-grow flex-col overflow-hidden socket-page">
            <div class="flex flex-grow flex-col overflow-hidden px-4 py-3">
                <header class="mb-3 flex shrink-0 flex-wrap items-center justify-between gap-2">
                    <div>
                        <h2 class="mb-1 text-xl font-semibold">{move || if i18n.get_locale() == Locale::vi { "Thị trường Futures" } else { "Futures Market" }}</h2>
                        <div class="text-sm text-[var(--text-secondary)]">{move || if i18n.get_locale() == Locale::vi { "Xếp hạng chuyển động giá realtime kể từ khi mở trang" } else { "Realtime price-move ranking from the moment this page opens" }}</div>
                    </div>
                    <div class="flex items-center gap-2 text-sm">
                        {move || status_badge(state.connection_status.get())}
                        <button
                            type="button"
                            class="min-h-9 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-1.5 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            on:click=move |_| state.reset_metrics()
                            title={move || t_string!(i18n, socket_reset_metrics)}
                        >
                            {move || t_string!(i18n, socket_reset_metrics)}
                        </button>
                        <button
                            type="button"
                            class="min-h-9 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-1.5 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            on:click=move |_| state.open_settings()
                        >
                            {move || t_string!(i18n, socket_settings)}
                        </button>
                    </div>
                </header>
                <Show when=move || state.analysis_error.get().is_some()>
                    <div class="mb-3 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 p-3 text-sm text-[var(--danger)]" role="alert">{move || state.analysis_error.get().unwrap_or_default()}</div>
                </Show>
                <Show when=move || state.trading_error.get().is_some()>
                    <div class="mb-3 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 p-3 text-sm text-[var(--danger)]" role="alert">
                        {move || state.trading_error.get().unwrap_or_default()}
                    </div>
                </Show>
                <Show when=move || state.trading_notice.get().is_some()>
                    <div class="mb-3 rounded-md border border-[var(--success)]/40 bg-[var(--success)]/10 p-3 text-sm text-[var(--success)]" role="status">
                        {move || state.trading_notice.get().unwrap_or_default()}
                    </div>
                </Show>
                <PortfolioPanel state=state summary=portfolio_summary />
                <div class="mb-3 flex shrink-0 flex-wrap items-center gap-2">
                    <div class="flex w-full max-w-sm items-center md:mr-auto">
                        <label class="sr-only" for="socket-search">"Search symbol"</label>
                        <input id="socket-search" type="search" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" placeholder={move || t_string!(i18n, socket_search)} prop:value=move || state.search_query.get() on:input=move |ev| state.search_query.set(event_target_value(&ev)) />
                    </div>
                    <div class="flex" role="group" aria-label=move || t_string!(i18n, socket_view)>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::All) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::All).to_string() on:click=move |_| state.view_mode.set(SocketViewMode::All)>{move || t_string!(i18n, socket_all)}</button>
                        <button class=move || view_button_class(state.view_mode.get() == SocketViewMode::PinnedOnly) type="button" aria-pressed=move || (state.view_mode.get() == SocketViewMode::PinnedOnly).to_string() on:click=move |_| state.view_mode.set(SocketViewMode::PinnedOnly)>{move || t_string!(i18n, socket_pinned)}</button>
                    </div>
                </div>

                <Show when=move || !visible.get().is_empty() fallback=move || empty_state(state.view_mode.get())>
                    <div class="mb-2">
                        <PaginationControls state=state total_items=Memo::new(move |_| visible.get().len()) />
                    </div>
                        <div class="hidden overflow-x-auto rounded-lg border border-[var(--border-color)] bg-[var(--surface)] md:block">
                            <table class="w-full min-w-[900px] border-collapse text-sm">
                                <caption class="sr-only">"Realtime Futures market tickers"</caption>
                                <thead>
                                    <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-[var(--text-secondary)]">
                                        <th class="px-3 py-2 text-center font-medium" scope="col">"Pin"</th>
                                        <th class="px-3 py-2 font-medium" scope="col">
                                            <SortHeader state=state mode=SocketSortMode::Symbol align="left">"Symbol" </SortHeader>
                                        </th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">
                                            <SortHeader state=state mode=SocketSortMode::Price align="right">"Price" </SortHeader>
                                        </th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">
                                            <SortHeader state=state mode=SocketSortMode::Change24h align="right">"24h" </SortHeader>
                                        </th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">
                                            <SortHeader state=state mode=SocketSortMode::Funding align="right">{move || t_string!(i18n, socket_funding)} </SortHeader>
                                        </th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">
                                            <SortHeader state=state mode=SocketSortMode::Ranking align="right">{move || t_string!(i18n, socket_ranking)} </SortHeader>
                                        </th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_direction)}</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_change1m)}</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_change3m)}</th>
                                        <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_analysis)}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || paginated.get().into_iter().map(|ticker| view! {
                                        <TickerTableRow ticker=ticker state=state />
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </div>
                        <div class="flex flex-col gap-2 md:hidden">
                            <div class="flex items-center justify-between gap-2 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                                <span class="shrink-0 text-xs font-medium text-[var(--text-secondary)]">{move || t_string!(i18n, socket_sort)}</span>
                                <div class="flex flex-wrap justify-end gap-1">
                                    {socket_mobile_sort_button("symbol", SocketSortMode::Symbol, state, i18n)}
                                    {socket_mobile_sort_button("ranking", SocketSortMode::Ranking, state, i18n)}
                                    {socket_mobile_sort_button("price", SocketSortMode::Price, state, i18n)}
                                    {socket_mobile_sort_button("change24h", SocketSortMode::Change24h, state, i18n)}
                                    {socket_mobile_sort_button("funding", SocketSortMode::Funding, state, i18n)}
                                    {socket_mobile_sort_button("volume24h", SocketSortMode::Volume24h, state, i18n)}
                                </div>
                            </div>
                            {move || paginated.get().into_iter().map(|ticker| view! {
                                <TickerMobileCard ticker=ticker state=state />
                            }).collect_view()}
                        </div>
                </Show>
            </div>
            {move || if state.analysis_modal_open.get() {
                if let Some(result) = state.analysis_result.get() {
                    view! { <SocketAnalysisModal state=state result=result i18n=i18n /> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

#[component]
fn PaginationControls(state: SocketState, total_items: Memo<usize>) -> impl IntoView {
    let total_pages = Memo::new(move |_| {
        let size = state.page_size.get();
        if size == usize::MAX {
            1
        } else {
            total_items.get().div_ceil(size).max(1)
        }
    });
    let page = Memo::new(move |_| state.current_page.get().clamp(1, total_pages.get()));
    let can_go_previous = Memo::new(move |_| page.get() > 1);
    let can_go_next = Memo::new(move |_| page.get() < total_pages.get());

    view! {
        <div class="mt-3 flex flex-wrap items-center justify-between gap-2">
            <div class="flex items-center gap-2 text-sm text-[var(--text-secondary)]">
                <label for="socket-page-size">"Items"</label>
                <select
                    id="socket-page-size"
                    class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1.5 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                    prop:value=move || state.page_size.get().to_string()
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let size = if value == "all" {
                            usize::MAX
                        } else {
                            value.parse::<usize>().unwrap_or(10)
                        };
                        state.set_page_size(size);
                    }
                >
                    <option value="10">"10"</option>
                    <option value="20">"20"</option>
                    <option value="50">"50"</option>
                    <option value="all">"All"</option>
                </select>
            </div>
            <nav class="flex items-center gap-1" aria-label="Pagination">
                <button
                    type="button"
                    class="rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    disabled=move || !can_go_previous.get()
                    on:click=move |_| state.set_page(page.get().saturating_sub(1))
                >"Previous"</button>
                <span class="px-2 text-sm text-[var(--text-secondary)]">{move || format!("{} / {}", page.get(), total_pages.get())}</span>
                <button
                    type="button"
                    class="rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    disabled=move || !can_go_next.get()
                    on:click=move |_| state.set_page(page.get() + 1)
                >"Next"</button>
            </nav>
        </div>
    }
}

#[component]
fn SortHeader(
    state: SocketState,
    mode: SocketSortMode,
    align: &'static str,
    children: Children,
) -> impl IntoView {
    let is_active = move || state.sort_mode.get() == mode;
    let indicator = move || {
        if !is_active() {
            " ↕"
        } else {
            match state.sort_direction.get() {
                SocketSortDirection::Ascending => " ↑",
                SocketSortDirection::Descending => " ↓",
            }
        }
    };
    let alignment = if align == "right" {
        "ml-auto justify-end"
    } else {
        "justify-start"
    };

    view! {
        <button
            type="button"
            class=format!("flex w-full items-center gap-1 rounded px-1 py-1 font-medium text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] {alignment}")
            aria-label=move || format!("Sort by {}", if is_active() { "selected column" } else { "column" })
            aria-pressed=move || is_active().to_string()
            on:click=move |_| state.set_sort(mode)
        >
            <span>{children()}</span>
            <span aria-hidden="true">{indicator}</span>
        </button>
    }
}

fn socket_mobile_sort_button(
    key: &'static str,
    selected: SocketSortMode,
    state: SocketState,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=move || if state.sort_mode.get() == selected {
                "min-h-9 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-2 text-xs font-medium text-[var(--accent)]"
            } else {
                "min-h-9 rounded-md border border-[var(--border-color)] px-2 text-xs text-[var(--text-primary)] hover:bg-[var(--surface)]"
            }
            on:click=move |_| state.set_sort(selected)
            aria-label=move || format!("{} {}", t_string!(i18n, socket_sort), socket_sort_label(i18n, key))
        >
            {move || if state.sort_mode.get() == selected {
                format!("{} {}", socket_sort_label(i18n, key), match state.sort_direction.get() {
                    SocketSortDirection::Ascending => "↑",
                    SocketSortDirection::Descending => "↓",
                })
            } else {
                socket_sort_label(i18n, key)
            }}
        </button>
    }
}

fn socket_sort_label(i18n: leptos_i18n::I18nContext<Locale>, key: &'static str) -> String {
    match key {
        "symbol" => "Symbol".to_string(),
        "ranking" => t_string!(i18n, socket_ranking).to_string(),
        "price" => t_string!(i18n, socket_price).to_string(),
        "change24h" => t_string!(i18n, socket_change24h).to_string(),
        "funding" => t_string!(i18n, socket_funding).to_string(),
        "volume24h" => t_string!(i18n, socket_volume24h).to_string(),
        _ => t_string!(i18n, socket_sort).to_string(),
    }
}

#[component]
fn TickerTableRow(ticker: TrackedFuturesTicker, state: SocketState) -> impl IntoView {
    let symbol = ticker.ticker.symbol.clone();
    let is_pinned = Memo::new({
        let pinned_symbols = state.pinned_symbols;
        let symbol = symbol.clone();
        move |_| pinned_symbols.get().iter().any(|item| item == &symbol)
    });
    let funding_rate = Memo::new({
        let funding_rates = state.funding_rates;
        let symbol = symbol.clone();
        move |_| {
            funding_rates
                .get()
                .and_then(|snapshot| snapshot.get(&symbol))
        }
    });
    let change_class = change_class(ticker.ticker.change_24h);
    let symbol_title = symbol.clone();
    let symbol_aria_title = symbol.clone();
    let symbol_aria_label = symbol.clone();
    let symbol_click = symbol.clone();
    let trade_label_title = symbol.clone();
    let trade_label_aria = symbol.clone();
    let trade_click_symbol = symbol.clone();
    let is_held = Memo::new({
        let trading_snapshot = state.trading_snapshot;
        let symbol = symbol.clone();
        move |_| {
            trading_snapshot
                .get()
                .portfolio
                .positions
                .iter()
                .any(|position| position.symbol == symbol)
        }
    });
    view! {
        <tr class=move || if is_pinned.get() { "border-b border-[var(--accent)]/50 bg-[var(--accent)]/5 last:border-b-0 hover:bg-[var(--surface-hover)]" } else { "border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]" }>
            <td class="px-3 py-2 text-center">
                <div class="flex items-center justify-center gap-1">
                    <button type="button" class="min-h-11 min-w-11 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                        title=move || if is_pinned.get() { format!("Unpin {}", symbol_title) } else { format!("Pin {}", symbol_aria_title) }
                        aria-label=move || if is_pinned.get() { format!("Unpin {}", symbol_aria_label) } else { format!("Pin {}", symbol_aria_label) }
                        on:click=move |_| state.toggle_pin(&symbol_click)>
                        {move || if is_pinned.get() { "★" } else { "☆" }}
                    </button>
                    <button type="button" class="min-h-9 rounded-md border border-[var(--success)]/60 px-2 text-xs font-bold text-[var(--success)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--success)]/40"
                        title=move || format!("Trade {}", trade_label_title)
                        aria-label=move || format!("Trade {}", trade_label_aria)
                        on:click=move |_| state.trade(&trade_click_symbol)>
                        {move || if is_held.get() { "CLOSE" } else if state.trading_snapshot.get().settings.position_side == crate::domain::trading::PositionSide::Short { "SELL" } else { "BUY" }}
                    </button>
                </div>
            </td>
            <th class="px-3 py-2 text-left font-semibold text-[var(--text-primary)]" scope="row">
                <span class="font-mono">{symbol.clone()}</span>
            </th>
            <td class="px-3 py-2 text-right">
                <div class="flex min-w-28 items-center justify-end gap-2">
                    <progress class="socket-ticker-progress w-20" max="100" value=ticker.ranking.ranking_score().to_string() aria-label="Ranking"></progress>
                    <span class="font-mono font-semibold text-[var(--accent)]">{ticker.ranking.ranking_score()}</span>
                </div>
            </td>
            <td class="px-3 py-2 text-right font-mono font-medium text-[var(--text-primary)]">{format_number(ticker.ticker.last_price)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_percent(ticker.ticker.change_24h)}</td>
            <td class=move || format!("px-3 py-2 text-right {}", funding_rate_class(funding_rate.get()))>{move || format_funding_rate(funding_rate.get())}</td>
            <td class="px-3 py-2 text-right font-semibold">{ranking_direction_label(ticker.ranking.ranking_direction())}</td>
            <td class="px-3 py-2 text-right font-mono">{format_short_percent(ticker.ranking.return_1m())}</td>
            <td class="px-3 py-2 text-right font-mono">{format_short_percent(ticker.ranking.return_3m())}</td>
            <td class="px-3 py-2">{socket_analysis_actions(symbol.clone(), state)}</td>
        </tr>
    }
}

#[component]
fn TickerMobileCard(ticker: TrackedFuturesTicker, state: SocketState) -> impl IntoView {
    let symbol = ticker.ticker.symbol.clone();
    let is_pinned = Memo::new({
        let pinned_symbols = state.pinned_symbols;
        let symbol = symbol.clone();
        move |_| pinned_symbols.get().iter().any(|item| item == &symbol)
    });
    let funding_rate = Memo::new({
        let funding_rates = state.funding_rates;
        let symbol = symbol.clone();
        move |_| {
            funding_rates
                .get()
                .and_then(|snapshot| snapshot.get(&symbol))
        }
    });
    let change_class = change_class(ticker.ticker.change_24h);
    let symbol_title = symbol.clone();
    let symbol_aria_title = symbol.clone();
    let symbol_aria_label = symbol.clone();
    let symbol_click = symbol.clone();
    let trade_label_title = symbol.clone();
    let trade_label_aria = symbol.clone();
    let trade_click_symbol = symbol.clone();
    let is_held = Memo::new({
        let trading_snapshot = state.trading_snapshot;
        let symbol = symbol.clone();
        move |_| {
            trading_snapshot
                .get()
                .portfolio
                .positions
                .iter()
                .any(|position| position.symbol == symbol)
        }
    });
    view! {
        <article class=move || if is_pinned.get() { "rounded-lg border border-[var(--accent)]/60 bg-[var(--accent)]/5 p-3 shadow-sm" } else { "rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3 shadow-sm" }>
            <div class="flex items-start gap-2">
                <div class="min-w-0 flex-1"><h3 class="m-0 truncate font-mono text-base font-semibold text-[var(--text-primary)]">{symbol.clone()}</h3>
                    <div class="mt-1 flex items-center gap-2"><span class="font-mono font-medium text-[var(--text-primary)]">{format_number(ticker.ticker.last_price)}</span><span class=format!("font-medium {change_class}")>{format_percent(ticker.ticker.change_24h)}</span></div></div>
                <div class="flex shrink-0 items-center gap-1">
                    <button type="button" class="min-h-11 min-w-11 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                        title=move || if is_pinned.get() { format!("Unpin {}", symbol_title) } else { format!("Pin {}", symbol_aria_title) }
                        aria-label=move || if is_pinned.get() { format!("Unpin {}", symbol_aria_label) } else { format!("Pin {}", symbol_aria_label) }
                        on:click=move |_| state.toggle_pin(&symbol_click)>
                        {move || if is_pinned.get() { "★" } else { "☆" }}
                    </button>
                    <button type="button" class="min-h-11 rounded-md border border-[var(--success)]/60 px-3 text-xs font-bold text-[var(--success)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--success)]/40"
                        title=move || format!("Trade {}", trade_label_title)
                        aria-label=move || format!("Trade {}", trade_label_aria)
                        on:click=move |_| state.trade(&trade_click_symbol)>
                        {move || if is_held.get() { "CLOSE" } else if state.trading_snapshot.get().settings.position_side == crate::domain::trading::PositionSide::Short { "SELL" } else { "BUY" }}
                    </button>
                </div>
            </div>
            <div class="mt-3 flex flex-wrap gap-2">
                {socket_analysis_actions(symbol.clone(), state)}
            </div>
            <div class="mt-3 grid grid-cols-2 gap-x-3 gap-y-2 text-sm">
                <div><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_funding)}</span><span class=move || funding_rate_class(funding_rate.get())>{move || format_funding_rate(funding_rate.get())}</span></div>
                <div class="text-right"><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_ranking)}</span><span class="font-mono text-[var(--accent)]">{ticker.ranking.ranking_score()}</span></div>
                <div><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_direction)}</span><span class="font-semibold">{ranking_direction_label(ticker.ranking.ranking_direction())}</span></div>
                <div><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_change1m)}</span><span class="font-mono">{format_short_percent(ticker.ranking.return_1m())}</span></div>
                <div class="text-right"><span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(use_i18n(),socket_change3m)}</span><span class="font-mono">{format_short_percent(ticker.ranking.return_3m())}</span></div>
            </div>
        </article>
    }
}

#[component]
fn SocketAnalysisModal(
    state: SocketState,
    result: crate::domain::technical_analysis::AnalysisResult,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-[var(--overlay)] p-3" role="presentation">
            <div class="flex max-h-[90vh] w-full max-w-3xl flex-col overflow-hidden rounded-lg border border-[var(--border-color)] bg-[var(--surface)] shadow-lg" role="dialog" aria-modal="true" aria-label="Technical analysis">
                <header class="flex shrink-0 items-center justify-between gap-3 border-b border-[var(--border-color)] px-4 py-3">
                    <div>
                        <h2 class="m-0 text-lg font-semibold">{format!("{} · {}", result.asset.symbol, result.asset.timeframe)}</h2>
                        <p class="m-0 mt-1 text-xs text-[var(--text-secondary)]">{result.engine.name.clone()}</p>
                    </div>
                    <button type="button" class="min-h-10 min-w-10 rounded-md border border-[var(--border-color)] px-2 text-lg text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title=move || t_string!(i18n, common_close) aria-label=move || t_string!(i18n, common_close) on:click=move |_| state.close_analysis()>"×"</button>
                </header>
                <div class="min-h-0 overflow-y-auto p-4">
                    <div class="grid gap-3 md:grid-cols-2">
                        <section class="rounded-md border border-[var(--border-color)] p-3">
                            <h3 class="mb-2 text-sm font-semibold">{move || t_string!(i18n, market_analysis_summary)}</h3>
                            <p class="m-0">{format!("{} · {}", socket_analysis_term(i18n, &result.trend.state), socket_analysis_term(i18n, &result.trend.strength))}</p>
                            <p class="m-0 mt-1 text-sm text-[var(--text-secondary)]">{format!("Close: {:.4} · RSI: {}", result.snapshot.close, result.momentum.rsi.value.map(|v| format!("{v:.2}")).unwrap_or_else(|| "N/A".to_string()))}</p>
                        </section>
                        <section class="rounded-md border border-[var(--border-color)] p-3">
                            <h3 class="mb-2 text-sm font-semibold">{move || t_string!(i18n, market_analysis_signals)}</h3>
                            {if result.signals.is_empty() {
                                view! { <p class="m-0 text-[var(--text-secondary)]">{move || t_string!(i18n, market_analysis_none)}</p> }.into_any()
                            } else {
                                view! { <ul class="m-0 pl-5">{result.signals.iter().map(|signal| view! { <li>{format!("{} — {} / {}", socket_analysis_term(i18n, &signal.direction), socket_analysis_term(i18n, &signal.category), socket_analysis_term(i18n, &signal.strength))}</li> }).collect_view()}</ul> }.into_any()
                            }}
                        </section>
                        <section class="rounded-md border border-[var(--border-color)] p-3">
                            <h3 class="mb-2 text-sm font-semibold">{move || t_string!(i18n, market_analysis_momentum_section)}</h3>
                            <p class="m-0">{format!("RSI {} · MACD {} / {} / {}", result.momentum.rsi.value.map(|v| format!("{v:.2}")).unwrap_or_else(|| "N/A".to_string()), result.momentum.macd.macd.map(|v| format!("{v:.4}")).unwrap_or_else(|| "N/A".to_string()), result.momentum.macd.signal.map(|v| format!("{v:.4}")).unwrap_or_else(|| "N/A".to_string()), result.momentum.macd.histogram.map(|v| format!("{v:.4}")).unwrap_or_else(|| "N/A".to_string()))}</p>
                        </section>
                        <section class="rounded-md border border-[var(--border-color)] p-3">
                            <h3 class="mb-2 text-sm font-semibold">{move || t_string!(i18n, market_analysis_structure_section)}</h3>
                            <p class="m-0">{if result.market_structure.structure_sequence.is_empty() { t_string!(i18n, market_analysis_none).to_string() } else { result.market_structure.structure_sequence.join(" → ") }}</p>
                            <p class="m-0 mt-1 text-sm text-[var(--text-secondary)]">{format!("ATR: {} · Volume: {:.2}", result.volatility.atr.map(|v| format!("{v:.4}")).unwrap_or_else(|| "N/A".to_string()), result.volume.current)}</p>
                        </section>
                    </div>
                    <section class="mt-3 rounded-md border border-[var(--border-color)] p-3">
                        <h3 class="mb-2 text-sm font-semibold">{move || t_string!(i18n, market_analysis_data_quality)}</h3>
                        <p class="m-0">{format!("{} / {} · {}", result.data_quality.candles_used, result.data_quality.candles_received, if result.data_quality.sufficient_for_analysis { t_string!(i18n, market_analysis_yes) } else { t_string!(i18n, market_analysis_no) })}</p>
                    </section>
                    <Show when=move || state.analysis_error.get().is_some()>
                        <p class="m-0 mt-3 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 p-3 text-sm text-[var(--danger)]">{move || state.analysis_error.get().unwrap_or_default()}</p>
                    </Show>
                </div>
                <footer class="flex shrink-0 justify-end gap-2 border-t border-[var(--border-color)] px-4 py-3">
                    <button type="button" class="min-h-10 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.close_analysis()>{move || t_string!(i18n, common_close)}</button>
                    <button type="button" class="min-h-10 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-3 py-2 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.copy_analysis()>{move || if state.analysis_copied.get() { t_string!(i18n, common_copied) } else { t_string!(i18n, common_copy) }}</button>
                </footer>
            </div>
        </div>
    }
}

fn socket_analysis_actions(symbol: String, state: SocketState) -> impl IntoView {
    let symbol_4h = symbol.clone();
    let symbol_1d = symbol.clone();
    let symbol_copy_4h = symbol.clone();
    let symbol_copy_1d = symbol;
    let i18n = use_i18n();
    view! {
        <div class="flex flex-wrap gap-1">
            <button type="button" class="min-h-9 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-2 py-1 text-xs font-medium text-[var(--accent)] hover:bg-[var(--accent)]/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50" disabled=move || state.analysis_loading.get() on:click=move |_| state.analyze_symbol(&symbol_4h, "4H")>{move || t_string!(i18n, socket_analyze_4h)}</button>
            <button type="button" class="min-h-9 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-2 py-1 text-xs font-medium text-[var(--accent)] hover:bg-[var(--accent)]/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50" disabled=move || state.analysis_loading.get() on:click=move |_| state.analyze_symbol(&symbol_1d, "1D")>{move || t_string!(i18n, socket_analyze_1d)}</button>
            <button type="button" class="min-h-9 rounded-md border border-[var(--border-color)] px-2 py-1 text-xs text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50" disabled=move || state.analysis_loading.get() on:click=move |_| state.copy_symbol_analysis(&symbol_copy_4h, "4H")>{move || t_string!(i18n, socket_copy_4h)}</button>
            <button type="button" class="min-h-9 rounded-md border border-[var(--border-color)] px-2 py-1 text-xs text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50" disabled=move || state.analysis_loading.get() on:click=move |_| state.copy_symbol_analysis(&symbol_copy_1d, "1D")>{move || t_string!(i18n, socket_copy_1d)}</button>
        </div>
    }
}

fn socket_analysis_term(i18n: leptos_i18n::I18nContext<Locale>, value: &str) -> String {
    match value {
        "bullish" => t_string!(i18n, market_analysis_bullish).to_string(),
        "bearish" => t_string!(i18n, market_analysis_bearish).to_string(),
        "neutral" => t_string!(i18n, market_analysis_neutral).to_string(),
        "positive" => t_string!(i18n, market_analysis_positive).to_string(),
        "negative" => t_string!(i18n, market_analysis_negative).to_string(),
        "weak" => t_string!(i18n, market_analysis_weak).to_string(),
        "moderate" => t_string!(i18n, market_analysis_moderate).to_string(),
        "strong" => t_string!(i18n, market_analysis_strong).to_string(),
        "confirmed" => t_string!(i18n, market_analysis_confirmed).to_string(),
        "possible" => t_string!(i18n, market_analysis_possible).to_string(),
        "active" => t_string!(i18n, market_analysis_active).to_string(),
        "invalidated" => t_string!(i18n, market_analysis_invalidated).to_string(),
        "increasing" => t_string!(i18n, market_analysis_increasing).to_string(),
        "decreasing" => t_string!(i18n, market_analysis_decreasing).to_string(),
        "unavailable" => t_string!(i18n, market_analysis_unavailable).to_string(),
        "none" => t_string!(i18n, market_analysis_none).to_string(),
        _ => value.replace('_', " "),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_visible(
    all: MarketSnapshot,
    mode: SocketViewMode,

    sort: SocketSortMode,
    direction: SocketSortDirection,
    pinned_symbols: Vec<String>,
    funding_rates: Option<FundingRateSnapshot>,
    search_query: String,
) -> Vec<TrackedFuturesTicker> {
    let query = search_query.trim().to_uppercase();
    let sort_fn = |left: &TrackedFuturesTicker, right: &TrackedFuturesTicker| {
        let cmp = match sort {
            SocketSortMode::Symbol => left.ticker.symbol.cmp(&right.ticker.symbol),
            SocketSortMode::Ranking => right
                .momentum
                .ranking_score()
                .cmp(&left.momentum.ranking_score())
                .then_with(|| left.ticker.symbol.cmp(&right.ticker.symbol)),
            SocketSortMode::Ranking => right
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
        };

        match direction {
            SocketSortDirection::Descending => cmp,
            SocketSortDirection::Ascending => cmp.reverse(),
        }
    };
    let pinned_set = pinned_symbols
        .iter()
        .collect::<std::collections::HashSet<_>>();

    if mode == SocketViewMode::PinnedOnly {
        let mut pinned = all
            .values()
            .filter(|item| item.ticker.symbol.contains(&query))
            .filter(|item| pinned_set.contains(&item.ticker.symbol))
            .cloned()
            .collect::<Vec<_>>();

        pinned.sort_unstable_by(sort_fn);
        return pinned;
    }

    let mut pinned = all
        .values()
        .filter(|item| pinned_set.contains(&item.ticker.symbol))
        .filter(|item| item.ticker.symbol.contains(&query))
        .cloned()
        .collect::<Vec<_>>();
    pinned.sort_unstable_by(sort_fn);

    let mut dynamic = all
        .values()
        .filter(|item| !pinned_set.contains(&item.ticker.symbol))
        .filter(|item| item.ticker.symbol.contains(&query))
        .cloned()
        .collect::<Vec<_>>();
    dynamic.sort_unstable_by(sort_fn);

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

fn ranking_direction_label(direction: i8) -> &'static str {
    match direction {
        1 => "LONG ↑",
        -1 => "SHORT ↓",
        _ => "—",
    }
}

fn format_short_percent(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:+.2}%", number = number * 100.0))
        .unwrap_or_else(|| "—".into())
}

fn format_funding_rate(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:+.4}%", number = number * 100.0))
        .unwrap_or_else(|| "—".into())
}
