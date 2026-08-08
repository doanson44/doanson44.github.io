use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// A block of Mermaid diagram code extracted from Markdown.
#[derive(Debug, Clone, PartialEq)]
pub struct MermaidBlock {
    /// Unique identifier for this block (used for rendering).
    pub id: String,
    /// The raw Mermaid diagram definition.
    pub code: String,
}

/// The result of rendering a Markdown document.
///
/// Separates normal HTML output from Mermaid diagram blocks,
/// allowing the presentation layer to render each appropriately.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedMarkdown {
    /// Segments of the rendered output, in order.
    /// HTML segments alternate with Mermaid block placeholders.
    pub segments: Vec<RenderSegment>,
}

/// A segment of rendered output — either HTML or a Mermaid diagram.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderSegment {
    /// Rendered HTML content (sanitized).
    Html(String),
    /// A Mermaid diagram to be rendered separately.
    Mermaid(MermaidBlock),
}

impl Default for RenderedMarkdown {
    fn default() -> Self {
        Self {
            segments: vec![RenderSegment::Html(String::new())],
        }
    }
}

/// Parse Markdown source into a `RenderedMarkdown` structure.
///
/// This function:
/// - Converts Markdown to HTML using `pulldown-cmark`
/// - Extracts fenced code blocks with language `mermaid` as separate segments
/// - Does NOT enable raw HTML passthrough (XSS prevention)
/// - Uses GFM extensions (tables, strikethrough, task lists)
///
/// # Security
///
/// Raw HTML in Markdown input is escaped, not rendered. This prevents XSS
/// attacks from user-supplied Markdown content. Mermaid blocks are extracted
/// and rendered separately via the Mermaid.js library with `securityLevel: 'strict'`.
pub fn render_markdown(input: &str) -> RenderedMarkdown {
    if input.is_empty() {
        return RenderedMarkdown::default();
    }

    // Enable GFM extensions but NOT raw HTML
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(input, options);

    let mut segments: Vec<RenderSegment> = Vec::new();
    let mut current_html = String::new();
    let mut in_mermaid_block = false;
    let mut mermaid_code = String::new();
    let mut mermaid_counter: usize = 0;

    // We need to iterate event by event to detect mermaid blocks
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang)))
                if is_mermaid_language(lang) =>
            {
                in_mermaid_block = true;
                mermaid_code.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_mermaid_block => {
                in_mermaid_block = false;
                mermaid_counter += 1;

                // Flush any accumulated HTML
                if !current_html.is_empty() {
                    segments.push(RenderSegment::Html(std::mem::take(&mut current_html)));
                }

                // Add the mermaid block
                segments.push(RenderSegment::Mermaid(MermaidBlock {
                    id: format!("mermaid-{mermaid_counter}"),
                    code: std::mem::take(&mut mermaid_code),
                }));
            }
            Event::Text(ref text) if in_mermaid_block => {
                mermaid_code.push_str(text);
            }
            _ if in_mermaid_block => {
                // Ignore other events inside mermaid blocks
            }
            // Security: strip raw HTML from Markdown input to prevent XSS.
            // Convert raw HTML events to escaped text instead of passing through.
            Event::Html(html) | Event::InlineHtml(html) => {
                let escaped = html
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                current_html.push_str(&escaped);
            }
            _ => {
                // Render non-mermaid events to HTML
                pulldown_cmark::html::push_html(&mut current_html, std::iter::once(event));
            }
        }
    }

    // Flush remaining HTML
    if !current_html.is_empty() {
        segments.push(RenderSegment::Html(current_html));
    }

    // Ensure at least one segment exists
    if segments.is_empty() {
        segments.push(RenderSegment::Html(String::new()));
    }

    RenderedMarkdown { segments }
}

/// Check if a code block language tag indicates Mermaid.
fn is_mermaid_language(lang: &str) -> bool {
    let lang = lang.trim().to_lowercase();
    lang == "mermaid"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = render_markdown("");
        assert_eq!(result.segments.len(), 1);
        match &result.segments[0] {
            RenderSegment::Html(h) => assert!(h.is_empty()),
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_simple_paragraph() {
        let result = render_markdown("Hello, world!");
        assert_eq!(result.segments.len(), 1);
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<p>"));
                assert!(h.contains("Hello, world!"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_heading() {
        let result = render_markdown("# Title\n\nParagraph");
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<h1>"));
                assert!(h.contains("Title"));
                assert!(h.contains("<p>"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_bold_italic() {
        let result = render_markdown("**bold** and *italic*");
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<strong>bold</strong>"));
                assert!(h.contains("<em>italic</em>"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_code_block_not_mermaid() {
        let result = render_markdown("```rust\nfn main() {}\n```");
        assert_eq!(result.segments.len(), 1);
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<code"));
                assert!(h.contains("fn main()"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_mermaid_block_extraction() {
        let input = "Before\n\n```mermaid\nflowchart LR\n    A-->B\n```\n\nAfter";
        let result = render_markdown(input);

        assert_eq!(result.segments.len(), 3);

        match &result.segments[0] {
            RenderSegment::Html(h) => assert!(h.contains("Before")),
            _ => panic!("Expected HTML segment"),
        }

        match &result.segments[1] {
            RenderSegment::Mermaid(m) => {
                assert_eq!(m.id, "mermaid-1");
                assert!(m.code.contains("flowchart LR"));
                assert!(m.code.contains("A-->B"));
            }
            _ => panic!("Expected Mermaid segment"),
        }

        match &result.segments[2] {
            RenderSegment::Html(h) => assert!(h.contains("After")),
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_multiple_mermaid_blocks() {
        let input = "```mermaid\ngraph TD\n    A-->B\n```\n\nText\n\n```mermaid\nsequenceDiagram\n    A->>B: Hi\n```";
        let result = render_markdown(input);

        let mermaid_count = result
            .segments
            .iter()
            .filter(|s| matches!(s, RenderSegment::Mermaid(_)))
            .count();
        assert_eq!(mermaid_count, 2);
    }

    #[test]
    fn test_table_rendering() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |";
        let result = render_markdown(input);
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<table"), "Expected <table in output");
                assert!(h.contains("<thead"), "Expected <thead in output");
                assert!(h.contains("<tbody"), "Expected <tbody in output");
                assert!(h.contains("<th"), "Expected <th in output");
                assert!(h.contains("A"), "Expected cell content A");
                assert!(h.contains("1"), "Expected cell content 1");
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_raw_html_is_escaped() {
        let input = "<script>alert('xss')</script>";
        let result = render_markdown(input);
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                // Raw HTML should NOT be passed through
                assert!(!h.contains("<script>"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_link_rendering() {
        let result = render_markdown("[Rust](https://rust-lang.org)");
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<a"));
                assert!(h.contains("href=\"https://rust-lang.org\""));
                assert!(h.contains("Rust"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }

    #[test]
    fn test_horizontal_rule() {
        let result = render_markdown("---");
        match &result.segments[0] {
            RenderSegment::Html(h) => {
                assert!(h.contains("<hr"));
            }
            _ => panic!("Expected HTML segment"),
        }
    }
}
