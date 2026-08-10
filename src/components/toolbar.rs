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
        icon: "bi-type-bold",
        title: "Bold",
        prefix: "**",
        suffix: "**",
        default_text: "bold text",
        block: false,
    },
    ToolbarButton {
        icon: "bi-type-italic",
        title: "Italic",
        prefix: "*",
        suffix: "*",
        default_text: "italic text",
        block: false,
    },
    ToolbarButton {
        icon: "bi-type-strikethrough",
        title: "Strikethrough",
        prefix: "~~",
        suffix: "~~",
        default_text: "strikethrough",
        block: false,
    },
    ToolbarButton {
        icon: "bi-type-h1",
        title: "Heading 1",
        prefix: "# ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "bi-type-h2",
        title: "Heading 2",
        prefix: "## ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "bi-type-h3",
        title: "Heading 3",
        prefix: "### ",
        suffix: "",
        default_text: "Heading",
        block: true,
    },
    ToolbarButton {
        icon: "bi-link-45deg",
        title: "Link",
        prefix: "[",
        suffix: "](url)",
        default_text: "link text",
        block: false,
    },
    ToolbarButton {
        icon: "bi-image",
        title: "Image",
        prefix: "![",
        suffix: "](url)",
        default_text: "alt text",
        block: false,
    },
    ToolbarButton {
        icon: "bi-code-slash",
        title: "Inline Code",
        prefix: "`",
        suffix: "`",
        default_text: "code",
        block: false,
    },
    ToolbarButton {
        icon: "bi-code-square",
        title: "Code Block",
        prefix: "```\n",
        suffix: "\n```",
        default_text: "code here",
        block: true,
    },
    ToolbarButton {
        icon: "bi-quote",
        title: "Blockquote",
        prefix: "> ",
        suffix: "",
        default_text: "quote",
        block: true,
    },
    ToolbarButton {
        icon: "bi-list-ul",
        title: "Unordered List",
        prefix: "- ",
        suffix: "",
        default_text: "list item",
        block: true,
    },
    ToolbarButton {
        icon: "bi-list-ol",
        title: "Ordered List",
        prefix: "1. ",
        suffix: "",
        default_text: "list item",
        block: true,
    },
    ToolbarButton {
        icon: "bi-table",
        title: "Table",
        prefix: "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |",
        suffix: "",
        default_text: "",
        block: true,
    },
    ToolbarButton {
        icon: "bi-hr",
        title: "Horizontal Rule",
        prefix: "\n---\n",
        suffix: "",
        default_text: "",
        block: true,
    },
    ToolbarButton {
        icon: "bi-diagram-3",
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

    // Restore cursor position
    let _ = textarea.set_selection_start(Some(new_cursor as u32));
    let _ = textarea.set_selection_end(Some(new_cursor as u32));
    let _ = textarea.focus();
}

/// Markdown editing toolbar component.
///
/// Provides quick-access buttons for common Markdown formatting operations.
/// Uses Bootstrap Icons and button groups for a clean UI.
///
/// # Props
/// * `source` - The reactive Markdown source signal to modify
/// * `textarea_id` - The DOM ID of the textarea element
/// * `toggle_preview` - Callback to toggle preview-only mode
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
            let icon_class = format!("bi {}", btn.icon);
            let title = btn.title;

            // Add separator after certain groups
            let needs_separator = idx == 2 || idx == 5 || idx == 8 || idx == 10 || idx == 12;

            let button_view = view! {
                <button
                    type="button"
                    class="btn btn-outline-secondary btn-sm toolbar-btn"
                    title=title
                    on:click=move |_| {
                        insert_markdown(textarea_id, source, prefix, suffix, default_text, block);
                    }
                >
                    <i class=icon_class.clone()></i>
                </button>
            };

            if needs_separator {
                view! {
                    <>
                        {button_view}
                        <div class="vr mx-1 toolbar-separator"></div>
                    </>
                }
                .into_any()
            } else {
                button_view.into_any()
            }
        })
        .collect();

    view! {
        <div class="toolbar d-flex flex-wrap align-items-center gap-1 p-2 border-bottom border-secondary" id="editor-toolbar">
            {buttons}
            <div class="ms-auto d-flex gap-1">
                // Preview-only toggle
                {if let Some(toggle) = toggle_preview {
                    view! {
                        <button
                            type="button"
                            class="btn btn-outline-success btn-sm"
                            title="Preview only (hide editor)"
                            on:click=move |_| toggle.run(())
                        >
                            <i class="bi bi-eye-fill"></i>
                            <span class="d-none d-lg-inline ms-1">"Preview"</span>
                        </button>
                    }.into_any()
                } else {
                    ().into_any()
                }}
                <button
                    type="button"
                    class="btn btn-outline-danger btn-sm"
                    title="Clear editor"
                    on:click=move |_| {
                        source.set(String::new());
                        if let Some(textarea) = crate::infrastructure::browser::get_element_by_id::<HtmlTextAreaElement>(textarea_id) {
                            textarea.set_value("");
                            let _ = textarea.focus();
                        }
                    }
                >
                    <i class="bi bi-trash3"></i>
                    <span class="d-none d-lg-inline ms-1">"Clear"</span>
                </button>
                <button
                    type="button"
                    class="btn btn-outline-primary btn-sm"
                    title="Reset to sample"
                    on:click=move |_| {
                        let doc = crate::domain::document::MarkdownDocument::sample();
                        source.set(doc.content.clone());
                        if let Some(textarea) = crate::infrastructure::browser::get_element_by_id::<HtmlTextAreaElement>(textarea_id) {
                            textarea.set_value(&doc.content);
                        }
                    }
                >
                    <i class="bi bi-arrow-counterclockwise"></i>
                    <span class="d-none d-lg-inline ms-1">"Reset"</span>
                </button>
            </div>
        </div>
    }
}
