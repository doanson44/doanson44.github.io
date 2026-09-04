# Dark Mode Design Rules

These rules keep the platform readable, accessible, and visually consistent in dark mode.

## Core Principles

### 1. Never Use Pure Black
Do not use `#000` or `#000000` for UI backgrounds. Use the project dark surface tokens instead.

### 2. Use Semantic Tokens
Prefer `--ms-*` theme tokens and Tailwind utilities referencing them. Do not introduce framework-specific color variables.

### 3. Keep Boundaries Visible
Use `var(--ms-border-default)` or the corresponding Tailwind border utility. Inputs, panels, tables, and interactive controls must remain distinguishable from their background.

## Theme Tokens

```css
--ms-bg-primary: #0d1117;
--ms-bg-secondary: #161b22;
--ms-bg-surface: #1c2128;
--ms-bg-elevated: #21262d;
--ms-border-default: #30363d;
--ms-border-muted: #21262d;
--ms-text-primary: #e6edf3;
--ms-text-secondary: #8b949e;
--ms-text-tertiary: #6e7681;
--ms-accent-blue: #58a6ff;
--ms-accent-green: #3fb950;
--ms-accent-purple: #bc8cff;
--ms-accent-orange: #d29922;
--ms-accent-red: #f85149;
```

## Field Rules

| Area | Guidance |
|------|----------|
| Body text | `--ms-text-primary` on `--ms-bg-primary` |
| Muted text | `--ms-text-secondary`, while maintaining WCAG contrast |
| Main surface | `--ms-bg-primary` |
| Elevated surface | `--ms-bg-secondary` or `--ms-bg-elevated` |
| Cards/panels | `--ms-bg-surface` with a visible border |
| Inputs | Dark surface with visible `--ms-border-default` |
| Focus | Visible accent-colored outline with sufficient contrast |
| Links/actions | `--ms-accent-blue` or another semantic accent |

## Tailwind Pattern

Use utilities such as:

```html
<div class="border border-[var(--border-color)] bg-[var(--surface)] text-[var(--text-primary)]">
```

## Interaction

- Keyboard focus must remain clearly visible.
- Disabled controls must remain distinguishable without relying only on color.
- Copy, loading, success, and error states need non-color feedback where appropriate.
- Avoid excessive glow or contrast that makes code-heavy screens fatiguing.

## Mermaid

Mermaid diagrams must remain readable against the dark preview surface. Verify SVG text, lines, and labels in the dark theme.

## Testing Checklist

- [ ] No pure-black UI background
- [ ] Text meets WCAG contrast requirements
- [ ] Borders remain visible
- [ ] Inputs and controls are distinguishable
- [ ] Focus indicators are visible
- [ ] Mermaid diagrams remain readable
- [ ] Loading/error/success states remain understandable without color alone
