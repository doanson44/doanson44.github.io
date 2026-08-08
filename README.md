# Markdown Studio ✨

A **Rust-first, client-side Markdown editor** with live preview and Mermaid diagram support — powered by WebAssembly.

[![Deploy to GitHub Pages](https://github.com/doanson44/doanson44.github.io/actions/workflows/deploy.yml/badge.svg)](https://github.com/doanson44/doanson44.github.io/actions/workflows/deploy.yml)

## Features

- 📝 **Live Markdown Editor** — Write Markdown with instant preview
- 📊 **Mermaid Diagrams** — Flowcharts, sequence diagrams, and more
- 📋 **One-Click Copy** — Instantly copy raw code, table data, and export Mermaid diagrams as PNG images
- 🦀 **Rust-Powered** — Core logic written in Rust, compiled to WebAssembly
- 🔒 **100% Client-Side** — No data ever leaves your browser
- 🎨 **Dark Theme** — Beautiful dark UI with Bootstrap 5
- ⚡ **Reactive** — Powered by Leptos reactive framework
- 🧰 **Formatting Toolbar** — Quick buttons for common Markdown syntax

## Tech Stack

| Technology | Purpose |
|---|---|
| [Rust](https://www.rust-lang.org/) | Core language |
| [Leptos](https://leptos.dev/) | Reactive UI framework |
| [WebAssembly](https://webassembly.org/) | Browser execution target |
| [Trunk](https://trunkrs.dev/) | Build tool & dev server |
| [pulldown-cmark](https://docs.rs/pulldown-cmark/) | Markdown parsing |
| [Mermaid.js](https://mermaid.js.org/) | Diagram rendering |
| [Bootstrap 5](https://getbootstrap.com/) | UI styling |

## Architecture

```
Presentation (Leptos Components)
    ↓
Application (Services, Ports)
    ↓
Domain (Document, Markdown)
    ↑
Infrastructure (Mermaid JS Interop, Browser APIs)
```

The architecture follows clean layering with explicit dependency direction, enabling future backend integration without restructuring the frontend.

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Run Development Server

```bash
trunk serve --open
```

### Build for Production

```bash
trunk build --release
```

The output will be in the `dist/` directory.

### Quality Checks

```bash
cargo fmt --check
cargo check --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## Deployment

The application deploys automatically to GitHub Pages via GitHub Actions on push to `main`.

**Live site:** [https://doanson44.github.io/](https://doanson44.github.io/)

## License

MIT
