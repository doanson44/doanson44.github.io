# Light Mode Design Rules — doanson44.github.io

These rules ensure the light theme is clean, accessible, and consistent with the platform's visual identity.

## Core Principles

### 1. Comfortable White (Never Maximum Brightness)
Pure white (`#ffffff`) is acceptable as a body background in light mode, but avoid making it the ONLY surface color. Use layered backgrounds to create depth.

### 2. Maintain Adequate Contrast
Light mode can easily become "gray on gray" — ensure body text has at least 4.5:1 contrast (WCAG AA). Bootstrap light mode provides this by default.

### 3. Visible Borders Still Matter
In light mode, `border-secondary` stays visible and consistent. Do not use `border-light` (too subtle) or remove borders entirely.

### 4. Custom Variables Must Work in Both Modes
Every `--ms-*` CSS variable MUST have both `[data-bs-theme="dark"]` and `[data-bs-theme="light"]` definitions. Adding a new variable without a light mode counterpart will break the UI when the user switches.

## Bootstrap Light Mode CSS Variables

```css
/* Bootstrap light mode defaults */
--bs-body-bg: #ffffff;
--bs-body-bg-rgb: 255, 255, 255;
--bs-body-color: #212529;
--bs-body-color-rgb: 33, 37, 41;
--bs-secondary-bg: #e9ecef;
--bs-tertiary-bg: #f8f9fa;
--bs-border-color: #dee2e6;
--bs-link-color: #0d6efd;
--bs-link-hover-color: #0a58ca;
--bs-code-color: #d63384;
```

## Platform Custom Variables — Light Mode

```css
[data-bs-theme="light"] {
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
    --ms-shadow-glow: 0 0 20px rgba(9, 105, 218, 0.08);
    --ms-shadow-card: 0 2px 8px rgba(0, 0, 0, 0.08);
}
```

## Field-by-Field Rules

### Text
| Rule | Good | Bad |
|------|------|-----|
| Body text | `color: var(--bs-body-color)` on `var(--bs-body-bg)` | #555 gray on white (too low contrast) |
| Muted text | `text-body-secondary` | Light gray `#ccc` on white |
| Links | `--bs-link-color: #0d6efd` | Custom low-contrast blue |
| Code inline | `var(--bs-code-color)` (pink/red) | Same color as body text |

### Backgrounds
| Rule | Good | Bad |
|------|------|-----|
| Main background | `var(--bs-body-bg)` = `#ffffff` | Off-white that looks dirty |
| Elevated surfaces | `var(--bs-secondary-bg)` = `#e9ecef` | Same as body (no depth) |
| Cards | `bg-body-tertiary` = `#f8f9fa` | No visual distinction from body |
| Editor textarea | Platform `--ms-bg-primary` = `#ffffff` | Dark gray in light mode (jarring) |

### Borders
| Rule | Good | Bad |
|------|------|-----|
| Panel separators | `border-secondary` | `border-light` (invisible), removed borders |
| Card edges | `border border-secondary` | No border on white bg (floats visually) |
| Focus rings | Bootstrap default blue (`rgba(13,110,253,.25)`) | Custom low-contrast focus |

### Interactive Elements
| Element | Light Mode Approach |
|---------|-------------------|
| Toolbar buttons | `btn-outline-secondary` — visible border on light bg |
| Copy buttons (overlay) | `btn-dark` still works, or `btn-outline-secondary` |
| Badges | `bg-primary bg-opacity-10 text-primary-emphasis` (subtle tint) |
| Hover states | Bootstrap defaults, slightly darker background |
| Theme toggle icon | `bi-sun-fill` shown (🌙 hidden), hover color: `--ms-accent-orange` |

### Gradients & Decorative
| Rule | Good | Bad |
|------|------|-----|
| Navbar brand icon | `text-primary` (solid blue) | Dark gradient invisible on white |
| Heading 1 gradient | Gradient with opaque fallback | Transparent gradient invisible on light |
| Horizontal rules | `border-top: 2px solid var(--ms-border-default)` | Same as dark (low contrast) |

## Heading Gradient Fix in Light Mode

The h1 gradient uses `background-clip: text` with transparent fill — this breaks in light mode if colors are too bright. Ensure light mode gradients use darker shades:

```css
[data-bs-theme="light"] .markdown-body h1 {
    background: linear-gradient(135deg, #0969da, #8250df);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}
```

## Scrollbar Styling

```css
[data-bs-theme="light"] ::-webkit-scrollbar-track {
    background: var(--ms-bg-surface);
}

[data-bs-theme="light"] ::-webkit-scrollbar-thumb {
    background: var(--ms-border-default);
}

[data-bs-theme="light"] ::-webkit-scrollbar-thumb:hover {
    background: var(--ms-text-secondary);
}
```

## Mermaid SVG in Light Mode

- SVG backgrounds should remain transparent to blend with light preview
- Mermaid text uses `theme: neutral` or `theme: default` (handled by Mermaid)
- No need to change Mermaid config between themes — transparent SVG adapts automatically

## Theme Toggle Button

```css
/* Show/hide icons based on theme */
[data-bs-theme="dark"] .theme-icon-light { display: none; }
[data-bs-theme="dark"] .theme-icon-dark { display: inline; }
[data-bs-theme="light"] .theme-icon-light { display: inline; }
[data-bs-theme="light"] .theme-icon-dark { display: none; }
```

## Testing Checklist

- [ ] All `--ms-*` variables have both dark and light definitions
- [ ] Body text contrast ≥ 4.5:1
- [ ] Borders visible (`border-secondary`) on all panels
- [ ] Cards visually distinct from body background
- [ ] Toolbar buttons distinguishable
- [ ] Copy buttons visible on light code blocks
- [ ] Placeholder text not too faint
- [ ] Focus indicators clearly visible on white bg
- [ ] Heading gradient renders correctly
- [ ] Scrollbar visible and styled
- [ ] Mermaid diagrams readable
- [ ] Toggle icon switches correctly (☀️↔🌙)
- [ ] No pure black elements (use dark grays)
- [ ] No white text that becomes invisible on light bg
