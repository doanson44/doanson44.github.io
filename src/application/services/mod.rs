pub mod base64;
pub mod developer_tools;
pub mod json;
pub mod jwt;

use crate::domain::markdown::{render_markdown, RenderedMarkdown};

pub struct MarkdownService;

impl MarkdownService {
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
