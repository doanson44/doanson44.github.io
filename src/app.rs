use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::features::cv::page::CvPage;
use crate::features::games::page::GamesPage;
use crate::features::home::page::HomePage;
use crate::features::socket::page::SocketPage;
use crate::features::tools::json::page::JsonPage;
use crate::features::tools::jwt::page::JwtPage;
use crate::features::tools::markdown::page::MarkdownPage;
use crate::features::tools::page::ToolsPage;

/// Platform shell with hash-based routing for GitHub Pages compatibility.
#[component]
pub fn App() -> impl IntoView {
    let current_hash = create_hash_signal();

    view! {
        <div class="app-container d-flex flex-column vh-100" id="app">
            <Navbar />
            <main class="flex-grow-1 d-flex overflow-hidden" style="min-height: 0;">
                {move || render_page(current_hash.get())}
            </main>
            <Footer />
        </div>
    }
}

/// Create a reactive signal that tracks `window.location.hash`.
fn create_hash_signal() -> RwSignal<String> {
    let initial = window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
        .trim_start_matches('#')
        .to_string();

    let hash = RwSignal::new(if initial.is_empty() {
        "/".into()
    } else {
        initial
    });

    let hash_clone = hash;
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        if let Some(win) = window() {
            if let Ok(loc) = win.location().hash() {
                let h = loc.trim_start_matches('#').to_string();
                hash_clone.set(if h.is_empty() { "/".into() } else { h });
            }
        }
    }) as Box<dyn FnMut()>);

    if let Some(win) = window() {
        let _ =
            win.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
    }
    closure.forget();

    hash
}

/// Render the appropriate page component based on the current route.
fn render_page(route: String) -> leptos::prelude::AnyView {
    match route.as_str() {
        "/" => view! { <HomePage /> }.into_any(),
        "/tools" => view! { <ToolsPage /> }.into_any(),
        "/tools/markdown" => view! { <MarkdownPage /> }.into_any(),
        "/tools/json" => view! { <JsonPage /> }.into_any(),
        "/tools/jwt" => view! { <JwtPage /> }.into_any(),
        "/games" => view! { <GamesPage /> }.into_any(),
        "/cv" => view! { <CvPage /> }.into_any(),
        "/socket" => view! { <SocketPage /> }.into_any(),
        _ => view! {
            <div class="container py-5 text-center flex-grow-1">
                <h3 class="text-body-secondary">"404"</h3>
                <p class="text-body-tertiary">"Page not found."</p>
                <a href="#/" class="btn btn-outline-secondary btn-sm">"Go Home"</a>
            </div>
        }
        .into_any(),
    }
}
