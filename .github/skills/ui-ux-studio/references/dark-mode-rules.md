# Dark Mode Design Rules — Markdown Studio

These rules ensure our dark-themed editor is comfortable, accessible, and professional.

## Core Principles

### 1. Never Use Pure Black (#000)
Pure black (`#000`) backgrounds cause eye strain against white text. Bootstrap dark mode uses `#212529` as body background. Always use Bootstrap's dark tokens.

### 2. Reduce Contrast Slightly
Maximum contrast (white on pure black) is harsh. Bootstrap dark mode targets ~15:1 for body text — comfortable for reading but not blinding.

### 3. Visible Borders
Dark backgrounds make borders hard to see. Always use `border-secondary` (not `border-light` which would be too subtle).

## Bootstrap Dark Mode CSS Variables

```css
/* In :root or inherited from data-bs-theme="dark" */
--bs-body-bg: #212529;
--bs-body-bg-rgb: 33, 37, 41;
--bs-body-color: #dee2e6;
--bs-body-color-rgb: 222, 226, 230;
--bs-secondary-bg: #343a40;
--bs-tertiary-bg: #2b3035;
--bs-border-color: #495057;
--bs-link-color: #6ea8fe;
--bs-link-hover-color: #8bb9fe;
--bs-code-color: #e685b5;
```

## Field-by-Field Rules

### Text
| Rule | Good | Bad |
|------|------|-----|
| Body text | `color: var(--bs-body-color)` on `var(--bs-body-bg)` | White text on pure black |
| Muted text | `text-body-secondary` | Gray on dark gray (too low contrast) |
| Links | Use Bootstrap link tokens | Custom low-contrast link colors |
| Code inline | `var(--bs-code-color)` (pinkish) | Same color as body text |

### Backgrounds
| Rule | Good | Bad |
|------|------|-----|
| Main background | `var(--bs-body-bg)` = `#212529` | `#000000` |
| Elevated surfaces | `var(--bs-secondary-bg)` = `#343a40` | Same as body (no depth) |
| Input/textarea | `var(--bs-body-bg)` or slightly lighter | White input on dark page |
| Card/panel | `var(--bs-tertiary-bg)` = `#2b3035` | No visual distinction |

### Borders
| Rule | Good | Bad |
|------|------|-----|
| Panel separators | `border-secondary` | `border-light` (invisible), `border-dark` (same as bg) |
| Input borders | `var(--bs-border-color)` | Removed borders (hard to see input area) |
| Focus rings | Bootstrap default blue glow | Custom low-contrast focus indicator |

### Interactive Elements
| Element | Dark Mode Approach |
|---------|-------------------|
| Buttons | Use Bootstrap button variants (`.btn-primary`, `.btn-outline-secondary`) |
| Toolbar buttons | `btn-outline-secondary` — visible border, subtle hover |
| Copy buttons (overlay) | `btn-dark` for contrast against code blocks |
| Icons | `text-primary` for emphasis, default for regular |
| Hover states | Bootstrap handles automatically with `.btn-outline-*` |

## Mermaid SVG Integration

Mermaid diagrams render as inline SVG in the preview panel. Rules:
- SVG backgrounds should be transparent to blend with dark preview
- Mermaid text should use light colors (handled by Mermaid's `dark` theme)
- Ensure Mermaid `theme: dark` or `theme: neutral` in render config

## Testing Checklist

- [ ] Text readable against all backgrounds
- [ ] Borders visible between panels
- [ ] Toolbar buttons distinguishable
- [ ] Copy buttons visible on code blocks
- [ ] Placeholder text has sufficient contrast
- [ ] Focus indicators clearly visible
- [ ] Mermaid diagrams readable in dark mode
- [ ] No elements use pure black or maximum contrast white
