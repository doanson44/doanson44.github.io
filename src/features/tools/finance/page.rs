use leptos::prelude::*;

use crate::application::services::finance::FinanceService;
use crate::domain::finance::FinanceTool;
use crate::features::tools::finance::state::FinanceState;

const ALL_TOOLS: &[FinanceTool] = &[
    FinanceTool::CompoundInterest, FinanceTool::Loan, FinanceTool::Mortgage, FinanceTool::InvestmentReturn, FinanceTool::PresentFutureValue, FinanceTool::Roi, FinanceTool::Cagr, FinanceTool::BreakEven,
    FinanceTool::Budget, FinanceTool::SavingsGoal, FinanceTool::EmergencyFund, FinanceTool::DebtPayoff, FinanceTool::NetWorth, FinanceTool::Budget503020,
    FinanceTool::Dca, FinanceTool::StockReturn, FinanceTool::Dividend, FinanceTool::PortfolioAllocation, FinanceTool::PositionSize, FinanceTool::RealReturn,
    FinanceTool::ProfitMargin, FinanceTool::MarkupMargin, FinanceTool::Ebitda, FinanceTool::CashFlow, FinanceTool::BurnRate, FinanceTool::Runway, FinanceTool::CacLtv,
    FinanceTool::Dcf, FinanceTool::Npv, FinanceTool::Irr, FinanceTool::BondYtm, FinanceTool::FuturesPnl, FinanceTool::OptionsPnl, FinanceTool::RiskReward, FinanceTool::LeverageLiquidation,
    FinanceTool::CurrencyConverter, FinanceTool::Inflation, FinanceTool::PurchasingPower, FinanceTool::CurrencyChange, FinanceTool::Discount, FinanceTool::TaxPrice, FinanceTool::PercentageChange,
];

/// Finance toolkit landing page and calculator page.
#[component]
pub fn FinancePage(#[prop(optional)] tool: Option<FinanceTool>) -> impl IntoView {
    match tool { Some(tool) => view! { <Calculator tool /> }.into_any(), None => view! { <FinanceIndex /> }.into_any() }
}

#[component]
fn FinanceIndex() -> impl IntoView {
    let categories = ["Core Finance", "Personal Finance", "Investment", "Business Finance", "Valuation", "Trading", "Currency & Utilities"];
    view! {
        <div class="container-fluid py-4 overflow-auto finance-page">
            <div class="container">
                <div class="mb-4"><h2 class="mb-1"><i class="bi bi-cash-coin me-2 text-primary"></i>"Finance Toolkit"</h2><p class="text-body-secondary mb-0">"Client-side financial calculators for planning, analysis, valuation, and trading math."</p></div>
                <div class="row g-3">
                    {categories.into_iter().map(|category| view! {
                        <section class="col-12 col-md-6 col-xl-4"><div class="card bg-body-tertiary border-secondary h-100"><div class="card-body"><h5 class="card-title">{category}</h5><div class="d-flex flex-column gap-1">
                            {ALL_TOOLS.iter().copied().filter(move |tool| tool.category() == category).map(|tool| view! { <a class="btn btn-sm btn-outline-secondary text-start" href=format!("#/tools/finance/{}", tool.route())>{tool.title()}</a> }).collect_view()}
                        </div></div></div></section>
                    }).collect_view()}
                </div>
                <p class="small text-body-secondary mt-4 mb-0">"For informational and calculation purposes only. Results are estimates and are not financial, tax, legal, lending, brokerage, or investment advice."</p>
            </div>
        </div>
    }
}

#[component]
fn Calculator(tool: FinanceTool) -> impl IntoView {
    let state = FinanceState::new(tool);
    let labels = tool.fields();
    let inputs = state.inputs;
    let series = state.series;
    let result = state.result;
    let error = state.error;

    let calculate = move |_| {
        let raw = inputs.get();
        let parsed = raw.iter().map(|value| value.trim().parse::<f64>().map_err(|_| "Every input must contain a valid number.".to_string())).collect::<Result<Vec<_>, _>>();
        let cashflows = series.get().split(|c: char| c == ',' || c == '\n' || c == ';').filter(|s| !s.trim().is_empty()).map(|s| s.trim().parse::<f64>().map_err(|_| "Cash flows must be comma- or line-separated numbers.".to_string())).collect::<Result<Vec<_>, _>>();
        match (parsed, cashflows) {
            (Ok(values), Ok(cashflows)) => match FinanceService::calculate(tool, &values, &cashflows) { Ok(value) => { result.set(Some(value)); error.set(None); }, Err(message) => { result.set(None); error.set(Some(message)); } },
            (Err(message), _) | (_, Err(message)) => { result.set(None); error.set(Some(message)); }
        }
    };
    let reset = move |_| { inputs.set(vec![String::new(); labels.len()]); series.set(String::new()); result.set(None); error.set(None); };

    view! {
        <div class="container-fluid py-4 overflow-auto finance-page"><div class="container">
            <div class="mb-3"><a href="#/tools/finance" class="btn btn-sm btn-outline-secondary mb-2"><i class="bi bi-arrow-left me-1"></i>"Finance Toolkit"</a><h2 class="mb-1">{tool.title()}</h2><p class="text-body-secondary mb-0">{tool.category()}</p></div>
            <div class="row g-3">
                <section class="col-12 col-lg-6"><div class="card bg-body-tertiary border-secondary h-100"><div class="card-body"><h5 class="card-title mb-3">"Inputs"</h5><div class="row g-3">
                    {labels.iter().enumerate().map(|(index, label)| view! { <div class="col-12 col-md-6"><label class="form-label" for=format!("finance-input-{index}")>{*label}</label><input id=format!("finance-input-{index}") class="form-control" type="number" step="any" value=move || inputs.get().get(index).cloned().unwrap_or_default() on:input=move |ev| { let mut values = inputs.get(); if let Some(value) = values.get_mut(index) { *value = event_target_value(&ev); } inputs.set(values); } /></div> }).collect_view()}
                    <div class="col-12"><label class="form-label" for="finance-series">"Cash flows (optional; comma or newline separated)"</label><textarea id="finance-series" class="form-control" rows="3" placeholder="-1000, 300, 400, 500" prop:value=move || series.get() on:input=move |ev| series.set(event_target_value(&ev))></textarea></div>
                </div><div class="d-flex justify-content-end gap-2 mt-3"><button type="button" class="btn btn-outline-secondary" on:click=reset>"Reset"</button><button type="button" class="btn btn-primary" on:click=calculate>"Calculate"</button></div></div></div></section>
                <section class="col-12 col-lg-6"><div class="card bg-body-tertiary border-secondary h-100"><div class="card-body"><h5 class="card-title mb-3">"Results"</h5>
                    {move || error.get().map(|message| view! { <div class="alert alert-danger" role="alert">{message}</div> })}
                    {move || result.get().map(|value| view! { <div class="row g-3">{value.metrics.into_iter().map(|metric| view! { <div class="col-12 col-sm-6"><div class="border border-secondary rounded p-3 h-100"><div class="small text-body-secondary">{metric.label}</div><div class="fs-4 fw-semibold mt-1">{format_number(metric.value)}</div></div></div> }).collect_view()}</div> })}
                    {move || if result.get().is_none() && error.get().is_none() { view! { <p class="text-body-secondary mb-0">"Enter values and calculate to see the result."</p> }.into_any() } else { view! { <span></span> }.into_any() }}
                </div></div></section>
            </div>
            <div class="alert alert-secondary mt-3 mb-0" role="note">"Results are estimates. Trading, lending, tax, currency, and liquidation rules can differ from the simplified models used here."</div>
        </div></div>
    }
}

fn format_number(value: f64) -> String { if value.abs() >= 1.0 { format!("{value:.2}") } else { format!("{value:.6}") } }
