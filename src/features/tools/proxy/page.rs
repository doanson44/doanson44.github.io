use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::proxy::state::ProxyState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn ProxyPage() -> impl IntoView {
    let state = ProxyState::new();

    let run = move |_| state.run();
    let reset = move |_| state.reset();

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
            <header class="flex shrink-0 flex-col gap-1 border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-2">
                <h1 class="m-0 text-base font-semibold text-[var(--text-primary)]">
                    <span class="mr-2 text-[var(--accent)]" aria-hidden="true">"↗"</span>
                    "HTTP Proxy Playground"
                </h1>
                <p class="m-0 text-sm text-[var(--text-secondary)]">
                    "Call a JSON API through the proxy, then reduce the response for a compact model-friendly view."
                </p>
            </header>

            {move || state.error.get().map(|error| view! {
                <div class="flex shrink-0 items-start gap-2 border-b border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-sm text-[var(--danger)]" role="alert">
                    <span aria-hidden="true">"!"</span>
                    <span>{error}</span>
                </div>
            })}

            <ToolSplit initial_ratio=40>
                <ToolPanel side=ToolPanelSide::First>
                    <div class="flex h-full flex-col overflow-auto">
                        <div class="border-b border-[var(--border-color)] px-3 py-2 font-medium text-[var(--text-primary)]">
                            "Request"
                        </div>
                        <div class="flex flex-col gap-3 p-3">
                            <div>
                                <label class="mb-1 block text-sm font-medium text-[var(--text-primary)]" for="proxy-target-url">
                                    "Target URL"
                                </label>
                                <input
                                    id="proxy-target-url"
                                    type="url"
                                    class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/25"
                                    aria-describedby="proxy-target-help"
                                    prop:value=move || state.target_url.get()
                                    on:input=move |ev| state.target_url.set(event_target_value(&ev))
                                />
                                <div id="proxy-target-help" class="mt-1 text-xs text-[var(--text-secondary)]">
                                    "GET only in this first version. Example: JSONPlaceholder /posts returns 100 items."
                                </div>
                            </div>

                            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                                <div>
                                    <label class="mb-1 block text-sm font-medium text-[var(--text-primary)]" for="proxy-array-limit">
                                        "Array items"
                                    </label>
                                    <select
                                        id="proxy-array-limit"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/25"
                                        prop:value=move || state.max_array_items.get()
                                        on:change=move |ev| state.max_array_items.set(event_target_value(&ev))
                                    >
                                        <option value="5">"5"</option>
                                        <option value="10">"10"</option>
                                        <option value="25">"25"</option>
                                        <option value="50">"50"</option>
                                        <option value="all">"All"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="mb-1 block text-sm font-medium text-[var(--text-primary)]" for="proxy-string-limit">
                                        "String chars"
                                    </label>
                                    <select
                                        id="proxy-string-limit"
                                        class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/25"
                                        prop:value=move || state.max_string_chars.get()
                                        on:change=move |ev| state.max_string_chars.set(event_target_value(&ev))
                                    >
                                        <option value="200">"200"</option>
                                        <option value="500">"500"</option>
                                        <option value="1000">"1000"</option>
                                        <option value="all">"All"</option>
                                    </select>
                                </div>
                            </div>

                            <label class="flex min-h-11 items-center gap-2 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-primary)]">
                                <input
                                    type="checkbox"
                                    prop:checked=move || state.compact.get()
                                    on:change=move |ev| state.compact.set(event_target_checked(&ev))
                                />
                                <span>"Compact JSON output"</span>
                            </label>

                            <div class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] p-3 text-xs leading-5 text-[var(--text-secondary)]">
                                <div class="mb-1 font-medium text-[var(--text-primary)]">"How minimization works"</div>
                                <ul class="m-0 pl-4">
                                    <li>"Array limits apply recursively, including nested arrays."</li>
                                    <li>"String limits reduce long text values without changing their JSON type."</li>
                                    <li>"The original response is not modified on the proxy; reduction happens in WASM after the response is received."</li>
                                </ul>
                            </div>
                        </div>

                        <div class="mt-auto flex gap-2 border-t border-[var(--border-color)] p-3">
                            <button
                                type="button"
                                class="min-h-11 rounded-md border border-[var(--accent)] px-3 py-2 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/10 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                                title="Call target through proxy"
                                disabled=move || state.loading.get()
                                on:click=run
                            >
                                {move || if state.loading.get() { "Loading..." } else { "Run through proxy" }}
                            </button>
                            <button
                                type="button"
                                class="min-h-11 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40"
                                title="Reset proxy request"
                                on:click=reset
                            >
                                "Reset"
                            </button>
                        </div>
                    </div>
                </ToolPanel>

                <ToolDivider />

                <ToolPanel side=ToolPanelSide::Second>
                    <div class="flex h-full min-h-0 flex-col overflow-hidden">
                        <div class="flex shrink-0 items-center border-b border-[var(--border-color)] px-3 py-2">
                            <span class="font-medium text-[var(--text-primary)]">"Minimized response"</span>
                            <span class="ml-2 text-xs text-[var(--text-secondary)]" aria-live="polite">
                                {move || if state.loading.get() { "Fetching..." } else if state.output.get().is_empty() { "No response" } else { "Ready" }}
                            </span>
                            <button
                                type="button"
                                class="ml-auto min-h-11 rounded-md border border-[var(--accent)] px-3 py-2 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/10 focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/40 disabled:cursor-not-allowed disabled:opacity-50"
                                disabled=move || state.output.get().is_empty()
                                title="Copy minimized response"
                                aria-label="Copy minimized response"
                                on:click=on_copy
                            >
                                {move || if state.copied.get() { "Copied" } else { "Copy" }}
                            </button>
                        </div>
                        <div class="preview-content min-h-0 flex-grow overflow-auto p-3" aria-live="polite">
                            {move || if state.output.get().is_empty() {
                                view! {
                                    <div class="flex h-full min-h-32 items-center justify-center text-center text-sm text-[var(--text-secondary)]">
                                        "Run the request to see the minimized JSON response."
                                    </div>
                                }.into_any()
                            } else {
                                let output = state.output.get();
                                view! {
                                    <pre class="m-0 max-w-full overflow-auto rounded-md border border-[var(--border-color)] bg-[var(--surface)] p-3 text-sm leading-6"><code class="font-mono text-[var(--text-primary)]">{output}</code></pre>
                                }.into_any()
                            }}
                        </div>
                    </div>
                </ToolPanel>
            </ToolSplit>
        </div>
    }
}
