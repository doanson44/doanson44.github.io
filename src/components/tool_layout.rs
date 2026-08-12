use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

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
}

/// Reusable responsive workspace for two tool panels and a draggable divider.
///
/// On desktop the panels are arranged horizontally. On mobile CSS switches the
/// workspace to a vertical layout, while the divider automatically uses the
/// corresponding pointer axis.
#[component]
pub fn ToolSplit(
    #[prop(default = 50)] initial_ratio: u32,
    children: Children,
) -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let ratio = RwSignal::new(initial_ratio.clamp(20, 80));

    provide_context(ToolSplitContext {
        container_ref,
        ratio,
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
pub fn ToolPanel(
    side: ToolPanelSide,
    children: Children,
) -> impl IntoView {
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
        let Some(target) = ev.current_target() else {
            return;
        };
        let Ok(target) = target.dyn_into::<HtmlElement>() else {
            return;
        };
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };

        let ratio = context.ratio;
        let container_ref = context.container_ref;
        let is_vertical = web_sys::window()
            .and_then(|window| window.match_media("(max-width: 767.98px)").ok())
            .map(|media| media.matches())
            .unwrap_or(false);

        let on_move = move |move_ev: PointerEvent| {
            let Some(container) = container_ref.get() else {
                return;
            };
            let rect = container.get_bounding_client_rect();
            let (offset, size) = if is_vertical {
                (
                    move_ev.client_y() as f64 - rect.top(),
                    rect.height(),
                )
            } else {
                (
                    move_ev.client_x() as f64 - rect.left(),
                    rect.width(),
                )
            };

            if size > 0.0 {
                ratio.set(((offset / size) * 100.0).clamp(20.0, 80.0) as u32);
            }
        };

        let on_up = move |_: PointerEvent| {};

        let on_move_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_move) as Box<dyn FnMut(PointerEvent)>
        );
        let on_up_cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(on_up) as Box<dyn FnMut(PointerEvent)>
        );

        let _ = body.add_event_listener_with_callback(
            "pointermove",
            on_move_cb.as_ref().unchecked_ref(),
        );
        let _ = body.add_event_listener_with_callback(
            "pointerup",
            on_up_cb.as_ref().unchecked_ref(),
        );

        let _ = target.set_pointer_capture(ev.pointer_id());
        on_move_cb.forget();
        on_up_cb.forget();
    };

    view! {
        <div
            class="tool-divider"
            role="separator"
            aria-label="Resize tool panels"
            tabindex="0"
            on:pointerdown=on_pointer_down
        ></div>
    }
}
