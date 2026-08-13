use leptos::prelude::*;

use crate::infrastructure::browser::toggle_theme_js;

#[component]
pub fn Navbar() -> impl IntoView {
    let on_toggle_theme = move |_| toggle_theme_js();
    view! {
        <nav class="navbar navbar-expand-md border-bottom border-secondary" id="main-navbar">
            <div class="container-fluid">
                <a class="navbar-brand d-flex align-items-center gap-2" href="#/"><i class="bi bi-code-slash fs-3 text-primary" aria-hidden="true"></i><span class="fw-bold">"doanson44"</span></a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#platform-nav" aria-controls="platform-nav" aria-expanded="false" aria-label="Toggle navigation"><span class="navbar-toggler-icon"></span></button>
                <div class="collapse navbar-collapse" id="platform-nav">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item"><a class="nav-link" href="#/"><i class="bi bi-house-door me-1" aria-hidden="true"></i>"Home"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/tools"><i class="bi bi-tools me-1" aria-hidden="true"></i>"Tools"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/games"><i class="bi bi-joystick me-1" aria-hidden="true"></i>"Games"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/cv"><i class="bi bi-person-badge me-1" aria-hidden="true"></i>"CV"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/socket"><i class="bi bi-diagram-3 me-1" aria-hidden="true"></i>"Socket"</a></li>
                    </ul>
                    <div class="d-flex align-items-center gap-3">
                        <button type="button" class="btn btn-outline-secondary btn-sm theme-toggle-btn" title="Toggle dark/light mode" aria-label="Toggle dark/light mode" on:click=on_toggle_theme><i class="bi bi-sun-fill theme-icon-light" aria-hidden="true"></i><i class="bi bi-moon-fill theme-icon-dark" aria-hidden="true"></i></button>
                        <span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle"><i class="bi bi-cpu me-1" aria-hidden="true"></i>"WASM"</span>
                        <a href="https://github.com/doanson44/doanson44.github.io" target="_blank" rel="noopener noreferrer" class="btn btn-outline-secondary btn-sm d-flex align-items-center gap-1" title="View on GitHub"><i class="bi bi-github" aria-hidden="true"></i><span class="d-none d-md-inline">"GitHub"</span></a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
