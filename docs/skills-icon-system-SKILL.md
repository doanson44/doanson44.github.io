# Icon System Skill — doanson44.github.io

Use this guidance whenever adding, replacing, reviewing, or debugging UI icons.

## Source of Truth

- Component and semantic enum: `src/components/icons.rs`
- SVG sprite: `public/icons.svg`
- Usage guide: `docs/icon-system.md`

Import with `use crate::components::icons::{Icon, IconName};` and render the shared `Icon` component.

## Mandatory Rules

- Use the shared `Icon` component and semantic `IconName` for UI icons.
- Never use emoji, Unicode pictographs, arrows, dingbats, or decorative special characters as UI icons. Normal text and user-authored content may still contain Unicode.
- Do not add icon fonts, external icon CSS dependencies, duplicated inline SVG paths in feature pages, or a parallel icon abstraction.
- Check signatures before editing mappings. Text metadata methods returning `&'static str` (for example `slug()` and `title()`) must return string literals. Icon accessors returning `IconName` must return enum variants. Do not mix these types.
- Adding an icon requires both an `IconName` / stable kebab-case ID mapping and exactly one matching `<symbol>` in `public/icons.svg`. Use unique IDs and monochrome `currentColor` symbols.
- Icon-only controls need an accessible name and visible focus. Decorative icons must not duplicate accessible labels.
- Review the diff for leftover UI glyphs, invalid mappings, missing/duplicate SVG IDs, and accidental text changes.

## Validation

Run available checks, including `cargo fmt --check`, `cargo check --target wasm32-unknown-unknown`, `cargo test`, `cargo clippy --target wasm32-unknown-unknown -- -D warnings`, and `trunk build --release`. State which checks passed, failed, or were not run. Never claim unexecuted validation passed.
