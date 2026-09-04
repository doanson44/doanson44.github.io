use leptos::prelude::*;
use web_sys::HtmlTextAreaElement;

/// Toolbar button descriptor.
struct ToolbarButton {
    icon: &'static str,
    title: &'static str,
    prefix: &'static str,
    suffix: &'static str,
    default_text: &'static str,
    block: bool,
}

const TOOLBAR_BUTTONS: &[ToolbarButton] = &[
    ToolbarButton {
        icon: "B",
        title: "Bold",
        prefix: "**",
        suffix: "**",
        default_text: "bold text",
        block: false,
    },
    ToolbarButton {
        icon: "I",
        title: "Italic",
        prefix: "*",
        suffix: "*",
        default_text: "italic text",
        block: false,
    },
    ToolbarButton {
        icon: "S",
        title: "Strikethrough",
        prefix: "~~",
        suffix: "~~",
        default_text: "strikethrough",
        block: false,
    },
    ToolbarButton {
        icon: "H1",
        title: "Heading 1",
        prefix: "# ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "H2",
        title: "Heading 2",
        prefix: "## ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "H3",
        title: "Heading 3",
        prefix: "### ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "↗",
        title: "Link",
        prefix: "[",
        suffix: "](url)",
        default_text: "link text",
        block: false,
    },
    ToolbarButton {
        icon: "▧",
        title: "Image",
        prefix: "![",
        suffix: "](url)",
        default_text: "alt text",
        block: false,
    },
    ToolbarButton {
        icon: "<>",
        title: "Inline Code",
        prefix: "`",
        suffix: "`",
        default_text: "code",
        block: false,
    },
    ToolbarButton {
        icon: "{}",
        title: "Code Block",
        prefix: "```\n",
        suffix: "\n```",
        default_text: "code here",
        block: true,
    },
    ToolbarButton {
        icon: "❝",
        title: "Blockquote",
        prefix: "> ",
        suffix: "",
        default_text: "quote",
        block: true,
    },
    ToolbarButton {
        icon: "•",
        title: "Unordered List",
        prefix: "- ",
        suffix: "",
        default_text: "list item",
        block: true,
    },
    ToolbarButton {
        icon: "1.",
        title: "Ordered List",
        prefix: "1. ",
        suffix: "",
        default_text: "list item",
        block: true,
    },
    ToolbarButton {
        icon: "▦",
        title: "Table",
        prefix: "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |",
        suffix: "",
        default_text: "",
        block: true,
    },
    ToolbarButton {
        icon: "—",
        title: "Horizontal Rule",
        prefix: "\n---\n",
        suffix: "",
        default_text: "",
        block: true,
    },
    ToolbarButton {
        icon: "◇",
        title: "Mermaid Diagram",
        prefix: "```mermaid\nflowchart LR\n    A[Start] --> B[End]",
        suffix: "\n```",
        default_text: "",
        block: true,
    },
];

/// Insert Markdown syntax into the textarea at the cursor position.
fn insert_markdown(
    textarea_id: &str,
    source: RwSignal<String>,
    prefix: &str,
    suffix: &str,
    default_text: &str,
    _block: bool,
) {
    let Some(textarea) =
        crate::infrastructure::browser::get_element_by_id::<HtmlTextAreaElement>(textarea_id)
    else {
        return;
    };
    let value = textarea.value();
    let start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
    let end = textarea.selection_end().ok().flatten().unwrap_or(0) as usize;
    let selected = &value[start..end];
    let insert_text = if selected.is_empty() {
        default_text
    } else {
        selected
    };
    let new_text = format!(
        "{}{}{}{}{}",
        &value[..start],
        prefix,
        insert_text,
        suffix,
        &value[end..]
    );
    let new_cursor = start + prefix.len() + insert_text.len();
    source.set(new_text.clone());
    textarea.set_value(&new_text);
    let _ = textarea.set_selection_start(Some(new_cursor as u32));
    let _ = textarea.set_selection_end(Some(new_cursor as u32));
    let _ = textarea.focus();
}

/// Markdown editing toolbar component.
#[component]
pub fn Toolbar(
    source: RwSignal<String>,
    #[prop(default = "markdown-editor")] textarea_id: &'static str,
    #[prop(optional)] toggle_preview: Option<Callback<()>>,
) -> impl IntoView {
    let buttons: Vec<_> = TOOLBAR_BUTTONS
        .iter()
        .enumerate()
        .map(|(idx, btn)| {
            let prefix = btn.prefix;
            let suffix = btn.suffix;
            let default_text = btn.default_text;
            let block = btn.block;
            let icon = btn.icon;
            let title = btn.title;
            let needs_separator = idx == 2 || idx == 5 || idx == 8 || idx == 10 || idx == 12;
            let button_view = view! {
                <button type="button" class="inline-flex min-h-8 min-w-8 items-center justify-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" title=title on:click=move |_| insert_markdown(textarea_id, source, prefix, suffix, default_text, block)>
                    {icon}
                </button>
            };
            if needs_separator {
                view! { <><div>{button_view}</div><div class="mx-1 h-5 w-px bg-[var(--border-color)]" aria-hidden="true"></div></> }.into_any()
            } else {
                button_view.into_any()
            }
        })
        .collect();

    view! {
        <div class="flex flex-wrap items-center gap-1 border-b border-[var(--border-color)] p-2" id="editor-toolbar">
            {buttons}
            <div class="ml-auto flex gap-1">
                {if let Some(toggle) = toggle_preview {
                    view! { <button type="button" class="inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" title="Preview only (hide editor)" on:click=move |_| toggle.run(())><span aria-hidden="true">"◉"</span><span class="ml-1 hidden lg:inline">"Preview"</span></button> }.into_any()
                } else {
                    ().into_any()
                }}
                <button type="button" class="inline-flex min-h-8 items-center rounded border border-red-400/50 px-2 text-xs font-semibold text-red-400 transition hover:bg-red-400/10 focus:outline-none focus:ring-2 focus:ring-red-400" title="Clear editor" on:click=move |_| { source.set(String::new()); if let Some(textarea) = crate::infrastructure::browser::get_element_by_id::<HtmlTextAreaElement>(textarea_id) { textarea.set_value(""); let _ = textarea.focus(); } }><span aria-hidden="true">"×"</span><span class="ml-1 hidden lg:inline">"Clear"</span></button>
                <button type="button" class="inline-flex min-h-8 items-center rounded border border-[var(--border-color)] px-2 text-xs font-semibold text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" title="Reset to sample" on:click=move |_| { let doc = crate::domain::document::MarkdownDocument::sample(); source.set(doc.content.clone()); if let Some(textarea) = crate::infrastructure::browser::get_element_by_id::<HtmlTextAreaElement>(textarea_id) { textarea.set_value(&doc.content); } }><span aria-hidden="true">"↶"</span><span class="ml-1 hidden lg:inline">"Reset"</span></button>
            </div>
        </div>
    }
}
