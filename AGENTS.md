# Agent Instructions — doanson44.github.io

All coding agents must follow `.agents/AGENTS.md` and the repository's `.github/copilot-instructions.md`. The skill-specific guidance in `.github/skills/` supplements those rules; it does not override them.

## Mandatory Icon Rule

- Use the shared SVG icon system: `src/components/icons.rs` (`Icon`, `IconName`) and `public/icons.svg`.
- Never use emoji, Unicode pictographs, arrows, dingbats, or decorative special characters as UI icons. Unicode in normal text and user-authored content is allowed.
- Do not introduce icon fonts, external icon CSS dependencies, duplicated inline SVG paths in feature pages, or parallel icon abstractions.
- Preserve Rust types during icon refactors: functions returning `&'static str` (such as `slug()` / `title()`) return string literals; functions returning `IconName` return enum variants. Inspect signatures before editing.
- New icons must update both the `IconName` / stable ID mapping and exactly one matching symbol in `public/icons.svg`. Use unique IDs and `currentColor`.
- Icon-only controls require accessible names and visible keyboard focus.

## Engineering Guardrails

- Preserve Rust 2021, Leptos 0.7 CSR/WASM, Tailwind CSS 4, Trunk, GitHub Pages, and the Clean Architecture boundaries described in `.agents/AGENTS.md`.
- Keep changes scoped. Do not make unrelated refactors.
- Do not claim a build, test, or static check passed unless it was actually run. Report unavailable or unrun checks explicitly.
- Follow the project's no-self-commit rule; do not create commits unless the user explicitly authorizes committing.
