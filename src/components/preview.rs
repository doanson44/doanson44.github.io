use leptos::prelude::*;
use leptos::html::Div;

use crate::domain::markdown::{RenderedMarkdown, RenderSegment};
use crate::infrastructure::mermaid::{render_mermaid, MermaidResult};

/// Markdown preview component.
///
/// Renders the parsed Markdown as HTML and Mermaid diagrams as SVG.
/// HTML segments use `inner_html` for rendering. Mermaid blocks are
/// rendered asynchronously via the infrastructure Mermaid adapter.
///
/// # Props
/// * `rendered` - Memo signal containing the parsed Markdown output
#[component]
pub fn Preview(rendered: Memo<RenderedMarkdown>) -> impl IntoView {
    view! {
        <div class="preview-panel d-flex flex-column h-100" id="preview-panel">
            <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                <span class="panel-title">
                    <i class="bi bi-eye me-2 text-success"></i>
                    "Preview"
                </span>
            </div>
            <div class="preview-content flex-grow-1 p-3 overflow-auto">
                {move || {
                    let rendered = rendered.get();
                    rendered.segments.into_iter().enumerate().map(|(idx, segment)| {
                        match segment {
                            RenderSegment::Html(html) => {
                                view! {
                                    <div class="markdown-body" inner_html=html></div>
                                }.into_any()
                            }
                            RenderSegment::Mermaid(block) => {
                                view! {
                                    <MermaidDiagram id=block.id code=block.code _key=idx />
                                }.into_any()
                            }
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

/// Component for rendering a single Mermaid diagram.
///
/// Renders asynchronously — shows a loading state while the Mermaid.js
/// library processes the diagram, then either displays the SVG or an
/// error message.
///
/// Invalid Mermaid syntax produces an isolated error state without
/// breaking the rest of the preview.
#[component]
fn MermaidDiagram(
    id: String,
    code: String,
    #[prop(optional)] _key: usize,
) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let id = id.clone();
    let code = code.clone();

    let mermaid_result = LocalResource::new(move || {
        let id = id.clone();
        let code = code.clone();
        async move {
            render_mermaid(&id, &code).await
        }
    });

    view! {
        <div class="mermaid-container my-3" node_ref=container_ref>
            <Suspense fallback=move || view! {
                <div class="mermaid-loading d-flex align-items-center justify-content-center p-4">
                    <div class="spinner-border spinner-border-sm text-primary me-2" role="status">
                        <span class="visually-hidden">"Loading..."</span>
                    </div>
                    <span class="text-body-secondary">"Rendering diagram..."</span>
                </div>
            }>
                {move || {
                    mermaid_result.get().map(|result| {
                        match &*result {
                            MermaidResult::Success(svg) => {
                                let svg = svg.clone();
                                view! {
                                    <div class="mermaid-diagram text-center p-3" inner_html=svg></div>
                                }.into_any()
                            }
                            MermaidResult::Error(err) => {
                                let err = err.clone();
                                view! {
                                    <div class="mermaid-error alert alert-danger d-flex align-items-start gap-2" role="alert">
                                        <i class="bi bi-exclamation-triangle-fill flex-shrink-0 mt-1"></i>
                                        <div>
                                            <strong>"Mermaid diagram error"</strong>
                                            <p class="mb-0 mt-1 small font-monospace">{err}</p>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
