# Shared SVG Icon System

The UI uses project-owned SVG symbols through `src/components/icons.rs` and `public/icons.svg`. These are the single source of truth for UI iconography.

## Use an icon

Import the shared component and semantic name:

```rust
use crate::components::icons::{Icon, IconName};

view! {
    <button type="button" aria-label="Refresh data">
        <Icon name=IconName::Refresh class="h-4 w-4" />
    </button>
}
```

Prefer an existing `IconName`. Do not use emoji, Unicode pictographs, arrows, dingbats, or decorative special characters as UI icons. This restriction applies to UI iconography, not normal text or user-authored content.

## Type-safety rule

Check the declared return type before changing icon-related code. Text metadata methods such as `slug()` and `title()` that return `&'static str` must return string literals. Icon accessors that return `IconName` must return enum variants such as `IconName::Grid`. Do not replace textual values with enum variants or icon variants with strings.

## Add a new icon

1. Add a semantic variant to `IconName` and map it to a stable kebab-case ID in `as_str()`.
2. Add exactly one matching `<symbol id="...">...</symbol>` to `public/icons.svg`.
3. Ensure the sprite is copied into Trunk output by keeping `<link data-trunk rel="copy-file" href="public/icons.svg" />` in `index.html`. Do not assume files under `public/` are emitted automatically. Verify `dist/icons.svg` exists after building and `/icons.svg#<id>` does not return 404.
3. Keep symbols monochrome and based on `currentColor` so they adapt to the active theme.
4. Use the shared `Icon` component from feature and component code. Do not duplicate SVG markup in pages.
5. Give icon-only buttons and links an accessible name using visible text, `aria-label`, or an appropriate `title`. The icon itself is decorative and hidden from assistive technology.
6. Do not add icon fonts, external icon CSS libraries, or a parallel icon system.

## Review and validation

After an icon refactor, inspect changed files for leftover UI glyphs, duplicate/missing SVG IDs, and type mismatches. Run the available Rust/WASM checks and explicitly report checks that were not run.

The sprite is served from `/icons.svg`; this project is deployed as a GitHub Pages user site with `public_url = "/"`.
