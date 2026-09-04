use leptos::prelude::*;

/// Shared platform footer component.
///
/// Displays platform branding, tech stack links, and privacy notice.
/// Used across all pages via the platform shell.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer flex items-center justify-between border-t border-[var(--border-color)] px-3 py-1">
            <span class="text-xs text-[var(--text-secondary)]">
                "doanson44.github.io · Powered by "
                <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer" class="no-underline hover:underline">"Rust"</a>
                " + "
                <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer" class="no-underline hover:underline">"Leptos"</a>
                " + "
                <a href="https://webassembly.org/" target="_blank" rel="noopener noreferrer" class="no-underline hover:underline">"WebAssembly"</a>
            </span>
            <span class="text-xs text-[var(--text-secondary)]">
                <span aria-hidden="true">"🔒"</span>
                " 100% client-side · No data leaves your browser"
            </span>
        </footer>
    }
}
