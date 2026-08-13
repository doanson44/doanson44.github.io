use leptos::prelude::*;

use crate::features::socket::state::{ChangeFilter, QuoteFilter, SocketState, SortColumn};

#[component]
pub fn SocketPage() -> impl IntoView {
    let state = SocketState::new();
    let set_sort = move |column: SortColumn| {
        let current_col = state.sort_column.get_untracked();
        let current_desc = state.sort_descending.get_untracked();

        if current_col == column {
            if !current_desc {
                state.sort_descending.set(true);
            } else {
                // Reset to default sort
                state.sort_column.set(SortColumn::Symbol);
                state.sort_descending.set(false);
            }
        } else {
            state.sort_column.set(column);
            state.sort_descending.set(false);
        }
        state.page.set(1);
    };

    let reset_page = move || state.page.set(1);

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-hidden">
            <div class="container-fluid py-3 d-flex flex-column flex-grow-1 overflow-hidden">
                <header class="d-flex flex-wrap justify-content-between align-items-center gap-2 mb-3">
                    <div>
                        <h2 class="mb-1">
                            <i class="bi bi-broadcast me-2 text-primary"></i>
                            "MEXC Futures"
                        </h2>
                        <div class="small text-body-secondary">
                            "Public market data · all perpetual contracts"
                        </div>
                    </div>
                    <div class="small">
                        {move || status_badge(state.connection_status.get())}
                    </div>
                </header>

                <div class="card bg-body-tertiary border-secondary mb-3">
                    <div class="card-body p-2">
                        <div class="row g-2 align-items-end">
                            <div class="col-12 col-md-5 col-lg-4">
                                <label class="form-label small text-body-secondary" for="socket-search">
                                    "Search"
                                </label>
                                <input
                                    id="socket-search"
                                    class="form-control form-control-sm"
                                    type="search"
                                    placeholder="BTC_USDT"
                                    aria-label="Search Futures contracts"
                                    prop:value=state.search
                                    on:input=move |ev| {
                                        state.search.set(event_target_value(&ev));
                                        reset_page();
                                    }
                                />
                            </div>
                            <div class="col-6 col-md-3 col-lg-2">
                                <label class="form-label small text-body-secondary" for="socket-quote">
                                    "Quote"
                                </label>
                                <select
                                    id="socket-quote"
                                    class="form-select form-select-sm"
                                    aria-label="Filter by quote asset"
                                    prop:value=move || quote_value(state.quote_filter.get())
                                    on:change=move |ev| {
                                        state.quote_filter.set(parse_quote(&event_target_value(&ev)));
                                        reset_page();
                                    }
                                >
                                    <option value="all">"All"</option>
                                    <option value="usdt">"USDT"</option>
                                    <option value="usdc">"USDC"</option>
                                </select>
                            </div>
                            <div class="col-6 col-md-3 col-lg-2">
                                <label class="form-label small text-body-secondary" for="socket-change">
                                    "24h change"
                                </label>
                                <select
                                    id="socket-change"
                                    class="form-select form-select-sm"
                                    aria-label="Filter by 24 hour change"
                                    prop:value=move || change_value(state.change_filter.get())
                                    on:change=move |ev| {
                                        state.change_filter.set(parse_change(&event_target_value(&ev)));
                                        reset_page();
                                    }
                                >
                                    <option value="all">"All"</option>
                                    <option value="positive">"Positive"</option>
                                    <option value="negative">"Negative"</option>
                                </select>
                            </div>
                            <div class="col-6 col-md-3 col-lg-2">
                                <label class="form-label small text-body-secondary" for="socket-page-size">
                                    "Rows"
                                </label>
                                <select
                                    id="socket-page-size"
                                    class="form-select form-select-sm"
                                    aria-label="Rows per page"
                                    prop:value=move || state.page_size.get().to_string()
                                    on:change=move |ev| {
                                        let size = event_target_value(&ev).parse().unwrap_or(25);
                                        state.page_size.set(size.clamp(10, 100));
                                        reset_page();
                                    }
                                >
                                    <option value="10">"10"</option>
                                    <option value="25">"25"</option>
                                    <option value="50">"50"</option>
                                    <option value="100">"100"</option>
                                </select>
                            </div>
                            <div class="col-6 col-md-auto ms-md-auto small text-body-secondary">
                                {move || format!("{} contracts", state.filtered_sorted.get().len())}
                            </div>
                        </div>
                    </div>
                </div>

                <div class="card border-secondary flex-grow-1 overflow-hidden">
                    <div class="table-responsive h-100">
                        <table class="table table-hover table-sm align-middle mb-0">
                            <thead>
                                <tr>
                                    <th scope="col" style="width: 20%">
                                        <SortButton
                                            column=SortColumn::Symbol
                                            label="Symbol"
                                            state=state
                                            set_sort=set_sort
                                        />
                                    </th>
                                    <th scope="col" class="text-end" style="width: 20%">
                                        <SortButton
                                            column=SortColumn::LastPrice
                                            label="Last Price"
                                            state=state
                                            set_sort=set_sort
                                        />
                                    </th>
                                    <th scope="col" class="text-end" style="width: 20%">
                                        <SortButton
                                            column=SortColumn::Change24h
                                            label="24h Change"
                                            state=state
                                            set_sort=set_sort
                                        />
                                    </th>
                                    <th scope="col" class="text-end d-none d-md-table-cell" style="width: 20%">
                                        <SortButton
                                            column=SortColumn::Volume24h
                                            label="24h Vol"
                                            state=state
                                            set_sort=set_sort
                                        />
                                    </th>
                                    <th scope="col" class="text-end d-none d-lg-table-cell" style="width: 20%">
                                        <SortButton
                                            column=SortColumn::FairPrice
                                            label="Fair Price"
                                            state=state
                                            set_sort=set_sort
                                        />
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=move || {
                                        state
                                            .visible_rows
                                            .with(|rows| rows.iter().map(|t| t.symbol.clone()).collect::<Vec<_>>())
                                    }
                                    key=|symbol| symbol.clone()
                                    children=move |symbol| {
                                        let sym = symbol.clone();
                                        let ticker_memo = Memo::new(move |_| {
                                            state.tickers.with(|all| {
                                                all.iter().find(|t| t.symbol == sym).cloned()
                                            })
                                        });
                                        row_view(ticker_memo)
                                    }
                                />
                            </tbody>
                        </table>
                    </div>
                </div>

                <footer class="d-flex flex-wrap justify-content-between align-items-center gap-2 pt-2">
                    <div class="small text-body-secondary">
                        {move || range_text(&state)}
                    </div>
                    <div class="d-flex align-items-center gap-2">
                        <button
                            class="btn btn-outline-secondary btn-sm"
                            type="button"
                            disabled=move || state.page.get() <= 1
                            on:click=move |_| state.page.update(|p| *p = p.saturating_sub(1).max(1))
                        >
                            "Previous"
                        </button>
                        <span class="small text-body-secondary" aria-live="polite">
                            {move || format!(
                                "Page {} of {}",
                                state.page.get().min(state.page_count.get()),
                                state.page_count.get()
                            )}
                        </span>
                        <button
                            class="btn btn-outline-secondary btn-sm"
                            type="button"
                            disabled=move || state.page.get() >= state.page_count.get()
                            on:click=move |_| {
                                let max = state.page_count.get_untracked();
                                state.page.update(|p| *p = (*p + 1).min(max));
                            }
                        >
                            "Next"
                        </button>
                    </div>
                </footer>
            </div>
        </div>
    }
}

#[component]
fn SortButton(
    label: &'static str,
    column: SortColumn,
    state: SocketState,
    set_sort: impl Fn(SortColumn) + Clone + 'static,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-link btn-sm text-body text-decoration-none p-0"
            type="button"
            on:click=move |_| set_sort(column)
        >
            {label} " "
            {move || sort_indicator(state.sort_column.get(), state.sort_descending.get(), column)}
        </button>
    }
}

fn row_view(ticker_memo: Memo<Option<crate::domain::futures::FuturesTicker>>) -> impl IntoView {
    let change_class = move || match ticker_memo.get().and_then(|t| t.change_24h) {
        Some(value) if value > 0.0 => "text-success",
        Some(value) if value < 0.0 => "text-danger",
        _ => "text-body",
    };

    view! {
        <tr>
            <th scope="row" class="font-monospace fw-normal">
                {move || ticker_memo.get().map(|t| t.symbol).unwrap_or_default()}
            </th>
            <td class="text-end font-monospace">
                {move || format_number(ticker_memo.get().and_then(|t| t.last_price))}
            </td>
            <td class=move || format!("text-end font-monospace {}", change_class())>
                {move || format_percent(ticker_memo.get().and_then(|t| t.change_24h))}
            </td>
            <td class="text-end font-monospace">
                {move || format_number(ticker_memo.get().and_then(|t| t.volume_24h))}
            </td>
            <td class="text-end font-monospace">
                {move || format_number(ticker_memo.get().and_then(|t| t.fair_price))}
            </td>
        </tr>
    }
}

fn range_text(state: &SocketState) -> String {
    let total = state.filtered_sorted.get().len();
    let size = state.page_size.get().max(1);
    let page = state.page.get().min(state.page_count.get());
    if total == 0 {
        "Showing 0 contracts".into()
    } else {
        let start = (page - 1) * size + 1;
        let end = (start + size - 1).min(total);
        format!("Showing {start}–{end} of {total}")
    }
}

fn status_badge(
    status: crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus,
) -> impl IntoView {
    match status {
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Connected => view! {
            <span class="badge bg-success-subtle text-success-emphasis border border-success-subtle">
                "Connected"
            </span>
        }
        .into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Connecting => view! {
            <span class="badge bg-warning-subtle text-warning-emphasis border border-warning-subtle">
                "Connecting"
            </span>
        }
        .into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Disconnected => view! {
            <span class="badge bg-secondary-subtle text-secondary-emphasis border border-secondary-subtle">
                "Disconnected"
            </span>
        }
        .into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Error(message) => view! {
            <span
                class="badge bg-danger-subtle text-danger-emphasis border border-danger-subtle"
                title=message
            >
                "Connection error"
            </span>
        }
        .into_any(),
    }
}

fn sort_indicator(current: SortColumn, descending: bool, column: SortColumn) -> &'static str {
    if current != column {
        ""
    } else if descending {
        "▼"
    } else {
        "▲"
    }
}

fn quote_value(filter: QuoteFilter) -> &'static str {
    match filter {
        QuoteFilter::All => "all",
        QuoteFilter::Usdt => "usdt",
        QuoteFilter::Usdc => "usdc",
    }
}

fn parse_quote(value: &str) -> QuoteFilter {
    match value {
        "usdt" => QuoteFilter::Usdt,
        "usdc" => QuoteFilter::Usdc,
        _ => QuoteFilter::All,
    }
}

fn change_value(filter: ChangeFilter) -> &'static str {
    match filter {
        ChangeFilter::All => "all",
        ChangeFilter::Positive => "positive",
        ChangeFilter::Negative => "negative",
    }
}

fn parse_change(value: &str) -> ChangeFilter {
    match value {
        "positive" => ChangeFilter::Positive,
        "negative" => ChangeFilter::Negative,
        _ => ChangeFilter::All,
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
        .map(|number| format!("{:.2}%", number * 100.0))
        .unwrap_or_else(|| "—".into())
}
