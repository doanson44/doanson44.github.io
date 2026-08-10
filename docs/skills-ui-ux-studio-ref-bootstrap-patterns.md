# Bootstrap 5 Component Patterns â€” Markdown Studio

Bootstrap 5.3 recipes adapted for our dark-themed Leptos/WASM app.

## Panel Pattern (Editor / Preview)

```html
<div class="panel-name d-flex flex-column h-100">
    <div class="panel-header d-flex align-items-center justify-content-between px-3 py-2 border-bottom border-secondary">
        <span class="panel-title">
            <i class="bi bi-icon me-2 text-primary"></i>
            "Panel Title"
        </span>
        <span class="text-body-secondary small">
            <!-- Optional: status, counter, metadata -->
        </span>
    </div>
    <div class="panel-body flex-grow-1 overflow-auto">
        <!-- Content -->
    </div>
</div>
```

**Rules:**
- Always use `h-100` + `d-flex flex-column` for full-height panels
- Panel body gets `flex-grow-1 overflow-auto` to fill remaining space
- Headers use `border-bottom border-secondary` for separation
- Title text color: default (inherits body color)
- Metadata text: `text-body-secondary small`

## Button Styles

```html
<!-- Primary action -->
<button class="btn btn-primary btn-sm">
    <i class="bi bi-play me-1"></i> Action
</button>

<!-- Secondary / outline -->
<button class="btn btn-outline-secondary btn-sm">
    <i class="bi bi-gear"></i>
</button>

<!-- Toolbar icon-only -->
<button class="btn btn-sm btn-outline-secondary" title="Tooltip text">
    <i class="bi bi-type-bold"></i>
</button>

<!-- Dark variant (for overlay buttons) -->
<button class="btn btn-sm btn-dark">
    <i class="bi bi-clipboard"></i> Copy
</button>
```

**Rules:**
- Toolbar buttons: `btn-sm btn-outline-secondary` with `title` attribute
- Icon + text buttons: add `me-1` or `gap-1` spacing
- Icon-only buttons: MUST have `title` for accessibility
- Never use buttons without visible text or a `title` tooltip

## Badge / Tag Pattern

```html
<span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle">
    <i class="bi bi-cpu me-1"></i>
    "WASM"
</span>
```

**Rules:**
- Use `bg-opacity-25` for subtle badges on dark backgrounds
- `text-primary-emphasis` ensures text is readable
- `border border-primary-subtle` adds definition

## Navbar Pattern

```html
<nav class="navbar navbar-expand-lg border-bottom border-secondary">
    <div class="container-fluid">
        <a class="navbar-brand d-flex align-items-center gap-2" href="#">
            <i class="bi bi-icon fs-3 text-primary"></i>
            <span class="fw-bold">"Brand"</span>
        </a>
        <!-- Right side items -->
    </div>
</nav>
```

**Rules:**
- `border-bottom border-secondary` for bottom edge
- Brand icon: `fs-3 text-primary`
- Brand text: `fw-bold`
- Right-side items: `d-flex align-items-center gap-3`

## Footer Pattern

```html
<footer class="app-footer d-flex align-items-center justify-content-between px-3 py-1 border-top border-secondary">
    <span class="text-body-secondary small">"Left content"</span>
    <span class="text-body-secondary small">"Right content"</span>
</footer>
```

**Rules:**
- `border-top border-secondary` for top edge
- All text: `text-body-secondary small`
- Use `justify-content-between` for left/right split

## Split Pane Layout

```html
<div class="editor-preview-container flex-grow-1 d-flex overflow-hidden">
    <div class="editor-pane"><!-- Editor --></div>
    <div class="divider"></div>
    <div class="preview-pane"><!-- Preview --></div>
</div>
```

CSS for divider:
```css
.divider {
    width: 1px;
    background-color: var(--bs-border-color);
    cursor: col-resize;
}
```

**Rules:**
- Container: `flex-grow-1 d-flex overflow-hidden`
- Panes get equal flex by default: no `flex-grow` needed (defaults to 0)
- Divider: 1px wide, uses `var(--bs-border-color)`, cursor `col-resize`
- Mobile: stack vertically instead of horizontal split

## Responsive Rules

### Mobile (< 576px)
- Editor font-size: 16px (required by iOS to prevent auto-zoom)
- Toolbar: icon-only, horizontal scroll (`overflow-x: auto`)
- Copy buttons: always visible (`opacity: 0.7`), no hover needed
- Code blocks: `white-space: pre-wrap`
- Tables: `display: block; overflow-x: auto`
- Footer: stacked vertically
- Home page cards: 1 column
- Divider: horizontal (`row-resize`)

### Tablet (576px â€“ 767px)
- Home page cards: 2 columns (`col-sm-6`)
- Split pane: stacked
- Toolbar: icon + text hidden for space

### Tablet Landscape / Small Laptop (768px â€“ 1023px)
- Navbar: expands (`navbar-expand-md`)
- Split pane: side by side
- Font sizes: slightly reduced from desktop
- Toolbar text labels: hidden

### Desktop (â‰¥ 1024px)
- Full layout with all text labels visible
- Split pane with divider
- Home page cards: 3 columns (`col-lg-4`)

## Textarea (Editor) Pattern

```html
<textarea
    id="markdown-editor"
    class="editor-textarea form-control flex-grow-1"
    placeholder="Write your Markdown here..."
    spellcheck="false"
></textarea>
```

**CSS considerations:**
- Monospace font for code editing feel
- `resize: none` to prevent user resize breaking layout
- Dark background matching `--bs-body-bg`
- Focus ring: use Bootstrap default `:focus` style
