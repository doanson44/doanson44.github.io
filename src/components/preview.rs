use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::domain::markdown::{RenderSegment, RenderedMarkdown};
use crate::infrastructure::browser::{copy_preview_as_html, copy_svg_as_png, copy_to_clipboard};
use crate::infrastructure::mermaid::{render_mermaid, MermaidResult};

/// Markdown preview component.
#[component]
pub fn Preview(rendered: Memo<RenderedMarkdown>) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let copy_status = RwSignal::new("Copy for Word");

    Effect::new(move |_| {
        let _ = rendered.get();
        if let Some(container) = container_ref.get() {
            leptos::task::spawn_local(async move {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let window = web_sys::window().unwrap();
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 50)
                        .unwrap();
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                let doc = web_sys::window().unwrap().document().unwrap();
                if let Ok(pres) = container.query_selector_all("pre") {
                    for i in 0..pres.length() {
                        if let Some(pre) = pres.item(i) {
                            if let Ok(pre_el) = pre.dyn_into::<Element>() {
                                if pre_el.query_selector(".copy-btn").unwrap_or(None).is_none() {
                                    let btn = doc.create_element("button").unwrap();
                                    btn.set_class_name("copy-btn rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-primary)] hover:bg-[var(--surface-hover)]");
                                    btn.set_text_content(Some("Copy"));
                                    let _ = pre_el.append_child(&btn);
                                }
                            }
                        }
                    }
                }
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
                                    btn.set_class_name("copy-btn rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-primary)] hover:bg-[var(--surface-hover)]");
                                    btn.set_text_content(Some("Copy"));
                                    let _ = table_el.append_child(&btn);
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let on_click = move |ev: leptos::ev::MouseEvent| {
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<Element>() {
                if let Ok(Some(btn_el)) = el.closest(".copy-btn") {
                    if let Ok(btn) = btn_el.dyn_into::<HtmlElement>() {
                        if let Some(parent) = btn.parent_element() {
                            let tag = parent.tag_name().to_lowercase();
                            let text = if tag == "pre" {
                                if let Ok(Some(code_el)) = parent.query_selector("code") {
                                    code_el.unchecked_into::<HtmlElement>().inner_text()
                                } else {
                                    parent.unchecked_into::<HtmlElement>().inner_text().replace(" Copy", "")
                                }
                            } else if tag == "table" {
                                parent.unchecked_into::<HtmlElement>().inner_text().replace(" Copy", "")
                            } else {
                                String::new()
                            };
                            if !text.is_empty() {
                                leptos::task::spawn_local(async move {
                                    if copy_to_clipboard(&text).await.is_ok() {
                                        let _ = btn.class_list().add_1("copied");
                                        btn.set_text_content(Some("Copied"));
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            let window = web_sys::window().unwrap();
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve, 2000,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        let _ = btn.class_list().remove_1("copied");
                                        btn.set_text_content(Some("Copy"));
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    };

    let on_copy_for_word = move |_| {
        if copy_status.get_untracked() == "Copying..." {
            return;
        }
        copy_status.set("Copying...");
        leptos::task::spawn_local(async move {
            if copy_preview_as_html("markdown-preview-content")
                .await
                .is_ok()
            {
                copy_status.set("Copied");
            } else {
                copy_status.set("Copy failed");
            }
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let window = web_sys::window().unwrap();
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000)
                    .unwrap();
            });
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            copy_status.set("Copy for Word");
        });
    };

    view! {
        <div class="preview-panel flex h-full flex-col" id="preview-panel">
            <div class="panel-header flex items-center border-b border-[var(--border-color)] px-3 py-2">
                <span class="panel-title">"Preview"</span>
                <button type="button" class="ml-auto rounded-md border border-[var(--accent)] px-3 py-1.5 text-sm font-medium text-[var(--accent)] hover:bg-[var(--accent)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title="Copy preview for Word" aria-label="Copy preview for Word" on:click=on_copy_for_word>
                    {move || copy_status.get()}
                </button>
            </div>
            <div class="preview-content custom-scrollbar flex-grow overflow-auto p-3" id="markdown-preview-content" node_ref=container_ref on:click=on_click>
                {move || {
                    let rendered = rendered.get();
                    rendered.segments.into_iter().enumerate().map(|(idx, segment)| match segment {
                        RenderSegment::Html(html) => view! { <div class="markdown-body" inner_html=html></div> }.into_any(),
                        RenderSegment::Mermaid(block) => view! { <MermaidDiagram id=block.id code=block.code _key=idx /> }.into_any(),
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[component]
fn MermaidDiagram(id: String, code: String, #[prop(optional)] _key: usize) -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let id_clone = id.clone();
    let code_for_data = code.clone();
    let mermaid_result = LocalResource::new(move || {
        let id = id_clone.clone();
        let code = code.clone();
        async move { render_mermaid(&id, &code).await }
    });

    view! {
        <div class="mermaid-container my-3" node_ref=container_ref data-mermaid-code=code_for_data>
            <Suspense fallback=move || view! {
                <div class="mermaid-loading flex items-center justify-center p-4">
                    <span class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-[var(--accent)] border-t-transparent" role="status"></span>
                    <span class="text-sm text-[var(--text-secondary)]">"Rendering diagram..."</span>
                </div>
            }>
                {move || mermaid_result.get().map(move |result| match &*result {
                    MermaidResult::Success(svg) => {
                        let svg = svg.clone();
                        let copy_image = move |_| {
                            if let Some(container) = container_ref.get() {
                                if let Ok(Some(btn)) = container
                                    .clone()
                                    .unchecked_into::<Element>()
                                    .query_selector(".copy-btn")
                                {
                                    let btn_el = btn.unchecked_into::<HtmlElement>();
                                    leptos::task::spawn_local(async move {
                                        if let Ok(Some(svg_node)) = container
                                            .unchecked_into::<Element>()
                                            .query_selector("svg")
                                        {
                                            if let Some(svg_id) = svg_node.get_attribute("id") {
                                                if copy_svg_as_png(&svg_id).await.is_ok() {
                                                    let _ = btn_el.class_list().add_1("copied");
                                                    btn_el.set_text_content(Some("Copied"));
                                                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                                                        let window = web_sys::window().unwrap();
                                                        window
                                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                                &resolve, 2000,
                                                            )
                                                            .unwrap();
                                                    });
                                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                                    let _ = btn_el.class_list().remove_1("copied");
                                                    btn_el.set_text_content(Some("Copy Image"));
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        };
                        view! {
                            <>
                                <button class="copy-btn rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-2 py-1 text-xs text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=copy_image>"Copy Image"</button>
                                <div class="mermaid-diagram p-3 text-center" inner_html=svg></div>
                            </>
                        }.into_any()
                    }
                    MermaidResult::Error(err) => {
                        let err = err.clone();
                        view! { <div class="mermaid-error flex items-start gap-2 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-[var(--danger)]" role="alert"><div><strong>"Mermaid diagram error"</strong><p class="mb-0 mt-1 font-mono text-sm">{err}</p></div></div> }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
