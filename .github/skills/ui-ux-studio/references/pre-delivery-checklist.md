# Pre-Delivery Checklist — Markdown Studio

Run through this checklist before finalizing any UI change.

## Accessibility (CRITICAL)

- [ ] All toolbar buttons have `title` attribute for screen readers
- [ ] Textarea has accessible label (via `aria-label` or associated `<label>`)
- [ ] Focus rings visible on all interactive elements (keyboard-only users)
- [ ] Color contrast ≥4.5:1 for all text (use [WebAIM checker](https://webaim.org/resources/contrastchecker/))
- [ ] Mermaid SVG diagrams have `role="img"` and alt text
- [ ] No information conveyed by color alone

## Dark Mode Quality

- [ ] No pure black (`#000`) backgrounds anywhere
- [ ] Borders use `border-secondary` class (visible on dark bg)
- [ ] Placeholder text has adequate contrast
- [ ] Copy/overlay buttons visible against code blocks
- [ ] Badge text readable with `text-primary-emphasis`
- [ ] Links distinguishable from body text

## Layout & Responsive

- [ ] Split-pane layout works at ≥1024px (side by side)
- [ ] Split-pane stacks vertically at <768px (mobile)
- [ ] Toolbar scrolls horizontally on narrow screens
- [ ] Editor font is 16px on mobile (prevents iOS zoom)
- [ ] No horizontal scrollbar at any viewport width
- [ ] Textarea fills available height, not fixed height
- [ ] Copy buttons visible on mobile (no hover required)
- [ ] Tables scroll horizontally on mobile
- [ ] Code blocks wrap on mobile
- [ ] Preview scrolls independently of editor
- [ ] Footer stays at bottom, stacks on mobile
- [ ] Navbar collapses correctly at 768px, dropdown works on mobile
- [ ] Home page cards 1-col mobile, 2-col tablet, 3-col desktop

## Interaction & Feedback

- [ ] Toolbar buttons insert formatting at cursor position (not just append)
- [ ] Character count and line count update in real-time
- [ ] Mermaid diagrams show loading state while rendering
- [ ] Mermaid errors show inline error message (don't crash preview)
- [ ] Copy buttons show feedback (icon change, toast, or brief text change)
- [ ] No 0ms state changes — transitions ≥150ms for visual feedback

## Typography

- [ ] Editor uses monospace font
- [ ] Preview uses readable serif or sans-serif
- [ ] Base font size ≥14px (never below 12px)
- [ ] Line height ≥1.5 for prose text
- [ ] Heading hierarchy clear in preview (h1 → h2 → h3)
- [ ] Code blocks in preview use monospace with visible background

## Consistency

- [ ] All panels use same header pattern (`panel-header` class)
- [ ] All borders use `border-secondary` consistently
- [ ] Icon sizes consistent (use `fs-*` or fixed dimensions)
- [ ] Spacing follows scale: `px-3 py-2` (panels), `gap-2` (toolbar), `p-1` (footer)
- [ ] Bootstrap variables used instead of hardcoded colors

## Performance (WASM-specific)

- [ ] Preview re-render is debounced (memo-based in Leptos, auto)
- [ ] Mermaid diagrams render lazily (only visible ones)
- [ ] No unnecessary WASM ↔ JS round-trips
- [ ] Textarea doesn't lag on large documents (>10K lines)

## Final Visual Check

- [ ] Editor and preview text vertically aligned at top
- [ ] Panel borders look intentional, not accidental
- [ ] Empty state looks intentional (placeholder text, not blank)
- [ ] Footer info is accurate (copyright, tech stack, privacy claim)
