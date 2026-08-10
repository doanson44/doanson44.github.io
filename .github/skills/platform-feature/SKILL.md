---
name: platform-feature
description: "Add a new top-level feature to the doanson44.github.io platform. Use when: adding a new feature area (new tool under Tools, new game, new page), registering routes in the hash router, creating feature module structure, adding navigation entries."
---

# Platform Feature Skill — doanson44.github.io

## Feature Development Checklist

When adding a new feature to the platform:

### 1. Determine Feature Type

| Type | Location | Example |
|------|----------|---------|
| Tool (under /tools) | `features/tools/<name>/` | markdown, json, jwt |
| Top-level page | `features/<name>/` | games, cv, socket |
| Shared platform component | `components/<name>.rs` | navbar, footer, modal |

### 2. Create Feature Module

For a new tool:
```
features/tools/<name>/
├── mod.rs       # pub mod page; pub mod state;
├── page.rs      # #[component] pub fn XxxPage() -> impl IntoView
└── state.rs     # Feature-specific RwSignal/Memo state (if needed)
```

For a new top-level page:
```
features/<name>/
├── mod.rs       # pub mod page;
└── page.rs      # #[component] pub fn XxxPage() -> impl IntoView
```

### 3. Register the Module

**`features/<name>/mod.rs`:**
```rust
pub mod page;
```

**`features/tools/mod.rs`** (for tools):
```rust
pub mod <name>;
```

**`features/mod.rs`:**
```rust
pub mod <name>;
```

**`components/mod.rs`** (for shared components):
```rust
pub mod <name>;
```

### 4. Register the Route

In `src/app.rs`, add to the `render_page()` match:

```rust
"/tools/<name>" => view! { <XxxPage /> }.into_any(),
// or
"/<name>" => view! { <XxxPage /> }.into_any(),
```

### 5. Add Navigation

In `src/components/navbar.rs`, add to the appropriate section:

- **Tool:** Add to the Tools dropdown menu
- **Top-level page:** Add as a nav item

Use hash links: `href="#/path"`

### 6. Template: Page Component

```rust
use leptos::prelude::*;

/// Description of this page.
#[component]
pub fn XxxPage() -> impl IntoView {
    view! {
        <div class="d-flex flex-column flex-grow-1">
            <div class="container py-4">
                <h2>
                    <i class="bi bi-icon me-2 text-primary"></i>
                    "Page Title"
                </h2>
                // Page content
            </div>
        </div>
    }
}
```

### 7. Template: Feature Tool Page (with state)

```rust
use leptos::prelude::*;

use crate::features::tools::<name>::state::XxxState;

/// Feature tool page with reactive state.
#[component]
pub fn XxxPage() -> impl IntoView {
    let state = XxxState::new();

    view! {
        <div class="d-flex flex-column flex-grow-1" style="min-height: 0;">
            // Tool-specific UI
        </div>
    }
}
```

## Platform Conventions

### Layout
- All pages: use `d-flex flex-column flex-grow-1` as root
- Content: wrap in `container py-4` or similar
- Tool pages: use `style="min-height: 0"` to prevent flex overflow
- Placeholder pages: centered icon + heading + "Coming soon" text

### Styling
- Use Bootstrap 5 classes, no hardcoded colors
- Text: `text-body-secondary` for muted, `text-body-tertiary` for very muted
- Cards: `card bg-body-tertiary border-secondary`
- Icons: Bootstrap Icons with `me-2 text-primary` (or appropriate color)
- Coming soon: `<span class="badge bg-secondary">"Coming soon"</span>`

### Links
- Always use hash-based: `href="#/path"`
- External links: `target="_blank" rel="noopener noreferrer"`

## Routing System

Hash-based routing in `src/app.rs`:
- Uses `window.location.hash` + `hashchange` event
- `create_hash_signal()` — tracks current hash as `RwSignal<String>`
- `render_page()` — match route string to page component
- Fallback: 404 page with "Go Home" link

To add a route:
1. Add match arm in `render_page()`
2. Add `use crate::features::<...>::page::XxxPage;` import
3. No other changes needed

## Anti-Patterns

- ❌ Putting feature logic in `app.rs`
- ❌ Duplicating Navbar/Footer inside feature pages
- ❌ Adding leptos_router dependency (use hash routing)
- ❌ Hardcoding colors in `view!`
- ❌ Forgetting to register module in `mod.rs`
- ❌ Using absolute paths (`href="/path"`) instead of hash (`href="#/path"`)
