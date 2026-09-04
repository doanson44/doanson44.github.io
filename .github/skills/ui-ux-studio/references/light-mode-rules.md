# Light Mode Design Rules

These rules keep the light theme clean, accessible, and consistent.

## Core Principles

1. Use layered surfaces rather than making every element pure white.
2. Maintain WCAG AA contrast, especially for muted text and borders.
3. Keep panel, card, table, and input boundaries visible.
4. Every `--ms-*` theme token must have an appropriate light-mode value.
5. Never use pure black for UI backgrounds.

## Light Theme Tokens

```css
[data-theme="light"] {
    --ms-bg-primary: #ffffff;
    --ms-bg-secondary: #f6f8fa;
    --ms-bg-surface: #f0f2f5;
    --ms-bg-elevated: #e8eaed;
    --ms-border-default: #d0d7de;
    --ms-border-muted: #e0e4e8;
    --ms-text-primary: #1f2328;
    --ms-text-secondary: #656d76;
    --ms-text-tertiary: #8b949e;
    --ms-accent-blue: #0969da;
    --ms-accent-green: #1a7f37;
    --ms-accent-purple: #8250df;
    --ms-accent-orange: #bf8700;
    --ms-accent-red: #cf222e;
}
```

## Field Rules

| Area | Guidance |
|------|----------|
| Body text | `--ms-text-primary` on `--ms-bg-primary` |
| Muted text | `--ms-text-secondary` with sufficient contrast |
| Main background | `--ms-bg-primary` |
| Elevated surfaces | `--ms-bg-secondary` / `--ms-bg-elevated` |
| Cards | `--ms-bg-surface` with visible borders |
| Inputs | Visible border and readable text/placeholder |
| Focus | Visible accent outline |
| Links | Semantic accent color with hover feedback |

## Tailwind Pattern

```html
<div class="border border-[var(--border-color)] bg-[var(--surface)] text-[var(--text-primary)]">
```

## Mermaid and Code

Code blocks and Mermaid diagrams must remain visually distinct from the page background while preserving readable text and borders.

## Testing Checklist

- [ ] Text contrast meets WCAG AA
- [ ] Borders remain visible
- [ ] Cards and inputs are visually distinct
- [ ] Focus indicators are visible
- [ ] Code and Mermaid content remains readable
- [ ] Hover/disabled states remain understandable without color alone
- [ ] Theme toggle remains usable
- [ ] No pure-black UI backgrounds
