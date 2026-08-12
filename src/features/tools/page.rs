use leptos::prelude::*;

use crate::domain::developer_tools::ToolKind;

#[component]
pub fn ToolsPage() -> impl IntoView {
    let developer_tools = [ToolKind::Xml, ToolKind::Yaml, ToolKind::Sql, ToolKind::Html, ToolKind::Css, ToolKind::Javascript, ToolKind::Regex, ToolKind::Url, ToolKind::Hash, ToolKind::Uuid, ToolKind::Color, ToolKind::Cron, ToolKind::HttpStatus, ToolKind::Subnet, ToolKind::Qr];
    view! {
        <div class="d-flex flex-column flex-grow-1"><div class="container py-4">
            <h2 class="mb-4"><i class="bi bi-tools me-2 text-primary"></i>"Tools"</h2>
            <div class="row g-3">
                <ToolCard href="#/tools/markdown" icon="bi-markdown-fill" title="Markdown Studio" description="Live Markdown editor with Mermaid diagram support." />
                <ToolCard href="#/tools/json" icon="bi-braces" title="JSON Formatter" description="Validate, format, and minify JSON in your browser." />
                <ToolCard href="#/tools/jwt" icon="bi-key" title="JWT Decoder" description="Decode JWT header, payload, and signature locally." />
                <ToolCard href="#/tools/base64" icon="bi-file-binary" title="Base64 Encoder / Decoder" description="Encode and decode UTF-8 text as standard Base64 locally." />
                <ToolCard href="#/tools/time" icon="bi-clock-history" title="Time & Utilities" description="World clock, countdown, stopwatch, ruler, and timestamp conversion." />
                <ToolCard href="#/tools/finance" icon="bi-cash-coin" title="Finance Toolkit" description="Financial calculators for savings, loans, investing, valuation, trading, and more." />
                {developer_tools.into_iter().map(|kind| view! { <ToolCard href=format!("#/tools/{}", kind.route()) icon="bi-wrench-adjustable" title=kind.title() description=kind.description() /> }).collect_view()}
            </div>
        </div></div>
    }
}

#[component]
fn ToolCard(#[prop(into)] href: String, icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! { <div class="col-12 col-sm-6 col-lg-4"><a href=href class="text-decoration-none"><div class="card bg-body-tertiary border-secondary h-100"><div class="card-body p-3"><h6 class="card-title mb-1"><i class=format!("bi {} text-primary me-2", icon)></i>{title}</h6><p class="card-text text-body-secondary small mb-0">{description}</p></div></div></a></div> }
}
