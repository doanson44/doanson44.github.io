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

    let search = RwSignal::new(String::new());

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-auto">
            <div class="container py-4">
                <div class="d-flex flex-column flex-md-row align-items-md-center justify-content-between gap-3 mb-4">
                    <h2 class="mb-0">
                        <i class="bi bi-tools me-2 text-primary"></i>"Tools"
                    </h2>
                    <div class="input-group" style="max-width: 420px;">
                        <span class="input-group-text" id="tools-search-label">
                            <i class="bi bi-search" aria-hidden="true"></i>
                        </span>
                        <input
                            type="search"
                            class="form-control"
                            placeholder="Search tools..."
                            aria-label="Search tools"
                            aria-describedby="tools-search-label"
                            prop:value=search
                            on:input=move |ev| search.set(event_target_value(&ev))
                        />
                        <Show when=move || !search.get().is_empty()>
                            <button
                                type="button"
                                class="btn btn-outline-secondary"
                                title="Clear search"
                                aria-label="Clear search"
                                on:click=move |_| search.set(String::new())
                            >
                                <i class="bi bi-x-lg" aria-hidden="true"></i>
                            </button>
                        </Show>
                    </div>
                </div>

                <Show
                    when=move || search.get().trim().is_empty()
                    fallback=move || view! {
                        <SearchResults
                            query=search.get()
                            developer_tools=developer_tools
                        />
                    }
                >
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

                    <FinanceSection title="Finance — Core Finance" icon="bi-cash-stack" tools=CORE_FINANCE_TOOLS />
                    <FinanceSection title="Finance — Personal Finance" icon="bi-wallet2" tools=PERSONAL_FINANCE_TOOLS />
                    <FinanceSection title="Finance — Investment" icon="bi-graph-up-arrow" tools=INVESTMENT_TOOLS />
                    <FinanceSection title="Finance — Business Finance" icon="bi-building" tools=BUSINESS_FINANCE_TOOLS />
                    <FinanceSection title="Finance — Valuation" icon="bi-bar-chart-line" tools=VALUATION_TOOLS />
                    <FinanceSection title="Finance — Trading" icon="bi-activity" tools=TRADING_TOOLS />
                    <FinanceSection title="Finance — Currency & Utilities" icon="bi-currency-exchange" tools=CURRENCY_TOOLS />
                </Show>
            </div>
        </div>
    }
}

#[component]
fn SearchResults(
    query: String,
    developer_tools: [ToolKind; 15],
) -> impl IntoView {
    let query = query.trim().to_lowercase();
    let matches = move || {
        let mut results: Vec<View> = Vec::new();
        let general = [
            ("#/tools/markdown", "bi-markdown-fill", "Markdown Studio", "Live Markdown editor with Mermaid diagram support."),
            ("#/tools/json", "bi-braces", "JSON Formatter", "Validate, format, and minify JSON in your browser."),
            ("#/tools/jwt", "bi-key", "JWT Decoder", "Decode JWT header, payload, and signature locally."),
            ("#/tools/base64", "bi-file-binary", "Base64 Encoder / Decoder", "Encode and decode UTF-8 text as standard Base64 locally."),
            ("#/tools/time", "bi-clock-history", "Time & Utilities", "World clock, countdown, stopwatch, ruler, and timestamp conversion."),
        ];

        for (href, icon, title, description) in general {
            if title.to_lowercase().contains(&query) || description.to_lowercase().contains(&query) {
                results.push(view! { <ToolCard href=href icon=icon title=title description=description /> }.into_view());
            }
        }

        for kind in developer_tools {
            if kind.title().to_lowercase().contains(&query) || kind.description().to_lowercase().contains(&query) {
                results.push(view! {
                    <ToolCard href=format!("#/tools/{}", kind.route()) icon="bi-wrench-adjustable" title=kind.title() description=kind.description() />
                }.into_view());
            }
        }

        let finance_tools = [
            CORE_FINANCE_TOOLS,
            PERSONAL_FINANCE_TOOLS,
            INVESTMENT_TOOLS,
            BUSINESS_FINANCE_TOOLS,
            VALUATION_TOOLS,
            TRADING_TOOLS,
            CURRENCY_TOOLS,
        ];
        for tools in finance_tools {
            for tool in tools.iter().copied() {
                if tool.title().to_lowercase().contains(&query) || tool.category().to_lowercase().contains(&query) {
                    results.push(view! { <FinanceToolCard tool /> }.into_view());
                }
            }
        }
        results
    };

    view! {
        <section aria-live="polite">
            <div class="mb-3 text-body-secondary small">{move || format!("{} result(s)", matches().len())}</div>
            <Show
                when=move || !matches().is_empty()
                fallback=|| view! {
                    <div class="alert alert-secondary" role="status">
                        <i class="bi bi-search me-2" aria-hidden="true"></i>
                        "No tools found. Try another search term."
                    </div>
                }
            >
                <div class="row g-3">{move || matches()}</div>
            </Show>
        </section>
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
            <div class="row g-3">{children()}</div>
        </section>
    }
}

#[component]
fn FinanceSection(title: &'static str, icon: &'static str, tools: &'static [FinanceTool]) -> impl IntoView {
    view! {
        <ToolSection title=title icon=icon>
            {tools.iter().copied().map(|tool| view! { <FinanceToolCard tool /> }).collect_view()}
        </ToolSection>
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
            <div class="card bg-body-tertiary border-secondary h-100">
                <div class="card-body p-3 d-flex gap-3">
                    <a href=href class="text-decoration-none text-body flex-grow-1 min-w-0">
                        <h6 class="card-title mb-1">
                            <i class=format!("bi {} text-primary me-2", icon) aria-hidden="true"></i>
                            {title}
                        </h6>
                        <p class="card-text text-body-secondary small mb-0">{description}</p>
                    </a>
                    <button
                        type="button"
                        class="btn btn-sm btn-outline-secondary flex-shrink-0 align-self-start"
                        title=format!("Show information about {}", title)
                        aria-label=format!("Show information about {}", title)
                        data-bs-toggle="modal"
                        data-bs-target="#tool-info-modal"
                    >
                        <i class="bi bi-info-lg" aria-hidden="true"></i>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FinanceToolCard(tool: FinanceTool) -> impl IntoView {
    view! {
        <div class="col-12 col-sm-6 col-lg-4">
            <div class="card bg-body-tertiary border-secondary h-100">
                <div class="card-body p-3 d-flex gap-3">
                    <a href=format!("#/tools/finance/{}", tool.route()) class="text-decoration-none text-body flex-grow-1 min-w-0">
                        <h6 class="card-title mb-1">
                            <i class="bi bi-cash-coin text-primary me-2" aria-hidden="true"></i>
                            {tool.title()}
                        </h6>
                        <p class="card-text text-body-secondary small mb-0">{tool.category()}</p>
                    </a>
                    <button
                        type="button"
                        class="btn btn-sm btn-outline-secondary flex-shrink-0 align-self-start"
                        title=format!("Show information about {}", tool.title())
                        aria-label=format!("Show information about {}", tool.title())
                        data-bs-toggle="modal"
                        data-bs-target="#tool-info-modal"
                    >
                        <i class="bi bi-info-lg" aria-hidden="true"></i>
                    </button>
                </div>
            </div>
        </div>
    }
}
