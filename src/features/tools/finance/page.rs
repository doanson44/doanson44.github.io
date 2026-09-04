use leptos::prelude::*;

use crate::application::services::finance::FinanceService;
use crate::domain::finance::FinanceTool;
use crate::features::tools::finance::state::FinanceState;

/// Finance calculator page.
#[component]
pub fn FinancePage(tool: FinanceTool) -> impl IntoView {
    let state = FinanceState::new(tool);
    let labels = tool.fields();
    let inputs = state.inputs;
    let series = state.series;
    let result = state.result;
    let error = state.error;

    let calculate = move |_| {
        let raw = inputs.get();
        let parsed = raw
            .iter()
            .map(|value| {
                value
                    .trim()
                    .parse::<f64>()
                    .map_err(|_| "Every input must contain a valid number.".to_string())
            })
            .collect::<Result<Vec<_>, _>>();
        let cashflows = series
            .get()
            .split([',', '\n', ';'])
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.trim()
                    .parse::<f64>()
                    .map_err(|_| "Cash flows must be comma- or line-separated numbers.".to_string())
            })
            .collect::<Result<Vec<_>, _>>();
        match (parsed, cashflows) {
            (Ok(values), Ok(cashflows)) => {
                match FinanceService::calculate(tool, &values, &cashflows) {
                    Ok(value) => {
                        result.set(Some(value));
                        error.set(None);
                    }
                    Err(message) => {
                        result.set(None);
                        error.set(Some(message));
                    }
                }
            }
            (Err(message), _) | (_, Err(message)) => {
                result.set(None);
                error.set(Some(message));
            }
        }
    };
    let reset = move |_| {
        inputs.set(vec![String::new(); labels.len()]);
        series.set(String::new());
        result.set(None);
        error.set(None);
    };

    view! {
        <div class="h-full overflow-auto px-4 py-6 finance-page">
            <div class="mx-auto max-w-6xl">
                <div class="mb-4">
                    <a href="#/tools" class="mb-2 inline-flex items-center rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]">
                        "← Back to Tools"
                    </a>
                    <h2 class="mb-1 text-2xl font-semibold">{tool.title()}</h2>
                    <p class="mb-0 text-sm text-[var(--text-secondary)]">{tool.category()}</p>
                </div>
                <div class="grid grid-cols-1 gap-3 lg:grid-cols-2">
                    <section>
                        <div class="h-full rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-5 shadow-sm">
                            <h5 class="mb-4 text-base font-semibold">"Inputs"</h5>
                            <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
                                {labels
                                    .iter()
                                    .enumerate()
                                    .map(|(index, label)| {
                                        view! {
                                            <div>
                                                <label class="mb-1 block text-sm font-medium" for=format!("finance-input-{index}")>{*label}</label>
                                                <input
                                                    id=format!("finance-input-{index}")
                                                    class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                                                    type="number"
                                                    step="any"
                                                    value=move || inputs.get().get(index).cloned().unwrap_or_default()
                                                    on:input=move |ev| {
                                                        let mut values = inputs.get();
                                                        if let Some(value) = values.get_mut(index) {
                                                            *value = event_target_value(&ev);
                                                        }
                                                        inputs.set(values);
                                                    }
                                                />
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                                <div class="md:col-span-2">
                                    <label class="mb-1 block text-sm font-medium" for="finance-series">"Cash flows (optional; comma or newline separated)"</label>
                                    <textarea
                                        id="finance-series"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-[var(--text-primary)] placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25"
                                        rows="3"
                                        placeholder="-1000, 300, 400, 500"
                                        prop:value=move || series.get()
                                        on:input=move |ev| series.set(event_target_value(&ev))
                                    ></textarea>
                                </div>
                            </div>
                            <div class="mt-4 flex justify-end gap-2">
                                <button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=reset>"Reset"</button>
                                <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=calculate>"Calculate"</button>
                            </div>
                        </div>
                    </section>
                    <section>
                        <div class="h-full rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-5 shadow-sm">
                            <h5 class="mb-4 text-base font-semibold">"Results"</h5>
                            {move || error.get().map(|message| view! {
                                <div class="mb-3 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">{message}</div>
                            })}
                            {move || result.get().map(|value| view! {
                                <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                                    {value.metrics.into_iter().map(|metric| view! {
                                        <div class="h-full rounded-md border border-[var(--border-color)] p-3">
                                            <div class="text-xs text-[var(--text-secondary)]">{metric.label}</div>
                                            <div class="mt-1 text-2xl font-semibold">{format_number(metric.value)}</div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            })}
                            {move || if result.get().is_none() && error.get().is_none() {
                                view! { <p class="mb-0 text-sm text-[var(--text-secondary)]">"Enter values and calculate to see the result."</p> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    </section>
                </div>
                <div class="mt-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-secondary)]" role="note">
                    "Results are estimates. Trading, lending, tax, currency, and liquidation rules can differ from the simplified models used here."
                </div>
            </div>
        </div>
    }
}

fn format_number(value: f64) -> String {
    if value.abs() >= 1.0 {
        format!("{value:.2}")
    } else {
        format!("{value:.6}")
    }
}
