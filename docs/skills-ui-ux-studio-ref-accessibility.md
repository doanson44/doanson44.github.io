# Accessibility Guidelines â€” Markdown Studio

Adapted from WCAG 2.1 AA standards, focused on our editor + preview split-pane app.

## Priority 1: Must Have (CRITICAL)

### Color Contrast
- **Normal text**: 4.5:1 minimum contrast ratio
- **Large text** (â‰¥18px or â‰¥14px bold): 3:1 minimum
- Use [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/) to verify
- Bootstrap dark mode tokens generally pass AA, but verify custom colors

### Keyboard Navigation
- All interactive elements reachable via Tab
- Focus order follows visual order: Navbar â†’ Toolbar â†’ Editor â†’ Preview â†’ Footer
- No keyboard traps (user can always Tab away)
- Toolbar buttons: all individually focusable

### Focus Indicators
- Bootstrap default focus ring (`box-shadow: 0 0 0 0.25rem rgba(13,110,253,.25)`) is sufficient
- Never use `outline: none` without providing an alternative focus style
- Custom focus styles: minimum 2px visible outline with 3:1 contrast

### Screen Readers
- Toolbar icon-only buttons: MUST have `title` attribute or `aria-label`
- Textarea: associate with visible or hidden `<label>`
- Mermaid SVGs: add `role="img"` and `aria-label="Diagram: {diagram type}"`
- Status updates (char count, line count): use `aria-live="polite"` region

## Priority 2: Should Have (HIGH)

### Touch Targets
- Minimum 44Ã—44px for all interactive elements (WCAG 2.5.5)
- Toolbar buttons: ensure sufficient padding for touch
- Copy buttons on code blocks: large enough to tap on mobile

### Semantic HTML in Preview
- Rendered Markdown uses proper heading hierarchy (h1â†’h6)
- Lists use `<ul>/<ol>/<li>`
- Tables have `<th>` with scope attributes
- Code blocks: `<pre><code>` with language class

### ARIA Landmarks (in `index.html`)
```html
<nav role="navigation" aria-label="Main navigation">
<main role="main">
<footer role="contentinfo">
```

## Priority 3: Nice to Have (MEDIUM)

### Reduced Motion
- Respect `prefers-reduced-motion` media query
- Mermaid render transitions: disable or simplify when reduced motion preferred
- Any custom animations: gate behind `@media (prefers-reduced-motion: no-preference)`

### Error Identification
- Mermaid parse errors: show as text, not just missing diagram
- Copy failures: show visible error feedback
- Don't rely on color alone to indicate errors

### Text Resize
- Page should work at 200% zoom without horizontal scroll
- Text should wrap, not truncate, when zoomed
- Preview content should reflow

## Testing Tools

| Tool | Purpose |
|------|---------|
| [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/) | Verify color contrast |
| [axe DevTools](https://www.deque.com/axe/) | Automated accessibility audit |
| Keyboard-only navigation | Tab through entire app |
| Screen reader (NVDA/VoiceOver) | Test toolbar + editor flow |
| Browser zoom 200% | Verify layout holds |
