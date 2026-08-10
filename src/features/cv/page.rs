use leptos::prelude::*;

/// CV / Portfolio page — placeholder.
#[component]
pub fn CvPage() -> impl IntoView {
    view! {
        <div class="container py-5 text-center flex-grow-1">
            <i class="bi bi-person-badge fs-1 text-secondary mb-3 d-block"></i>
            <h3 class="text-body-secondary">"CV / Portfolio"</h3>
            <p class="text-body-tertiary">"Coming soon — public CV and portfolio."</p>
        </div>
    }
}
