use serde::{Deserialize, Serialize};

/// Represents a Markdown document with metadata.
///
/// This is a pure domain type with no dependencies on browser APIs,
/// Leptos, or any infrastructure concerns.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarkdownDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl MarkdownDocument {
    /// Creates a new document with the given content.
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            content: content.into(),
            created_at: None,
            updated_at: None,
        }
    }

    /// Returns a default document pre-filled with sample Markdown
    /// showcasing all supported features.
    pub fn sample() -> Self {
        Self::new(
            "Welcome",
            r#"# Welcome to Markdown Studio ✨

A **Rust-powered** Markdown editor with _live preview_ and Mermaid diagram support — running entirely in your browser via WebAssembly.

---

## Features

### Text Formatting

You can write **bold text**, *italic text*, and ***bold italic*** text. You can also use `inline code` for technical terms.

### Lists

#### Unordered List
- 🦀 Written in **Rust** with Leptos
- ⚡ Compiled to **WebAssembly**
- 🎨 Styled with **Bootstrap 5**
- 📊 **Mermaid** diagram support

#### Ordered List
1. Write your Markdown on the left
2. See the live preview on the right
3. Use the toolbar for quick formatting
4. Export or share your work

### Links & Images

Check out the [Rust Programming Language](https://www.rust-lang.org/) website.

![Ferris the Crab](https://rustacean.net/assets/rustacean-flat-noshadow.svg)

### Blockquotes

> "Any sufficiently advanced technology is indistinguishable from magic."
> — Arthur C. Clarke

### Code Blocks

```rust
fn main() {
    println!("Hello from Rust + WASM! 🚀");
    let greeting = "Markdown Studio";
    println!("Welcome to {greeting}");
}
```

```javascript
// JavaScript interop is minimal and isolated
const result = await mermaid.render('id', code);
```

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Headings | ✅ | H1 through H6 |
| Bold/Italic | ✅ | Standard Markdown |
| Lists | ✅ | Ordered and unordered |
| Code Blocks | ✅ | Syntax highlighting |
| Tables | ✅ | GFM tables |
| Mermaid | ✅ | Flowcharts, sequence diagrams |
| Links | ✅ | Internal and external |
| Images | ✅ | With alt text |

---

## Mermaid Diagrams

### Flowchart

```mermaid
flowchart LR
    A[Browser] -->|WASM| B[Rust/Leptos App]
    B --> C[Markdown Parser]
    B --> D[Mermaid Renderer]
    C --> E[HTML Preview]
    D --> F[SVG Diagram]
    E --> G[Live Preview]
    F --> G
```

### Sequence Diagram

```mermaid
sequenceDiagram
    participant User
    participant Editor
    participant Parser
    participant Preview

    User->>Editor: Type Markdown
    Editor->>Parser: Parse content
    Parser->>Preview: Render HTML
    Preview-->>User: Live update
```

### Architecture Diagram

```mermaid
graph TD
    subgraph Presentation
        UI[Leptos Components]
    end
    subgraph Application
        SVC[Services]
        PORT[Ports/Traits]
    end
    subgraph Domain
        DOC[Document Model]
        MD[Markdown Processing]
    end
    subgraph Infrastructure
        MER[Mermaid Adapter]
        BR[Browser APIs]
    end

    UI --> SVC
    SVC --> PORT
    SVC --> MD
    SVC --> DOC
    PORT -.-> MER
    PORT -.-> BR
```

---

### Horizontal Rule

The line above and below are horizontal rules (`---`).

---

*Start editing to see the magic happen!* 🎉
"#,
        )
    }
}

impl Default for MarkdownDocument {
    fn default() -> Self {
        Self::sample()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document_has_unique_id() {
        let doc1 = MarkdownDocument::new("Test", "Content");
        let doc2 = MarkdownDocument::new("Test", "Content");
        assert_ne!(doc1.id, doc2.id);
    }

    #[test]
    fn test_sample_document_has_content() {
        let doc = MarkdownDocument::sample();
        assert!(!doc.content.is_empty());
        assert_eq!(doc.title, "Welcome");
    }

    #[test]
    fn test_document_serialization() {
        let doc = MarkdownDocument::new("Test", "# Hello");
        let json = serde_json::to_string(&doc).unwrap();
        let deserialized: MarkdownDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(doc, deserialized);
    }
}
