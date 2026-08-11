use leptos::prelude::*;

use crate::infrastructure::browser::toggle_theme_js;

/// Platform navigation bar with hash-based routing for GitHub Pages.
///
/// All links use `href="#/path"` format for client-side hash routing.
/// Includes navigation for Home, Tools (dropdown), Games, CV, and Socket.
#[component]
pub fn Navbar() -> impl IntoView {
    let on_toggle_theme = move |_| {
        toggle_theme_js();
    };

    view! {
        <nav class="navbar navbar-expand-md border-bottom border-secondary" id="main-navbar">
            <div class="container-fluid">
                <a class="navbar-brand d-flex align-items-center gap-2" href="#/">
                    <i class="bi bi-code-slash fs-3 text-primary"></i>
                    <span class="fw-bold">"doanson44"</span>
                </a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#platform-nav" aria-controls="platform-nav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="platform-nav">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item"><a class="nav-link" href="#/"><i class="bi bi-house-door me-1"></i>"Home"</a></li>
                        <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false"><i class="bi bi-tools me-1"></i>"Tools"</a>
                            <ul class="dropdown-menu">
                                <li><a class="dropdown-item" href="#/tools/markdown"><i class="bi bi-markdown-fill me-2 text-primary"></i>"Markdown Studio"</a></li>
                                <li><a class="dropdown-item" href="#/tools/json"><i class="bi bi-braces me-2 text-primary"></i>"JSON Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/jwt"><i class="bi bi-key me-2 text-primary"></i>"JWT Decoder"</a></li>
                            </ul>
                        </li>
                        <li class="nav-item"><a class="nav-link" href="#/games"><i class="bi bi-joystick me-1"></i>"Games"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/cv"><i class="bi bi-person-badge me-1"></i>"CV"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/socket"><i class="bi bi-diagram-3 me-1"></i>"Socket"</a></li>
                    </ul>
                    <div class="d-flex align-items-center gap-3">
                        <button class="btn btn-outline-secondary btn-sm theme-toggle-btn" title="Toggle dark/light mode" on:click=on_toggle_theme>
                            <i class="bi bi-sun-fill theme-icon-light"></i>
                            <i class="bi bi-moon-fill theme-icon-dark"></i>
                        </button>
                        <span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle"><i class="bi bi-cpu me-1"></i>"WASM"</span>
                        <a href="https://github.com/doanson44/doanson44.github.io" target="_blank" rel="noopener noreferrer" class="btn btn-outline-secondary btn-sm d-flex align-items-center gap-1" title="View on GitHub">
                            <i class="bi bi-github"></i><span class="d-none d-md-inline">"GitHub"</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
