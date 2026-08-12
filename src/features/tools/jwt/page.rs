use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::jwt::state::JwtState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn JwtPage() -> impl IntoView {
    let state = JwtState::new();
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();

    let on_input_scroll = move |_| {
        if let (Some(input), Some(line_numbers)) = (input_ref.get(), line_numbers_ref.get()) {
            line_numbers.set_scroll_top(input.scroll_top());
        }
    };

    view! {
        <div class="d-flex flex-column flex-grow-1 jwt-page">
            <div class="toolbar d-flex flex-nowrap align-items-center gap-1 p-2 border-bottom border-secondary" id="jwt-toolbar">
                <div class="ms-auto d-flex flex-nowrap gap-1">
                    <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Decode JWT" on:click=move |_| state.decode()>
                        <i class="bi bi-unlock"></i><span class="d-none d-lg-inline ms-1">"Decode"</span>
                    </button>
                    <button type="button" class="btn btn-outline-secondary btn-sm toolbar-btn" title="Reset to sample JWT" on:click=move |_| state.reset()>
                        <i class="bi bi-arrow-counterclockwise"></i><span class="d-none d-lg-inline ms-1">"Reset"</span>
                    </button>
                    <button type="button" class="btn btn-outline-danger btn-sm toolbar-btn" title="Clear JWT" on:click=move |_| state.clear()>
                        <i class="bi bi-trash3"></i><span class="d-none d-lg-inline ms-1">"Clear"</span>
                    </button>
                </div>
            </div>

            <div class="alert alert-warning rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="note">
                <i class="bi bi-shield-exclamation"></i>
                <span>"Decoded JWT data is not cryptographically verified. No token is sent to a server."</span>
            </div>

            {move || state.error.get().map(|error| view! {
                <div class="alert alert-danger rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="alert">
                    <i class="bi bi-exclamation-triangle-fill"></i><span>{error}</span>
                </div>
            })}

            <ToolSplit initial_ratio=40>
                <ToolPanel side=ToolPanelSide::First>
                    <div class="editor-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-key me-2 text-primary"></i>"Encoded JWT"</span>
                            <span class="text-body-secondary small">{move || format!("{} lines", line_count(&state.source.get()))}</span>
                        </div>
                        <div class="editor-body d-flex flex-grow-1 overflow-hidden">
                            <div class="line-numbers d-flex flex-column align-items-end pe-2" node_ref=line_numbers_ref aria-hidden="true">
                                {move || {
                                    let count = line_count(&state.source.get()).max(1);
                                    (1..=count).map(|number| view! { <span class="line-number">{number}</span> }).collect_view()
                                }}
                            </div>
                            <textarea id="jwt-input" class="editor-textarea form-control flex-grow-1" placeholder="Paste a JWT here..." spellcheck="false" aria-label="Encoded JWT input" prop:value=move || state.source.get() on:input=move |ev| state.set_content(event_target_value(&ev)) on:scroll=on_input_scroll node_ref=input_ref></textarea>
                        </div>
                    </div>
                </ToolPanel>

                <ToolDivider />

                <ToolPanel side=ToolPanelSide::Second>
                    <div class="preview-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-braces me-2 text-success"></i>"Decoded JWT"</span>
                        </div>
                        <div class="preview-content flex-grow-1 p-3 overflow-auto d-flex flex-column gap-3">
                            <JwtJsonPanel title="Header" icon="bi-file-earmark-code" value=state.header />
                            <JwtJsonPanel title="Payload" icon="bi-file-earmark-text" value=state.payload />
                            <div class="jwt-result-panel border rounded">
                                <div class="d-flex align-items-center px-3 py-2 border-bottom">
                                    <span class="panel-title"><i class="bi bi-fingerprint me-2 text-warning"></i>"Signature"</span>
                                    <button type="button" class="btn btn-outline-secondary btn-sm ms-auto" title="Copy signature" aria-label="Copy signature" disabled=move || state.signature.get().is_none() on:click=move |_| {
                                        if let Some(value) = state.signature.get_untracked() {
                                            wasm_bindgen_futures::spawn_local(async move { let _ = copy_to_clipboard(&value).await; });
                                        }
                                    }><i class="bi bi-clipboard"></i></button>
                                </div>
                                <pre class="mb-0 p-3 overflow-auto"><code class="font-monospace">{move || state.signature.get().unwrap_or_else(|| "No decoded signature".into())}</code></pre>
                            </div>
                        </div>
                    </div>
                </ToolPanel>
            </ToolSplit>
        </div>
    }
}

#[component]
fn JwtJsonPanel(
    title: &'static str,
    icon: &'static str,
    value: RwSignal<Option<serde_json::Value>>,
) -> impl IntoView {
    view! {
        <div class="jwt-result-panel border rounded">
            <div class="d-flex align-items-center px-3 py-2 border-bottom">
                <span class="panel-title"><i class=format!("bi {icon} me-2 text-primary")></i>{title}</span>
                <button type="button" class="btn btn-outline-secondary btn-sm ms-auto" title=format!("Copy {title}") aria-label=format!("Copy {title}") disabled=move || value.get().is_none() on:click=move |_| {
                    if let Some(json) = value.get_untracked() {
                        if let Ok(text) = serde_json::to_string_pretty(&json) {
                            wasm_bindgen_futures::spawn_local(async move { let _ = copy_to_clipboard(&text).await; });
                        }
                    }
                }><i class="bi bi-clipboard"></i></button>
            </div>
            <pre class="mb-0 p-3 overflow-auto"><code class="font-monospace">{move || value.get().and_then(|json| serde_json::to_string_pretty(&json).ok()).unwrap_or_else(|| "No decoded data".into())}</code></pre>
        </div>
    }
}

fn line_count(content: &str) -> usize {
    if content.is_empty() {
        0
    } else {
        content.lines().count()
    }
}
