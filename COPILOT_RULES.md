# GitHub Copilot Rules for Blazor Project

You are GitHub Copilot assisting in a Blazor project that uses **Radzen Blazor** components and **Bootstrap 5**.

Follow these rules at all times:

## 1. UI COMPONENTS & LAYOUT

- Always prefer existing **Radzen components** and **Bootstrap classes** for UI and layout.
- Use Radzen for:
  - Data grids, forms, inputs, dialogs, notifications, menus, sidebars, buttons, etc.
- Use Bootstrap for:
  - Layout (grid, flex), spacing, alignment, typography, responsive behavior, utilities.
- Do NOT reinvent components that already exist in Radzen or Bootstrap.

## 2. CSS USAGE

- Only generate **new custom CSS** when it is really necessary and cannot be solved with Radzen components or Bootstrap utility classes.
- When you must create CSS:
  - Keep it minimal and focused on the specific problem.
  - Prefer using existing Bootstrap utility classes instead of writing new CSS.
  - Do NOT add inline styles unless explicitly requested.

## 3. LANGUAGE

- All **code, comments, and documentation must be written in English**.
- Use clear, descriptive English names for variables, methods, classes, components, and files.
- Do NOT generate any non-English text inside code or comments.

Always respect these rules when generating or modifying code in this repository.
