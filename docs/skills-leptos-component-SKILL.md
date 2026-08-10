# Leptos Component Skill â€” doanson44.github.io

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
    // Derived signals
    let derived = Memo::new(move |_| {
        let val = some_signal.get();
        // transformation...
        val
    });

    // DOM refs
    let container_ref = NodeRef::<Div>::new();

    // Effects
    Effect::new(move |_| {
        let _ = derived.get();
        // side effect when derived changes
    });

    // Event handlers
    let on_click = move |ev: leptos::ev::MouseEvent| {
        // handle event
    };

    view! {
        <div class="component-root" node_ref=container_ref>
            <div class="panel-header">
                <span class="panel-title">
                    <i class="bi bi-icon-name me-2 text-primary"></i>
                    "Title"
                </span>
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
| Input event â†’ value | `event_target_value(&ev)` |
| Conditional rendering | `{move \|\| if condition { view! {...} } else { view! {...} }}` |
| Class binding | `class=("class-name", move \|\| condition)` |

## Bootstrap 5 Styling Conventions

- Dark theme: Bootstrap dark mode is enabled via `data-bs-theme="dark"` on `<html>`
- Use Bootstrap utility classes: `d-flex`, `flex-grow-1`, `gap-2`, `p-2`, `border`, `border-secondary`
- Borders between panels: class `border-secondary`
- Text colors: `text-body-secondary`, `text-primary`
- Icons: Bootstrap Icons via `<i class="bi bi-xxx"></i>`
