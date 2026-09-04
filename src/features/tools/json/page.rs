use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::json::state::JsonState;
use crate::infrastructure::browser::copy_to_clipboard;

/// JSON Formatter page for validating, formatting, and minifying JSON.
#[component]
pub fn JsonPage() -> impl IntoView {
    let state = JsonState::new();
    let input_ref = NodeRef::<leptos::html::Textarea>::new();
    let line_numbers_ref = NodeRef::<leptos::html::Div>::new();
    let button = "inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";

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
        <main class="flex flex-1 flex-col overflow-hidden">
            <div class="flex flex-nowrap items-center gap-1 border-b border-[var(--border-color)] p-2" id="json-toolbar">
                <div class="ml-auto flex flex-nowrap gap-1">
                    <button type="button" class=button title="Format JSON" on:click=move |_| state.format()><span aria-hidden="true">"≡"</span><span class="ml-1 hidden lg:inline">"Format"</span></button>
                    <button type="button" class=button title="Minify JSON" on:click=move |_| state.minify()><span aria-hidden="true">"↕"</span><span class="ml-1 hidden lg:inline">"Minify"</span></button>
                    <button type="button" class=button title="Reset to sample JSON" on:click=move |_| state.reset()><span aria-hidden="true">"↶"</span><span class="ml-1 hidden lg:inline">"Reset"</span></button>
                    <button type="button" class=button title="Clear JSON" on:click=move |_| state.clear()><span aria-hidden="true">"×"</span><span class="ml-1 hidden lg:inline">"Clear"</span></button>
                </div>
            </div>
            {move || state.error.get().map(|error| view! { <div class="flex items-start gap-2 border-b border-red-400/40 bg-red-400/10 px-3 py-2 text-sm text-red-300" role="alert"><span aria-hidden="true">"⚠"</span><span>{error}</span></div> })}
            <ToolSplit initial_ratio=50>
                <ToolPanel side=ToolPanelSide::First>
                    <div class="flex h-full flex-col overflow-hidden">
                        <div class="flex items-center justify-between border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"✎"</span>"Input"</span><span class="text-xs text-[var(--text-secondary)]" aria-live="polite">{move || format!("{} lines", line_count(&state.source.get()))}</span></div>
                        <div class="flex flex-1 overflow-hidden">
                            <div class="flex flex-col items-end pe-2 text-xs text-[var(--text-secondary)]" node_ref=line_numbers_ref aria-hidden="true">{move || { let lines: Vec<_> = state.source.get().lines().enumerate().map(|(i, _)| view! { <span class="line-number">{i + 1}</span> }).collect(); if lines.is_empty() { view! { <span class="line-number">"1"</span> }.into_any() } else { lines.into_iter().map(|line| line.into_any()).collect::<Vec<_>>().into_any() } }}</div>
                            <textarea id="json-input" class="editor-textarea min-w-0 flex-1 resize-none border-0 bg-transparent p-3 font-mono text-sm text-[var(--text-primary)] outline-none" placeholder="Paste JSON here..." spellcheck="false" aria-label="JSON input" prop:value=move || state.source.get() on:input=move |ev| state.set_content(event_target_value(&ev)) on:scroll=on_input_scroll node_ref=input_ref></textarea>
                        </div>
                    </div>
                </ToolPanel>
                <ToolDivider />
                <ToolPanel side=ToolPanelSide::Second>
                    <div class="flex h-full flex-col overflow-hidden">
                        <div class="flex items-center border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"◉"</span>"Preview"</span><button type="button" class=format!("{} ml-auto", button) disabled=move || state.output.get().is_empty() on:click=on_copy title="Copy formatted JSON" aria-label="Copy formatted JSON"><span aria-hidden="true">"⧉"</span><span class="ml-1 hidden md:inline" aria-live="polite">{move || if state.copied.get() { "Copied" } else { "Copy" }}</span></button></div>
                        <div class="flex-1 overflow-auto p-3">
                            {move || { let output = state.output.get(); if output.is_empty() { view! { <div class="flex h-full items-center justify-center text-sm text-[var(--text-secondary)]"><div class="text-center"><div class="mb-2 text-3xl" aria-hidden="true">"{}"</div><span>"Format or minify JSON to see the result."</span></div></div> }.into_any() } else { view! { <pre class="m-0 whitespace-pre-wrap"><code class="font-mono text-sm text-[var(--text-primary)]">{output}</code></pre> }.into_any() } }}
                        </div>
                    </div>
                </ToolPanel>
            </ToolSplit>
        </main>
    }
}

fn line_count(content: &str) -> usize {
    if content.is_empty() {
        0
    } else {
        content.lines().count()
    }
}
