# doanson44.github.io

A **Rust-first, client-side multi-feature web platform** with developer tools, Markdown Studio, games, CV/portfolio, and a realtime socket playground — powered by WebAssembly.

[![Deploy to GitHub Pages](https://github.com/doanson44/doanson44.github.io/actions/workflows/deploy.yml/badge.svg)](https://github.com/doanson44/doanson44.github.io/actions/workflows/deploy.yml)

## Features

- 🧰 **Developer Tools** — JSON, JWT, Base64, time utilities, finance tools, and developer generators
- 📝 **Markdown Studio** — Live Markdown preview with Mermaid diagrams
- 📊 **Mermaid Diagrams** — Diagram rendering and PNG export
- 🎮 **Games** — Browser-based experiments
- 📄 **CV / Portfolio** — Public professional profile
- 🔌 **Socket Playground** — External-backend realtime/WebSocket experiments
- 🦀 **Rust-Powered** — Core logic written in Rust, compiled to WebAssembly
- 🔒 **100% Client-Side** — Application execution stays in the browser
- 🎨 **Dark/Light Theme** — Project-owned CSS variables with Tailwind utilities
- ⚡ **Reactive** — Powered by Leptos reactive framework

## Tech Stack

| Technology | Purpose |
|---|---|
| [Rust](https://www.rust-lang.org/) | Core language |
| [Leptos](https://leptos.dev/) | Reactive UI framework |
| [WebAssembly](https://webassembly.org/) | Browser execution target |
| [Trunk](https://trunkrs.dev/) | Build tool & dev server |
| [Tailwind CSS](https://tailwindcss.com/) | Utility-first UI styling |
| [pulldown-cmark](https://docs.rs/pulldown-cmark/) | Markdown parsing |
| [Mermaid.js](https://mermaid.js.org/) | Diagram rendering |

## Architecture

```
Presentation (Leptos Components)
    ↓
Features (Reactive State + Composition)
    ↓
Application (Services, Ports)
    ↓
Domain (Pure Rust Business Logic)
    ↑
Infrastructure (Browser APIs, JS Interop, WebSocket)
```

The architecture follows clean layering with explicit dependency direction.

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install Node.js 20+ and project dependencies
npm install
```

### Run Development Server

```bash
trunk serve --open
```

Tailwind CSS is generated automatically by the Trunk pre-build hook.

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
trunk build --release
```

## Deployment

The application deploys to GitHub Pages via GitHub Actions.

**Live site:** [https://doanson44.github.io/](https://doanson44.github.io/)

## License

MIT
