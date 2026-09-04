use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::domain::developer::ToolId;
use crate::domain::finance::FinanceTool;
use crate::features::cv::page::CvPage;
use crate::features::games::page::GamesPage;
use crate::features::home::page::HomePage;
use crate::features::socket::page::SocketPage;
use crate::features::tools::base64::page::Base64Page;
use crate::features::tools::developer::page::DeveloperToolPage;
use crate::features::tools::finance::page::FinancePage;
use crate::features::tools::json::page::JsonPage;
use crate::features::tools::jwt::page::JwtPage;
use crate::features::tools::markdown::page::MarkdownPage;
use crate::features::tools::page::ToolsPage;
use crate::features::tools::time::page::TimePage;
use crate::infrastructure::funding_rate::FundingRateApi;
use crate::infrastructure::mexc_futures::MexcFuturesStream;

#[component]
pub fn App() -> impl IntoView {
    let current_hash = create_hash_signal();
    view! {
        <div class="app-container flex min-h-screen flex-col" id="app">
            <Navbar />
            <main class="app-main min-h-0 flex flex-1 overflow-auto">
                {move || render_page(current_hash.get())}
            </main>
            <Footer />
        </div>
    }
}

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

fn render_page(route: String) -> leptos::prelude::AnyView {
    if let Some(slug) = route.strip_prefix("/tools/finance/") {
        if let Some(tool) = FinanceTool::from_route(slug) {
            return view! { <FinancePage tool=tool /> }.into_any();
        }
    }
    if let Some(slug) = route.strip_prefix("/tools/") {
        if let Some(tool) = ToolId::from_route(slug) {
            return view! { <DeveloperToolPage tool=tool /> }.into_any();
        }
    }
    match route.as_str() {
        "/" => view! { <HomePage /> }.into_any(),
        "/tools" => view! { <ToolsPage /> }.into_any(),
        "/tools/markdown" => view! { <MarkdownPage /> }.into_any(),
        "/tools/json" => view! { <JsonPage /> }.into_any(),
        "/tools/jwt" => view! { <JwtPage /> }.into_any(),
        "/tools/base64" => view! { <Base64Page /> }.into_any(),
        "/tools/time" => view! { <TimePage /> }.into_any(),
        "/games" => view! { <GamesPage /> }.into_any(),
        "/cv" => view! { <CvPage /> }.into_any(),
        "/socket" => view! {
            <SocketPage
                stream=Rc::new(MexcFuturesStream)
                funding_provider=Rc::new(FundingRateApi)
            />
        }.into_any(),
        _ => view! {
            <div class="flex flex-1 flex-col items-center px-4 py-20 text-center">
                <h3 class="text-xl font-semibold text-[var(--text-secondary)]">"404"</h3>
                <p class="mt-2 text-sm text-[var(--text-tertiary)]">"Page not found."</p>
                <a href="#/" class="mt-4 inline-flex items-center rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]">"Go Home"</a>
            </div>
        }.into_any(),
    }
}
