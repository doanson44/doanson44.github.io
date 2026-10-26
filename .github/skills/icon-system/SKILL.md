---
name: icon-system
description: "Use when adding, replacing, reviewing, or debugging UI icons in doanson44.github.io. Enforces the shared SVG sprite, semantic IconName enum, accessibility, and Rust type safety."
---

# Shared SVG Icon System

Use this skill whenever changing UI iconography.

## Source of Truth

- Component and semantic enum: `src/components/icons.rs`
- SVG sprite: `public/icons.svg`
- Usage documentation: `docs/icon-system.md`

Import icons with:

```rust
use crate::components::icons::{Icon, IconName};
```

Example:

```rust
view! {
    <button type="button" aria-label="Refresh data">
        <Icon name=IconName::Refresh class="h-4 w-4" />
    </button>
}
```

## Rules

1. Always use the shared `Icon` component and a semantic `IconName` for UI icons.
2. Never use emoji, Unicode pictographs, arrows, dingbats, or decorative special characters as UI icons. This does not prohibit Unicode in normal text or user-authored content.
3. Do not introduce icon fonts, external icon CSS dependencies, duplicate inline SVG paths in feature pages, or a parallel icon abstraction.
4. Before editing an icon mapping, read the function signature and preserve the return type. Text metadata such as `slug()` and `title()` returning `&'static str` must remain string literals. Icon accessors returning `IconName` must return variants such as `IconName::Grid`. Never substitute one type for the other.
5. When adding an icon, add its semantic variant and stable kebab-case ID to `IconName` / `as_str()`, then add exactly one matching `<symbol>` to `public/icons.svg`. Keep IDs unique and symbols monochrome using `currentColor`.
6. Preserve accessibility: icon-only controls need an accessible name and visible keyboard focus; decorative icons must not duplicate the control's accessible name.
7. When replacing glyphs, inspect the entire changed diff for remaining UI glyphs, mismatched types, and accidental changes to text content.

## Validation

Run the checks available in the environment, prioritizing:

```bash
cargo fmt --check
cargo check --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown -- -D warnings
trunk build --release
```

Report each check as passed, failed, or not run. Do not claim compilation or tests passed unless they were actually executed.
