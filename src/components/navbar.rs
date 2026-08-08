use leptos::prelude::*;

/// Top navigation bar component.
///
/// Displays the application title and branding.
/// Uses Bootstrap 5 navbar classes with dark theme.
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar navbar-expand-lg border-bottom border-secondary" id="main-navbar">
            <div class="container-fluid">
                <a class="navbar-brand d-flex align-items-center gap-2" href="#">
                    <i class="bi bi-markdown-fill fs-3 text-primary"></i>
                    <span class="fw-bold">"Markdown Studio"</span>
                </a>
                <div class="d-flex align-items-center gap-3">
                    <span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle">
                        <i class="bi bi-cpu me-1"></i>
                        "WASM"
                    </span>
                    <a
                        href="https://github.com/doanson44/doanson44.github.io"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-outline-secondary btn-sm d-flex align-items-center gap-1"
                        title="View on GitHub"
                    >
                        <i class="bi bi-github"></i>
                        <span class="d-none d-md-inline">"GitHub"</span>
                    </a>
                </div>
            </div>
        </nav>
    }
}
