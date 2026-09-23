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

                    <label class="sr-only" for="market-sort">"Sort market stocks"</label>
                    <select
                        id="market-sort"
                        class="min-h-11 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/40"
                        on:change=move |event| {
                            sort.set(match event_target_value(&event).as_str() {
                                "price" => MarketSort::Price,
                                "change" => MarketSort::ChangePercent,
                                "volume" => MarketSort::Volume,
                                "market_cap" => MarketSort::MarketCap,
                                _ => MarketSort::Symbol,
                            });
                        }
                    >
                        <option value="symbol">"Sort: Symbol"</option>
                        <option value="price">"Sort: Price"</option>
                        <option value="change">"Sort: Change %"</option>
                        <option value="volume">"Sort: Volume"</option>
                        <option value="market_cap">"Sort: Market Cap"</option>
                    </select>

                    <button
                        type="button"
                        class="min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                        title="Toggle sort direction"
                        aria-label="Toggle sort direction"
                        on:click=move |_| descending.update(|value| *value = !*value)
                    >
                        {move || if descending.get() { "↓" } else { "↑" }}
                    </button>
                </div>

                <p class="m-0 text-sm text-[var(--text-secondary)]">"Market data supplied directly by CafeF."</p>
            </header>

            {move || state.error.get().map(|error| view! {
                <div class="flex shrink-0 items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">
                    <span aria-hidden="true">"!"</span><span>{error}</span>
                </div>
            })}

            <div class="min-h-0 flex-grow overflow-auto p-3">
                <div class="overflow-x-auto rounded-lg border border-[var(--border-color)] bg-[var(--surface)]">
                    <table class="w-full min-w-[820px] border-collapse text-sm">
                        <caption class="sr-only">"CafeF market stock data"</caption>
                        <thead>
                            <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-left text-[var(--text-secondary)]">
                                <th class="px-3 py-2 font-medium" scope="col">"Pin"</th>
                                <th class="px-3 py-2 font-medium" scope="col">"Symbol"</th>
                                <th class="px-3 py-2 font-medium" scope="col">"Name"</th>
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Price"</th>
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Change"</th>
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Change %"</th>
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Volume"</th>
                                <th class="px-3 py-2 text-right font-medium" scope="col">"Market Cap"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || visible_stocks.get().into_iter().map(|stock| {
                                let change_class = if stock.change_percent > 0.0 {
                                    "text-[var(--success)]"
                                } else if stock.change_percent < 0.0 {
                                    "text-[var(--danger)]"
                                } else {
                                    "text-[var(--text-secondary)]"
                                };
                                let symbol = stock.symbol.clone();
                                let pin_symbol = symbol.clone();
                                let is_pinned = move || state.pinned_symbols.get().iter().any(|item| item == &symbol);
                                view! {
                                    <tr class="border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]">
                                        <td class="px-3 py-2 text-center">
                                            <button
                                                type="button"
                                                class="min-h-9 min-w-9 rounded-md border border-[var(--border-color)] px-2 py-1 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                                                title=move || if is_pinned() { "Unpin stock" } else { "Pin stock" }
                                                aria-label=move || if is_pinned() { "Unpin stock" } else { "Pin stock" }
                                                on:click=move |_| state.toggle_pin(&pin_symbol)
                                            >
                                                {move || if is_pinned() { "★" } else { "☆" }}
                                            </button>
                                        </td>
                                        <th class="px-3 py-2 text-left font-semibold text-[var(--text-primary)]" scope="row">{stock.symbol}</th>
                                        <td class="max-w-[28rem] px-3 py-2 text-[var(--text-secondary)]">{stock.name}</td>
                                        <td class="px-3 py-2 text-right font-medium text-[var(--text-primary)]">{format_price(stock.price)}</td>
                                        <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_price(stock.change)}</td>
                                        <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_percent(stock.change_percent)}</td>
                                        <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.total_volume)}</td>
                                        <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.market_cap)}</td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                    </table>
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
