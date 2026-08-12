use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::base64::state::Base64State;
use crate::infrastructure::browser::copy_to_clipboard;

/// Base64 encoder and decoder page.
#[component]
pub fn Base64Page() -> impl IntoView {
    let state = Base64State::new();
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();

    let on_input_scroll = move |_| {
        if let (Some(input), Some(line_numbers)) = (input_ref.get(), line_numbers_ref.get()) {
            line_numbers.set_scroll_top(input.scroll_top());
        }
    };

    let copy_output = move |_| {
        let output = state.output.get_untracked();
        if output.is_empty() {
            return;
        }
        wasm_bindgen_futures::spawn_local(async move {
            let _ = copy_to_clipboard(&output).await;
        });
    };

    view! {
        <div class="d-flex flex-column flex-grow-1 base64-page">
            <div class="toolbar d-flex flex-nowrap align-items-center gap-1 p-2 border-bottom border-secondary" id="base64-toolbar">
                <div class="ms-auto d-flex flex-nowrap gap-1">
                    <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Encode text as Base64" on:click=move |_| state.encode()>
                        <i class="bi bi-arrow-up-circle"></i><span class="d-none d-lg-inline ms-1">"Encode"</span>
                    </button>
                    <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Decode Base64 as UTF-8 text" on:click=move |_| state.decode()>
                        <i class="bi bi-arrow-down-circle"></i><span class="d-none d-lg-inline ms-1">"Decode"</span>
                    </button>
                    <button type="button" class="btn btn-outline-secondary btn-sm toolbar-btn" title="Reset to sample text" on:click=move |_| state.reset()>
                        <i class="bi bi-arrow-counterclockwise"></i><span class="d-none d-lg-inline ms-1">"Reset"</span>
                    </button>
                    <button type="button" class="btn btn-outline-danger btn-sm toolbar-btn" title="Clear input and output" on:click=move |_| state.clear()>
                        <i class="bi bi-trash3"></i><span class="d-none d-lg-inline ms-1">"Clear"</span>
                    </button>
                </div>
            </div>

            {move || state.error.get().map(|error| view! {
                <div class="alert alert-danger rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="alert">
                    <i class="bi bi-exclamation-triangle-fill"></i>
                    <span>{error}</span>
                </div>
            })}

            <ToolSplit initial_ratio=45>
                <ToolPanel side=ToolPanelSide::First>
                    <div class="editor-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-input-cursor-text me-2 text-primary"></i>"Input"</span>
                            <span class="text-body-secondary small">{move || format!("{} lines", line_count(&state.source.get()))}</span>
                        </div>
                        <div class="editor-body d-flex flex-grow-1 overflow-hidden">
                            <div class="line-numbers d-flex flex-column align-items-end pe-2" node_ref=line_numbers_ref aria-hidden="true">
                                {move || {
                                    let count = line_count(&state.source.get()).max(1);
                                    (1..=count).map(|number| view! { <span class="line-number">{number}</span> }).collect_view()
                                }}
                            </div>
                            <textarea
                                id="base64-input"
                                class="editor-textarea form-control flex-grow-1"
                                placeholder="Enter text or Base64..."
                                spellcheck="false"
                                aria-label="Base64 input"
                                prop:value=move || state.source.get()
                                on:input=move |ev| state.set_content(event_target_value(&ev))
                                on:scroll=on_input_scroll
                                node_ref=input_ref
                            ></textarea>
                        </div>
                    </div>
                </ToolPanel>

                <ToolDivider />

                <ToolPanel side=ToolPanelSide::Second>
                    <div class="preview-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-braces-asterisk me-2 text-success"></i>"Output"</span>
                            <button type="button" class="btn btn-outline-secondary btn-sm ms-auto" title="Copy output" aria-label="Copy output" disabled=move || state.output.get().is_empty() on:click=copy_output>
                                <i class="bi bi-clipboard"></i>
                            </button>
                        </div>
                        <div class="preview-content flex-grow-1 p-3 overflow-auto">
                            <pre class="base64-output mb-0"><code class="font-monospace">{move || state.output.get()}</code></pre>
                        </div>
                    </div>
                </ToolPanel>
            </ToolSplit>
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
