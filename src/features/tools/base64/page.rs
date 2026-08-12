use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::features::tools::base64::state::Base64State;
use crate::infrastructure::browser::copy_to_clipboard;

/// Base64 encoder and decoder page.
#[component]
pub fn Base64Page() -> impl IntoView {
    let state = Base64State::new();
    let split_pct = RwSignal::new(45u32);
    let dragging = RwSignal::new(false);
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();

    let on_divider_down = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        dragging.set(true);
        let Some(document) = window().and_then(|window| window.document()) else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };

        let on_move = move |ev: web_sys::MouseEvent| {
            if !dragging.get_untracked() {
                return;
            }
            let width = window()
                .and_then(|window| window.inner_width().ok())
                .and_then(|value| value.as_f64())
                .unwrap_or(1.0);
            split_pct.set(((ev.client_x() as f64 / width) * 100.0).clamp(25.0, 65.0) as u32);
        };
        let on_up = move |_: web_sys::MouseEvent| dragging.set(false);
        let on_move_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_move) as Box<dyn FnMut(web_sys::MouseEvent)>
        );
        let on_up_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_up) as Box<dyn FnMut(web_sys::MouseEvent)>
        );
        let _ =
            body.add_event_listener_with_callback("mousemove", on_move_cb.as_ref().unchecked_ref());
        let _ = body.add_event_listener_with_callback("mouseup", on_up_cb.as_ref().unchecked_ref());
        on_move_cb.forget();
        on_up_cb.forget();
    };

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
            <div class="toolbar d-flex flex-wrap align-items-center gap-1 p-2 border-bottom border-secondary" id="base64-toolbar">
                <div class="ms-auto d-flex flex-wrap gap-1">
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

            <div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
                <div class="editor-pane" style=move || format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", split_pct.get(), split_pct.get())>
                    <div class="editor-panel d-flex flex-column h-100" id="base64-editor-panel">
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
                </div>

                <div class="divider" on:mousedown=on_divider_down title="Drag to resize panels" role="separator" aria-label="Resize Base64 input and output"></div>

                <div class="preview-pane" style=move || format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", 100 - split_pct.get(), 100 - split_pct.get())>
                    <div class="preview-panel d-flex flex-column h-100" id="base64-preview-panel">
                        <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-braces-asterisk me-2 text-success"></i>"Output"</span>
                            <button
                                type="button"
                                class="btn btn-outline-secondary btn-sm ms-auto"
                                title="Copy output"
                                aria-label="Copy output"
                                disabled=move || state.output.get().is_empty()
                                on:click=copy_output
                            ><i class="bi bi-clipboard"></i></button>
                        </div>
                        <div class="preview-content flex-grow-1 p-3 overflow-auto">
                            <pre class="base64-output mb-0"><code class="font-monospace">{move || state.output.get()}</code></pre>
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
