---
name: design-review
description: "Review doanson44.github.io platform UI for quality, accessibility, and consistency. Use when: auditing any platform page before deployment, checking dark/light mode quality, verifying responsive layout, reviewing accessibility compliance, or as a pre-release checklist."
---

# Design Review — doanson44.github.io

Review checklist for the multi-feature web platform. Run this before any significant UI change or deployment.

## Review Workflow

### Phase 1: Visual Scan (Quick — 2 min)
Look at the app and check for obvious issues:
- [ ] Nothing looks broken or misaligned
- [ ] Colors are consistent (no random different shades)
- [ ] Spacing is even and intentional
- [ ] No text cut off or overflowing
- [ ] Empty states look intentional

### Phase 2: Dark Mode Audit (5 min)
Use `ui-ux-studio` skill → `references/dark-mode-rules.md`:
- [ ] No pure black backgrounds
- [ ] All borders visible using project semantic tokens/Tailwind utilities
- [ ] Text contrast comfortable (not blinding white on black)
- [ ] Mermaid diagrams readable in dark context
- [ ] Copy buttons visible on code blocks
- [ ] Placeholder text has adequate contrast

### Phase 3: Responsive Check (5 min)
Test at these widths:
| Width | Check |
|-------|-------|
| 1920px | Split pane balanced, no wasted space |
| 1280px | Both panes usable, text not cramped |
| 1024px | Toolbar labels hidden, font sizes reduced |
| 768px | Panes stack vertically, nav collapses |
| 576px | All mobile optimizations active |
| 375px | No clipping, scroll works correctly |

- [ ] No horizontal scrollbar at any width
- [ ] Toolbar doesn't break layout
- [ ] Footer stays at bottom

### Phase 4: Accessibility Audit (5 min)
Use `ui-ux-studio` skill → `references/accessibility.md`:
- [ ] Tab through all interactive elements
- [ ] Focus rings visible everywhere
- [ ] Toolbar buttons have accessible names
- [ ] Color contrast ≥4.5:1 for body text
- [ ] Textarea has accessible label

### Phase 5: Interaction Testing (5 min)
- [ ] Type text — preview updates in real-time
- [ ] Click each toolbar button — formatting inserts at cursor
- [ ] Character/line counter updates live
- [ ] Mermaid diagrams render (test with sample content)
- [ ] Copy buttons work and show feedback
- [ ] Links open correctly (GitHub, footer links)

### Phase 6: Edge Cases (3 min)
- [ ] Very long line (1000+ chars) — doesn't break layout
- [ ] Very long document (1000+ lines) — editor remains responsive
- [ ] Empty editor — preview shows empty state or nothing
- [ ] Invalid Mermaid syntax — shows error, doesn't crash preview
- [ ] Special characters (emoji, CJK) — render correctly
- [ ] Rapid typing — no lag or flicker

### Phase 7: Code Quality (3 min)
- [ ] No hardcoded UI colors — use Tailwind theme utilities or project CSS variables
- [ ] No inline styles in Rust `view!` macro (use CSS classes)
- [ ] All new components have `///` doc comments
- [ ] Components registered in `mod.rs`
- [ ] No Leptos warnings in console
- [ ] No Bootstrap classes, variables, JavaScript, or icon-font dependencies

## Common Issues & Fixes

| Issue | Fix |
|---|---|
| Border invisible in dark mode | Use project border tokens or Tailwind border utilities |
| Toolbar button has no accessible name | Add a visible label, `title`, or appropriate ARIA label |
| Preview flickers on type | Verify `Memo` is used for `rendered` signal |
| Mermaid doesn't render | Check `__mermaid` loaded in console |
| Copy button stuck on "Copied!" | Reset after 2 seconds |
| Horizontal scroll on mobile | Check responsive overflow utilities and focused project CSS |
| Focus ring missing | Remove `outline: none` styles and preserve visible focus indicators |

## Reporting

After review, produce a summary:
```
## Design Review — [Date]

### Status: ✅ Pass / ⚠️ Issues Found

### Issues Found:
1. [Issue description] — [Location] — [Severity: High/Med/Low]
2. ...

### Verified:
- Dark mode: ✅
- Responsive (375/576/768/1024/1280/1920): ✅
- Accessibility basics: ✅
- Interaction: ✅
```
