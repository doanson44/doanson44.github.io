use leptos::prelude::*;

use crate::domain::developer_tools::ToolKind;
use crate::domain::finance::FinanceTool;

const CORE_FINANCE_TOOLS: &[FinanceTool] = &[
    FinanceTool::CompoundInterest,
    FinanceTool::Loan,
    FinanceTool::Mortgage,
    FinanceTool::InvestmentReturn,
    FinanceTool::PresentFutureValue,
    FinanceTool::Roi,
    FinanceTool::Cagr,
    FinanceTool::BreakEven,
];

const PERSONAL_FINANCE_TOOLS: &[FinanceTool] = &[
    FinanceTool::Budget,
    FinanceTool::SavingsGoal,
    FinanceTool::EmergencyFund,
    FinanceTool::DebtPayoff,
    FinanceTool::NetWorth,
    FinanceTool::Budget503020,
];

const INVESTMENT_TOOLS: &[FinanceTool] = &[
    FinanceTool::Dca,
    FinanceTool::StockReturn,
    FinanceTool::Dividend,
    FinanceTool::PortfolioAllocation,
    FinanceTool::PositionSize,
    FinanceTool::RealReturn,
];

const BUSINESS_FINANCE_TOOLS: &[FinanceTool] = &[
    FinanceTool::ProfitMargin,
    FinanceTool::MarkupMargin,
    FinanceTool::Ebitda,
    FinanceTool::CashFlow,
    FinanceTool::BurnRate,
    FinanceTool::Runway,
    FinanceTool::CacLtv,
];

const VALUATION_TOOLS: &[FinanceTool] = &[
    FinanceTool::Dcf,
    FinanceTool::Npv,
    FinanceTool::Irr,
    FinanceTool::BondYtm,
];

const TRADING_TOOLS: &[FinanceTool] = &[
    FinanceTool::FuturesPnl,
    FinanceTool::OptionsPnl,
    FinanceTool::RiskReward,
    FinanceTool::LeverageLiquidation,
];

const CURRENCY_TOOLS: &[FinanceTool] = &[
    FinanceTool::CurrencyConverter,
    FinanceTool::Inflation,
    FinanceTool::PurchasingPower,
    FinanceTool::CurrencyChange,
    FinanceTool::Discount,
    FinanceTool::TaxPrice,
    FinanceTool::PercentageChange,
];

#[component]
pub fn ToolsPage() -> impl IntoView {
    let developer_tools = [
        ToolKind::Xml,
        ToolKind::Yaml,
        ToolKind::Sql,
        ToolKind::Html,
        ToolKind::Css,
        ToolKind::Javascript,
        ToolKind::Regex,
        ToolKind::Url,
        ToolKind::Hash,
        ToolKind::Uuid,
        ToolKind::Color,
        ToolKind::Cron,
        ToolKind::HttpStatus,
        ToolKind::Subnet,
        ToolKind::Qr,
    ];

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-auto">
            <div class="container py-4">
                <h2 class="mb-4">
                    <i class="bi bi-tools me-2 text-primary"></i>"Tools"
                </h2>

                <ToolSection title="General Tools" icon="bi-grid-3x3-gap">
                    <ToolCard href="#/tools/markdown" icon="bi-markdown-fill" title="Markdown Studio" description="Live Markdown editor with Mermaid diagram support." />
                    <ToolCard href="#/tools/json" icon="bi-braces" title="JSON Formatter" description="Validate, format, and minify JSON in your browser." />
                    <ToolCard href="#/tools/jwt" icon="bi-key" title="JWT Decoder" description="Decode JWT header, payload, and signature locally." />
                    <ToolCard href="#/tools/base64" icon="bi-file-binary" title="Base64 Encoder / Decoder" description="Encode and decode UTF-8 text as standard Base64 locally." />
                    <ToolCard href="#/tools/time" icon="bi-clock-history" title="Time & Utilities" description="World clock, countdown, stopwatch, ruler, and timestamp conversion." />
                </ToolSection>

                <ToolSection title="Developer Tools" icon="bi-code-slash">
                    {developer_tools.into_iter().map(|kind| view! {
                        <ToolCard
                            href=format!("#/tools/{}", kind.route())
                            icon="bi-wrench-adjustable"
                            title=kind.title()
                            description=kind.description()
                        />
                    }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Core Finance" icon="bi-cash-stack">
                    {CORE_FINANCE_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Personal Finance" icon="bi-wallet2">
                    {PERSONAL_FINANCE_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Investment" icon="bi-graph-up-arrow">
                    {INVESTMENT_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Business Finance" icon="bi-building">
                    {BUSINESS_FINANCE_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Valuation" icon="bi-bar-chart-line">
                    {VALUATION_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Trading" icon="bi-activity">
                    {TRADING_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>

                <ToolSection title="Finance — Currency & Utilities" icon="bi-currency-exchange">
                    {CURRENCY_TOOLS.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
                </ToolSection>
            </div>
        </div>
    }
}

#[component]
fn ToolSection(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="mb-5" aria-labelledby=format!("section-{}", title.to_lowercase().replace(' ', "-"))>
            <div class="d-flex align-items-center gap-2 mb-3">
                <i class=format!("bi {} text-primary", icon) aria-hidden="true"></i>
                <h3 class="h5 mb-0" id=format!("section-{}", title.to_lowercase().replace(' ', "-"))>{title}</h3>
            </div>
            <div class="row g-3">
                {children()}
            </div>
        </section>
    }
}

#[component]
fn ToolCard(
    #[prop(into)] href: String,
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="col-12 col-sm-6 col-lg-4">
            <a href=href class="text-decoration-none">
                <div class="card bg-body-tertiary border-secondary h-100">
                    <div class="card-body p-3">
                        <h6 class="card-title mb-1">
                            <i class=format!("bi {} text-primary me-2", icon) aria-hidden="true"></i>
                            {title}
                        </h6>
                        <p class="card-text text-body-secondary small mb-0">{description}</p>
                    </div>
                </div>
            </a>
        </div>
    }
}

#[component]
fn FinanceToolCard(tool: FinanceTool) -> impl IntoView {
    view! {
        <div class="col-12 col-sm-6 col-lg-4">
            <a href=format!("#/tools/finance/{}", tool.route()) class="text-decoration-none">
                <div class="card bg-body-tertiary border-secondary h-100">
                    <div class="card-body p-3">
                        <h6 class="card-title mb-1">
                            <i class="bi bi-cash-coin text-primary me-2" aria-hidden="true"></i>
                            {tool.title()}
                        </h6>
                        <p class="card-text text-body-secondary small mb-0">{tool.category()}</p>
                    </div>
                </div>
            </a>
        </div>
    }
}
