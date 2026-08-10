use leptos::prelude::*;

/// Tools index page — lists available developer tools.
#[component]
pub fn ToolsPage() -> impl IntoView {
    view! {
        <div class="d-flex flex-column flex-grow-1">
            <div class="container py-4">
                <h2 class="mb-4">
                    <i class="bi bi-tools me-2 text-primary"></i>
                    "Tools"
                </h2>
                <div class="row g-3">
                    <div class="col-12 col-sm-6 col-lg-4">
                        <a href="#/tools/markdown" class="text-decoration-none">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body p-3">
                                    <h6 class="card-title mb-1">
                                        <i class="bi bi-markdown-fill text-primary me-2"></i>
                                        "Markdown Studio"
                                    </h6>
                                    <p class="card-text text-body-secondary small mb-0">
                                        "Live Markdown editor with Mermaid diagram support."
                                    </p>
                                </div>
                            </div>
                        </a>
                    </div>
                    <div class="col-12 col-sm-6 col-lg-4">
                        <div class="card bg-body-tertiary border-secondary h-100 opacity-50">
                            <div class="card-body p-3">
                                <h6 class="card-title mb-1">
                                    <i class="bi bi-braces me-2"></i>
                                    "JSON Formatter"
                                </h6>
                                <p class="card-text text-body-secondary small mb-0">
                                    <span class="badge bg-secondary">"Coming soon"</span>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-12 col-sm-6 col-lg-4">
                        <div class="card bg-body-tertiary border-secondary h-100 opacity-50">
                            <div class="card-body p-3">
                                <h6 class="card-title mb-1">
                                    <i class="bi bi-key me-2"></i>
                                    "JWT Decoder"
                                </h6>
                                <p class="card-text text-body-secondary small mb-0">
                                    <span class="badge bg-secondary">"Coming soon"</span>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
