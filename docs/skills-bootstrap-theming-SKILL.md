# Bootstrap Theming â€” doanson44.github.io

Bootstrap 5.3 theming guide for our dark/light-mode Leptos/WASM platform.

## CSS File Locations

| File | Purpose |
|------|---------|
| `styles/app.css` | Custom styles â€” edit freely |
| `index.html` | Bootstrap CDN + Bootstrap Icons CDN (do NOT modify these URLs) |

## Adding Custom Styles

### Where to Add
Always add custom CSS in `styles/app.css`. Never use inline styles in Leptos `view!` macros.

### CSS Variables (Custom Properties)
Use Bootstrap's CSS variables when possible:

```css
/* CORRECT â€” uses Bootstrap tokens */
.my-custom-panel {
    background: var(--bs-body-bg);
    border: 1px solid var(--bs-border-color);
    color: var(--bs-body-color);
}

/* WRONG â€” hardcoded values */
.my-custom-panel {
    background: #212529;
    border: 1px solid #495057;
    color: #dee2e6;
}
```

### Defining Custom Variables
Add new tokens in `:root` or `[data-bs-theme="dark"]`:

```css
[data-bs-theme="dark"] {
    /* Custom app tokens */
    --app-editor-font: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', monospace;
    --app-preview-font: system-ui, -apple-system, sans-serif;
    --app-toolbar-height: 40px;
    --app-footer-height: 32px;
}
```

## Editor Textarea Styling

```css
.editor-textarea {
    font-family: var(--app-editor-font, monospace);
    font-size: 14px;
    line-height: 1.6;
    resize: none;              /* Prevent user resizing */
    border: none;               /* Panel borders handle edges */
    border-radius: 0;           /* Flush with panel */
    background: var(--bs-body-bg);
    color: var(--bs-body-color);
    padding: 1rem;
    tab-size: 4;
}

.editor-textarea:focus {
    box-shadow: none;           /* Panel handles focus indicator */
    border-color: transparent;
}

.editor-textarea::placeholder {
    color: var(--bs-secondary-color);
    opacity: 0.6;
}
```

## Preview Content Styling

```css
/* Prose in preview panel */
.preview-content {
    padding: 1rem 1.5rem;
    font-family: var(--app-preview-font, system-ui, sans-serif);
    font-size: 16px;
    line-height: 1.7;
    color: var(--bs-body-color);
    max-width: 75ch;            /* Readable line length */
}

/* Code blocks in preview */
.preview-content pre {
    background: var(--bs-tertiary-bg);
    border: 1px solid var(--bs-border-color);
    border-radius: 0.375rem;
    padding: 1rem;
    position: relative;         /* For copy button positioning */
}

.preview-content code {
    font-family: var(--app-editor-font, monospace);
    font-size: 0.875em;
    color: var(--bs-code-color);
}

/* Tables */
.preview-content table {
    width: 100%;
    margin: 1rem 0;
    border-collapse: collapse;
}

.preview-content th,
.preview-content td {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--bs-border-color);
    text-align: left;
}

.preview-content th {
    background: var(--bs-tertiary-bg);
    font-weight: 600;
}

/* Blockquotes */
.preview-content blockquote {
    border-left: 3px solid var(--bs-primary);
    padding-left: 1rem;
    color: var(--bs-secondary-color);
    margin: 1rem 0;
}
```

## Copy Button Styling

```css
/* Copy button overlay on code blocks & tables */
.copy-btn {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    opacity: 0;
    transition: opacity 150ms ease;
}

pre:hover .copy-btn,
table:hover .copy-btn {
    opacity: 1;
}

.copy-btn.copied {
    background: var(--bs-success) !important;
}
```

## Toolbar Styling

```css
.toolbar {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background: var(--bs-tertiary-bg);
    border-bottom: 1px solid var(--bs-border-color);
    overflow-x: auto;
    flex-wrap: wrap;
    min-height: var(--app-toolbar-height, 40px);
}

/* Toolbar button group separator */
.toolbar-separator {
    width: 1px;
    height: 24px;
    background: var(--bs-border-color);
    margin: 0 0.25rem;
}
```

## Responsive Breakpoints

Platform breakpoints (mobile-first):
```css
/* Default: mobile (< 576px) â€” stacked layout, 16px editor font, 
   always-visible copy buttons, scrollable tables */

/* sm â‰¥ 576px */
@media (min-width: 576px) { }

/* md â‰¥ 768px â€” navbar expands, split pane side-by-side */
@media (min-width: 768px) { 
    .editor-preview-container { flex-direction: row; }
}

/* lg â‰¥ 992px */
@media (min-width: 992px) { }

/* xl â‰¥ 1200px */
@media (min-width: 1200px) { }

/* xxl â‰¥ 1400px */
@media (min-width: 1400px) { }
```

Key mobile rules (â‰¤ 576px):
- Editor font 16px (prevents iOS auto-zoom)
- Copy buttons `opacity: 0.7` (no hover on touch)
- Tables `display: block; overflow-x: auto`
- Code blocks `white-space: pre-wrap`
- Footer stacks vertically
- Toolbar icon-only, horizontal scroll
- Navbar collapses at 768px (`navbar-expand-md`)

## Adding a New CSS Class

1. Open `styles/app.css`
2. Add the class at the bottom (or in the relevant section)
3. Use Bootstrap variables where possible
4. Use the class in Leptos `view!` with `class="my-class"`

## Bootstrap Utility Classes Quick Reference

| Purpose | Class |
|---------|-------|
| Flexbox row | `d-flex` |
| Flex column | `d-flex flex-column` |
| Full height | `h-100` or `vh-100` |
| Grow to fill | `flex-grow-1` |
| Hidden overflow | `overflow-hidden` |
| Scroll overflow | `overflow-auto` |
| Gap between children | `gap-1` (0.25rem) through `gap-5` (3rem) |
| Padding | `p-0` through `p-5`, `px-*`, `py-*` |
| Margin | `m-0` through `m-5`, `mx-auto` for centering |
| Alignment | `align-items-center`, `justify-content-between` |
| Border | `border`, `border-top`, `border-bottom`, `border-secondary` |
| Text size | `small`, `fs-6` through `fs-1` |
| Text color | `text-body`, `text-body-secondary`, `text-primary` |

## Do Not

- âŒ Add `!important` unless absolutely necessary
- âŒ Hardcode colors in Rust `view!` (use CSS classes)
- âŒ Modify Bootstrap CDN files
- âŒ Use `style="..."` inline attributes
- âŒ Override Bootstrap variables globally without understanding side effects
