use leptos::prelude::*;

use crate::application::services::MarkdownService;
use crate::domain::document::MarkdownDocument;
use crate::domain::markdown::RenderedMarkdown;

/// Reactive state for the Markdown editor feature.
///
/// Uses Leptos signals for reactive state management.
/// No global state framework — state is scoped to the component tree.
#[derive(Clone)]
pub struct MarkdownState {
    /// The raw Markdown source text.
    pub source: RwSignal<String>,

    /// The rendered Markdown output (derived signal).
    pub rendered: Memo<RenderedMarkdown>,

    /// The current document metadata.
    pub document: RwSignal<MarkdownDocument>,
}

impl Default for MarkdownState {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownState {
    /// Create a new `MarkdownState` with a sample document.
    pub fn new() -> Self {
        let doc = MarkdownDocument::sample();
        let initial_content = doc.content.clone();

        let source = RwSignal::new(initial_content);
        let document = RwSignal::new(doc);

        // Derived signal that re-renders whenever source changes
        let rendered = Memo::new(move |_| {
            let content = source.get();
            MarkdownService::render(&content)
        });

        Self {
            source,
            rendered,
            document,
        }
    }

    /// Update the Markdown source content.
    pub fn set_content(&self, content: String) {
        self.source.set(content);
    }

    /// Get the current source content.
    pub fn get_content(&self) -> String {
        self.source.get()
    }

    /// Clear the editor content.
    pub fn clear(&self) {
        self.source.set(String::new());
    }

    /// Reset to the sample document.
    pub fn reset_to_sample(&self) {
        let doc = MarkdownDocument::sample();
        self.source.set(doc.content.clone());
        self.document.set(doc);
    }
}
