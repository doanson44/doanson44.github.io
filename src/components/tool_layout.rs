use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

/// Identifies which side of a responsive tool split a panel occupies.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToolPanelSide {
    First,
    Second,
}

#[derive(Clone, Copy)]
struct ToolSplitContext {
    container_ref: NodeRef<leptos::html::Div>,
    ratio: RwSignal<u32>,
    dragging: RwSignal<bool>,
}

/// Reusable responsive workspace for two tool panels and a draggable divider.
///
/// On desktop the panels are arranged horizontally. On mobile CSS switches the
/// workspace to a vertical layout, while the divider automatically uses the
/// corresponding pointer axis.
#[component]
pub fn ToolSplit(#[prop(default = 50)] initial_ratio: u32, children: Children) -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let ratio = RwSignal::new(initial_ratio.clamp(20, 80));
    let dragging = RwSignal::new(false);

    provide_context(ToolSplitContext {
        container_ref,
        ratio,
        dragging,
    });

    Effect::new(move |_| {
        let Some(container) = container_ref.get() else {
            return;
        };

        let ratio = ratio.get();
        let style = container.style();
        let _ = style.set_property("--tool-first-size", &format!("{}%", ratio));
        let _ = style.set_property("--tool-second-size", &format!("{}%", 100 - ratio));
    });

    view! {
        <div class="tool-split" node_ref=container_ref>
            {children()}
        </div>
    }
}

/// Reusable panel wrapper used inside [`ToolSplit`].
#[component]
pub fn ToolPanel(side: ToolPanelSide, children: Children) -> impl IntoView {
    let side_class = match side {
        ToolPanelSide::First => "tool-panel-first",
        ToolPanelSide::Second => "tool-panel-second",
    };

    view! {
        <div class=format!("tool-panel {side_class}")>
            {children()}
        </div>
    }
}

/// Reusable draggable divider for [`ToolSplit`].
#[component]
pub fn ToolDivider() -> impl IntoView {
    let context = expect_context::<ToolSplitContext>();

    let on_pointer_down = move |ev: PointerEvent| {
        ev.prevent_default();
        context.dragging.set(true);

        if let Some(target) = ev
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }
    };

    let on_pointer_move = move |ev: PointerEvent| {
        if !context.dragging.get_untracked() {
            return;
        }

        let Some(container) = context.container_ref.get() else {
            return;
        };

        let rect = container.get_bounding_client_rect();
        let is_vertical = web_sys::window()
            .and_then(|window| window.match_media("(max-width: 767.98px)").ok())
            .map(|media| media.matches())
            .unwrap_or(false);

        let (offset, size) = if is_vertical {
            (ev.client_y() as f64 - rect.top(), rect.height())
        } else {
            (ev.client_x() as f64 - rect.left(), rect.width())
        };

        if size > 0.0 {
            context
                .ratio
                .set(((offset / size) * 100.0).clamp(20.0, 80.0) as u32);
        }
    };

    let on_pointer_up = move |ev: PointerEvent| {
        context.dragging.set(false);
        if let Some(target) = ev
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = target.release_pointer_capture(ev.pointer_id());
        }
    };

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        let is_vertical = web_sys::window()
            .and_then(|window| window.match_media("(max-width: 767.98px)").ok())
            .map(|media| media.matches())
            .unwrap_or(false);

        let delta = match (is_vertical, ev.key().as_str()) {
            (false, "ArrowLeft") => Some(-5i32),
            (false, "ArrowRight") => Some(5i32),
            (true, "ArrowUp") => Some(-5i32),
            (true, "ArrowDown") => Some(5i32),
            _ => None,
        };

        if let Some(delta) = delta {
            ev.prevent_default();
            let next = (context.ratio.get_untracked() as i32 + delta).clamp(20, 80) as u32;
            context.ratio.set(next);
        }
    };

    view! {
        <div
            class="tool-divider"
            role="separator"
            aria-label="Resize tool panels"
            tabindex="0"
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointercancel=move |_| context.dragging.set(false)
            on:keydown=on_key_down
        ></div>
    }
}
