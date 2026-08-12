use leptos::prelude::*;

/// Markdown editor component with a textarea and line numbers.
///
/// Provides a full-height, monospace textarea for Markdown input
/// with a synchronized line number gutter.
///
/// # Props
/// * `source` - The reactive Markdown source signal
/// * `textarea_id` - The DOM ID for the textarea element
#[component]
pub fn Editor(
    source: RwSignal<String>,
    #[prop(default = "markdown-editor")] textarea_id: &'static str,
) -> impl IntoView {
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();

    let on_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        source.set(value);
    };

    let on_scroll = move |_| {
        if let (Some(ta), Some(ln)) = (textarea_ref.get(), line_numbers_ref.get()) {
            let scroll_top = ta.scroll_top();
            ln.set_scroll_top(scroll_top);
        }
    };

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
            <div class="editor-body d-flex flex-grow-1 overflow-hidden">
                <div
                    class="line-numbers d-flex flex-column align-items-end pe-2"
                    node_ref=line_numbers_ref
                    style="overflow: hidden; user-select: none;"
                >
                    {move || {
                        let lines: Vec<_> = source.get().lines().enumerate().map(|(i, _)| {
                            view! { <span class="line-number">{i + 1}</span> }
                        }).collect();
                        if lines.is_empty() {
                            view! { <span class="line-number">"1"</span> }.into_any()
                        } else {
                            lines.into_iter().map(|l| l.into_any()).collect::<Vec<_>>().into_any()
                        }
                    }}
                </div>
                <textarea
                    id=textarea_id
                    class="editor-textarea form-control flex-grow-1"
                    placeholder="Write your Markdown here..."
                    spellcheck="false"
                    prop:value=move || source.get()
                    on:input=on_input
                    on:scroll=on_scroll
                    node_ref=textarea_ref
                ></textarea>
            </div>
        </div>
    }
}
