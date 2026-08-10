---
name: domain-service
description: "Work with Domain + Application + Infrastructure layers in the doanson44.github.io platform. Use when: adding business logic, creating parsers/renderers (Markdown, JSON, JWT, etc.), defining service ports, adding JS/browser interop, implementing export/copy features, or working with pulldown-cmark pipeline."
---

# Domain & Service Skill — doanson44.github.io

Guide for developing across the three lower Clean Architecture layers. Applicable to any tool: Markdown Studio, JSON Formatter, JWT Decoder, Base64, Regex Tester, etc.

## Layer Map

```
┌─────────────────────────────────────────────┐
│ FEATURES     │ RwSignal, Memo, page comp.   │
├─────────────────────────────────────────────┤
│ APPLICATION  │ Services + Port traits        │  ← this skill covers
├─────────────────────────────────────────────┤
│ DOMAIN       │ Pure Rust, zero framework deps│  ← this skill covers
├─────────────────────────────────────────────┤
│ INFRASTRUCTURE│ Browser APIs, JS interop      │  ← this skill covers
└─────────────────────────────────────────────┘
```

## Quick Decision: Which Layer?

| I need to... | Go to |
|---|---|
| Parse/validate/transform input data | **Domain** |
| Define a trait/port for external dependency | **Application/ports** |
| Coordinate domain logic + ports | **Application/services** |
| Access browser API (clipboard, localStorage, window) | **Infrastructure/browser** |
| Call JavaScript (Mermaid, highlight.js, external lib) | **Infrastructure** (new module or browser) |
| Make HTTP/WebSocket calls | **Infrastructure** (http/ or websocket/) |
| Hold reactive UI state | Features (use `platform-feature` skill instead) |

## Domain Layer (`src/domain/`)

### Rules
- **Zero framework deps** — NO `leptos`, `web-sys`, `wasm-bindgen`
- Pure Rust, testable with `cargo test` (no WASM needed)
- Types derive `Debug, Clone, PartialEq`
- All pub items documented with `///`

### Template: Adding a Domain Type

```rust
/// Represents a parsed/processed result.
#[derive(Debug, Clone, PartialEq)]
pub struct MyResult {
    pub output: String,
    pub metadata: Option<String>,
}

/// Parse raw input into structured output.
///
/// # Errors
/// Returns `Err` with a human-readable message if parsing fails.
pub fn parse(input: &str) -> Result<MyResult, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".into());
    }
    Ok(MyResult {
        output: input.to_string(),
        metadata: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let result = parse("hello").unwrap();
        assert_eq!(result.output, "hello");
    }

    #[test]
    fn test_parse_empty_errors() {
        assert!(parse("").is_err());
    }
}
```

### Register the Module
```rust
// src/domain/mod.rs
pub mod my_tool;
```

## Application Layer (`src/application/`)

### Ports (`application/ports/mod.rs`)
Define traits for external dependencies:

```rust
/// Port for persisting tool state.
pub trait ToolStateRepository {
    fn save(&self, key: &str, value: &str) -> Result<(), String>;
    fn load(&self, key: &str) -> Result<Option<String>, String>;
}
```

### Services (`application/services/`)
Coordinate domain + ports:

```rust
use crate::domain::my_tool;

/// Application service for MyTool use cases.
pub struct MyToolService;

impl MyToolService {
    /// Process input through domain logic.
    pub fn process(input: &str) -> Result<my_tool::MyResult, String> {
        my_tool::parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = MyToolService::process("test").unwrap();
        assert_eq!(result.output, "test");
    }
}
```

## Infrastructure Layer (`src/infrastructure/`)

### Browser APIs (`infrastructure/browser/mod.rs`)
Wraps `web-sys` behind safe Rust:

```rust
/// Copy text to the system clipboard.
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or("No window")?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|e| e.as_string().unwrap_or_else(|| "Clipboard failed".into()))
}
```

### JS Interop Pattern
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Calls global JS function. Use `catch` for error handling.
    #[wasm_bindgen(js_name = "__my_js_function", catch)]
    async fn my_js_function(input: &str) -> Result<JsValue, JsValue>;
}

/// Safe Rust wrapper for the JS function.
pub async fn call_my_function(input: &str) -> Result<String, String> {
    match my_js_function(input).await {
        Ok(js_val) => Ok(js_val.as_string().unwrap_or_default()),
        Err(e) => Err(e.as_string().unwrap_or_else(|| "JS call failed".into())),
    }
}
```

### Register a New Infrastructure Module
```rust
// src/infrastructure/mod.rs
pub mod browser;
pub mod mermaid;
pub mod my_new_module;  // NEW
```

## Feature Wiring Pattern

Once domain + application + infrastructure are done, wire them in a feature:

```rust
// features/tools/my_tool/state.rs
use leptos::prelude::*;
use crate::application::services::MyToolService;

#[derive(Clone)]
pub struct MyToolState {
    pub input: RwSignal<String>,
    pub output: Memo<String>,
    pub error: RwSignal<Option<String>>,
}

impl MyToolState {
    pub fn new() -> Self {
        let input = RwSignal::new(String::new());
        let error = RwSignal::new(None);

        let output = Memo::new(move |_| {
            error.set(None);
            match MyToolService::process(&input.get()) {
                Ok(result) => result.output,
                Err(e) => {
                    error.set(Some(e));
                    String::new()
                }
            }
        });

        Self { input, output, error }
    }
}
```

## Markdown-Specific: pulldown-cmark Pipeline

The Markdown tool uses `pulldown-cmark 0.13`. Key files:

| File | Purpose |
|------|---------|
| `domain/markdown.rs` | `render_markdown()`, `RenderedMarkdown`, `RenderSegment` enum |
| `domain/document.rs` | `MarkdownDocument` type |
| `application/services/mod.rs` | `MarkdownService::render()` |

### Adding a New Segment Type
1. Add variant to `RenderSegment` in `domain/markdown.rs`
2. Extend `render_markdown()` to emit the new variant from pulldown-cmark events
3. Add rendering branch in `components/preview.rs`

```rust
// 1. New variant
pub enum RenderSegment {
    Html(String),
    Mermaid(MermaidBlock),
    // NEW:
    MyNewBlock(MyNewData),
}

// 3. Render in preview
RenderSegment::MyNewBlock(data) => {
    view! { <div class="my-block">{data.content}</div> }.into_any()
}
```

### pulldown-cmark Event Loop Skeleton
```rust
use pulldown_cmark::{Parser, Event, Tag, TagEnd, CodeBlockKind, Options};

let mut options = Options::all();
options.remove(Options::ENABLE_RAW_HTML_DASHLESS_ATTRIBUTES);
let parser = Parser::new_ext(content, options);

for event in parser {
    match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            // Detect language: "mermaid" → MermaidBlock, "rust" → syntax highlight, etc.
        }
        Event::Text(text) => { /* accumulate */ }
        Event::End(TagEnd::CodeBlock) => { /* flush block */ }
        Event::Html(raw) => { /* ESCAPED — not rendered (XSS prevention) */ }
        _ => {}
    }
}
```

## Security Rules (All Tools)

- Raw HTML input must be escaped, never rendered (XSS)
- No API keys, tokens, or secrets in client-side code
- JWT/Base64 tools: clearly distinguish **decode/view** from **verify/auth**
- JS interop: always use `catch` variants
- Validate user input before processing (empty, too large, invalid chars)
- Use `Result<T, String>` for error handling — never `panic!` on user input

## Checklist

When adding a new tool's domain/service logic:

- [ ] Domain types are `Debug + Clone + PartialEq`
- [ ] Domain has zero framework imports
- [ ] `///` docs on all pub items
- [ ] Tests in `#[cfg(test)] mod tests`
- [ ] Service delegates to domain (no direct domain call from components)
- [ ] JS interop uses `catch` + safe Rust wrapper
- [ ] New module registered in `mod.rs` files
- [ ] Errors are human-readable (not raw debug dumps)
- [ ] `cargo test` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown -- -D warnings` passes
