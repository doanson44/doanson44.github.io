use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::features::tools::json::state::JsonState;
use crate::infrastructure::browser::copy_to_clipboard;

/// JSON Formatter page for validating, formatting, and minifying JSON.
#[component]
pub fn JsonPage() -> impl IntoView {
    let state = JsonState::new();
    let split_pct = RwSignal::new(50u32);
    let dragging = RwSignal::new(false);
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();

    let on_divider_down = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        dragging.set(true);
        let doc = window().unwrap().document().unwrap();
        let body = doc.body().unwrap();

        let on_move = move |ev: web_sys::MouseEvent| {
            if !dragging.get_untracked() {
                return;
            }
            let width = window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap_or(1.0);
            let x = ev.client_x() as f64;
            let new_pct = ((x / width) * 100.0).clamp(20.0, 80.0) as u32;
            split_pct.set(new_pct);
        };

        let on_up = move |_: web_sys::MouseEvent| {
            dragging.set(false);
        };

        let on_move_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_move) as Box<dyn FnMut(web_sys::MouseEvent)>
        );
        let on_up_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_up) as Box<dyn FnMut(web_sys::MouseEvent)>
        );

        body.add_event_listener_with_callback("mousemove", on_move_cb.as_ref().unchecked_ref())
            .ok();
        body.add_event_listener_with_callback("mouseup", on_up_cb.as_ref().unchecked_ref())
            .ok();

        on_move_cb.forget();
        on_up_cb.forget();
    };

    let on_input_scroll = move |_| {
        if let (Some(input), Some(line_numbers)) = (input_ref.get(), line_numbers_ref.get()) {
            line_numbers.set_scroll_top(input.scroll_top());
        }
    };

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
        <div class="d-flex flex-column flex-grow-1" style="min-height: 0;">
            <div class="toolbar d-flex flex-wrap align-items-center gap-1 p-2 border-bottom border-secondary" id="json-toolbar">
                <button
                    type="button"
                    class="btn btn-outline-primary btn-sm toolbar-btn"
                    title="Format JSON"
                    on:click=move |_| state.format()
                >
                    <i class="bi bi-text-indent-left"></i>
                    <span class="d-none d-lg-inline ms-1">"Format"</span>
                </button>
                <button
                    type="button"
                    class="btn btn-outline-secondary btn-sm toolbar-btn"
                    title="Minify JSON"
                    on:click=move |_| state.minify()
                >
                    <i class="bi bi-arrows-collapse"></i>
                    <span class="d-none d-lg-inline ms-1">"Minify"</span>
                </button>
                <button
                    type="button"
                    class="btn btn-outline-secondary btn-sm toolbar-btn"
                    title="Reset to sample JSON"
                    on:click=move |_| state.reset()
                >
                    <i class="bi bi-arrow-counterclockwise"></i>
                    <span class="d-none d-lg-inline ms-1">"Reset"</span>
                </button>
                <button
                    type="button"
                    class="btn btn-outline-danger btn-sm toolbar-btn"
                    title="Clear JSON"
                    on:click=move |_| state.clear()
                >
                    <i class="bi bi-trash3"></i>
                    <span class="d-none d-lg-inline ms-1">"Clear"</span>
                </button>
            </div>

            {move || state.error.get().map(|error| view! {
                <div class="alert alert-danger rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="alert">
                    <i class="bi bi-exclamation-triangle-fill"></i>
                    <span>{error}</span>
                </div>
            })}

            <div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
                <div
                    class="editor-pane"
                    style=move || format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", split_pct.get(), split_pct.get())
                >
                    <div class="editor-panel d-flex flex-column h-100" id="json-editor-panel">
                        <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title">
                                <i class="bi bi-pencil-square me-2 text-primary"></i>
                                "Input"
                            </span>
                            <span class="text-body-secondary small" aria-live="polite">
                                {move || format!("{} lines", line_count(&state.source.get()))}
                            </span>
                        </div>
                        <div class="editor-body d-flex flex-grow-1 overflow-hidden">
                            <div
                                class="line-numbers d-flex flex-column align-items-end pe-2"
                                node_ref=line_numbers_ref
                                aria-hidden="true"
                            >
                                {move || {
                                    let lines: Vec<_> = state.source.get().lines().enumerate().map(|(i, _)| {
                                        view! { <span class="line-number">{i + 1}</span> }
                                    }).collect();
                                    if lines.is_empty() {
                                        view! { <span class="line-number">"1"</span> }.into_any()
                                    } else {
                                        lines.into_iter().map(|line| line.into_any()).collect::<Vec<_>>().into_any()
                                    }
                                }}
                            </div>
                            <textarea
                                id="json-input"
                                class="editor-textarea form-control flex-grow-1"
                                placeholder="Paste JSON here..."
                                spellcheck="false"
                                aria-label="JSON input"
                                prop:value=move || state.source.get()
                                on:input=move |ev| state.set_content(event_target_value(&ev))
                                on:scroll=on_input_scroll
                                node_ref=input_ref
                            ></textarea>
                        </div>
                    </div>
                </div>

                <div class="divider" on:mousedown=on_divider_down title="Drag to resize panels"></div>

                <div
                    class="preview-pane"
                    style=move || format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", 100 - split_pct.get(), 100 - split_pct.get())
                >
                    <div class="preview-panel d-flex flex-column h-100" id="json-preview-panel">
                        <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title">
                                <i class="bi bi-eye me-2 text-success"></i>
                                "Preview"
                            </span>
                            <button
                                type="button"
                                class="btn btn-outline-primary btn-sm ms-auto"
                                disabled=move || state.output.get().is_empty()
                                on:click=on_copy
                                title="Copy formatted JSON"
                                aria-label="Copy formatted JSON"
                            >
                                <i class="bi bi-clipboard"></i>
                                <span class="d-none d-md-inline ms-1" aria-live="polite">
                                    {move || if state.copied.get() { "Copied" } else { "Copy" }}
                                </span>
                            </button>
                        </div>
                        <div class="preview-content flex-grow-1 p-3 overflow-auto" id="json-preview-content">
                            {move || {
                                let output = state.output.get();
                                if output.is_empty() {
                                    view! {
                                        <div class="h-100 d-flex align-items-center justify-content-center text-body-secondary">
                                            <div class="text-center">
                                                <i class="bi bi-braces display-6 d-block mb-2"></i>
                                                <span>"Format or minify JSON to see the result."</span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <pre class="mb-0"><code class="font-monospace">{output}</code></pre>
                                    }.into_any()
                                }
                            }}
                        </div>
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
