use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::domain::developer_tools::ToolKind;
use crate::features::tools::developer::state::DeveloperToolsState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn DeveloperToolPage(kind: ToolKind) -> impl IntoView {
    let state = DeveloperToolsState::new(kind);
    let split_pct = RwSignal::new(50u32);
    let dragging = RwSignal::new(false);
    let mode = RwSignal::new(if kind == ToolKind::Url { "decode" } else { "run" });
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();
    let secondary_ref = NodeRef::<leptos::html::Textarea>::new();

    let on_divider_down = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        dragging.set(true);
        let Some(body) = window().and_then(|w| w.document()).and_then(|d| d.body()) else { return; };
        let on_move = move |ev: web_sys::MouseEvent| {
            if !dragging.get_untracked() { return; }
            let width = window().and_then(|w| w.inner_width().ok()).and_then(|v| v.as_f64()).unwrap_or(1.0);
            split_pct.set(((ev.client_x() as f64 / width) * 100.0).clamp(20.0, 80.0) as u32);
        };
        let on_up = move |_: web_sys::MouseEvent| dragging.set(false);
        let move_cb = wasm_bindgen::closure::Closure::wrap(Box::new(on_move) as Box<dyn FnMut(_)>);
        let up_cb = wasm_bindgen::closure::Closure::wrap(Box::new(on_up) as Box<dyn FnMut(_)>);
        let _ = body.add_event_listener_with_callback("mousemove", move_cb.as_ref().unchecked_ref());
        let _ = body.add_event_listener_with_callback("mouseup", up_cb.as_ref().unchecked_ref());
        move_cb.forget();
        up_cb.forget();
    };

    let on_input_scroll = move |_| {
        if let (Some(input), Some(lines)) = (input_ref.get(), line_numbers_ref.get()) { lines.set_scroll_top(input.scroll_top()); }
    };

    let run = move || {
        if kind == ToolKind::Url { state.set_secondary(mode.get_untracked().to_string()); }
        state.run(kind);
    };

    let on_copy = move |_| {
        let output = state.output.get_untracked();
        if output.is_empty() { return; }
        state.copied.set(false);
        let copied = state.copied;
        wasm_bindgen_futures::spawn_local(async move { if copy_to_clipboard(&output).await.is_ok() { copied.set(true); } });
    };

    let secondary_visible = kind == ToolKind::Regex;
    let output_is_svg = kind == ToolKind::Qr;
    let title = kind.title();
    let description = kind.description();

    view! {
        <div class="d-flex flex-column flex-grow-1">
            <div class="toolbar d-flex flex-wrap align-items-center gap-1 p-2 border-bottom border-secondary">
                <div class="ms-auto d-flex flex-wrap gap-1">
                    {if kind == ToolKind::Url { view! {
                        <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Encode URL" on:click=move |_| { mode.set("encode"); run(); }><i class="bi bi-lock"></i><span class="d-none d-lg-inline ms-1">"Encode"</span></button>
                        <button type="button" class="btn btn-outline-secondary btn-sm toolbar-btn" title="Decode URL" on:click=move |_| { mode.set("decode"); run(); }><i class="bi bi-unlock"></i><span class="d-none d-lg-inline ms-1">"Decode"</span></button>
                    }.into_any() } else if kind == ToolKind::Uuid { view! {
                        <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Generate UUID" on:click=move |_| run()><i class="bi bi-arrow-repeat"></i><span class="d-none d-lg-inline ms-1">"Generate"</span></button>
                    }.into_any() } else { view! {
                        <button type="button" class="btn btn-outline-primary btn-sm toolbar-btn" title="Run tool" on:click=move |_| run()><i class="bi bi-play"></i><span class="d-none d-lg-inline ms-1">"Run"</span></button>
                    }.into_any() }}
                    <button type="button" class="btn btn-outline-secondary btn-sm toolbar-btn" title="Reset sample" on:click=move |_| state.reset(kind)><i class="bi bi-arrow-counterclockwise"></i><span class="d-none d-lg-inline ms-1">"Reset"</span></button>
                    <button type="button" class="btn btn-outline-danger btn-sm toolbar-btn" title="Clear input" on:click=move |_| state.clear(kind)><i class="bi bi-trash3"></i><span class="d-none d-lg-inline ms-1">"Clear"</span></button>
                </div>
            </div>

            <div class="px-3 py-2 border-bottom border-secondary bg-body-tertiary">
                <div class="d-flex align-items-center gap-2"><i class="bi bi-tools text-primary"></i><strong>{title}</strong></div>
                <div class="small text-body-secondary">{description}</div>
            </div>

            {move || state.error.get().map(|error| view! { <div class="alert alert-danger rounded-0 border-0 border-bottom d-flex align-items-start gap-2 mb-0" role="alert"><i class="bi bi-exclamation-triangle-fill"></i><span>{error}</span></div> })}

            <div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
                <div class=move || format!("editor-pane flex-grow-0 flex-shrink-0 {}", split_class(split_pct.get()))>
                    <div class="editor-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-pencil-square me-2 text-primary"></i>{if kind == ToolKind::Regex { "Pattern" } else { "Input" }}</span>
                            <span class="text-body-secondary small">{move || format!("{} lines", line_count(&state.source.get()))}</span>
                        </div>
                        <div class="editor-body d-flex flex-grow-1 overflow-hidden">
                            <div class="line-numbers d-flex flex-column align-items-end pe-2" node_ref=line_numbers_ref aria-hidden="true">
                                {move || (1..=line_count(&state.source.get()).max(1)).map(|n| view! { <span class="line-number">{n}</span> }).collect_view()}
                            </div>
                            <textarea class="editor-textarea form-control flex-grow-1" placeholder="Enter input..." spellcheck="false" aria-label=format!("{} input", title) prop:value=move || state.source.get() on:input=move |ev| state.set_source(kind, event_target_value(&ev)) on:scroll=on_input_scroll node_ref=input_ref></textarea>
                        </div>
                        {if secondary_visible { view! {
                            <div class="border-top border-secondary p-2"><label class="form-label small text-body-secondary mb-1" for="regex-test-input">"Test String"</label><textarea id="regex-test-input" class="form-control form-control-sm font-monospace" rows="4" prop:value=move || state.secondary.get() on:input=move |ev| state.set_secondary(event_target_value(&ev)) node_ref=secondary_ref></textarea></div>
                        }.into_any() } else { view! { <span class="d-none"></span> }.into_any() }}
                    </div>
                </div>

                <div class="divider" on:mousedown=on_divider_down title="Drag to resize panels" role="separator" aria-label="Resize editor and result panels"></div>

                <div class=move || format!("preview-pane flex-grow-0 flex-shrink-0 {}", split_class(100 - split_pct.get()))>
                    <div class="preview-panel d-flex flex-column h-100">
                        <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                            <span class="panel-title"><i class="bi bi-eye me-2 text-success"></i>"Result"</span>
                            <button type="button" class="btn btn-outline-primary btn-sm ms-auto" disabled=move || state.output.get().is_empty() on:click=on_copy title="Copy result" aria-label="Copy result"><i class="bi bi-clipboard"></i><span class="d-none d-md-inline ms-1" aria-live="polite">{move || if state.copied.get() { "Copied" } else { "Copy" }}</span></button>
                        </div>
                        <div class="preview-content flex-grow-1 p-3 overflow-auto">
                            {move || { let output = state.output.get(); if output.is_empty() { view! { <div class="h-100 d-flex align-items-center justify-content-center text-body-secondary"><div class="text-center"><i class="bi bi-tools display-6 d-block mb-2"></i><span>"Run the tool to see the result."</span></div></div> }.into_any() } else if output_is_svg { view! { <div class="h-100 d-flex align-items-center justify-content-center" inner_html=output></div> }.into_any() } else { view! { <pre class="mb-0"><code class="font-monospace">{output}</code></pre> }.into_any() } }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn line_count(content: &str) -> usize { if content.is_empty() { 0 } else { content.lines().count() } }
fn split_class(value: u32) -> &'static str { if value < 38 { "w-25" } else if value < 63 { "w-50" } else { "w-75" } }
