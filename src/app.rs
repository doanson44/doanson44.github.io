use leptos::prelude::*;

use crate::components::editor::Editor;
use crate::components::navbar::Navbar;
use crate::components::preview::Preview;
use crate::components::toolbar::Toolbar;
use crate::features::markdown::state::MarkdownState;

/// Root application component.
///
/// Creates the Markdown state and composes the UI from Navbar,
/// Toolbar, Editor, and Preview components.
///
/// Layout uses Bootstrap's grid system for a responsive split-pane design.
#[component]
pub fn App() -> impl IntoView {
    let state = MarkdownState::new();

    view! {
        <div class="app-container d-flex flex-column vh-100" id="app">
            <Navbar />
            <Toolbar source=state.source />
            <div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
                <div class="editor-pane">
                    <Editor source=state.source />
                </div>
                <div class="divider"></div>
                <div class="preview-pane">
                    <Preview rendered=state.rendered />
                </div>
            </div>
            <footer class="app-footer d-flex align-items-center justify-content-between px-3 py-1 border-top border-secondary">
                <span class="text-body-secondary small">
                    "Markdown Studio · Powered by "
                    <a href="https://www.rust-lang.org/" target="_blank" rel="noopener" class="text-decoration-none">"Rust"</a>
                    " + "
                    <a href="https://leptos.dev/" target="_blank" rel="noopener" class="text-decoration-none">"Leptos"</a>
                    " + "
                    <a href="https://webassembly.org/" target="_blank" rel="noopener" class="text-decoration-none">"WebAssembly"</a>
                </span>
                <span class="text-body-secondary small">
                    <i class="bi bi-shield-lock me-1"></i>
                    "100% client-side · No data leaves your browser"
                </span>
            </footer>
        </div>
    }
}
