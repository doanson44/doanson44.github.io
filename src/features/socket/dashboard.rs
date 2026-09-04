use leptos::prelude::*;

use crate::features::socket::state::{ChangeFilter, QuoteFilter, SocketState, SortColumn};

/// MEXC Futures public market dashboard.
#[component]
pub fn SocketPage() -> impl IntoView {
    let state = SocketState::new();
    let set_sort = move |column: SortColumn| {
        if state.sort_column.get_untracked() == column {
            state.sort_descending.update(|value| *value = !*value);
        } else {
            state.sort_column.set(column);
            state.sort_descending.set(false);
        }
        state.page.set(1);
    };
    let reset_page = move || state.page.set(1);
    let previous_page = move |_| state.page.update(|page| *page = page.saturating_sub(1).max(1));
    let next_page = move |_| {
        let max_page = state.page_count.get_untracked();
        state.page.update(|page| *page = (*page + 1).min(max_page));
    };

    view! {
        <div class="flex flex-grow flex-col overflow-hidden">
            <div class="flex flex-grow flex-col overflow-hidden px-4 py-3">
                <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                    <div>
                        <h2 class="mb-1 text-xl font-semibold">"MEXC Futures"</h2>
                        <div class="text-sm text-[var(--text-secondary)]">"Public market data · all perpetual contracts"</div>
                    </div>
                    <div class="text-sm">{move || status_badge(state.connection_status.get())}</div>
                </div>

                <div class="mb-3 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3">
                    <div class="grid grid-cols-1 items-end gap-3 md:grid-cols-2 lg:grid-cols-5">
                        <div>
                            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for="socket-search">"Search contracts"</label>
                            <input id="socket-search" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" type="search" placeholder="BTC_USDT" aria-label="Search Futures contracts" prop:value=state.search on:input=move |ev| { state.search.set(event_target_value(&ev)); reset_page(); } />
                        </div>
                        <div>
                            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for="socket-quote">"Quote"</label>
                            <select id="socket-quote" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Filter by quote asset" prop:value=move || match state.quote_filter.get() { QuoteFilter::All => "all", QuoteFilter::Usdt => "usdt", QuoteFilter::Usdc => "usdc" } on:change=move |ev| { state.quote_filter.set(match event_target_value(&ev).as_str() { "usdt" => QuoteFilter::Usdt, "usdc" => QuoteFilter::Usdc, _ => QuoteFilter::All }); reset_page(); }>
                                <option value="all">"All quotes"</option><option value="usdt">"USDT"</option><option value="usdc">"USDC"</option>
                            </select>
                        </div>
                        <div>
                            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for="socket-change">"24h change"</label>
                            <select id="socket-change" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Filter by 24 hour change" prop:value=move || match state.change_filter.get() { ChangeFilter::All => "all", ChangeFilter::Positive => "positive", ChangeFilter::Negative => "negative" } on:change=move |ev| { state.change_filter.set(match event_target_value(&ev).as_str() { "positive" => ChangeFilter::Positive, "negative" => ChangeFilter::Negative, _ => ChangeFilter::All }); reset_page(); }>
                                <option value="all">"All"</option><option value="positive">"Positive"</option><option value="negative">"Negative"</option>
                            </select>
                        </div>
                        <div>
                            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for="socket-page-size">"Rows"</label>
                            <select id="socket-page-size" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Rows per page" prop:value=move || state.page_size.get().to_string() on:change=move |ev| { let size = event_target_value(&ev).parse::<usize>().unwrap_or(25); state.page_size.set(size.clamp(10, 100)); reset_page(); }>
                                <option value="10">"10"</option><option value="25">"25"</option><option value="50">"50"</option><option value="100">"100"</option>
                            </select>
                        </div>
                        <div class="text-sm text-[var(--text-secondary)] md:text-right">{move || format!("{} contracts", state.filtered_sorted.get().len())}</div>
                    </div>
                </div>

                <div class="min-h-0 flex-grow overflow-hidden rounded-lg border border-[var(--border-color)]">
                    <div class="h-full overflow-auto">
                        <table class="w-full border-collapse text-sm">
                            <thead class="sticky top-0 bg-[var(--surface)] text-left">
                                <tr class="border-b border-[var(--border-color)]">
                                    <th scope="col" class="px-3 py-2"><SortButton label="Contract" column=SortColumn::Symbol state=state set_sort=set_sort.clone() /></th>
                                    <th scope="col" class="px-3 py-2 text-right"><SortButton label="Last Price" column=SortColumn::LastPrice state=state set_sort=set_sort.clone() /></th>
                                    <th scope="col" class="px-3 py-2 text-right"><SortButton label="24h Change" column=SortColumn::Change24h state=state set_sort=set_sort.clone() /></th>
                                    <th scope="col" class="px-3 py-2 text-right"><SortButton label="24h Volume" column=SortColumn::Volume24h state=state set_sort=set_sort.clone() /></th>
                                    <th scope="col" class="px-3 py-2 text-right"><SortButton label="Fair Price" column=SortColumn::FairPrice state=state set_sort=set_sort /></th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || state.visible_rows.get().into_iter().map(|ticker| {
                                    let change_class = match ticker.change_24h { Some(value) if value > 0.0 => "text-[var(--success)]", Some(value) if value < 0.0 => "text-[var(--danger)]", _ => "text-[var(--text-primary)]" };
                                    view! {
                                        <tr class="border-b border-[var(--border-color)] hover:bg-[var(--surface-hover)]">
                                            <th scope="row" class="px-3 py-2 font-mono font-normal">{ticker.symbol}</th>
                                            <td class="px-3 py-2 text-right font-mono">{format_number(ticker.last_price)}</td>
                                            <td class=format!("px-3 py-2 text-right font-mono {change_class}")>{format_percent(ticker.change_24h)}</td>
                                            <td class="px-3 py-2 text-right font-mono">{format_number(ticker.volume_24h)}</td>
                                            <td class="px-3 py-2 text-right font-mono">{format_number(ticker.fair_price)}</td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                </div>

                <div class="flex flex-wrap items-center justify-between gap-2 pt-2">
                    <div class="text-sm text-[var(--text-secondary)]">{move || { let total = state.filtered_sorted.get().len(); let size = state.page_size.get().max(1); let page = state.page.get().min(state.page_count.get()); if total == 0 { "Showing 0 contracts".to_string() } else { let start = (page - 1) * size + 1; let end = (start + size - 1).min(total); format!("Showing {start}–{end} of {total}") } }}</div>
                    <div class="flex items-center gap-2">
                        <button class="rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm text-[var(--text-primary)] disabled:cursor-not-allowed disabled:opacity-50 hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" type="button" disabled=move || state.page.get() <= 1 on:click=previous_page>"Previous"</button>
                        <span class="text-sm text-[var(--text-secondary)]" aria-live="polite">{move || format!("Page {} of {}", state.page.get().min(state.page_count.get()), state.page_count.get())}</span>
                        <button class="rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm text-[var(--text-primary)] disabled:cursor-not-allowed disabled:opacity-50 hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" type="button" disabled=move || state.page.get() >= state.page_count.get() on:click=next_page>"Next"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SortButton(label: &'static str, column: SortColumn, state: SocketState, set_sort: impl Fn(SortColumn) + Clone + 'static) -> impl IntoView {
    view! { <button class="p-0 text-sm font-medium text-[var(--text-primary)] hover:underline focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" type="button" on:click=move |_| set_sort(column)>{label} " " {move || sort_indicator(state.sort_column.get(), state.sort_descending.get(), column)}</button> }
}

fn status_badge(status: crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus) -> impl IntoView {
    match status {
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Connected => view! { <span class="rounded-full border border-[var(--success)]/40 bg-[var(--success)]/10 px-2 py-1 text-xs text-[var(--success)]">"Connected"</span> }.into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Connecting => view! { <span class="rounded-full border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-2 py-1 text-xs text-[var(--warning)]">"Connecting"</span> }.into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Disconnected => view! { <span class="rounded-full border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-secondary)]">"Disconnected"</span> }.into_any(),
        crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus::Error(message) => view! { <span class="rounded-full border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-2 py-1 text-xs text-[var(--danger)]" title=message>"Connection error"</span> }.into_any(),
    }
}

fn sort_indicator(current: SortColumn, descending: bool, column: SortColumn) -> &'static str {
    if current != column { "" } else if descending { "▼" } else { "▲" }
}

fn format_number(value: Option<f64>) -> String { value.map(|number| if number.abs() >= 1.0 { format!("{number:.4}") } else { format!("{number:.8}") }).unwrap_or_else(|| "—".into()) }
fn format_percent(value: Option<f64>) -> String { value.map(|number| format!("{:.2}%", number * 100.0)).unwrap_or_else(|| "—".into()) }
