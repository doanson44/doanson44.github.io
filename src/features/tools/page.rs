use leptos::prelude::*;

use crate::domain::developer::ToolId;
use crate::domain::finance::FinanceTool;

#[component]
pub fn ToolsPage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    view! {
        <main class="flex flex-1 flex-col overflow-auto">
            <div class="mx-auto w-full max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
                <div class="mb-6 flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
                    <h1 class="text-2xl font-bold text-[var(--text-primary)]">"⚒ Tools"</h1>
                    <div class="relative w-full md:max-w-sm">
                        <span class="pointer-events-none absolute inset-y-0 left-3 flex items-center text-[var(--text-secondary)]" aria-hidden="true">"⌕"</span>
                        <input type="search" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] py-2 pl-9 pr-3 text-sm text-[var(--text-primary)] outline-none placeholder:text-[var(--text-secondary)] focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]" placeholder="Search tools..." aria-label="Search tools" prop:value=search on:input=move |ev| search.set(event_target_value(&ev)) />
                    </div>
                </div>
                <Show
                    when=move || search.get().trim().is_empty()
                    fallback=move || view! { <SearchResults query=search.get() developer_tools=ToolId::all().collect() finance_tools=finance_tools() /> }
                >
                    <ToolSection title="General Tools" icon="▦">
                        <ToolCard href="#/tools/markdown" title="Markdown Studio" description="Live Markdown editor with Mermaid diagram support." />
                        <ToolCard href="#/tools/json" title="JSON Formatter" description="Validate, format, and minify JSON in your browser." />
                        <ToolCard href="#/tools/jwt" title="JWT Decoder" description="Decode JWT header, payload, and signature locally." />
                        <ToolCard href="#/tools/base64" title="Base64 Encoder / Decoder" description="Encode and decode UTF-8 text as standard Base64 locally." />
                        <ToolCard href="#/tools/time" title="Time & Utilities" description="World clock, countdown, stopwatch, ruler, and timestamp utilities." />
                    </ToolSection>
                    <ToolSection title="Developer Tools" icon="</>">
                        {ToolId::all()
                            .map(|tool| view! { <ToolCard href=format!("#/tools/{}", tool.route()) title=tool.title() description=tool.description() /> })
                            .collect_view()}
                    </ToolSection>
                    <ToolSection title="Finance Tools" icon="$">
                        {finance_tools()
                            .into_iter()
                            .map(|tool| view! { <ToolCard href=format!("#/tools/finance/{}", tool.route()) title=tool.title() description=format!("{} calculator.", tool.category()) /> })
                            .collect_view()}
                    </ToolSection>
                </Show>
            </div>
        </main>
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
            results.push(
                view! { <ToolCard href=format!("#/tools/{}", tool.route()) title=tool.title() description=tool.description()/> }
                    .into_any(),
            );
        }
    }
    for tool in finance_tools {
        if tool.title().to_lowercase().contains(&query)
            || tool.category().to_lowercase().contains(&query)
        {
            results.push(
                view! { <ToolCard href=format!("#/tools/finance/{}", tool.route()) title=tool.title() description=format!("{} calculator.", tool.category())/> }
                    .into_any(),
            );
        }
    }
    let count = results.len();
    view! {
        <section aria-live="polite">
            <div class="mb-3 text-sm text-[var(--text-secondary)]">{format!("{} result(s)", count)}</div>
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
                {if results.is_empty() {
                    view! { <div class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4 text-sm text-[var(--text-secondary)]" role="status">"No tools found. Try another search term."</div> }.into_any()
                } else {
                    results.into_iter().collect_view().into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn ToolSection(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    let section_id = format!("section-{}", title.to_lowercase().replace(' ', "-"));
    let section_id2 = section_id.clone();
    view! {
        <section class="mb-10" aria-labelledby=section_id>
            <div class="mb-3 flex items-center gap-2">
                <span class="text-[var(--accent)]" aria-hidden="true">{icon}</span>
                <h2 class="text-lg font-semibold text-[var(--text-primary)]" id=section_id2>{title}</h2>
            </div>
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">{children()}</div>
        </section>
    }
}

#[component]
fn ToolCard(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <a href=href class="group rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4 no-underline transition hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]">
            <h3 class="mb-1 text-sm font-semibold text-[var(--text-primary)]"><span class="mr-2 text-[var(--accent)]" aria-hidden="true">"◆"</span>{title}</h3>
            <p class="m-0 text-sm leading-6 text-[var(--text-secondary)]">{description}</p>
        </a>
    }
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
