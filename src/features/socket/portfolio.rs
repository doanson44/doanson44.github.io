use leptos::prelude::*;

use crate::domain::trading::PortfolioSummary;
use crate::features::socket::state::SocketState;
use crate::i18n::*;

/// Portfolio summary and holdings for Socket paper trading.
#[component]
pub fn PortfolioPanel(state: SocketState, summary: Memo<PortfolioSummary>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section
            class="mb-3 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3"
            aria-labelledby="socket-portfolio-title"
        >
            <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                <div>
                    <h3 id="socket-portfolio-title" class="text-sm font-semibold text-[var(--text-primary)]">
                        {move || t_string!(i18n, socket_portfolio)}
                    </h3>
                    <span class="text-xs text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_paper_trading)}
                    </span>
                </div>
                <button
                    type="button"
                    class="min-h-9 rounded-md border border-[var(--border-color)] px-3 py-1.5 text-xs font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=move |_| state.open_settings()
                >
                    {move || t_string!(i18n, socket_settings)}
                </button>
            </div>

            <div class="grid grid-cols-2 gap-2 sm:grid-cols-5">
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_initial_capital)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || format_currency(state.trading_snapshot.get().settings.initial_capital)}
                    </div>
                </div>
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_cash)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || format_currency(summary.get().cash)}
                    </div>
                </div>
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_equity)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || format_currency(summary.get().equity)}
                    </div>
                </div>
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_pnl)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || format_signed_currency(summary.get().total_pnl)}
                    </div>
                </div>
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_holdings)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || summary.get().holdings.len().to_string()}
                    </div>
                </div>
            </div>

            <div class="mt-3 border-t border-[var(--border-color)] pt-3">
                <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
                    <h4 class="text-sm font-semibold text-[var(--text-primary)]">
                        {move || t_string!(i18n, socket_holdings)}
                    </h4>
                    <div class="text-xs text-[var(--text-secondary)]">
                        {move || format!(
                            "{}: {} · {}: {}",
                            t_string!(i18n, socket_realized_pnl),
                            format_signed_currency(summary.get().realized_pnl),
                            t_string!(i18n, socket_unrealized_pnl),
                            format_signed_currency(summary.get().unrealized_pnl)
                        )}
                    </div>
                </div>

                <Show
                    when=move || !summary.get().holdings.is_empty()
                    fallback=move || view! {
                        <div class="rounded-md border border-dashed border-[var(--border-color)] px-3 py-4 text-sm text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_no_holdings)}
                        </div>
                    }
                >
                    <div class="overflow-x-auto">
                        <table class="w-full min-w-[520px] text-sm">
                            <caption class="sr-only">
                                {move || t_string!(i18n, socket_holdings)}
                            </caption>
                            <thead>
                                <tr class="border-b border-[var(--border-color)] text-left text-xs text-[var(--text-secondary)]">
                                    <th class="px-2 py-2 font-medium" scope="col">{move || t_string!(i18n, socket_symbol)}</th>
                                    <th class="px-2 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_quantity)}</th>
                                    <th class="px-2 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_market_value)}</th>
                                    <th class="px-2 py-2 text-right font-medium" scope="col">{move || t_string!(i18n, socket_pnl)}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || summary.get().holdings.into_iter().map(|holding| {
                                    let pnl_class = if holding.pnl > 0.0 {
                                        "text-[var(--success)]"
                                    } else if holding.pnl < 0.0 {
                                        "text-[var(--danger)]"
                                    } else {
                                        "text-[var(--text-primary)]"
                                    };

                                    view! {
                                        <tr class="border-b border-[var(--border-color)] last:border-b-0">
                                            <th class="px-2 py-2 text-left font-mono font-medium text-[var(--text-primary)]" scope="row">
                                                {holding.symbol}
                                            </th>
                                            <td class="px-2 py-2 text-right font-mono text-[var(--text-secondary)]">
                                                {format_quantity(holding.quantity)}
                                            </td>
                                            <td class="px-2 py-2 text-right font-mono text-[var(--text-primary)]">
                                                {format_currency(holding.market_value)}
                                            </td>
                                            <td class=format!("px-2 py-2 text-right font-mono font-medium {pnl_class}")>
                                                {format_signed_currency(holding.pnl)}
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                </Show>
            </div>
        </section>

        {move || if state.settings_open.get() {
            view! { <TradingSettingsModal state=state /> }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }}
    }
}

#[component]
fn TradingSettingsModal(state: SocketState) -> impl IntoView {
    let i18n = use_i18n();
    let snapshot = state.trading_snapshot.get_untracked();
    let initial_capital = RwSignal::new(snapshot.settings.initial_capital.to_string());
    let fee_percent = RwSignal::new((snapshot.settings.fee_rate * 100.0).to_string());
    let leverage = RwSignal::new(snapshot.settings.leverage.to_string());
    let trade_allocation = RwSignal::new(snapshot.settings.trade_allocation_percent.to_string());

    let save = move |_| {
        let initial = initial_capital.get_untracked().trim().parse::<f64>();
        let fee = fee_percent.get_untracked().trim().parse::<f64>();
        let leverage = leverage.get_untracked().trim().parse::<f64>();
        let trade_allocation = trade_allocation.get_untracked().trim().parse::<f64>();

        match (initial, fee, leverage, trade_allocation) {
            (Ok(initial), Ok(fee), Ok(leverage), Ok(trade_allocation)) => {
                state.save_settings(initial, fee, leverage, trade_allocation)
            }
            _ => state
                .trading_error
                .set(Some("Enter valid numeric settings.".to_string())),
        }
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-[var(--overlay)] p-4" role="presentation">
            <div
                class="w-full max-w-lg rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-4 shadow-xl"
                role="dialog"
                aria-modal="true"
                aria-labelledby="socket-settings-title"
            >
                <div class="flex items-start justify-between gap-3">
                    <div>
                        <h3 id="socket-settings-title" class="text-lg font-semibold text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_settings)}
                        </h3>
                        <p class="mt-1 text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_paper_trading)}
                        </p>
                    </div>
                    <button
                        type="button"
                        class="min-h-9 min-w-9 rounded-md border border-[var(--border-color)] px-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        aria-label=move || t_string!(i18n, common_close)
                        on:click=move |_| state.close_settings()
                    >
                        "×"
                    </button>
                </div>

                <div class="mt-4 space-y-3">
                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_initial_capital)}
                        </span>
                        <input
                            type="number"
                            min="1"
                            step="0.01"
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || initial_capital.get()
                            on:input=move |ev| initial_capital.set(event_target_value(&ev))
                        />
                    </label>

                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_trading_fee)}
                        </span>
                        <input
                            type="number"
                            min="0"
                            max="100"
                            step="0.01"
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || fee_percent.get()
                            on:input=move |ev| fee_percent.set(event_target_value(&ev))
                        />
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_fee_hint)}
                        </span>
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_trade_size_hint)}
                        </span>
                    </label>

                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_api_key)}
                        </span>
                        <input
                            type="password"
                            autocomplete="off"
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || state.api_key.get()
                            on:input=move |ev| state.api_key.set(event_target_value(&ev))
                        />
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_api_key_hint)}
                        </span>
                    </label>

                    <div class="rounded-md border border-[var(--warning)]/40 bg-[var(--warning)]/10 px-3 py-2 text-xs text-[var(--warning)]">
                        {move || t_string!(i18n, socket_settings_reset_warning)}
                    </div>
                </div>

                <div class="mt-4 flex justify-end gap-2">
                    <button
                        type="button"
                        class="min-h-10 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        on:click=move |_| state.close_settings()
                    >
                        {move || t_string!(i18n, common_close)}
                    </button>
                    <button
                        type="button"
                        class="min-h-10 rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        on:click=save
                    >
                        {move || t_string!(i18n, socket_save_settings)}
                    </button>
                </div>
            </div>
        </div>
    }
}

fn format_currency(value: f64) -> String {
    format!("{value:.2} USDT")
}

fn format_signed_currency(value: f64) -> String {
    if value >= 0.0 {
        format!("+{value:.2} USDT")
    } else {
        format!("-{:.2} USDT", value.abs())
    }
}

fn format_quantity(value: f64) -> String {
    if value.abs() >= 1.0 {
        format!("{value:.4}")
    } else {
        format!("{value:.8}")
    }
}
