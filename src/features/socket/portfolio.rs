use leptos::prelude::*;

use crate::domain::trading::{ExecutionMode, PortfolioSummary, PositionSide};
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
                        {move || if state.execution_mode.get() == ExecutionMode::Real {
                            t_string!(i18n, socket_real_trading)
                        } else {
                            t_string!(i18n, socket_paper_trading)
                        }}
                    </span>
                </div>
                <div class="flex flex-wrap items-center gap-2 text-xs text-[var(--text-secondary)]">
                    <span class="rounded-md border border-[var(--border-color)] px-2 py-1">
                        {move || format!("{}: {:.1}%", t_string!(i18n, socket_trade_allocation), state.trading_snapshot.get().settings.trade_allocation_percent)}
                    </span>
                    <span class="rounded-md border border-[var(--border-color)] px-2 py-1">
                        {move || format!("{}: {:.1}x", t_string!(i18n, socket_leverage), state.trading_snapshot.get().settings.leverage)}
                    </span>
                    <label class="flex items-center gap-2 rounded-md border border-[var(--border-color)] px-2 py-1">
                        <span>{move || t_string!(i18n, socket_position_side)}</span>
                        <select
                            class="rounded border-0 bg-transparent px-1 py-0.5 text-xs font-medium text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                            aria-label=move || t_string!(i18n, socket_position_side)
                            prop:value=move || match state.trading_snapshot.get().settings.position_side {
                                PositionSide::Long => "long",
                                PositionSide::Short => "short",
                            }
                            on:change=move |ev| {
                                state.set_position_side(if event_target_value(&ev) == "short" {
                                    PositionSide::Short
                                } else {
                                    PositionSide::Long
                                });
                            }
                        >
                            <option value="long">{move || t_string!(i18n, socket_long)}</option>
                            <option value="short">{move || t_string!(i18n, socket_short)}</option>
                        </select>
                    </label>
                </div>
            </div>

            <div class="grid grid-cols-2 gap-2 sm:grid-cols-6">
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_initial_capital)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || if state.execution_mode.get() == ExecutionMode::Real {
                            state
                                .real_account
                                .get()
                                .map(|account| format_currency(account.equity))
                                .unwrap_or_else(|| "—".to_string())
                        } else {
                            format_currency(state.trading_snapshot.get().settings.initial_capital)
                        }}
                    </div>
                </div>
                <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2">
                    <div class="text-[11px] text-[var(--text-secondary)]">
                        {move || t_string!(i18n, socket_trade_allocation)}
                    </div>
                    <div class="mt-1 font-mono text-sm font-semibold text-[var(--text-primary)]">
                        {move || format!("{:.1}%", state.trading_snapshot.get().settings.trade_allocation_percent)}
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
                        <table class="w-full min-w-[600px] text-sm">
                            <caption class="sr-only">
                                {move || t_string!(i18n, socket_holdings)}
                            </caption>
                            <thead>
                                <tr class="border-b border-[var(--border-color)] text-left text-xs text-[var(--text-secondary)]">
                                    <th class="px-2 py-2 font-medium" scope="col">{move || t_string!(i18n, socket_symbol)}</th>
                                    <th class="px-2 py-2 font-medium" scope="col">{move || t_string!(i18n, socket_position_side)}</th>
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
                                    let side_label = if holding.side == PositionSide::Long {
                                        t_string!(i18n, socket_long)
                                    } else {
                                        t_string!(i18n, socket_short)
                                    };

                                    view! {
                                        <tr class="border-b border-[var(--border-color)] last:border-b-0">
                                            <th class="px-2 py-2 text-left font-mono font-medium text-[var(--text-primary)]" scope="row">
                                                {holding.symbol}
                                            </th>
                                            <td class="px-2 py-2 text-left text-xs font-medium text-[var(--text-secondary)]">
                                                {side_label}
                                            </td>
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
    let mode = RwSignal::new(state.execution_mode.get_untracked());
    let api_url = RwSignal::new(state.api_url.get_untracked());
    let api_key = RwSignal::new(state.api_key.get_untracked());
    let api_secret = RwSignal::new(state.api_secret.get_untracked());

    let save = move |_| {
        let initial = initial_capital.get_untracked().trim().parse::<f64>();
        let fee = fee_percent.get_untracked().trim().parse::<f64>();
        let leverage = leverage.get_untracked().trim().parse::<f64>();
        let trade_allocation = trade_allocation.get_untracked().trim().parse::<f64>();
        let execution_mode = mode.get_untracked();

        match (initial, fee, leverage, trade_allocation) {
            (Ok(initial), Ok(fee), Ok(leverage), Ok(trade_allocation)) => state.save_settings(TradingSettingsInput {
                initial_capital: initial,
                fee_percent: fee,
                leverage,
                trade_allocation_percent: trade_allocation,
                execution_mode,
                api_url: api_url.get_untracked(),
                api_key: api_key.get_untracked(),
                api_secret: api_secret.get_untracked(),
            }),
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
                            {move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                                t_string!(i18n, socket_real_trading)
                            } else {
                                t_string!(i18n, socket_paper_trading)
                            }}
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
                    <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] p-3">
                        <div class="flex items-center justify-between gap-3">
                            <div>
                                <div class="text-sm font-medium text-[var(--text-primary)]">
                                    {move || t_string!(i18n, socket_execution_mode)}
                                </div>
                                <div class="mt-1 text-xs text-[var(--text-secondary)]">
                                    {move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                                        t_string!(i18n, socket_real_trading_hint)
                                    } else {
                                        t_string!(i18n, socket_paper_trading_hint)
                                    }}
                                </div>
                            </div>
                            <label class="inline-flex cursor-pointer items-center gap-2">
                                <span class="text-xs font-medium text-[var(--text-secondary)]">
                                    {move || t_string!(i18n, socket_real_trading)}
                                </span>
                                <input
                                    type="checkbox"
                                    class="peer sr-only"
                                    prop:checked=move || mode.get() == crate::domain::trading::ExecutionMode::Real
                                    on:change=move |ev| {
                                        mode.set(if event_target_checked(&ev) {
                                            crate::domain::trading::ExecutionMode::Real
                                        } else {
                                            crate::domain::trading::ExecutionMode::Paper
                                        });
                                    }
                                />
                                <span class="relative h-6 w-11 rounded-full bg-[var(--border-color)] transition peer-focus-visible:outline peer-focus-visible:outline-2 peer-focus-visible:outline-[var(--accent)] peer-checked:bg-[var(--accent)]">
                                    <span class="absolute left-1 top-1 h-4 w-4 rounded-full bg-[var(--surface)] transition peer-checked:translate-x-5"></span>
                                </span>
                            </label>
                        </div>
                    </div>

                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_initial_capital)}
                        </span>
                        <input
                            type="number"
                            min="1"
                            step="0.01"
                            disabled=move || mode.get() == crate::domain::trading::ExecutionMode::Real
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] disabled:cursor-not-allowed disabled:opacity-60 focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                                state.real_account.get().map(|account| account.equity.to_string()).unwrap_or_else(|| initial_capital.get())
                            } else {
                                initial_capital.get()
                            }
                            on:input=move |ev| initial_capital.set(event_target_value(&ev))
                        />
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                                t_string!(i18n, socket_real_initial_capital_hint)
                            } else {
                                t_string!(i18n, socket_paper_initial_capital_hint)
                            }}
                        </span>
                    </label>

                    {move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                        view! {
                            <div class="space-y-3 rounded-md border border-[var(--warning)]/40 bg-[var(--warning)]/5 p-3">
                                <label class="block text-sm">
                                    <span class="mb-1 block font-medium text-[var(--text-primary)]">
                                        {move || t_string!(i18n, socket_api_url)}
                                    </span>
                                    <input
                                        type="url"
                                        autocomplete="url"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                                        prop:value=move || api_url.get()
                                        on:input=move |ev| api_url.set(event_target_value(&ev))
                                    />
                                </label>

                                <label class="block text-sm">
                                    <span class="mb-1 block font-medium text-[var(--text-primary)]">
                                        {move || t_string!(i18n, socket_api_key)}
                                    </span>
                                    <input
                                        type="password"
                                        autocomplete="off"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                                        prop:value=move || api_key.get()
                                        on:input=move |ev| api_key.set(event_target_value(&ev))
                                    />
                                </label>

                                <label class="block text-sm">
                                    <span class="mb-1 block font-medium text-[var(--text-primary)]">
                                        {move || t_string!(i18n, socket_api_secret)}
                                    </span>
                                    <input
                                        type="password"
                                        autocomplete="off"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                                        prop:value=move || api_secret.get()
                                        on:input=move |ev| api_secret.set(event_target_value(&ev))
                                    />
                                </label>

                                <div class="rounded-md border border-[var(--warning)]/40 px-3 py-2 text-xs text-[var(--warning)]">
                                    {move || t_string!(i18n, socket_real_trading_warning)}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <span></span>
                        }.into_any()
                    }}

                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_trade_allocation)}
                        </span>
                        <input
                            type="number"
                            min="0.1"
                            max="100"
                            step="0.1"
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || trade_allocation.get()
                            on:input=move |ev| trade_allocation.set(event_target_value(&ev))
                        />
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_trade_allocation_hint)}
                        </span>
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
                    </label>

                    <label class="block text-sm">
                        <span class="mb-1 block font-medium text-[var(--text-primary)]">
                            {move || t_string!(i18n, socket_leverage)}
                        </span>
                        <input
                            type="number"
                            min="1"
                            max="125"
                            step="0.1"
                            class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                            prop:value=move || leverage.get()
                            on:input=move |ev| leverage.set(event_target_value(&ev))
                        />
                        <span class="mt-1 block text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, socket_leverage_hint)}
                        </span>
                    </label>

                    <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2 text-xs text-[var(--text-secondary)]">
                        {move || if mode.get() == crate::domain::trading::ExecutionMode::Real {
                            t_string!(i18n, socket_real_storage_hint)
                        } else {
                            t_string!(i18n, socket_settings_reset_warning)
                        }}
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
                        class="min-h-10 rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-60"
                        disabled=move || state.real_account_loading.get()
                        on:click=save
                    >
                        {move || if state.real_account_loading.get() {
                            t_string!(i18n, socket_loading_account)
                        } else {
                            t_string!(i18n, socket_save_settings)
                        }}
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
