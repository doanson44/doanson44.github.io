use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::domain::developer::ToolId;
use crate::features::tools::developer::state::DeveloperToolsState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn DeveloperToolPage(tool: ToolId) -> impl IntoView {
    let state = DeveloperToolsState::new(tool);
    let title = tool.title();
    let description = tool.description();
    let secondary_label = tool.secondary_label();
    let secondary_options = tool.secondary_options();
    let output_is_svg = tool.is_svg_output();

    let run = move |_| state.run(tool);
    let on_copy = move |_| {
        let output = state.output.get_untracked();
        if output.is_empty() {
            return;
        }
        let copied = state.copied;
        wasm_bindgen_futures::spawn_local(async move {
            if copy_to_clipboard(&output).await.is_ok() {
                copied.set(true);
            }
        });
    };

    view! {
        <div class="flex flex-grow flex-col overflow-hidden">
            <div class="toolbar flex flex-wrap items-center gap-1 border-b border-[var(--border-color)] p-2">
                <div class="ml-auto flex flex-wrap gap-1">
                    <button type="button" class="rounded-md border border-[var(--accent)] px-3 py-1.5 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title=format!("Run {}", title) on:click=run>"Run"</button>
                    <button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-1.5 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title="Reset sample" on:click=move |_| state.reset(tool)>"Reset"</button>
                    <button type="button" class="rounded-md border border-[var(--danger)] px-3 py-1.5 text-sm font-medium text-[var(--danger)] hover:bg-[var(--danger)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title="Clear input" on:click=move |_| state.clear(tool)>"Clear"</button>
                </div>
            </div>
            <header class="flex shrink-0 flex-col border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-2">
                <div class="font-semibold">{title}</div>
                <div class="text-sm text-[var(--text-secondary)]">{description}</div>
            </header>
            {move || state.error.get().map(|error| view! { <div class="flex items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">{error}</div> })}
            <ToolSplit initial_ratio=50>
                <ToolPanel side=ToolPanelSide::First>
                    {move || view! { <crate::components::editor::Editor source=state.source title={if tool == ToolId::Regex { "Pattern" } else { "Input" }} placeholder="Enter input..." aria_label=title textarea_id="developer-input" on_change=Callback::new(move |s| state.set_source(tool, s)) /> }.into_any()}
                    {secondary_label.map(|label| view! {
                        <div class="shrink-0 border-t border-[var(--border-color)] p-2">
                            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for="developer-secondary-input">{label}</label>
                            {if let Some(options) = secondary_options {
                                view! {
                                    <select id="developer-secondary-input" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || state.secondary.get() on:change=move |ev| state.set_secondary(tool, event_target_value(&ev))>
                                        {options.iter().map(|(value, label)| view! { <option value=*value>{*label}</option> }).collect_view()}
                                    </select>
                                }.into_any()
                            } else {
                                view! {
                                    <textarea id="developer-secondary-input" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 font-mono text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" rows="4" prop:value=move || state.secondary.get() on:input=move |ev| state.set_secondary(tool, event_target_value(&ev))></textarea>
                                }.into_any()
                            }}
                        </div>
                    })}
                </ToolPanel>
                <ToolDivider />
                <ToolPanel side=ToolPanelSide::Second>
                    <div class="panel-header flex items-center border-b border-[var(--border-color)] px-3 py-2"><span class="panel-title" id="developer-result-title">"Result"</span><button type="button" class="ml-auto rounded-md border border-[var(--accent)] px-3 py-1.5 text-sm font-medium text-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50 hover:bg-[var(--accent)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" disabled=move || state.output.get().is_empty() on:click=on_copy title="Copy result" aria-label="Copy result">{move || if state.copied.get() { "Copied" } else { "Copy" }}</button></div>
                    <div class="preview-content flex-grow overflow-auto p-3">{move || { let output=state.output.get(); if output.is_empty() { view! { <div class="flex h-full items-center justify-center text-[var(--text-secondary)]"><div class="text-center">"Run the tool to see the result."</div></div> }.into_any() } else if output_is_svg { view! { <div class="flex h-full items-center justify-center" inner_html=output></div> }.into_any() } else { view! { <pre class="mb-0"><code class="font-mono">{output}</code></pre> }.into_any() } }}</div>
                </ToolPanel>
            </ToolSplit>
        </div>
    }
}
