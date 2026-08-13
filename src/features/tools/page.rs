use leptos::prelude::*;

use crate::domain::developer::ToolId;
use crate::domain::finance::FinanceTool;

#[component]
pub fn ToolsPage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let developer_tools = ToolId::all().collect::<Vec<_>>();
    let finance_tools = finance_tools();
    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-auto">
            <div class="container py-4">
                <div class="row align-items-center g-3 mb-4">
                    <div class="col"><h2 class="mb-0"><i class="bi bi-tools me-2 text-primary" aria-hidden="true"></i>"Tools"</h2></div>
                    <div class="col-12 col-md-5 col-lg-4"><div class="input-group"><span class="input-group-text"><i class="bi bi-search" aria-hidden="true"></i></span><input type="search" class="form-control" placeholder="Search tools..." aria-label="Search tools" prop:value=search on:input=move |ev| search.set(event_target_value(&ev)) /></div></div>
                </div>
                <Show when=move || search.get().trim().is_empty() fallback=move || view! { <SearchResults query=search.get() developer_tools=developer_tools.clone() finance_tools=finance_tools.clone() /> }>
                    <ToolSection title="General Tools" icon="bi-grid-3x3-gap">
                        <ToolCard href="#/tools/markdown" title="Markdown Studio" description="Live Markdown editor with Mermaid diagram support." />
                        <ToolCard href="#/tools/json" title="JSON Formatter" description="Validate, format, and minify JSON in your browser." />
                        <ToolCard href="#/tools/jwt" title="JWT Decoder" description="Decode JWT header, payload, and signature locally." />
                        <ToolCard href="#/tools/base64" title="Base64 Encoder / Decoder" description="Encode and decode UTF-8 text as standard Base64 locally." />
                        <ToolCard href="#/tools/time" title="Time & Utilities" description="World clock, countdown, stopwatch, ruler, and timestamp utilities." />
                        <ToolCard href="#/tools/finance" title="Finance Toolkit" description="Financial calculators for personal, investment, business, and valuation workflows." />
                    </ToolSection>
                    <ToolSection title="Developer Tools" icon="bi-code-slash">
                        {developer_tools.iter().copied().map(|tool| view! { <ToolCard href=format!("#/tools/{}", tool.route()) title=tool.title() description=tool.description() /> }).collect_view()}
                    </ToolSection>
                    <ToolSection title="Finance Tools" icon="bi-cash-stack">
                        {finance_tools.iter().copied().map(|tool| view! { <ToolCard href=format!("#/tools/finance/{}", tool.route()) title=tool.title() description=format!("{} calculator.", tool.category()) /> }).collect_view()}
                    </ToolSection>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn SearchResults(
    query: String,
    developer_tools: Vec<ToolId>,
    finance_tools: Vec<FinanceTool>,
) -> impl IntoView {
    let query = query.trim().to_lowercase();
    let mut results: Vec<AnyView> = Vec::new();
    let general = [
        (
            "#/tools/markdown",
            "Markdown Studio",
            "Live Markdown editor with Mermaid diagram support.",
        ),
        (
            "#/tools/json",
            "JSON Formatter",
            "Validate, format, and minify JSON in your browser.",
        ),
        (
            "#/tools/jwt",
            "JWT Decoder",
            "Decode JWT header, payload, and signature locally.",
        ),
        (
            "#/tools/base64",
            "Base64 Encoder / Decoder",
            "Encode and decode UTF-8 text as standard Base64 locally.",
        ),
        (
            "#/tools/time",
            "Time & Utilities",
            "World clock, countdown, stopwatch, ruler, and timestamp utilities.",
        ),
        (
            "#/tools/finance",
            "Finance Toolkit",
            "Financial calculators for personal, investment, business, and valuation workflows.",
        ),
    ];
    for (href, title, description) in general {
        if title.to_lowercase().contains(&query) || description.to_lowercase().contains(&query) {
            results.push(
                view! { <ToolCard href=href title=title description=description/> }.into_any(),
            );
        }
    }
    for tool in developer_tools {
        if tool.title().to_lowercase().contains(&query)
            || tool.description().to_lowercase().contains(&query)
        {
            results.push(view! { <ToolCard href=format!("#/tools/{}", tool.route()) title=tool.title() description=tool.description()/> }.into_any());
        }
    }
    for tool in finance_tools {
        if tool.title().to_lowercase().contains(&query)
            || tool.category().to_lowercase().contains(&query)
        {
            results.push(view! { <ToolCard href=format!("#/tools/finance/{}", tool.route()) title=tool.title() description=format!("{} calculator.", tool.category())/> }.into_any());
        }
    }
    let count = results.len();
    view! {
        <section aria-live="polite">
            <div class="small text-body-secondary mb-3">{format!("{} result(s)", count)}</div>
            <div class="row g-3">
                {if results.is_empty() { view! { <div class="col-12"><div class="alert alert-secondary" role="status">"No tools found. Try another search term."</div></div> }.into_any() } else { results.into_iter().collect_view().into_any() }}
            </div>
        </section>
    }
}

#[component]
fn ToolSection(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    let section_id = format!("section-{}", title.to_lowercase().replace(' ', "-"));
    view! {
        <section class="mb-5" aria-labelledby=section_id.clone()>
            <div class="d-flex align-items-center gap-2 mb-3"><i class=format!("bi {} text-primary", icon) aria-hidden="true"></i><h3 class="h5 mb-0" id=section_id>{title}</h3></div>
            <div class="row g-3">{children()}</div>
        </section>
    }
}

#[component]
fn ToolCard(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! { <div class="col-12 col-sm-6 col-lg-4"><a href=href class="card bg-body-tertiary border-secondary h-100 text-decoration-none text-body"><div class="card-body p-3"><h6 class="card-title mb-1"><i class="bi bi-wrench-adjustable text-primary me-2" aria-hidden="true"></i>{title}</h6><p class="card-text text-body-secondary small mb-0">{description}</p></div></a></div> }
}

fn finance_tools() -> Vec<FinanceTool> {
    vec![
        FinanceTool::CompoundInterest,
        FinanceTool::Loan,
        FinanceTool::Mortgage,
        FinanceTool::InvestmentReturn,
        FinanceTool::PresentFutureValue,
        FinanceTool::Roi,
        FinanceTool::Cagr,
        FinanceTool::BreakEven,
        FinanceTool::Budget,
        FinanceTool::SavingsGoal,
        FinanceTool::EmergencyFund,
        FinanceTool::DebtPayoff,
        FinanceTool::NetWorth,
        FinanceTool::Budget503020,
        FinanceTool::Dca,
        FinanceTool::StockReturn,
        FinanceTool::Dividend,
        FinanceTool::PortfolioAllocation,
        FinanceTool::PositionSize,
        FinanceTool::RealReturn,
        FinanceTool::ProfitMargin,
        FinanceTool::MarkupMargin,
        FinanceTool::Ebitda,
        FinanceTool::CashFlow,
        FinanceTool::BurnRate,
        FinanceTool::Runway,
        FinanceTool::CacLtv,
        FinanceTool::Dcf,
        FinanceTool::Npv,
        FinanceTool::Irr,
        FinanceTool::BondYtm,
        FinanceTool::FuturesPnl,
        FinanceTool::OptionsPnl,
        FinanceTool::RiskReward,
        FinanceTool::LeverageLiquidation,
        FinanceTool::CurrencyConverter,
        FinanceTool::Inflation,
        FinanceTool::PurchasingPower,
        FinanceTool::CurrencyChange,
        FinanceTool::Discount,
        FinanceTool::TaxPrice,
        FinanceTool::PercentageChange,
    ]
}
