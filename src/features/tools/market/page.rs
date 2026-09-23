use leptos::prelude::*;

use crate::domain::market::MarketStock;
use crate::features::tools::market::state::MarketState;

#[derive(Clone, Copy, PartialEq)]
enum MarketFilter {
    All,
    Gainers,
    Losers,
    Unchanged,
}

#[derive(Clone, Copy, PartialEq)]
enum MarketSort {
    Symbol,
    Price,
    ChangePercent,
    Volume,
    MarketCap,
}

#[component]
pub fn MarketPage() -> impl IntoView {
    let state = MarketState::new();
    let search = RwSignal::new(String::new());
    let filter = RwSignal::new(MarketFilter::All);
    let sort = RwSignal::new(MarketSort::Symbol);
    let descending = RwSignal::new(false);

    let toggle_sort = move |selected: MarketSort| {
        if sort.get_untracked() == selected {
            descending.update(|value| *value = !*value);
        } else {
            sort.set(selected);
            descending.set(false);
        }
    };

    state.load_pins();
    state.load();

    let visible_stocks = Memo::new(move |_| {
        let query = search.get().trim().to_lowercase();
        let selected_filter = filter.get();
        let selected_sort = sort.get();
        let is_descending = descending.get();
        let pinned_symbols = state.pinned_symbols.get();

        let mut stocks: Vec<MarketStock> = state
            .stocks
            .get()
            .into_iter()
            .filter(|stock| {
                let matches_search = query.is_empty()
                    || stock.symbol.to_lowercase().contains(&query)
                    || stock.name.to_lowercase().contains(&query);

                let matches_filter = match selected_filter {
                    MarketFilter::All => true,
                    MarketFilter::Gainers => stock.change_percent > 0.0,
                    MarketFilter::Losers => stock.change_percent < 0.0,
                    MarketFilter::Unchanged => stock.change_percent == 0.0,
                };

                matches_search && matches_filter
            })
            .collect();

        stocks.sort_by(|left, right| {
            let left_pinned = pinned_symbols.iter().any(|symbol| symbol == &left.symbol);
            let right_pinned = pinned_symbols.iter().any(|symbol| symbol == &right.symbol);
            let pin_ordering = right_pinned.cmp(&left_pinned);
            if pin_ordering != std::cmp::Ordering::Equal {
                return pin_ordering;
            }

            let ordering = match selected_sort {
                MarketSort::Symbol => left.symbol.cmp(&right.symbol),
                MarketSort::Price => left.price.total_cmp(&right.price),
                MarketSort::ChangePercent => left.change_percent.total_cmp(&right.change_percent),
                MarketSort::Volume => left.total_volume.total_cmp(&right.total_volume),
                MarketSort::MarketCap => left.market_cap.total_cmp(&right.market_cap),
            };

            if is_descending {
                ordering.reverse()
            } else {
                ordering
            }
        });

        stocks
    });

    view! {
        <div class="flex flex-grow flex-col overflow-hidden">
            <header class="flex shrink-0 flex-col gap-2 border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-2">
                <div class="flex flex-wrap items-center gap-2">
                    <h1 class="m-0 text-base font-semibold text-[var(--text-primary)]">
                        <span class="mr-2 text-[var(--accent)]" aria-hidden="true">"◈"</span>
                        "Market"
                    </h1>
                    <span class="text-xs text-[var(--text-secondary)]" aria-live="polite">
                        {move || if state.loading.get() {
                            "Loading...".to_string()
                        } else {
                            format!("{} / {}", visible_stocks.get().len(), state.total_items.get())
                        }}
                    </span>
                    <button
                        type="button"
                        class="ml-auto min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                        title="Refresh market data"
                        disabled=move || state.loading.get()
                        on:click=move |_| state.load()
                    >
                        "Refresh"
                    </button>
                </div>

                <div class="flex flex-col gap-2 lg:flex-row lg:items-center">
                    <label class="sr-only" for="market-search">"Search market"</label>
                    <div class="relative flex-grow lg:max-w-md">
                        <input
                            id="market-search"
                            type="search"
                            class="min-h-11 w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 pr-10 text-sm text-[var(--text-primary)] outline-none placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/40"
                            placeholder="Search symbol or company name..."
                            aria-label="Search symbol or company name"
                            prop:value=move || search.get()
                            on:input=move |event| search.set(event_target_value(&event))
                        />
                        {move || if search.get().is_empty() {
                            view! { <span></span> }.into_any()
                        } else {
                            view! {
                                <button
                                    type="button"
                                    class="absolute right-1 top-1/2 min-h-9 -translate-y-1/2 rounded px-2 text-sm text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                    title="Clear search"
                                    aria-label="Clear search"
                                    on:click=move |_| search.set(String::new())
                                >
                                    "×"
                                </button>
                            }.into_any()
                        }}
                    </div>

                    <label class="sr-only" for="market-filter">"Filter market stocks"</label>
                    <select
                        id="market-filter"
                        class="min-h-11 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/40"
                        on:change=move |event| {
                            filter.set(match event_target_value(&event).as_str() {
                                "gainers" => MarketFilter::Gainers,
                                "losers" => MarketFilter::Losers,
                                "unchanged" => MarketFilter::Unchanged,
                                _ => MarketFilter::All,
                            });
                        }
                    >
                        <option value="all">"All"</option>
                        <option value="gainers">"Gainers"</option>
                        <option value="losers">"Losers"</option>
                        <option value="unchanged">"Unchanged"</option>
                    </select>

                </div>

                <p class="m-0 text-sm text-[var(--text-secondary)]">"Market data supplied directly by CafeF."</p>
            </header>

            {move || state.error.get().map(|error| view! {
                <div class="flex shrink-0 items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">
                    <span aria-hidden="true">"!"</span><span>{error}</span>
                </div>
            })}

            <div class="min-h-0 flex-grow overflow-auto p-3">
                <div class="hidden overflow-x-auto rounded-lg border border-[var(--border-color)] bg-[var(--surface)] md:block">
                    <table class="w-full min-w-[820px] border-collapse text-sm">
                        <caption class="sr-only">"CafeF market stock data"</caption>
                        <thead>
                            <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-left text-[var(--text-secondary)]">
                                <th class="px-3 py-2 font-medium" scope="col">"Pin"</th>
                                {sortable_header("Symbol", MarketSort::Symbol, sort, descending, toggle_sort)}
                                <th class="px-3 py-2 font-medium" scope="col">"Name"</th>
                                {sortable_header("Price", MarketSort::Price, sort, descending, toggle_sort)}
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Change"</th>
                                {sortable_header("Change %", MarketSort::ChangePercent, sort, descending, toggle_sort)}
                                {sortable_header("Volume", MarketSort::Volume, sort, descending, toggle_sort)}
                                {sortable_header("Market Cap", MarketSort::MarketCap, sort, descending, toggle_sort)}
                            </tr>
                        </thead>
                        <tbody>
                            {move || visible_stocks.get().into_iter().map(|stock| market_table_row(stock, state)).collect_view()}
                        </tbody>
                    </table>
                </div>

                <div class="flex flex-col gap-2 md:hidden">
                    <div class="flex items-center justify-between gap-2 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                        <span class="shrink-0 text-xs font-medium text-[var(--text-secondary)]">"Sort"</span>
                        <div class="flex flex-wrap justify-end gap-1">
                            {mobile_sort_button("Symbol", MarketSort::Symbol, sort, descending, toggle_sort)}
                            {mobile_sort_button("Price", MarketSort::Price, sort, descending, toggle_sort)}
                            {mobile_sort_button("Change %", MarketSort::ChangePercent, sort, descending, toggle_sort)}
                            {mobile_sort_button("Volume", MarketSort::Volume, sort, descending, toggle_sort)}
                            {mobile_sort_button("Market Cap", MarketSort::MarketCap, sort, descending, toggle_sort)}
                        </div>
                    </div>
                    {move || visible_stocks.get().into_iter().map(|stock| market_mobile_card(stock, state)).collect_view()}
                </div>

                {move || if !state.loading.get() && state.error.get().is_none() && visible_stocks.get().is_empty() {
                    view! { <div class="mt-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-4 text-sm text-[var(--text-secondary)]" role="status">"No market data matches the current filters."</div> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                <div class="mt-3 text-xs text-[var(--text-secondary)]" role="note">
                    {move || format!("Displayed: {} · Total: {}", visible_stocks.get().len(), state.total_items.get())}
                </div>
            </div>
        </div>
    }
}


fn market_table_row(stock: MarketStock, state: MarketState) -> impl IntoView {
    let change_class = change_class(stock.change_percent);
    let symbol = stock.symbol.clone();
    let pin_symbol = symbol.clone();
    let is_pinned = Memo::new(move |_| {
        state.pinned_symbols.get().iter().any(|item| item == &symbol)
    });

    view! {
        <tr class=move || if is_pinned.get() {
            "border-b border-[var(--accent)]/50 bg-[var(--accent)]/5 last:border-b-0 hover:bg-[var(--surface-hover)]"
        } else {
            "border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]"
        }>
            <td class="px-3 py-2 text-center">{pin_button(pin_symbol, is_pinned, state)}</td>
            <th class="px-3 py-2 text-left font-semibold text-[var(--text-primary)]" scope="row">{stock.symbol}</th>
            <td class="max-w-[28rem] px-3 py-2 text-[var(--text-secondary)]">{stock.name}</td>
            <td class="px-3 py-2 text-right font-medium text-[var(--text-primary)]">{format_price(stock.price)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_price(stock.change)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_percent(stock.change_percent)}</td>
            <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.total_volume)}</td>
            <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.market_cap)}</td>
        </tr>
    }
}

fn market_mobile_card(stock: MarketStock, state: MarketState) -> impl IntoView {
    let change_class = change_class(stock.change_percent);
    let symbol = stock.symbol.clone();
    let pin_symbol = symbol.clone();
    let is_pinned = Memo::new(move |_| {
        state.pinned_symbols.get().iter().any(|item| item == &symbol)
    });

    view! {
        <article class=move || if is_pinned.get() {
            "rounded-lg border border-[var(--accent)]/60 bg-[var(--accent)]/5 p-3 shadow-sm"
        } else {
            "rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3 shadow-sm"
        }>
            <div class="flex items-start gap-2">
                <div class="min-w-0 flex-1">
                    <div class="flex items-center gap-2">
                        <h2 class="m-0 truncate text-base font-semibold text-[var(--text-primary)]">{stock.symbol}</h2>
                        <span class=format!("shrink-0 text-sm font-semibold {change_class}")>{format_percent(stock.change_percent)}</span>
                    </div>
                    <p class="m-0 mt-1 truncate text-xs text-[var(--text-secondary)]">{stock.name}</p>
                </div>
                {pin_button(pin_symbol, is_pinned, state)}
            </div>
            <div class="mt-3 grid grid-cols-2 gap-x-3 gap-y-2 text-sm">
                <div>
                    <span class="block text-xs text-[var(--text-secondary)]">"Price"</span>
                    <span class="font-semibold text-[var(--text-primary)]">{format_price(stock.price)}</span>
                </div>
                <div class="text-right">
                    <span class="block text-xs text-[var(--text-secondary)]">"Change"</span>
                    <span class=format!("font-medium {change_class}")>{format_price(stock.change)}</span>
                </div>
                <div>
                    <span class="block text-xs text-[var(--text-secondary)]">"Volume"</span>
                    <span class="font-medium text-[var(--text-primary)]">{format_integer(stock.total_volume)}</span>
                </div>
                <div class="text-right">
                    <span class="block text-xs text-[var(--text-secondary)]">"Market Cap"</span>
                    <span class="font-medium text-[var(--text-primary)]">{format_integer(stock.market_cap)}</span>
                </div>
            </div>
        </article>
    }
}

fn pin_button(symbol: String, is_pinned: Memo<bool>, state: MarketState) -> impl IntoView {
    view! {
        <button
            type="button"
            class="min-h-11 min-w-11 shrink-0 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
            title=move || if is_pinned.get() { "Unpin stock" } else { "Pin stock" }
            aria-label=move || if is_pinned.get() { "Unpin stock" } else { "Pin stock" }
            on:click=move |_| state.toggle_pin(&symbol)
        >
            {move || if is_pinned.get() { "★" } else { "☆" }}
        </button>
    }
}

fn mobile_sort_button(
    label: &'static str,
    selected: MarketSort,
    sort: RwSignal<MarketSort>,
    descending: RwSignal<bool>,
    on_sort: impl Fn(MarketSort) + Copy + 'static,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=move || if sort.get() == selected {
                "min-h-9 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-2 text-xs font-medium text-[var(--accent)]"
            } else {
                "min-h-9 rounded-md border border-[var(--border-color)] px-2 text-xs text-[var(--text-primary)]"
            }
            on:click=move |_| on_sort(selected)
            aria-label=move || format!("Sort by {label}")
        >
            {move || if sort.get() == selected {
                format!("{label} {}", if descending.get() { "↓" } else { "↑" })
            } else {
                label.to_string()
            }}
        </button>
    }
}

fn change_class(change_percent: f64) -> &'static str {
    if change_percent > 0.0 {
        "text-[var(--success)]"
    } else if change_percent < 0.0 {
        "text-[var(--danger)]"
    } else {
        "text-[var(--text-secondary)]"
    }
}

fn sortable_header(
    label: &'static str,
    selected: MarketSort,
    sort: RwSignal<MarketSort>,
    descending: RwSignal<bool>,
    on_sort: impl Fn(MarketSort) + Copy + 'static,
) -> impl IntoView {
    view! {
        <th class="px-3 py-2 font-medium" scope="col">
            <button
                type="button"
                class="flex min-h-9 items-center gap-1 rounded px-1 text-left hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                aria-label=move || {
                    if sort.get() == selected {
                        format!("Sort by {label} {}", if descending.get() { "descending" } else { "ascending" })
                    } else {
                        format!("Sort by {label}")
                    }
                }
                on:click=move |_| on_sort(selected)
            >
                <span>{label}</span>
                {move || if sort.get() == selected {
                    view! { <span aria-hidden="true">{if descending.get() { "↓" } else { "↑" }}</span> }.into_any()
                } else {
                    view! { <span class="text-[var(--text-secondary)] opacity-50" aria-hidden="true">"↕"</span> }.into_any()
                }}
            </button>
        </th>
    }
}

fn format_price(value: f64) -> String {
    format!("{value:.2}")
}

fn format_percent(value: f64) -> String {
    format!("{value:+.2}%")
}

fn format_integer(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
    }
}
