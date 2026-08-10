use leptos::prelude::*;

/// Shared platform footer component.
///
/// Displays platform branding, tech stack links, and privacy notice.
/// Used across all pages via the platform shell.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer d-flex align-items-center justify-content-between px-3 py-1 border-top border-secondary">
            <span class="text-body-secondary small">
                "doanson44.github.io · Powered by "
                <a href="https://www.rust-lang.org/" target="_blank" rel="noopener" class="text-decoration-none">"Rust"</a>
                " + "
                <a href="https://leptos.dev/" target="_blank" rel="noopener" class="text-decoration-none">"Leptos"</a>
                " + "
                <a href="https://webassembly.org/" target="_blank" rel="noopener" class="text-decoration-none">"WebAssembly"</a>
            </span>
            <span class="text-body-secondary small">
                <i class="bi bi-shield-lock me-1"></i>
                "100% client-side · No data leaves your browser"
            </span>
        </footer>
    }
}
