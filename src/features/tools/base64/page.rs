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
        if output.is_empty() { return; }
        wasm_bindgen_futures::spawn_local(async move { let _ = copy_to_clipboard(&output).await; });
    };

    let button = "inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";

    view! {
        <main class="flex flex-1 flex-col overflow-hidden">
            <div class="flex flex-nowrap items-center gap-1 border-b border-[var(--border-color)] p-2" id="base64-toolbar">
                <div class="ml-auto flex flex-nowrap gap-1">
                    <button type="button" class=button title="Encode text as Base64" on:click=move |_| state.encode()><span aria-hidden="true">"↑"</span><span class="ml-1 hidden lg:inline">"Encode"</span></button>
                    <button type="button" class=button title="Decode Base64 as UTF-8 text" on:click=move |_| state.decode()><span aria-hidden="true">"↓"</span><span class="ml-1 hidden lg:inline">"Decode"</span></button>
                    <button type="button" class=button title="Reset to sample text" on:click=move |_| state.reset()><span aria-hidden="true">"↶"</span><span class="ml-1 hidden lg:inline">"Reset"</span></button>
                    <button type="button" class=button title="Clear input and output" on:click=move |_| state.clear()><span aria-hidden="true">"×"</span><span class="ml-1 hidden lg:inline">"Clear"</span></button>
                </div>
            </div>
            {move || state.error.get().map(|error| view! { <div class="flex items-start gap-2 border-b border-red-400/40 bg-red-400/10 px-3 py-2 text-sm text-red-300" role="alert"><span aria-hidden="true">"⚠"</span><span>{error}</span></div> })}
            <ToolSplit initial_ratio=45>
                <ToolPanel side=ToolPanelSide::First>
                    <div class="flex h-full flex-col overflow-hidden">
                        <div class="flex items-center justify-between border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"⌨"</span>"Input"</span><span class="text-xs text-[var(--text-secondary)]">{move || format!("{} lines", line_count(&state.source.get()))}</span></div>
                        <div class="flex flex-1 overflow-hidden">
                            <div class="flex flex-col items-end pe-2 text-xs text-[var(--text-secondary)]" node_ref=line_numbers_ref aria-hidden="true">{move || { let count = line_count(&state.source.get()).max(1); (1..=count).map(|number| view! { <span class="line-number">{number}</span> }).collect_view() }}</div>
                            <textarea id="base64-input" class="editor-textarea min-w-0 flex-1 resize-none border-0 bg-transparent p-3 font-mono text-sm text-[var(--text-primary)] outline-none" placeholder="Enter text or Base64..." spellcheck="false" aria-label="Base64 input" prop:value=move || state.source.get() on:input=move |ev| state.set_content(event_target_value(&ev)) on:scroll=on_input_scroll node_ref=input_ref></textarea>
                        </div>
                    </div>
                </ToolPanel>
                <ToolDivider />
                <ToolPanel side=ToolPanelSide::Second>
                    <div class="flex h-full flex-col overflow-hidden">
                        <div class="flex items-center border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"{}"</span>"Output"</span><button type="button" class=format!("{} ml-auto", button) title="Copy output" aria-label="Copy output" disabled=move || state.output.get().is_empty() on:click=copy_output><span aria-hidden="true">"⧉"</span></button></div>
                        <div class="flex-1 overflow-auto p-3"><pre class="m-0 whitespace-pre-wrap"><code class="font-mono text-sm text-[var(--text-primary)]">{move || state.output.get()}</code></pre></div>
                    </div>
                </ToolPanel>
            </ToolSplit>
        </main>
    }
}

fn line_count(content: &str) -> usize {
    if content.is_empty() { 0 } else { content.lines().count() }
}
