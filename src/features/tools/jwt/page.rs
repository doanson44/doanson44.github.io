use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::jwt::state::JwtState;
use crate::infrastructure::browser::copy_to_clipboard;

/// JWT decoder page.
#[component]
pub fn JwtPage() -> impl IntoView {
    let state = JwtState::new();
    let button = "inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";
    view! {
        <main class="flex flex-1 flex-col overflow-hidden">
            <div class="flex flex-nowrap items-center gap-1 border-b border-[var(--border-color)] p-2" id="jwt-toolbar"><div class="ml-auto flex flex-nowrap gap-1">
                <button type="button" class=button title="Decode JWT" on:click=move |_| state.decode()><span aria-hidden="true">"🔓"</span><span class="ml-1 hidden lg:inline">"Decode"</span></button>
                <button type="button" class=button title="Reset to sample JWT" on:click=move |_| state.reset()><span aria-hidden="true">"↶"</span><span class="ml-1 hidden lg:inline">"Reset"</span></button>
                <button type="button" class=button title="Clear JWT" on:click=move |_| state.clear()><span aria-hidden="true">"×"</span><span class="ml-1 hidden lg:inline">"Clear"</span></button>
            </div></div>
            <div class="flex items-start gap-2 border-b border-amber-400/40 bg-amber-400/10 px-3 py-2 text-sm text-amber-200" role="note"><span aria-hidden="true">"⚠"</span><span>"Decoded JWT data is not cryptographically verified. No token is sent to a server."</span></div>
            {move || state.error.get().map(|error| view! { <div class="flex items-start gap-2 border-b border-red-400/40 bg-red-400/10 px-3 py-2 text-sm text-red-300" role="alert"><span aria-hidden="true">"⚠"</span><span>{error}</span></div> })}
            <ToolSplit initial_ratio=40>
                <ToolPanel side=ToolPanelSide::First>
                    <crate::components::editor::Editor source=state.source title="Encoded JWT" placeholder="Paste a JWT here..." aria_label="Encoded JWT input" textarea_id="jwt-input" on_change=Callback::new(move |s| state.set_content(s)) />
                </ToolPanel><ToolDivider /><ToolPanel side=ToolPanelSide::Second>
                    <div class="flex h-full flex-col overflow-hidden"><div class="border-b border-[var(--border-color)] px-3 py-2 font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"{}"</span>"Decoded JWT"</div><div class="flex flex-1 flex-col gap-3 overflow-auto p-3"><JwtJsonPanel title="Header" value=state.header /><JwtJsonPanel title="Payload" value=state.payload /><div class="overflow-hidden rounded-lg border border-[var(--border-color)]"><div class="flex items-center border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"◈"</span>"Signature"</span><button type="button" class=format!("{} ml-auto", button) title="Copy signature" aria-label="Copy signature" disabled=move || state.signature.get().is_none() on:click=move |_| { if let Some(value) = state.signature.get_untracked() { wasm_bindgen_futures::spawn_local(async move { let _ = copy_to_clipboard(&value).await; }); } }><span aria-hidden="true">"⧉"</span></button></div><pre class="m-0 overflow-auto p-3"><code class="font-mono text-sm text-[var(--text-primary)]">{move || state.signature.get().unwrap_or_else(|| "No decoded signature".into())}</code></pre></div></div></div>
                </ToolPanel>
            </ToolSplit>
        </main>
    }
}

#[component]
fn JwtJsonPanel(title: &'static str, value: RwSignal<Option<serde_json::Value>>) -> impl IntoView {
    let button = "inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";
    view! { <div class="overflow-hidden rounded-lg border border-[var(--border-color)]"><div class="flex items-center border-b border-[var(--border-color)] px-3 py-2"><span class="font-medium text-[var(--text-primary)]"><span class="mr-2" aria-hidden="true">"{}"</span>{title}</span><button type="button" class=format!("{} ml-auto", button) title=format!("Copy {title}") aria-label=format!("Copy {title}") disabled=move || value.get().is_none() on:click=move |_| { if let Some(json) = value.get_untracked() { if let Ok(text) = serde_json::to_string_pretty(&json) { wasm_bindgen_futures::spawn_local(async move { let _ = copy_to_clipboard(&text).await; }); } } }><span aria-hidden="true">"⧉"</span></button></div><pre class="m-0 overflow-auto p-3"><code class="font-mono text-sm text-[var(--text-primary)]">{move || value.get().and_then(|json| serde_json::to_string_pretty(&json).ok()).unwrap_or_else(|| "No decoded data".into())}</code></pre></div> }
}
