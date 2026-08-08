use leptos::prelude::*;

/// Markdown editor component with a textarea.
///
/// Provides a full-height, monospace textarea for Markdown input.
/// Updates the reactive source signal on every input event.
///
/// # Props
/// * `source` - The reactive Markdown source signal
/// * `textarea_id` - The DOM ID for the textarea element
#[component]
pub fn Editor(
    source: RwSignal<String>,
    #[prop(default = "markdown-editor")] textarea_id: &'static str,
) -> impl IntoView {
    let on_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        source.set(value);
    };

    // Character and line count derived signals
    let char_count = move || source.get().len();
    let line_count = move || {
        let content = source.get();
        if content.is_empty() {
            0
        } else {
            content.lines().count()
        }
    };

    view! {
        <div class="editor-panel d-flex flex-column h-100" id="editor-panel">
            <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
                <span class="panel-title">
                    <i class="bi bi-pencil-square me-2 text-primary"></i>
                    "Editor"
                </span>
                <span class="text-body-secondary small">
                    {move || format!("{} lines · {} chars", line_count(), char_count())}
                </span>
            </div>
            <textarea
                id=textarea_id
                class="editor-textarea form-control flex-grow-1"
                placeholder="Write your Markdown here..."
                spellcheck="false"
                prop:value=move || source.get()
                on:input=on_input
            ></textarea>
        </div>
    }
}
