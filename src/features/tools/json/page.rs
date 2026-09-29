use leptos::prelude::*;

use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::features::tools::json::state::JsonState;
use crate::i18n::*;
use crate::infrastructure::browser::copy_to_clipboard;

/// JSON Formatter page for validating, formatting, minifying, and generating dummy JSON.
#[component]
pub fn JsonPage() -> impl IntoView {
    let state = JsonState::new();
    let i18n = use_i18n();
    let button = "inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]";
    let input = "min-h-8 rounded border border-[var(--border-color)] bg-[var(--surface)] px-2 text-xs text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)]";

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
                <div class="ml-auto flex flex-wrap items-center justify-end gap-1">
                    <div class="flex items-center gap-1 rounded border border-[var(--border-color)] px-1" role="group" aria-label="JSON sample options">
                        <label for="json-sample-limit" class="px-1 text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, json_sample)}
                        </label>
                        <input
                            id="json-sample-limit"
                            type="number"
                            min="1"
                            max="100"
                            step="1"
                            class="w-12 bg-transparent px-1 py-1 text-center text-xs text-[var(--text-primary)] outline-none"
                            prop:value=move || state.sample_limit.get().to_string()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                    state.set_sample_limit(value);
                                }
                            }
                            aria-label=move || t_string!(i18n, json_sample_aria)
                        />
                        <span class="pr-1 text-xs text-[var(--text-secondary)]">
                            {move || t_string!(i18n, json_sample_items)}
                        </span>
                    </div>
                    <button type="button" class=button title=move || t_string!(i18n, json_sample_title) on:click=move |_| state.sample()>
                        <span aria-hidden="true">"▾"</span>
                        <span class="ml-1 hidden lg:inline">{move || t_string!(i18n, json_sample)}</span>
                    </button>
                    <button type="button" class=button title="Format JSON" on:click=move |_| state.format()>
                        <span aria-hidden="true">"≡"</span>
                        <span class="ml-1 hidden lg:inline">{move || if i18n.get_locale() == Locale::vi { "Định dạng" } else { "Format" }}</span>
                    </button>
                    <button type="button" class=button title="Minify JSON" on:click=move |_| state.minify()>
                        <span aria-hidden="true">"↕"</span>
                        <span class="ml-1 hidden lg:inline">{move || if i18n.get_locale() == Locale::vi { "Thu gọn" } else { "Minify" }}</span>
                    </button>
                    <button
                        type="button"
                        class=format!("{} border-[var(--accent)] text-[var(--accent)]", button)
                        title=move || if i18n.get_locale() == Locale::vi { "Mở trình tạo dữ liệu giả" } else { "Open dummy data generator" }
                        aria-expanded=move || state.generator_open.get().to_string()
                        on:click=move |_| state.generator_open.update(|open| *open = !*open)
                    >
                        <span aria-hidden="true">"✦"</span>
                        <span class="ml-1 hidden md:inline">{move || if i18n.get_locale() == Locale::vi { "Tạo dữ liệu" } else { "Generate" }}</span>
                    </button>
                    <button type="button" class=button title="Reset to sample JSON" on:click=move |_| state.reset()>
                        <span aria-hidden="true">"↶"</span>
                        <span class="ml-1 hidden lg:inline">{move || t_string!(i18n, common_reset)}</span>
                    </button>
                    <button type="button" class=button title="Clear JSON" on:click=move |_| state.clear()>
                        <span aria-hidden="true">"×"</span>
                        <span class="ml-1 hidden lg:inline">{move || t_string!(i18n, common_clear)}</span>
                    </button>
                </div>
            </div>

            {move || state.generator_open.get().then(|| view! {
                <section class="border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-3" aria-label=move || if i18n.get_locale() == Locale::vi { "Trình tạo dữ liệu giả" } else { "Dummy data generator" }>
                    <div class="mx-auto max-w-6xl">
                        <div class="flex flex-wrap items-start justify-between gap-3">
                            <div>
                                <h2 class="text-sm font-semibold text-[var(--text-primary)]">
                                    {move || if i18n.get_locale() == Locale::vi { "Tạo Dummy JSON" } else { "Generate Dummy JSON" }}
                                </h2>
                                <p class="mt-1 max-w-3xl text-xs text-[var(--text-secondary)]">
                                    {move || if i18n.get_locale() == Locale::vi {
                                        r#"Dùng JSON hiện tại làm template. "string", 0 và boolean là type hint; giá trị cố định sẽ được áp dụng sau khi generate."#
                                    } else {
                                        r#"Use the current JSON as a template. "string", 0, and booleans act as type hints; fixed values are applied after generation."#
                                    }}
                                </p>
                            </div>
                            <div class="flex gap-1">
                                <button type="button" class=button on:click=move |_| state.generate()>
                                    <span aria-hidden="true">"▶"</span>
                                    <span class="ml-1">{move || if i18n.get_locale() == Locale::vi { "Generate JSON" } else { "Generate JSON" }}</span>
                                </button>
                                <button type="button" class=button on:click=move |_| state.generator_open.set(false)>
                                    {move || if i18n.get_locale() == Locale::vi { "Đóng" } else { "Close" }}
                                </button>
                            </div>
                        </div>

                        <div class="mt-3 grid gap-3 md:grid-cols-2">
                            <div class="rounded border border-[var(--border-color)] p-3">
                                <div class="grid gap-2 sm:grid-cols-2">
                                    <label class="grid gap-1 text-xs text-[var(--text-secondary)]">
                                        <span>{move || if i18n.get_locale() == Locale::vi { "Số lượng record" } else { "Records" }}</span>
                                        <input
                                            type="number"
                                            min="1"
                                            max="1000"
                                            step="1"
                                            class=input
                                            prop:value=move || state.generate_count.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                                    state.set_generate_count(value);
                                                }
                                            }
                                        />
                                    </label>
                                    <label class="grid gap-1 text-xs text-[var(--text-secondary)]">
                                        <span>{move || if i18n.get_locale() == Locale::vi { "Seed" } else { "Seed" }}</span>
                                        <input
                                            type="text"
                                            class=input
                                            prop:value=move || state.generate_seed.get()
                                            on:input=move |ev| state.set_generate_seed(event_target_value(&ev))
                                            placeholder="12345"
                                        />
                                    </label>
                                </div>
                                <p class="mt-2 text-[11px] text-[var(--text-secondary)]">
                                    {move || if i18n.get_locale() == Locale::vi {
                                        "Cùng template + seed sẽ tạo cùng dataset, phù hợp cho test/reproduce."
                                    } else {
                                        "The same template and seed produce the same dataset for reproducible tests."
                                    }}
                                </p>
                            </div>

                            <div class="rounded border border-[var(--border-color)] p-3">
                                <div class="flex items-center justify-between gap-2">
                                    <div>
                                        <h3 class="text-xs font-semibold text-[var(--text-primary)]">
                                            {move || if i18n.get_locale() == Locale::vi { "Fixed values" } else { "Fixed values" }}
                                        </h3>
                                        <p class="mt-1 text-[11px] text-[var(--text-secondary)]">
                                            {move || if i18n.get_locale() == Locale::vi { r#"Ví dụ: $.extId = "123""# } else { r#"Example: $.extId = "123""# }}
                                        </p>
                                    </div>
                                    <div class="flex gap-1">
                                        <button type="button" class=button on:click=move |_| state.add_override()>
                                            {move || if i18n.get_locale() == Locale::vi { "+ Thêm" } else { "+ Add" }}
                                        </button>
                                        <button type="button" class=button on:click=move |_| state.clear_overrides()>
                                            {move || if i18n.get_locale() == Locale::vi { "Xóa hết" } else { "Clear" }}
                                        </button>
                                    </div>
                                </div>

                                <div class="mt-2 grid gap-2">
                                    {move || state.overrides.get().into_iter().enumerate().map(|(index, item)| {
                                        let path = item.path;
                                        let value = item.value;
                                        let value_type = item.value_type;
                                        view! {
                                            <div class="grid gap-1 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto_auto]">
                                                <input
                                                    type="text"
                                                    class=input
                                                    prop:value=path
                                                    placeholder="$.extId"
                                                    aria-label="Fixed value path"
                                                    on:input=move |ev| state.update_override(index, Some(event_target_value(&ev)), None, None)
                                                />
                                                <input
                                                    type="text"
                                                    class=input
                                                    prop:value=value
                                                    placeholder="123"
                                                    aria-label="Fixed value"
                                                    on:input=move |ev| state.update_override(index, None, Some(event_target_value(&ev)), None)
                                                />
                                                <select
                                                    class=input
                                                    prop:value=value_type
                                                    aria-label="Fixed value type"
                                                    on:change=move |ev| state.update_override(index, None, None, Some(event_target_value(&ev)))
                                                >
                                                    <option value="string">"String"</option>
                                                    <option value="number">"Number"</option>
                                                    <option value="boolean">"Boolean"</option>
                                                    <option value="null">"Null"</option>
                                                    <option value="json">"JSON"</option>
                                                </select>
                                                <button
                                                    type="button"
                                                    class=button
                                                    aria-label="Remove fixed value"
                                                    title=move || if i18n.get_locale() == Locale::vi { "Xóa fixed value" } else { "Remove fixed value" }
                                                    on:click=move |_| state.remove_override(index)
                                                >
                                                    "×"
                                                </button>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            })}

            {move || state.error.get().map(|error| view! {
                <div class="flex items-start gap-2 border-b border-red-400/40 bg-red-400/10 px-3 py-2 text-sm text-red-300" role="alert">
                    <span aria-hidden="true">"⚠"</span>
                    <span>{error}</span>
                </div>
            })}

            <ToolSplit initial_ratio=50>
                <ToolPanel side=ToolPanelSide::First>
                    <crate::components::editor::Editor
                        source=state.source
                        title="Input"
                        placeholder="Paste JSON template here..."
                        aria_label="JSON input"
                        textarea_id="json-input"
                        on_change=Callback::new(move |s| state.set_content(s))
                    />
                </ToolPanel>
                <ToolDivider />
                <ToolPanel side=ToolPanelSide::Second>
                    <div class="flex h-full flex-col overflow-hidden">
                        <div class="flex items-center border-b border-[var(--border-color)] px-3 py-2">
                            <span class="font-medium text-[var(--text-primary)]">
                                <span class="mr-2" aria-hidden="true">"◉"</span>
                                {move || t_string!(i18n, common_preview)}
                            </span>
                            <button
                                type="button"
                                class=format!("{} ml-auto", button)
                                disabled=move || state.output.get().is_empty()
                                on:click=on_copy
                                title="Copy generated JSON"
                                aria-label="Copy generated JSON"
                            >
                                <span aria-hidden="true">"⧉"</span>
                                <span class="ml-1 hidden md:inline" aria-live="polite">
                                    {move || if state.copied.get() { t_string!(i18n, common_copied) } else { t_string!(i18n, common_copy) }}
                                </span>
                            </button>
                        </div>
                        <div class="flex-1 overflow-auto p-3">
                            {move || {
                                let output = state.output.get();
                                if output.is_empty() {
                                    view! {
                                        <div class="flex h-full items-center justify-center text-sm text-[var(--text-secondary)]">
                                            <div class="text-center">
                                                <div class="mb-2 text-3xl" aria-hidden="true">"{}"</div>
                                                <span>
                                                    {move || if i18n.get_locale() == Locale::vi {
                                                        "Định dạng, thu gọn, tạo sample hoặc generate JSON để xem kết quả."
                                                    } else {
                                                        "Format, minify, sample, or generate JSON to see the result."
                                                    }}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <pre class="m-0 whitespace-pre-wrap">
                                            <code class="font-mono text-sm text-[var(--text-primary)]">{output}</code>
                                        </pre>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </ToolPanel>
            </ToolSplit>
        </main>
    }
}
