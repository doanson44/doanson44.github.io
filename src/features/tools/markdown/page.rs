use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::components::editor::Editor;
use crate::components::preview::Preview;
use crate::components::toolbar::Toolbar;
use crate::features::tools::markdown::state::MarkdownState;

/// Markdown Studio page — composed of Toolbar, Editor, and Preview.
///
/// Supports two view modes via toggle:
/// - **Split** (default): Editor | Preview side by side with draggable divider
/// - **Preview-only**: Full-width preview, editor hidden
#[component]
pub fn MarkdownPage() -> impl IntoView {
    let state = MarkdownState::new();
    let split_pct = RwSignal::new(50u32);
    let dragging = RwSignal::new(false);

    // Auto-save content to localStorage on every keystroke
    Effect::new(move |_| {
        let content = state.source.get();
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("ms-content", &content);
        }
    });

    // Divider drag handlers
    let on_divider_down = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        dragging.set(true);
        let doc = window().unwrap().document().unwrap();
        let body = doc.body().unwrap();

        let on_move = {
            move |ev: web_sys::MouseEvent| {
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
            }
        };

        let on_up = {
            move |_: web_sys::MouseEvent| {
                dragging.set(false);
            }
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

        // Leak closures to keep them alive for the duration of the drag
        on_move_cb.forget();
        on_up_cb.forget();
    };

    view! {
        <div class="d-flex flex-column flex-grow-1" style="min-height: 0;">
            <Toolbar source=state.source toggle_preview=Callback::new(move |_| state.toggle_preview_only()) />

            <div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
                {move || {
                    if !state.preview_only.get() {
                        let pct = split_pct.get();
                        view! {
                            <>
                                <div class="editor-pane" style=format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", pct, pct)>
                                    <Editor source=state.source />
                                </div>
                                <div class="divider" on:mousedown=on_divider_down title="Drag to resize panels"></div>
                            </>
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                }}

                <div class="preview-pane" style=move || {
                    if state.preview_only.get() { "flex: 1;".to_string() }
                    else { format!("flex: 0 0 {}%; max-width: calc({}% - 1.5px);", 100 - split_pct.get(), 100 - split_pct.get()) }
                }>
                    <Preview rendered=state.rendered />
                </div>
            </div>
        </div>
    }
}
