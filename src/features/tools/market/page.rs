use leptos::prelude::*;

use crate::features::tools::market::state::MarketState;

#[component]
pub fn MarketPage() -> impl IntoView {
    let state = MarketState::new();
    state.load();

    view! {
        <div class="flex flex-grow flex-col overflow-hidden">
            <header class="flex shrink-0 flex-col gap-1 border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-2">
                <div class="flex items-center gap-2">
                    <h1 class="m-0 text-base font-semibold text-[var(--text-primary)]">
                        <span class="mr-2 text-[var(--accent)]" aria-hidden="true">"◈"</span>
                        "Market"
                    </h1>
                    <span class="text-xs text-[var(--text-secondary)]" aria-live="polite">
                        {move || if state.loading.get() { "Loading...".to_string() } else { format!("{} / {}", state.displayed_items.get(), state.total_items.get()) }}
                    </span>
                    <button type="button"
                        class="ml-auto min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                        title="Refresh market data"
                        disabled=move || state.loading.get()
                        on:click=move |_| state.load()
                    >"Refresh"</button>
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
                    <table class="w-full min-w-[760px] border-collapse text-sm">
                        <caption class="sr-only">"CafeF market stock data"</caption>
                        <thead>
                            <tr class="border-b border-[var(--border-color)] bg-[var(--surface-hover)] text-left text-[var(--text-secondary)]">
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
                            {move || state.stocks.get().into_iter().map(|stock| {
                                let change_class = if stock.change_percent > 0.0 {
                                    "text-[var(--success)]"
                                } else if stock.change_percent < 0.0 {
                                    "text-[var(--danger)]"
                                } else {
                                    "text-[var(--text-secondary)]"
                                };
                                view! {
                                    <tr class="border-b border-[var(--border-color)] last:border-b-0 hover:bg-[var(--surface-hover)]">
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
                {move || if !state.loading.get() && state.error.get().is_none() && state.stocks.get().is_empty() {
                    view! { <div class="mt-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-4 text-sm text-[var(--text-secondary)]" role="status">"No market data returned."</div> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                <div class="mt-3 text-xs text-[var(--text-secondary)]" role="note">
                    {move || format!("Displayed: {} · Total: {}", state.displayed_items.get(), state.total_items.get())}
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
