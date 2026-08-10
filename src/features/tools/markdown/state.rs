use leptos::prelude::*;

use crate::application::services::MarkdownService;
use crate::domain::document::MarkdownDocument;
use crate::domain::markdown::RenderedMarkdown;

/// Reactive state for the Markdown editor feature.
///
/// Uses Leptos signals for reactive state management.
/// No global state framework — state is scoped to the component tree.
#[derive(Clone, Copy)]
pub struct MarkdownState {
    /// The raw Markdown source text.
    pub source: RwSignal<String>,

    /// The rendered Markdown output (derived signal).
    pub rendered: Memo<RenderedMarkdown>,

    /// The current document metadata.
    pub document: RwSignal<MarkdownDocument>,

    /// Whether the editor is hidden (preview-only mode).
    pub preview_only: RwSignal<bool>,
}

impl Default for MarkdownState {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownState {
    /// Create a new `MarkdownState`. Loads saved content from localStorage,
    /// falling back to the sample document if nothing is saved.
    pub fn new() -> Self {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());

        // Load saved content or use sample
        let initial_content = storage
            .as_ref()
            .and_then(|s| s.get_item("ms-content").ok().flatten())
            .unwrap_or_else(|| MarkdownDocument::sample().content);

        let doc = MarkdownDocument::sample();
        let source = RwSignal::new(initial_content);
        let document = RwSignal::new(doc);

        let rendered = Memo::new(move |_| {
            let content = source.get();
            MarkdownService::render(&content)
        });

        Self {
            source,
            rendered,
            document,
            preview_only: RwSignal::new(false),
        }
    }

    /// Toggle between split-pane and preview-only mode.
    pub fn toggle_preview_only(&self) {
        self.preview_only.update(|v| *v = !*v);
    }

    /// Save the current content to localStorage.
    fn save_content(content: &str) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("ms-content", content);
        }
    }

    /// Update the Markdown source content.
    pub fn set_content(&self, content: String) {
        Self::save_content(&content);
        self.source.set(content);
    }

    /// Get the current source content.
    pub fn get_content(&self) -> String {
        self.source.get()
    }

    /// Clear the editor content.
    pub fn clear(&self) {
        Self::save_content("");
        self.source.set(String::new());
    }

    /// Reset to the sample document.
    pub fn reset_to_sample(&self) {
        let doc = MarkdownDocument::sample();
        Self::save_content(&doc.content);
        self.source.set(doc.content.clone());
        self.document.set(doc);
    }
}
