# Leptos Component Skill — doanson44.github.io

## Component Creation Checklist

When creating a new Leptos component, follow these steps:

### 1. Determine the Layer

| If the component... | Place it in... |
|---|---|
| Is a UI widget/piece of layout | `src/components/<name>.rs` |
| Manages feature-specific reactive state | `src/features/<feature>/state.rs` |
| Wraps a browser API for use by components | `src/infrastructure/<module>/mod.rs` |

### 2. Create the Component File

For a new component in `src/components/`:
- Add `pub mod <name>;` to `src/components/mod.rs`
- No need to touch `src/lib.rs` (it only imports top-level modules)

### 3. Component Template

```rust
use leptos::prelude::*;

/// Description of what this component does.
#[component]
pub fn ComponentName(
    /// Prop documentation
    some_signal: RwSignal<String>,
    #[prop(default = "default-value")] optional_prop: &'static str,
) -> impl IntoView {
    let derived = Memo::new(move |_| {
        let val = some_signal.get();
        val
    });

    let container_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        let _ = derived.get();
    });

    let on_click = move |_ev: leptos::ev::MouseEvent| {
        // handle event
    };

    view! {
        <div class="component-root" node_ref=container_ref on:click=on_click>
            <div class="panel-header">
                <span class="panel-title">"Title"</span>
            </div>
            <div class="panel-body">
                {move || derived.get()}
            </div>
        </div>
    }
}
```

### 4. Wire into App

In `src/app.rs`, import and use the new component in the `view!` macro.

### 5. Register Module

In `src/components/mod.rs`:
```rust
pub mod <name>;
```

## Leptos 0.7 Patterns Reference

| Need | Pattern |
|---|---|
| Mutable state across components | `RwSignal<T>` passed as prop |
| Derived/computed value | `Memo::new(move \|_\| { ... })` |
| DOM element reference | `NodeRef::<Div>::new()` + `node_ref=ref` |
| Side effect on signal change | `Effect::new(move \|_\| { ... })` |
| Async in component | `leptos::task::spawn_local(async { ... })` |
| Input event → value | `event_target_value(&ev)` |
| Conditional rendering | `{move \|\| if condition { view! {...} } else { view! {...} }}` |
| Class binding | `class=("class-name", move \|\| condition)` |

## Tailwind CSS Styling Conventions

- Use Tailwind utilities directly in Leptos `view!` for layout, spacing, typography, controls, panels, borders, and responsive behavior.
- Use project semantic theme tokens through Tailwind arbitrary values when a project-specific color is required, for example `bg-[var(--surface)]`, `text-[var(--text-primary)]`, and `border-[var(--border-color)]`.
- Use focused project CSS classes only for behavior or styling that cannot reasonably be expressed with Tailwind utilities.
- Theme is controlled by `data-theme="dark|light"` on `<html>`.
- Do not introduce Bootstrap classes, Bootstrap variables, Bootstrap JavaScript, or icon-font dependencies.
- Do not use inline `style="..."` in Leptos views.
- Icon-only controls require an accessible name and visible focus state.
