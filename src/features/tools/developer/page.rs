use leptos::prelude::*;

use crate::domain::developer::ToolId;
use crate::features::tools::developer::state::DeveloperToolsState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn DeveloperToolPage(tool: ToolId) -> impl IntoView {
    let state = DeveloperToolsState::new(tool);
    let title = tool.title();
    let description = tool.description();
    let secondary_label = tool.secondary_label();
    let output_is_svg = tool.is_svg_output();

    let run = move |_| state.run(tool);
    let on_copy = move |_| {
        let output = state.output.get_untracked();
        if output.is_empty() { return; }
        let copied = state.copied;
        wasm_bindgen_futures::spawn_local(async move { if copy_to_clipboard(&output).await.is_ok() { copied.set(true); } });
    };

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-hidden">
            <div class="toolbar d-flex flex-wrap align-items-center gap-1 p-2 border-bottom border-secondary">
                <div class="ms-auto d-flex flex-wrap gap-1">
                    <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title=format!("Run {}", title) on:click=run>
                        <i class="bi bi-play" aria-hidden="true"></i><span class="d-none d-lg-inline ms-1">"Run"</span>
                    </button>
                    <button type="button" class="btn btn-outline-secondary btn-sm toolbar-btn" title="Reset sample" on:click=move |_| state.reset(tool)>
                        <i class="bi bi-arrow-counterclockwise" aria-hidden="true"></i><span class="d-none d-lg-inline ms-1">"Reset"</span>
                    </button>
                    <button type="button" class="btn btn-outline-danger btn-sm toolbar-btn" title="Clear input" on:click=move |_| state.clear(tool)>
                        <i class="bi bi-trash3" aria-hidden="true"></i><span class="d-none d-lg-inline ms-1">"Clear"</span>
                    </button>
                </div>
            </div>

            <header class="px-3 py-2 border-bottom border-secondary bg-body-tertiary flex-shrink-0">
                <div class="d-flex align-items-center gap-2"><i class="bi bi-tools text-primary" aria-hidden="true"></i><strong>{title}</strong></div>
                <div class="small text-body-secondary">{description}</div>
            </header>

            {move || state.error.get().map(|error| view! {
                <div class="alert alert-danger rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="alert">
                    <i class="bi bi-exclamation-triangle-fill" aria-hidden="true"></i><span>{error}</span>
                </div>
            })}

            <div class="developer-tool-grid flex-grow-1 overflow-hidden">
                <section class="editor-panel d-flex flex-column h-100" aria-labelledby="developer-input-title">
                    <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                        <span class="panel-title" id="developer-input-title"><i class="bi bi-pencil-square me-2 text-primary" aria-hidden="true"></i>{if tool == ToolId::Regex { "Pattern" } else { "Input" }}</span>
                        <span class="text-body-secondary small">{move || format!("{} lines", line_count(&state.source.get()))}</span>
                    </div>
                    <textarea class="editor-textarea form-control rounded-0 border-0 flex-grow-1" placeholder="Enter input..." spellcheck="false" aria-label=format!("{} input", title) prop:value=move || state.source.get() on:input=move |ev| state.set_source(tool, event_target_value(&ev))></textarea>
                    {secondary_label.map(|label| view! {
                        <div class="border-top border-secondary p-2 flex-shrink-0">
                            <label class="form-label small text-body-secondary mb-1" for="developer-secondary-input">{label}</label>
                            <textarea id="developer-secondary-input" class="form-control form-control-sm font-monospace" rows="4" prop:value=move || state.secondary.get() on:input=move |ev| state.set_secondary(event_target_value(&ev))></textarea>
                        </div>
                    })}
                </section>

                <section class="preview-panel d-flex flex-column h-100" aria-labelledby="developer-result-title">
                    <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                        <span class="panel-title" id="developer-result-title"><i class="bi bi-eye me-2 text-success" aria-hidden="true"></i>"Result"</span>
                        <button type="button" class="btn btn-outline-primary btn-sm ms-auto" disabled=move || state.output.get().is_empty() on:click=on_copy title="Copy result" aria-label="Copy result">
                            <i class="bi bi-clipboard" aria-hidden="true"></i><span class="d-none d-md-inline ms-1" aria-live="polite">{move || if state.copied.get() { "Copied" } else { "Copy" }}</span>
                        </button>
                    </div>
                    <div class="preview-content flex-grow-1 p-3 overflow-auto">
                        {move || { let output = state.output.get(); if output.is_empty() { view! { <div class="h-100 d-flex align-items-center justify-content-center text-body-secondary"><div class="text-center"><i class="bi bi-tools display-6 d-block mb-2" aria-hidden="true"></i><span>"Run the tool to see the result."</span></div></div> }.into_any() } else if output_is_svg { view! { <div class="h-100 d-flex align-items-center justify-content-center" inner_html=output></div> }.into_any() } else { view! { <pre class="mb-0"><code class="font-monospace">{output}</code></pre> }.into_any() } }}
                    </div>
                </section>
            </div>
        </div>
    }
}

fn line_count(content: &str) -> usize { if content.is_empty() { 0 } else { content.lines().count() } }
