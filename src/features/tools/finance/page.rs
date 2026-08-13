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
            (Ok(values), Ok(cashflows)) => match FinanceService::calculate(tool, &values, &cashflows) {
                Ok(value) => {
                    result.set(Some(value));
                    error.set(None);
                }
                Err(message) => {
                    result.set(None);
                    error.set(Some(message));
                }
            },
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
        <div class="container-fluid py-4 overflow-auto finance-page">
            <div class="container">
                <div class="mb-3">
                    <a href="#/tools" class="btn btn-sm btn-outline-secondary mb-2">
                        <i class="bi bi-arrow-left me-1" aria-hidden="true"></i>
                        "Back to Tools"
                    </a>
                    <h2 class="mb-1">{tool.title()}</h2>
                    <p class="text-body-secondary mb-0">{tool.category()}</p>
                </div>
                <div class="row g-3">
                    <section class="col-12 col-lg-6">
                        <div class="card bg-body-tertiary border-secondary h-100">
                            <div class="card-body">
                                <h5 class="card-title mb-3">"Inputs"</h5>
                                <div class="row g-3">
                                    {labels
                                        .iter()
                                        .enumerate()
                                        .map(|(index, label)| {
                                            view! {
                                                <div class="col-12 col-md-6">
                                                    <label class="form-label" for=format!("finance-input-{index}")>{*label}</label>
                                                    <input
                                                        id=format!("finance-input-{index}")
                                                        class="form-control"
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
                                    <div class="col-12">
                                        <label class="form-label" for="finance-series">"Cash flows (optional; comma or newline separated)"</label>
                                        <textarea
                                            id="finance-series"
                                            class="form-control"
                                            rows="3"
                                            placeholder="-1000, 300, 400, 500"
                                            prop:value=move || series.get()
                                            on:input=move |ev| series.set(event_target_value(&ev))
                                        ></textarea>
                                    </div>
                                </div>
                                <div class="d-flex justify-content-end gap-2 mt-3">
                                    <button type="button" class="btn btn-outline-secondary" on:click=reset>"Reset"</button>
                                    <button type="button" class="btn btn-primary" on:click=calculate>"Calculate"</button>
                                </div>
                            </div>
                        </div>
                    </section>
                    <section class="col-12 col-lg-6">
                        <div class="card bg-body-tertiary border-secondary h-100">
                            <div class="card-body">
                                <h5 class="card-title mb-3">"Results"</h5>
                                {move || error.get().map(|message| view! {
                                    <div class="alert alert-danger" role="alert">{message}</div>
                                })}
                                {move || result.get().map(|value| view! {
                                    <div class="row g-3">
                                        {value.metrics.into_iter().map(|metric| view! {
                                            <div class="col-12 col-sm-6">
                                                <div class="border border-secondary rounded p-3 h-100">
                                                    <div class="small text-body-secondary">{metric.label}</div>
                                                    <div class="fs-4 fw-semibold mt-1">{format_number(metric.value)}</div>
                                                </div>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                })}
                                {move || if result.get().is_none() && error.get().is_none() {
                                    view! { <p class="text-body-secondary mb-0">"Enter values and calculate to see the result."</p> }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        </div>
                    </section>
                </div>
                <div class="alert alert-secondary mt-3 mb-0" role="note">
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
