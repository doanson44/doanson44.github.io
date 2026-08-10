use leptos::prelude::*;

/// Home page — platform landing page.
///
/// Serves as the entry point for the doanson44.github.io platform,
/// providing navigation to all feature areas.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="d-flex flex-column flex-grow-1">
            <div class="container py-5">
                <div class="row justify-content-center">
                    <div class="col-12 col-md-10 col-lg-8 text-center">
                        <h1 class="display-4 fw-bold mb-3">
                            <i class="bi bi-terminal-fill text-primary me-2"></i>
                            "doanson44.github.io"
                        </h1>
                        <p class="lead text-body-secondary mb-4">
                            "A personal web platform — developer tools, games, CV, and more. Built with Rust, Leptos, and WebAssembly."
                        </p>
                    </div>
                </div>

                <div class="row g-4 mt-2 justify-content-center">
                    <div class="col-12 col-sm-6 col-lg-4">
                        <a href="#/tools" class="text-decoration-none">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body text-center p-4">
                                    <i class="bi bi-tools fs-1 text-primary mb-3 d-block"></i>
                                    <h5 class="card-title">"Tools"</h5>
                                    <p class="card-text text-body-secondary small">
                                        "Developer utilities — Markdown Studio, JSON, JWT, and more."
                                    </p>
                                </div>
                            </div>
                        </a>
                    </div>
                    <div class="col-12 col-sm-6 col-lg-4">
                        <a href="#/games" class="text-decoration-none">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body text-center p-4">
                                    <i class="bi bi-joystick fs-1 text-success mb-3 d-block"></i>
                                    <h5 class="card-title">"Games"</h5>
                                    <p class="card-text text-body-secondary small">
                                        "Small browser games and experiments."
                                    </p>
                                </div>
                            </div>
                        </a>
                    </div>
                    <div class="col-12 col-sm-6 col-lg-4">
                        <a href="#/cv" class="text-decoration-none">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body text-center p-4">
                                    <i class="bi bi-person-badge fs-1 text-info mb-3 d-block"></i>
                                    <h5 class="card-title">"CV"</h5>
                                    <p class="card-text text-body-secondary small">
                                        "Public CV and portfolio."
                                    </p>
                                </div>
                            </div>
                        </a>
                    </div>
                    <div class="col-12 col-sm-6 col-lg-4">
                        <a href="#/socket" class="text-decoration-none">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body text-center p-4">
                                    <i class="bi bi-diagram-3 fs-1 text-warning mb-3 d-block"></i>
                                    <h5 class="card-title">"Socket"</h5>
                                    <p class="card-text text-body-secondary small">
                                        "WebSocket and realtime playground."
                                    </p>
                                </div>
                            </div>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
