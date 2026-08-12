use leptos::prelude::*;

use crate::infrastructure::browser::toggle_theme_js;

/// Platform navigation bar with hash-based routing for GitHub Pages.
#[component]
pub fn Navbar() -> impl IntoView {
    let on_toggle_theme = move |_| { toggle_theme_js(); };
    view! {
        <nav class="navbar navbar-expand-md border-bottom border-secondary" id="main-navbar">
            <div class="container-fluid">
                <a class="navbar-brand d-flex align-items-center gap-2" href="#/"><i class="bi bi-code-slash fs-3 text-primary"></i><span class="fw-bold">"doanson44"</span></a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#platform-nav" aria-controls="platform-nav" aria-expanded="false" aria-label="Toggle navigation"><span class="navbar-toggler-icon"></span></button>
                <div class="collapse navbar-collapse" id="platform-nav">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item"><a class="nav-link" href="#/"><i class="bi bi-house-door me-1"></i>"Home"</a></li>
                        <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false"><i class="bi bi-tools me-1"></i>"Tools"</a>
                            <ul class="dropdown-menu">
                                <li><a class="dropdown-item" href="#/tools/markdown"><i class="bi bi-markdown-fill me-2 text-primary"></i>"Markdown Studio"</a></li>
                                <li><a class="dropdown-item" href="#/tools/json"><i class="bi bi-braces me-2 text-primary"></i>"JSON Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/jwt"><i class="bi bi-key me-2 text-primary"></i>"JWT Decoder"</a></li>
                                <li><a class="dropdown-item" href="#/tools/base64"><i class="bi bi-file-binary me-2 text-primary"></i>"Base64 Encoder / Decoder"</a></li>
                                <li><a class="dropdown-item" href="#/tools/time"><i class="bi bi-clock-history me-2 text-primary"></i>"Time & Utilities"</a></li>
                                <li><a class="dropdown-item" href="#/tools/finance"><i class="bi bi-cash-coin me-2 text-primary"></i>"Finance Toolkit"</a></li>
                                <li><hr class="dropdown-divider" /></li>
                                <li><a class="dropdown-item" href="#/tools/xml"><i class="bi bi-filetype-xml me-2 text-primary"></i>"XML Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/yaml"><i class="bi bi-filetype-yml me-2 text-primary"></i>"YAML Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/sql"><i class="bi bi-database me-2 text-primary"></i>"SQL Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/html"><i class="bi bi-filetype-html me-2 text-primary"></i>"HTML Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/css"><i class="bi bi-filetype-css me-2 text-primary"></i>"CSS Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/javascript"><i class="bi bi-filetype-js me-2 text-primary"></i>"JavaScript Formatter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/regex"><i class="bi bi-regex me-2 text-primary"></i>"Regex Tester"</a></li>
                                <li><a class="dropdown-item" href="#/tools/url"><i class="bi bi-link-45deg me-2 text-primary"></i>"URL Encoder / Decoder"</a></li>
                                <li><a class="dropdown-item" href="#/tools/hash"><i class="bi bi-hash me-2 text-primary"></i>"Hash Generator"</a></li>
                                <li><a class="dropdown-item" href="#/tools/uuid"><i class="bi bi-fingerprint me-2 text-primary"></i>"UUID Generator"</a></li>
                                <li><a class="dropdown-item" href="#/tools/timestamp"><i class="bi bi-clock-history me-2 text-primary"></i>"Timestamp Converter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/color"><i class="bi bi-palette me-2 text-primary"></i>"Color Converter"</a></li>
                                <li><a class="dropdown-item" href="#/tools/cron"><i class="bi bi-calendar3 me-2 text-primary"></i>"Cron Expression Generator"</a></li>
                                <li><a class="dropdown-item" href="#/tools/http-status"><i class="bi bi-globe2 me-2 text-primary"></i>"HTTP Status Lookup"</a></li>
                                <li><a class="dropdown-item" href="#/tools/subnet"><i class="bi bi-diagram-3 me-2 text-primary"></i>"IP / Subnet Calculator"</a></li>
                                <li><a class="dropdown-item" href="#/tools/qr"><i class="bi bi-qr-code me-2 text-primary"></i>"QR Code Generator"</a></li>
                            </ul>
                        </li>
                        <li class="nav-item"><a class="nav-link" href="#/games"><i class="bi bi-joystick me-1"></i>"Games"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/cv"><i class="bi bi-person-badge me-1"></i>"CV"</a></li>
                        <li class="nav-item"><a class="nav-link" href="#/socket"><i class="bi bi-diagram-3 me-1"></i>"Socket"</a></li>
                    </ul>
                    <div class="d-flex align-items-center gap-3">
                        <button class="btn btn-outline-secondary btn-sm theme-toggle-btn" title="Toggle dark/light mode" on:click=on_toggle_theme><i class="bi bi-sun-fill theme-icon-light"></i><i class="bi bi-moon-fill theme-icon-dark"></i></button>
                        <span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle"><i class="bi bi-cpu me-1"></i>"WASM"</span>
                        <a href="https://github.com/doanson44/doanson44.github.io" target="_blank" rel="noopener noreferrer" class="btn btn-outline-secondary btn-sm d-flex align-items-center gap-1" title="View on GitHub"><i class="bi bi-github"></i><span class="d-none d-md-inline">"GitHub"</span></a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
