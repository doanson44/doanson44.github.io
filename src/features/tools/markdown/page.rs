use leptos::prelude::*;

use crate::components::editor::Editor;
use crate::components::preview::Preview;
use crate::components::tool_layout::{ToolDivider, ToolPanel, ToolPanelSide, ToolSplit};
use crate::components::toolbar::Toolbar;
use crate::features::tools::markdown::state::MarkdownState;

/// Markdown Studio page — composed of Toolbar, Editor, and Preview.
///
/// Supports two view modes:
/// - **Split** (default): Editor | Preview with a responsive draggable divider
/// - **Preview-only**: Full-width preview, editor hidden
#[component]
pub fn MarkdownPage() -> impl IntoView {
    let state = MarkdownState::new();

    Effect::new(move |_| {
        let content = state.source.get();
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("ms-content", &content);
        }
    });

    view! {
        <main class="flex flex-1 flex-col overflow-hidden markdown-tool-page">
            <Toolbar source=state.source toggle_preview=Callback::new(move |_| state.toggle_preview_only()) />
            {move || {
                if state.preview_only.get() {
                    view! {
                        <div class="tool-workspace tool-preview-only flex flex-1 overflow-hidden">
                            <ToolPanel side=ToolPanelSide::Second><Preview rendered=state.rendered /></ToolPanel>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <ToolSplit initial_ratio=50>
                            <ToolPanel side=ToolPanelSide::First><Editor source=state.source /></ToolPanel>
                            <ToolDivider />
                            <ToolPanel side=ToolPanelSide::Second><Preview rendered=state.rendered /></ToolPanel>
                        </ToolSplit>
                    }.into_any()
                }
            }}
        </main>
    }
}
