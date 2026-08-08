use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::domain::markdown::{RenderSegment, RenderedMarkdown};
use crate::infrastructure::browser::{copy_svg_as_png, copy_to_clipboard};
use crate::infrastructure::mermaid::{render_mermaid, MermaidResult};

/// Markdown preview component.
#[component]
pub fn Preview(rendered: Memo<RenderedMarkdown>) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();

    // Effect to inject copy buttons into raw HTML blocks after render
    Effect::new(move |_| {
        // Depend on rendered to re-run when content changes
        let _ = rendered.get();

        if let Some(container) = container_ref.get() {
            // Need a small timeout to let the DOM update from inner_html first
            leptos::task::spawn_local(async move {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let window = web_sys::window().unwrap();
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 50)
                        .unwrap();
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                let doc = web_sys::window().unwrap().document().unwrap();

                // Inject into <pre> blocks
                if let Ok(pres) = container.query_selector_all("pre") {
                    for i in 0..pres.length() {
                        if let Some(pre) = pres.item(i) {
                            if let Ok(pre_el) = pre.dyn_into::<Element>() {
                                if pre_el.query_selector(".copy-btn").unwrap_or(None).is_none() {
                                    let btn = doc.create_element("button").unwrap();
                                    btn.set_class_name("copy-btn btn btn-sm btn-dark");
                                    btn.set_inner_html("<i class=\"bi bi-clipboard\"></i> Copy");
                                    let _ = pre_el.append_child(&btn);
                                }
                            }
                        }
                    }
                }

                // Inject into <table> blocks
                if let Ok(tables) = container.query_selector_all("table") {
                    for i in 0..tables.length() {
                        if let Some(table) = tables.item(i) {
                            if let Ok(table_el) = table.dyn_into::<Element>() {
                                if table_el
                                    .query_selector(".copy-btn")
                                    .unwrap_or(None)
                                    .is_none()
                                {
                                    let btn = doc.create_element("button").unwrap();
                                    btn.set_class_name("copy-btn btn btn-sm btn-dark");
                                    btn.set_inner_html("<i class=\"bi bi-clipboard\"></i> Copy");
                                    let _ = table_el.append_child(&btn);
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    // Event delegation for raw HTML copy buttons
    let on_click = move |ev: leptos::ev::MouseEvent| {
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<Element>() {
                if let Ok(Some(btn_el)) = el.closest(".copy-btn") {
                    if let Ok(btn) = btn_el.dyn_into::<HtmlElement>() {
                        if let Some(parent) = btn.parent_element() {
                            let tag = parent.tag_name().to_lowercase();
                            let text = if tag == "pre" {
                                // For pre, get the code text, excluding the button's text
                                if let Ok(Some(code_el)) = parent.query_selector("code") {
                                    code_el.unchecked_into::<HtmlElement>().inner_text()
                                } else {
                                    parent
                                        .unchecked_into::<HtmlElement>()
                                        .inner_text()
                                        .replace(" Copy", "")
                                }
                            } else if tag == "table" {
                                parent
                                    .unchecked_into::<HtmlElement>()
                                    .inner_text()
                                    .replace(" Copy", "")
                            } else {
                                String::new()
                            };

                            if !text.is_empty() {
                                leptos::task::spawn_local(async move {
                                    if copy_to_clipboard(&text).await.is_ok() {
                                        let _ = btn.class_list().add_1("copied");
                                        btn.set_inner_html("<i class=\"bi bi-check2\"></i> Copied");

                                        // Reset after 2s
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            let window = web_sys::window().unwrap();
                                            window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        let _ = btn.class_list().remove_1("copied");
                                        btn.set_inner_html(
                                            "<i class=\"bi bi-clipboard\"></i> Copy",
                                        );
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    };

    view! {
        <div class="preview-panel d-flex flex-column h-100" id="preview-panel">
            <div class="panel-header d-flex align-items-center px-3 py-2 border-bottom border-secondary">
                <span class="panel-title">
                    <i class="bi bi-eye me-2 text-success"></i>
                    "Preview"
                </span>
            </div>
            <div class="preview-content flex-grow-1 p-3 overflow-auto custom-scrollbar" node_ref=container_ref on:click=on_click>
                {move || {
                    let rendered = rendered.get();
                    rendered.segments.into_iter().enumerate().map(|(idx, segment)| {
                        match segment {
                            RenderSegment::Html(html) => {
                                view! {
                                    <div class="markdown-body" inner_html=html></div>
                                }.into_any()
                            }
                            RenderSegment::Mermaid(block) => {
                                view! {
                                    <MermaidDiagram id=block.id code=block.code _key=idx />
                                }.into_any()
                            }
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

/// Component for rendering a single Mermaid diagram.
#[component]
fn MermaidDiagram(id: String, code: String, #[prop(optional)] _key: usize) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let id_clone = id.clone();

    let mermaid_result = LocalResource::new(move || {
        let id = id_clone.clone();
        let code = code.clone();
        async move { render_mermaid(&id, &code).await }
    });

    view! {
        <div class="mermaid-container my-3" node_ref=container_ref>
            <Suspense fallback=move || view! {
                <div class="mermaid-loading d-flex align-items-center justify-content-center p-4">
                    <div class="spinner-border spinner-border-sm text-primary me-2" role="status">
                        <span class="visually-hidden">"Loading..."</span>
                    </div>
                    <span class="text-body-secondary">"Rendering diagram..."</span>
                </div>
            }>
                {move || {
                    mermaid_result.get().map(move |result| {
                        match &*result {
                            MermaidResult::Success(svg) => {
                                let svg = svg.clone();

                                // Copy SVG as image function
                                let copy_image = move |_| {
                                    if let Some(container) = container_ref.get() {
                                        if let Ok(Some(btn)) = container.clone().unchecked_into::<Element>().query_selector(".copy-btn") {
                                            let btn_el = btn.unchecked_into::<HtmlElement>();
                                            leptos::task::spawn_local(async move {
                                                if let Ok(Some(svg_node)) = container.unchecked_into::<Element>().query_selector("svg") {
                                                    if let Some(svg_id) = svg_node.get_attribute("id") {
                                                        if copy_svg_as_png(&svg_id).await.is_ok() {
                                                            let _ = btn_el.class_list().add_1("copied");
                                                            btn_el.set_inner_html("<i class=\"bi bi-check2\"></i> Copied");

                                                            let promise = js_sys::Promise::new(&mut |resolve, _| {
                                                                let window = web_sys::window().unwrap();
                                                                window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
                                                            });
                                                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                                            let _ = btn_el.class_list().remove_1("copied");
                                                            btn_el.set_inner_html("<i class=\"bi bi-image\"></i> Copy Image");
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }
                                };

                                view! {
                                    <>
                                        <button class="copy-btn btn btn-sm btn-dark" on:click=copy_image>
                                            <i class="bi bi-image"></i> " Copy Image"
                                        </button>
                                        <div class="mermaid-diagram text-center p-3" inner_html=svg></div>
                                    </>
                                }.into_any()
                            }
                            MermaidResult::Error(err) => {
                                let err = err.clone();
                                view! {
                                    <div class="mermaid-error alert alert-danger d-flex align-items-start gap-2" role="alert">
                                        <i class="bi bi-exclamation-triangle-fill flex-shrink-0 mt-1"></i>
                                        <div>
                                            <strong>"Mermaid diagram error"</strong>
                                            <p class="mb-0 mt-1 small font-monospace">{err}</p>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
