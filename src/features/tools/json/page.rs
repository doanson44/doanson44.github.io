use leptos::prelude::*;

use crate::features::tools::json::state::JsonState;
use crate::infrastructure::browser::copy_to_clipboard;

/// JSON Formatter page for validating, formatting, and minifying JSON.
#[component]
pub fn JsonPage() -> impl IntoView {
    let state = JsonState::new();

    let on_copy = move |_| {
        let output = state.output.get_untracked();
        if output.is_empty() {
            return;
        }

        state.copied.set(false);
        let copied = state.copied;
        wasm_bindgen_futures::spawn_local(async move {
            if copy_to_clipboard(&output).await.is_ok() {
                copied.set(true);
            }
        });
    };

    view! {
        <div class="d-flex flex-column flex-grow-1 overflow-hidden">
            <div class="container-fluid py-3 flex-grow-1 d-flex flex-column overflow-hidden">
                <div class="d-flex flex-wrap align-items-center justify-content-between gap-2 mb-3">
                    <div>
                        <h2 class="mb-1">
                            <i class="bi bi-braces text-primary me-2"></i>
                            "JSON Formatter"
                        </h2>
                        <p class="text-body-secondary small mb-0">
                            "Validate, format, and minify JSON entirely in your browser."
                        </p>
                    </div>
                    <div class="d-flex flex-wrap gap-2">
                        <button
                            type="button"
                            class="btn btn-primary"
                            on:click=move |_| state.format()
                            title="Format JSON"
                        >
                            <i class="bi bi-text-indent-left me-1"></i>
                            "Format"
                        </button>
                        <button
                            type="button"
                            class="btn btn-outline-primary"
                            on:click=move |_| state.minify()
                            title="Minify JSON"
                        >
                            <i class="bi bi-arrows-collapse me-1"></i>
                            "Minify"
                        </button>
                        <button
                            type="button"
                            class="btn btn-outline-secondary"
                            on:click=move |_| state.clear()
                            title="Clear JSON"
                        >
                            <i class="bi bi-x-lg me-1"></i>
                            "Clear"
                        </button>
                    </div>
                </div>

                {move || state.error.get().map(|error| view! {
                    <div class="alert alert-danger d-flex align-items-start gap-2" role="alert">
                        <i class="bi bi-exclamation-triangle-fill"></i>
                        <span>{error}</span>
                    </div>
                })}

                <div class="row g-3 flex-grow-1 overflow-hidden">
                    <div class="col-12 col-lg-6 d-flex flex-column">
                        <div class="d-flex align-items-center justify-content-between mb-2">
                            <label for="json-input" class="form-label fw-semibold mb-0">"Input"</label>
                            <span class="text-body-secondary small" aria-live="polite">
                                {move || format!("{} lines", line_count(&state.source.get()))}
                            </span>
                        </div>
                        <div class="json-editor flex-grow-1 d-flex overflow-hidden">
                            <div class="json-line-numbers text-body-secondary font-monospace" aria-hidden="true">
                                {move || {
                                    let count = line_count(&state.source.get()).max(1);
                                    (1..=count)
                                        .map(|line| view! { <div>{line}</div> })
                                        .collect_view()
                                }}
                            </div>
                            <textarea
                                id="json-input"
                                class="form-control font-monospace flex-grow-1"
                                placeholder="Paste JSON here..."
                                aria-label="JSON input"
                                prop:value=move || state.source.get()
                                on:input=move |ev| state.source.set(event_target_value(&ev))
                            ></textarea>
                        </div>
                    </div>

                    <div class="col-12 col-lg-6 d-flex flex-column">
                        <div class="d-flex align-items-center justify-content-between mb-2">
                            <label for="json-output" class="form-label fw-semibold mb-0">"Output"</label>
                            <button
                                type="button"
                                class="btn btn-sm btn-outline-secondary py-0 px-2"
                                disabled=move || state.output.get().is_empty()
                                on:click=on_copy
                                title="Copy formatted JSON"
                            >
                                <i class="bi bi-clipboard" aria-hidden="true"></i>
                                <span class="visually-hidden">"Copy formatted JSON"</span>
                                {move || if state.copied.get() { "Copied" } else { "Copy" }}
                            </button>
                        </div>
                        <textarea
                            id="json-output"
                            class="form-control font-monospace flex-grow-1"
                            readonly=true
                            aria-label="Formatted JSON output"
                            prop:value=move || state.output.get()
                        ></textarea>
                    </div>
                </div>
            </div>
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
