---
name: leptos-component
description: "Create Leptos 0.7 components for doanson44.github.io following clean architecture, reactive state patterns, Tailwind CSS conventions, accessibility, and responsive design."
---

# Leptos Component Skill — doanson44.github.io

## Component Creation Checklist

### 1. Determine the Layer

| If the component... | Place it in... |
|---|---|
| Is a UI widget/piece of layout | `src/components/<name>.rs` |
| Manages feature-specific reactive state | `src/features/<feature>/state.rs` |
| Wraps a browser API for use by components | `src/infrastructure/<module>/mod.rs` |

### 2. Create the Component File

For a shared component, add `pub mod <name>;` to `src/components/mod.rs` and keep feature-specific behavior outside shared components.

### 3. Component Template

```rust
use leptos::prelude::*;

#[component]
pub fn ComponentName(some_signal: RwSignal<String>) -> impl IntoView {
    let derived = Memo::new(move |_| some_signal.get());
    let container_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        let _ = derived.get();
    });

    view! {
        <div class="flex flex-col" node_ref=container_ref>
            <div class="border-b border-[var(--border-color)] px-3 py-2">
                <span class="font-medium text-[var(--text-primary)]">"Title"</span>
            </div>
            <div class="flex-1">
                {move || derived.get()}
            </div>
        </div>
    }
}
```

### 4. Wire into App

Import and use the component from `src/app.rs` or the owning feature as appropriate.

### 5. Register Module

In `src/components/mod.rs`:

```rust
pub mod <name>;
```

## Leptos 0.7 Patterns

| Need | Pattern |
|---|---|
| Mutable state | `RwSignal<T>` |
| Derived value | `Memo::new(move |\_| { ... })` |
| DOM reference | `NodeRef::<Div>::new()` |
| Side effect | `Effect::new(move |\_| { ... })` |
| Async component work | `leptos::task::spawn_local(async { ... })` |
| Input value | `event_target_value(&ev)` |
| Conditional view | `{move || if condition { view! {...} } else { view! {...} }}` |
| Conditional class | `class=("class-name", move || condition)` |

## Tailwind CSS Conventions

- Use Tailwind utilities directly in `view!` for layout, spacing, typography, controls, cards, borders, and responsive behavior.
- Use project semantic tokens such as `var(--surface)`, `var(--border-color)`, `var(--text-primary)`, and `var(--accent)` through Tailwind arbitrary values when needed.
- Use focused project CSS classes only for behavior or complex visuals that are not practical as utilities.
- Never add Bootstrap classes, Bootstrap variables, Bootstrap JavaScript, or icon-font dependencies.
- Do not use inline `style="..."` in Leptos views.
- Interactive controls need visible focus states and appropriate accessible names.
- Verify layouts from 375px through 1920px.
