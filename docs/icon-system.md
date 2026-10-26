# Shared SVG Icon System

The UI uses project-owned SVG symbols through `src/components/icons.rs` and `public/icons.svg`.

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

Prefer an existing `IconName`. Avoid Unicode glyphs, emoji, icon fonts, inline SVG path data in feature pages, and external icon libraries.

## Add a new icon

1. Add a semantic variant to `IconName` and map it to a stable kebab-case ID in `as_str()`.
2. Add a matching `<symbol id="...">...</symbol>` to `public/icons.svg`.
3. Keep symbols monochrome and based on `currentColor` so they adapt to the active theme.
4. Use the shared `Icon` component from feature and component code. Do not duplicate SVG markup in pages.
5. Give icon-only buttons and links an accessible name using visible text, `aria-label`, or an appropriate `title`. The icon itself is decorative and hidden from assistive technology.

The sprite is served from `/icons.svg`; this project is deployed as a GitHub Pages user site with `public_url = "/"`.
