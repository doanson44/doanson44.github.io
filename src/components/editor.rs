use leptos::prelude::*;

/// Markdown editor component with a textarea and line numbers.
#[component]
pub fn Editor(
    source: RwSignal<String>,
    #[prop(default = "markdown-editor")] textarea_id: &'static str,
) -> impl IntoView {
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();
    let on_input = move |ev: leptos::ev::Event| source.set(event_target_value(&ev));
    let on_scroll = move |_| {
        if let (Some(ta), Some(ln)) = (textarea_ref.get(), line_numbers_ref.get()) {
            ln.set_scroll_top(ta.scroll_top());
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
        <div class="flex h-full flex-col overflow-hidden" id="editor-panel">
            <div class="flex items-center justify-between border-b border-[var(--border-color)] px-3 py-2">
                <span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"✎"</span>"Editor"</span>
                <span class="text-xs text-[var(--text-secondary)]">{move || format!("{} lines · {} chars", line_count(), char_count())}</span>
            </div>
            <div class="flex flex-1 overflow-hidden">
                <div class="flex min-w-12 flex-col items-end overflow-hidden border-r border-[var(--border-color)] bg-[var(--surface)] pe-2 text-xs text-[var(--text-tertiary)]" node_ref=line_numbers_ref aria-hidden="true">
                    {move || {
                        let lines: Vec<_> = source
                            .get()
                            .lines()
                            .enumerate()
                            .map(|(i, _)| view! { <span class="line-number min-h-6">{i + 1}</span> })
                            .collect();
                        if lines.is_empty() {
                            view! { <span class="line-number min-h-6">"1"</span> }.into_any()
                        } else {
                            lines.into_iter().map(|line| line.into_any()).collect::<Vec<_>>().into_any()
                        }
                    }}
                </div>
                <textarea id=textarea_id class="editor-textarea min-w-0 flex-1 resize-none border-0 bg-transparent p-3 font-mono text-sm leading-6 text-[var(--text-primary)] outline-none" placeholder="Write your Markdown here..." spellcheck="false" prop:value=move || source.get() on:input=on_input on:scroll=on_scroll node_ref=textarea_ref></textarea>
            </div>
        </div>
    }
}
