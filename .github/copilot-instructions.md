---
description: "Always-on coding conventions for doanson44.github.io — a Rust/Leptos/WASM multi-feature web platform (Tools, Games, CV, Socket) with Clean Architecture, deployed to GitHub Pages. Apply to Rust, TOML, HTML, and CSS files."
applyTo: "**/*.rs,**/*.toml,**/*.html,**/*.css"
---

# doanson44.github.io — Project Conventions

## Project Identity
`doanson44.github.io` is a **client-side-first Web Utility Hub + Personal Portfolio + Playground** — not a single application. It is a multi-feature platform containing developer tools, games, a public CV/portfolio, and realtime/socket experiments, built with Rust/Leptos 0.7 CSR/WASM.

Primary areas:
- **Home** — platform landing page and navigation
- **Tools** — developer utilities (Markdown Studio, JSON, JWT, Base64, Regex, etc.)
- **Games** — small browser games and experiments
- **CV** — public CV / portfolio
- **Socket** — WebSocket/realtime playground and demonstrations
- **Shared Platform** — routing, navigation, theme, layout, reusable components, accessibility, common infrastructure

Markdown Studio is **one tool inside the platform**, not the repository identity.

## Technology Constraints
- Rust edition 2021, `wasm32-unknown-unknown`, Leptos 0.7 CSR only, Trunk as build tool
- Bootstrap 5.3.3 CDN — dark theme by default, CSS variables for theming
- Bootstrap Icons 1.11.3 CDN for iconography
- GitHub Pages deployment (`public_url = "/"`, `dist = "dist"`), GitHub Actions CI/CD
- Feature-specific deps (pulldown-cmark, Mermaid.js, WebSocket, etc.) only when justified

## Platform Architecture
```
Platform Shell (Routing, Navbar, Footer, Theme, Shared Components, A11y)
|
├── Home
├── Tools
│   ├── Markdown Studio
│   ├── JSON Formatter
│   ├── JWT Decoder
│   └── ...
├── Games
├── CV
└── Socket (external backend for WebSocket)
```

## Clean Architecture (Highest Priority)
```
src/
├── main.rs          # WASM entry: console_log init, mount App
├── lib.rs           # Top-level module declarations only
├── app.rs           # Root App, routing, platform shell
├── components/      # Shared PRESENTATION: Navbar, Footer, Cards, Modal, Theme switcher
├── features/        # FEATURES: Per-feature state + composition
│   ├── home/
│   ├── tools/
│   │   ├── markdown/
│   │   ├── json/
│   │   └── ...
│   ├── games/
│   ├── cv/
│   └── socket/
├── application/     # APPLICATION: Services + Port traits
├── domain/          # DOMAIN: Pure Rust, zero framework deps
└── infrastructure/  # INFRASTRUCTURE: Browser APIs, JS interop, HTTP, WebSocket
```

### Layer Rules
- **Domain** MUST NOT depend on Leptos, web-sys, wasm-bindgen, or browser APIs
- **Application** may depend on Domain only. Ports define traits; services call domain + ports
- **Infrastructure** wraps browser/JS/HTTP/WebSocket APIs behind safe Rust functions
- **Features** hold `RwSignal<T>` + `Memo<T>`, bridge Components ↔ Application. Must not bypass application services to call domain directly
- **Components** consume signals from features, call Application services, NEVER call Domain directly
- **Platform shell** owns routing, navigation, theme, global layout, shared components, a11y, global styles
- **Features** own feature-specific logic — do not put feature logic in global platform components

## Leptos 0.7 Patterns
```rust
// Components
#[component] pub fn Foo(...) -> impl IntoView { view! { ... } }

// State: RwSignal::new(val) mutable, Memo::new(move |_| ...) derived
// Props: function args, #[prop(default = ...)] for optionals
// DOM refs: NodeRef::<Div>::new(), access via .get()
// Effects: Effect::new(move |_| { ... }) re-runs on signal deps
// Async: leptos::task::spawn_local(async { ... })
// Events: on:input, on:click; event_target_value(&ev)
```
Prefer reactive state over manual DOM manipulation.

## Code Style
- **Code Comments**: Only add comments to the code when it is absolutely necessary (e.g., explaining complex logic, non-obvious design decisions, or working around compiler/framework quirks). Avoid redundant comments that simply repeat what the code does.
- All pub items: `///` docs. Domain types: `#[derive(Debug, Clone, PartialEq)]`
- Each dir has `mod.rs` re-exporting children. `lib.rs` declares top-level modules only
- Tests in `#[cfg(test)] mod tests`. Prefer `impl Trait` for abstractions, concrete types for signals
- Avoid unnecessary `pub`. Keep feature modules cohesive. All code/comments/commits in English

## UI / UX
- Bootstrap 5 dark theme as default. Use Bootstrap variables (`--bs-body-bg`, `--bs-border-color`, etc.)
- No hardcoded colors in `view!`. Use CSS variables and Bootstrap utility classes
- Icon-only buttons MUST have `title` attr. Never use pure black backgrounds
- Responsive at 375/576/768/1024/1280/1920px. No horizontal scroll. Editor font 16px on mobile (prevents iOS zoom). Nav collapses at 768px (navbar-expand-md).
- Keyboard accessible, visible focus rings, WCAG 2.1 AA contrast, no color-only state indicators

## Routing
```
/                    → Home
/tools               → Tools index
/tools/markdown      → Markdown Studio
/tools/json          → JSON Formatter
/games               → Games
/cv                  → CV / Portfolio
/socket              → Socket playground
```
When adding a feature: define route → register in router → add navigation → add feature module → reuse platform shell.

## Deployment
```bash
cargo fmt --check
cargo check --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown -- -D warnings
trunk build --release
```
Trunk.toml: `public_url = "/"`, `dist = "dist"`. GitHub Actions CI on push to master.
Release: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`.

## Anti-Patterns (NEVER)
- Domain importing Leptos/web-sys/wasm-bindgen
- Components calling domain functions directly (bypass application services)
- Hardcoded colors in `view!` (use CSS classes/variables)
- JS logic duplicated in Rust (use infrastructure interop)
- Feature-specific logic in global platform components
- Duplicating shared platform components across features
- Assuming GitHub Pages can host WebSocket servers
- Unnecessary `pub`, missing `mod.rs`, speculative abstractions
- Modifying unrelated features during a focused task
- Claiming tests/build passed without actually running them

## Core Rule
Treat `doanson44.github.io` as a **Rust/Leptos/WASM multi-feature web platform deployed to GitHub Pages** — with Tools, Games, CV, and Socket as separate features — not as a single Markdown editor and not as a backend application.
