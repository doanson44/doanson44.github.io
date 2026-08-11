pub mod json;

use crate::domain::markdown::{render_markdown, RenderedMarkdown};

/// Application service coordinating Markdown use cases.
///
/// This service sits between the presentation layer and domain logic,
/// providing a clean API for UI components. It can be extended with
/// repository injection for persistence use cases.
pub struct MarkdownService;

impl MarkdownService {
    /// Render Markdown source into structured output.
    ///
    /// Delegates to the domain layer's `render_markdown` function.
    /// This indirection exists so that future use cases (e.g., render
    /// with server-side extensions, cache results) can be added here
    /// without changing UI components.
    pub fn render(content: &str) -> RenderedMarkdown {
        render_markdown(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::markdown::RenderSegment;

    #[test]
    fn test_service_renders_markdown() {
        let result = MarkdownService::render("# Hello");
        assert!(!result.segments.is_empty());
        match &result.segments[0] {
            RenderSegment::Html(h) => assert!(h.contains("<h1>")),
            _ => panic!("Expected HTML segment"),
        }
    }
}
