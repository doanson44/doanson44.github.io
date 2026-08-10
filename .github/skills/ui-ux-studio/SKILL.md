---
name: ui-ux-studio
description: "UI/UX design intelligence for doanson44.github.io platform — Bootstrap 5 dark/light themed multi-feature web application. Use when: designing UI for any platform area (Home, Tools, Games, CV, Socket), choosing colors/typography/spacing, reviewing UI for accessibility/consistency, implementing dark/light mode patterns, designing responsive layouts."
---

# UI/UX Studio — doanson44.github.io Design Intelligence

Adapted from [ui-ux-pro-max](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill), tailored for our Bootstrap 5 + Leptos/WASM multi-feature platform.

## When to Apply

Use this skill when the task involves **UI structure, visual design decisions, or UX quality**:
- Designing new panels, components, or layout changes for any platform area
- Choosing colors, typography, spacing, or iconography
- Reviewing UI for accessibility, consistency, dark/light mode issues
- Implementing responsive behavior for pages and components
- Improving perceived quality and usability across the platform

**Skip** for: pure Rust domain logic, Markdown parsing, WASM compilation, build configuration.

## Design Context: doanson44.github.io Platform

| Aspect | Current State |
|--------|--------------|
| **Stack** | Leptos 0.7 (CSR) + Bootstrap 5.3.3 + WASM |
| **Theme** | Dark/light toggle, `data-bs-theme` on `<html>`, persisted in `localStorage` |
| **Routing** | Hash-based (`#/`, `#/tools`, `#/tools/markdown`, `#/games`, `#/cv`, `#/socket`) |
| **Layout** | Platform shell (Navbar + Router + Footer), feature pages fill content area |
| **Icons** | Bootstrap Icons 1.11.3 (`bi bi-xxx`) |
| **Typography** | 'Inter' (body), 'JetBrains Mono' (code) via Google Fonts |
| **User** | Developers, technical writers, visitors |

## Rule Categories by Priority

*Follow priority 1→10. Full details in `references/`.*

| Priority | Category | Impact | Key Checks |
|----------|----------|--------|------------|
| 1 | **Accessibility** | CRITICAL | Contrast ≥4.5:1 (both modes), Keyboard navigation, Focus visible, ARIA labels |
| 2 | **Touch & Interaction** | CRITICAL | Min 44×44px touch targets, Loading states, Button spacing, Feedback on actions |
| 3 | **Dark/Light Mode** | HIGH | Both modes tested, No pure-black (#000), No invisible borders, All `--ms-*` vars defined for both |
| 4 | **Layout & Responsive** | HIGH | No horizontal scroll, Pages work at 375/576/768/1024/1280/1920px, Nav collapses at 768px, Editor 16px on mobile, Copy buttons always visible on touch |
| 5 | **Typography** | MEDIUM | Base 16px min, Line-height ≥1.5, Monospace for code, Readable prose, Heading hierarchy |
| 6 | **Color System** | MEDIUM | Semantic tokens only, `--ms-*` variables, Consistent with Bootstrap theme, Mermaid adapts |
| 7 | **Animation & Feedback** | MEDIUM | Copy-button feedback, Loading indicators, Smooth transitions, No 0ms state changes |
| 8 | **Forms & Editor UX** | MEDIUM | Placeholder visible in both modes, Validation feedback, Toolbar inserts at cursor |
| 9 | **Navigation** | HIGH | All hash links work, Current page indicated, External links have `rel="noopener"` |
| 10 | **Performance** | LOW | WASM payload size, Re-render debounce, Lazy load where practical |

## Design Workflow

### Step 1: Analyze Requirements
Extract from the request:
- **What UI element?** — New component, restyle existing, fix UX issue
- **Where in platform?** — Home, Tools, Games, CV, Socket, or shared component
- **Theme constraint?** — Must work in BOTH `data-bs-theme="dark"` AND `data-bs-theme="light"`
- **Responsive need?** — Desktop vs mobile layout

### Step 2: Apply Bootstrap 5 Patterns
Consult `references/bootstrap-patterns.md` for component recipes.

### Step 3: Validate Against Checklist
After implementation, run through `references/pre-delivery-checklist.md`.
Verify both dark and light mode using `references/dark-mode-rules.md` and `references/light-mode-rules.md`.

## Quick Reference: Bootstrap 5 Theme Tokens

### Dark Theme (default)
```
--bs-body-bg: #212529       (dark background)
--bs-body-color: #dee2e6    (light text)
--bs-border-color: #495057  (visible borders)
--bs-primary: #0d6efd       (accent blue)
```

### Light Theme
```
--bs-body-bg: #ffffff       (white background)
--bs-body-color: #212529    (dark text)
--bs-border-color: #dee2e6  (visible borders)
--bs-primary: #0d6efd       (accent blue)
```

text-body-secondary → muted text (works in both modes)

### Border Conventions
- Panel separators: `border-secondary`
- Section dividers: `border-top border-secondary`
- Card/container edges: `border border-secondary`

### Icon Conventions
- Panel headers: icon + text with `me-2 text-primary`
- Toolbar: icon-only buttons with `title` attribute for tooltip
- Status badges: `bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle`

### Spacing Scale
- Panel padding: `px-3 py-2` (header), `p-0` (content area fills)
- Toolbar gaps: `gap-1` between buttons
- Footer: `px-3 py-1`
- Component gaps: `gap-2` or `gap-3`

## References

| Topic | File |
|-------|------|
| Bootstrap 5 Component Patterns | `references/bootstrap-patterns.md` |
| Dark Mode Design Rules | `references/dark-mode-rules.md` |
| Light Mode Design Rules | `references/light-mode-rules.md` |
| Pre-Delivery Checklist | `references/pre-delivery-checklist.md` |
| Accessibility Guidelines | `references/accessibility.md` |

## Resources

- [Bootstrap 5.3 Dark Mode Docs](https://getbootstrap.com/docs/5.3/customize/color-modes/)
- [Bootstrap Icons](https://icons.getbootstrap.com/)
- [WCAG 2.1 Contrast Checker](https://webaim.org/resources/contrastchecker/)
