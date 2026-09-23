use crate::i18n::*;
use leptos::prelude::*;
use leptos_i18n::t_string;

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
    let i18n = use_i18n();
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
                        {move || t_string!(i18n, nav_market)}
                    </h1>
                    <span class="text-xs text-[var(--text-secondary)]" aria-live="polite">
                        {move || if state.loading.get() {
                            t_string!(i18n, common_loading).to_string()
                        } else {
                            format!("{} / {}", visible_stocks.get().len(), state.total_items.get())
                        }}
                    </span>
                    <button
                        type="button"
                        class="ml-auto min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                        title=move || t_string!(i18n, market_refresh)
                        disabled=move || state.loading.get()
                        on:click=move |_| state.load()
                    >
                        {move || t_string!(i18n, market_refresh)}
                    </button>
                </div>

                <div class="flex flex-col gap-2 lg:flex-row lg:items-center">
                    <label class="sr-only" for="market-search">{move || t_string!(i18n, market_search)}</label>
                    <div class="relative flex-grow lg:max-w-md">
                        <input
                            id="market-search"
                            type="search"
                            class="min-h-11 w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 pr-10 text-sm text-[var(--text-primary)] outline-none placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/40"
                            placeholder=move || t_string!(i18n, market_search_placeholder)
                            aria-label=move || t_string!(i18n, market_search_placeholder)
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
                                    title=move || t_string!(i18n, common_clear)
                                    aria-label=move || t_string!(i18n, common_clear)
                                    on:click=move |_| search.set(String::new())
                                >
                                    "×"
                                </button>
                            }.into_any()
                        }}
                    </div>

                    <label class="sr-only" for="market-filter">{move || t_string!(i18n, market_filter)}</label>
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
                        <option value="all">{move || t_string!(i18n, market_all)}</option>
                        <option value="gainers">{move || t_string!(i18n, market_gainers)}</option>
                        <option value="losers">{move || t_string!(i18n, market_losers)}</option>
                        <option value="unchanged">{move || t_string!(i18n, market_unchanged)}</option>
                    </select>

                </div>

                <p class="m-0 text-sm text-[var(--text-secondary)]">{move || t_string!(i18n, market_source)}</p>
            </header>

            {move || state.error.get().map(|error| view! {
                <div class="flex shrink-0 items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">
                    <span aria-hidden="true">"!"</span><span>{error}</span>
                </div>
            })}
            {move || state.analysis_error.get().map(|error| view! {
                <div class="flex shrink-0 items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">
                    <span aria-hidden="true">"!"</span><span>{error}</span>
                </div>
            })}

            <div class="min-h-0 flex-grow overflow-auto p-3">
                <div class="hidden overflow-x-auto rounded-lg border border-[var(--border-color)] bg-[var(--surface)] md:block">
                    <table class="w-full min-w-[820px] border-collapse text-sm">
                        <caption class="sr-only">{move || t_string!(i18n, market_table_caption)}</caption>
                        <thead>
                            <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-left text-[var(--text-secondary)]">
                                <th class="px-3 py-2 font-medium" scope="col">{move || t_string!(i18n, market_pin)}</th>
                                {sortable_header("symbol", MarketSort::Symbol, sort, descending, toggle_sort, i18n)}
                                <th class="px-3 py-2 font-medium" scope="col">"Name"</th>
                                {sortable_header("price", MarketSort::Price, sort, descending, toggle_sort, i18n)}
                                <th class="px-3 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, market_change)}</th>
                                {sortable_header("change_percent", MarketSort::ChangePercent, sort, descending, toggle_sort, i18n)}
                                {sortable_header("volume", MarketSort::Volume, sort, descending, toggle_sort, i18n)}
                                {sortable_header("market_cap", MarketSort::MarketCap, sort, descending, toggle_sort, i18n)}
                                <th class="px-3 py-2 font-medium" scope="col">{move || t_string!(i18n, market_actions)}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || visible_stocks.get().into_iter().map(|stock| market_table_row(stock, state, i18n)).collect_view()}
                        </tbody>
                    </table>
                </div>

                <div class="flex flex-col gap-2 md:hidden">
                    <div class="flex items-center justify-between gap-2 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                        <span class="shrink-0 text-xs font-medium text-[var(--text-secondary)]">{move || t_string!(i18n, market_sort)}</span>
                        <div class="flex flex-wrap justify-end gap-1">
                            {mobile_sort_button("symbol", MarketSort::Symbol, sort, descending, toggle_sort, i18n)}
                            {mobile_sort_button("price", MarketSort::Price, sort, descending, toggle_sort, i18n)}
                            {mobile_sort_button("change_percent", MarketSort::ChangePercent, sort, descending, toggle_sort, i18n)}
                            {mobile_sort_button("volume", MarketSort::Volume, sort, descending, toggle_sort, i18n)}
                            {mobile_sort_button("market_cap", MarketSort::MarketCap, sort, descending, toggle_sort, i18n)}
                        </div>
                    </div>
                    {move || visible_stocks.get().into_iter().map(|stock| market_mobile_card(stock, state, i18n)).collect_view()}
                </div>

                {move || if !state.loading.get() && state.error.get().is_none() && visible_stocks.get().is_empty() {
                    view! { <div class="mt-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-4 text-sm text-[var(--text-secondary)]" role="status">{move || t_string!(i18n, market_no_results)}</div> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                <div class="mt-3 text-xs text-[var(--text-secondary)]" role="note">
                    {move || format!("{}: {} · {}: {}", t_string!(i18n, market_displayed), visible_stocks.get().len(), t_string!(i18n, market_total), state.total_items.get())}
                </div>
            </div>
            {move || if state.analysis_modal_open.get() {
                view! {
                    <div
                        class="market-analysis-backdrop fixed inset-0 z-50 flex items-center justify-center p-3"
                        role="presentation"
                        on:click=move |_| state.close_analysis()
                    >
                        <section
                            class="flex max-h-[90vh] w-full max-w-5xl flex-col overflow-hidden rounded-lg border border-[var(--border-color)] bg-[var(--surface)] shadow-xl"
                            role="dialog"
                            aria-modal="true"
                            aria-labelledby="market-analysis-title"
                            on:click=move |event| event.stop_propagation()
                        >
                            <header class="flex shrink-0 items-center gap-3 border-b border-[var(--border-color)] px-4 py-3">
                                <div class="min-w-0 flex-1">
                                    <h2 id="market-analysis-title" class="m-0 text-base font-semibold text-[var(--text-primary)]">
                                        {move || format!("{} — {}", t_string!(i18n, market_analysis), state.analysis_symbol.get().unwrap_or_default())}
                                    </h2>
                                </div>
                                <button
                                    type="button"
                                    class="min-h-11 min-w-11 rounded-md border border-[var(--border-color)] px-2 text-xl text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                                    title=move || t_string!(i18n, common_close)
                                    aria-label=move || t_string!(i18n, common_close)
                                    on:click=move |_| state.close_analysis()
                                >
                                    "×"
                                </button>
                            </header>
                            <div class="min-h-0 flex-1 overflow-auto p-4">
                                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] p-4 text-sm leading-relaxed text-[var(--text-primary)]">
                                    <pre class="m-0 whitespace-pre-wrap break-words font-sans">{move || state.analysis_text.get().unwrap_or_default()}</pre>
                                </div>
                            </div>
                            <footer class="flex shrink-0 flex-wrap items-center justify-end gap-2 border-t border-[var(--border-color)] px-4 py-3">
                                <button
                                    type="button"
                                    class="min-h-11 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-3 py-2 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/20 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                                    on:click=move |_| state.copy_analysis()
                                >
                                    {move || if state.analysis_copied.get() {
                                        t_string!(i18n, common_copied)
                                    } else {
                                        t_string!(i18n, market_copy_analysis)
                                    }}
                                </button>
                                <button
                                    type="button"
                                    class="min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                                    on:click=move |_| state.close_analysis()
                                >
                                    {move || t_string!(i18n, common_close)}
                                </button>
                            </footer>
                        </section>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

fn market_table_row(
    stock: MarketStock,
    state: MarketState,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    let change_class = change_class(stock.change_percent);
    let symbol = stock.symbol.clone();
    let pin_symbol = symbol.clone();
    let is_pinned = Memo::new(move |_| {
        state
            .pinned_symbols
            .get()
            .iter()
            .any(|item| item == &symbol)
    });

    view! {
        <tr class=move || if is_pinned.get() {
            "border-b border-[var(--accent)]/50 bg-[var(--accent)]/5 last:border-b-0 hover:bg-[var(--surface-hover)]"
        } else {
            "border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]"
        }>
            <td class="px-3 py-2 text-center">{pin_button(pin_symbol, is_pinned, state, i18n)}</td>
            <th class="px-3 py-2 text-left font-semibold text-[var(--text-primary)]" scope="row">{stock.symbol.clone()}</th>
            <td class="max-w-[28rem] px-3 py-2 text-[var(--text-secondary)]">{stock.name}</td>
            <td class="px-3 py-2 text-right font-medium text-[var(--text-primary)]">{format_price(stock.price)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_price(stock.change)}</td>
            <td class=format!("px-3 py-2 text-right font-medium {change_class}")>{format_percent(stock.change_percent)}</td>
            <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.total_volume)}</td>
            <td class="px-3 py-2 text-right text-[var(--text-secondary)]">{format_integer(stock.market_cap)}</td>
            <td class="px-3 py-2">{analysis_actions(stock.symbol.clone(), state, i18n)}</td>
        </tr>
    }
}

fn market_mobile_card(
    stock: MarketStock,
    state: MarketState,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    let change_class = change_class(stock.change_percent);
    let symbol = stock.symbol.clone();
    let pin_symbol = symbol.clone();
    let is_pinned = Memo::new(move |_| {
        state
            .pinned_symbols
            .get()
            .iter()
            .any(|item| item == &symbol)
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
                        <h2 class="m-0 truncate text-base font-semibold text-[var(--text-primary)]">{stock.symbol.clone()}</h2>
                        <span class=format!("shrink-0 text-sm font-semibold {change_class}")>{format_percent(stock.change_percent)}</span>
                    </div>
                    <p class="m-0 mt-1 truncate text-xs text-[var(--text-secondary)]">{stock.name}</p>
                </div>
                {pin_button(pin_symbol, is_pinned, state, i18n)}
            </div>
            <div class="mt-3 grid grid-cols-2 gap-x-3 gap-y-2 text-sm">
                <div>
                    <span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(i18n, market_price)}</span>
                    <span class="font-semibold text-[var(--text-primary)]">{format_price(stock.price)}</span>
                </div>
                <div class="text-right">
                    <span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(i18n, market_change)}</span>
                    <span class=format!("font-medium {change_class}")>{format_price(stock.change)}</span>
                </div>
                <div>
                    <span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(i18n, market_volume)}</span>
                    <span class="font-medium text-[var(--text-primary)]">{format_integer(stock.total_volume)}</span>
                </div>
                <div class="text-right">
                    <span class="block text-xs text-[var(--text-secondary)]">{move || t_string!(i18n, market_market_cap)}</span>
                    <span class="font-medium text-[var(--text-primary)]">{format_integer(stock.market_cap)}</span>
                </div>
            </div>
            <div class="mt-3 flex justify-end">
                {analysis_actions(stock.symbol.clone(), state, i18n)}
            </div>
        </article>
    }
}

fn analysis_actions(
    symbol: String,
    state: MarketState,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    let analyze_symbol = symbol.clone();
    let copy_symbol = symbol;
    view! {
        <div class="flex flex-wrap gap-2">
            <button
                type="button"
                class="min-h-10 rounded-md border border-[var(--accent)] bg-[var(--accent)]/10 px-3 py-2 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/20 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                disabled=move || state.analysis_loading.get()
                on:click=move |_| state.analyze_symbol(&analyze_symbol)
            >
                {move || if state.analysis_loading.get() {
                    t_string!(i18n, common_loading)
                } else {
                    t_string!(i18n, market_analyze)
                }}
            </button>
            <button
                type="button"
                class="min-h-10 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                disabled=move || state.analysis_loading.get()
                on:click=move |_| state.copy_symbol_analysis(&copy_symbol)
            >
                {move || if state.analysis_loading.get() {
                    t_string!(i18n, common_loading)
                } else if state.analysis_copied.get() {
                    t_string!(i18n, common_copied)
                } else {
                    t_string!(i18n, common_copy)
                }}
            </button>
        </div>
    }
}

fn pin_button(
    symbol: String,
    is_pinned: Memo<bool>,
    state: MarketState,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="min-h-11 min-w-11 shrink-0 rounded-md border border-[var(--accent)]/60 px-2 py-1 text-xl font-semibold leading-none text-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
            title=move || if is_pinned.get() { t_string!(i18n, market_unpin) } else { t_string!(i18n, market_pin) }
            aria-label=move || if is_pinned.get() { t_string!(i18n, market_unpin) } else { t_string!(i18n, market_pin) }
            on:click=move |_| state.toggle_pin(&symbol)
        >
            {move || if is_pinned.get() { "★" } else { "☆" }}
        </button>
    }
}

fn mobile_sort_button(
    key: &'static str,
    selected: MarketSort,
    sort: RwSignal<MarketSort>,
    descending: RwSignal<bool>,
    on_sort: impl Fn(MarketSort) + Copy + 'static,
    i18n: leptos_i18n::I18nContext<Locale>,
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
            aria-label=move || format!("{} {}", t_string!(i18n, market_sort), market_sort_label(i18n, key))
        >
            {move || if sort.get() == selected {
                format!("{} {}", market_sort_label(i18n, key), if descending.get() { "↓" } else { "↑" })
            } else {
                market_sort_label(i18n, key)
            }}
        </button>
    }
}

fn market_sort_label(i18n: leptos_i18n::I18nContext<Locale>, key: &'static str) -> String {
    match key {
        "symbol" => t_string!(i18n, market_sort_label_symbol).to_string(),
        "price" => t_string!(i18n, market_sort_label_price).to_string(),
        "change_percent" => t_string!(i18n, market_sort_label_change_percent).to_string(),
        "volume" => t_string!(i18n, market_sort_label_volume).to_string(),
        "market_cap" => t_string!(i18n, market_sort_label_market_cap).to_string(),
        _ => t_string!(i18n, market_sort).to_string(),
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
    key: &'static str,
    selected: MarketSort,
    sort: RwSignal<MarketSort>,
    descending: RwSignal<bool>,
    on_sort: impl Fn(MarketSort) + Copy + 'static,
    i18n: leptos_i18n::I18nContext<Locale>,
) -> impl IntoView {
    view! {
        <th class="px-3 py-2 font-medium" scope="col">
            <button
                type="button"
                class="flex min-h-9 items-center gap-1 rounded px-1 text-left hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                aria-label=move || {
                    if sort.get() == selected {
                        format!("{} {} {}", t_string!(i18n, market_sort), market_sort_label(i18n, key), if descending.get() { t_string!(i18n, market_descending) } else { t_string!(i18n, market_ascending) })
                    } else {
                        format!("{} {}", t_string!(i18n, market_sort), market_sort_label(i18n, key))
                    }
                }
                on:click=move |_| on_sort(selected)
            >
                <span>{move || market_sort_label(i18n, key)}</span>
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
