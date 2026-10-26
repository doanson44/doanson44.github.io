---
name: ui-ux-studio
description: "UI/UX design intelligence for the Leptos/WASM multi-feature platform. Use when designing or reviewing Home, Tools, Games, CV, Socket, shared components, responsive layouts, accessibility, or themes."
---

# UI/UX Studio — doanson44.github.io

Design guidance for the Tailwind CSS + Leptos/WASM platform.

## When to Apply

Use this skill for UI structure, visual design, UX quality, accessibility, responsive behavior, dark/light mode, and reusable component decisions. Skip it for pure Domain logic and non-UI build tasks.

## Design Context

| Aspect | Current State |
|--------|---------------|
| **Stack** | Leptos 0.7 CSR + Tailwind CSS 4 + WASM |
| **Theme** | `data-theme="dark|light"`, persisted in `localStorage` |
| **Routing** | Hash-based (`#/`, `#/tools`, `#/tools/markdown`, `#/games`, `#/cv`, `#/socket`) |
| **Layout** | Platform shell (Navbar + router + Footer), feature pages fill the content area |
| **Icons** | Project-owned text/SVG symbols; do not add an icon-font dependency |
| **Typography** | Inter for UI, JetBrains Mono for code |

## Priority Checks

1. **Accessibility:** WCAG 2.1 AA, contrast ≥4.5:1, keyboard access, visible focus, useful ARIA labels.
2. **Touch:** Interactive targets should be at least 44×44px where practical.
3. **Themes:** Test both themes, avoid pure black, keep borders visible, use project tokens.
4. **Responsive:** Verify 375/576/768/1024/1280/1920px and prevent accidental horizontal scrolling.
5. **Typography:** Base readable sizing, sufficient line-height, clear heading hierarchy, monospace for code.
6. **Color:** Prefer `--ms-*` semantic tokens and Tailwind utilities over hardcoded UI colors.
7. **Feedback:** Loading, success, error, disabled, and copy states must be visible without relying on color alone.
8. **Forms:** Labels, placeholders, validation feedback, focus states, and keyboard behavior must be correct.
9. **Navigation:** Hash links must work; external links use `target="_blank" rel="noopener noreferrer"`.
10. **Performance:** Keep WASM payload and reactive recomputation reasonable.

## Design Workflow

### Step 1: Analyze
Identify the component, feature boundary, interaction model, theme requirements, and responsive constraints.

### Step 2: Implement with Tailwind
Prefer Tailwind utilities directly in Leptos `view!`. Use project-owned classes in `styles/*.css` only for behavior or styling that cannot reasonably be expressed as utilities.

Do not introduce Bootstrap classes, Bootstrap variables, Bootstrap JavaScript, or icon-font dependencies.

### Step 3: Validate
Check both themes, responsive breakpoints, keyboard/focus behavior, and the pre-delivery checklist.

## Theme Tokens

Use the project semantic tokens:

```text
--ms-bg-primary
--ms-bg-secondary
--ms-bg-surface
--ms-bg-elevated
--ms-border-default
--ms-border-muted
--ms-text-primary
--ms-text-secondary
--ms-text-tertiary
--ms-accent-blue
--ms-accent-green
--ms-accent-purple
--ms-accent-orange
--ms-accent-red
```

Tailwind utilities should reference these tokens when a semantic project color is required, for example `bg-[var(--surface)]`, `text-[var(--text-primary)]`, and `border-[var(--border-color)]`.

## Shared Icon System (Mandatory)

- Use `Icon` and `IconName` from `src/components/icons.rs` with the symbols in `public/icons.svg`.
- Never use emoji, Unicode pictographs, arrows, dingbats, or decorative special characters as UI icons. Do not introduce icon fonts or external icon CSS libraries.
- Prefer a semantic existing icon. If one is missing, add the enum/ID mapping and matching SVG symbol together; symbols must use `currentColor` and IDs must be unique.
- Verify `index.html` explicitly copies `public/icons.svg` into Trunk output and `dist/icons.svg` exists. If all icons are blank, check asset packaging and the browser Network panel for 404s before editing individual symbols.
- Icon-only controls need an accessible name and visible focus. Avoid duplicating accessible text in decorative icons.
- Review changed UI code for leftover glyph icons and verify both themes and responsive behavior after replacement.

## Component Conventions

- Use Tailwind utilities for layout: `flex`, `grid`, `gap-*`, `items-*`, `justify-*`, responsive prefixes, and overflow utilities.
- Use Tailwind utilities for controls, cards, panels, and focus states.
- Keep reusable non-utility behavior in focused project CSS classes.
- Icon-only controls require `title` and/or an appropriate ARIA label.
- Do not communicate state using color alone.
- Do not use inline `style="..."` in Leptos views.

## References

- `references/dark-mode-rules.md`
- `references/light-mode-rules.md`
- `references/pre-delivery-checklist.md`
- `references/accessibility.md`
