use leptos::prelude::*;

/// Games page — placeholder.
#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <div class="container py-5 text-center flex-grow-1">
            <i class="bi bi-joystick fs-1 text-secondary mb-3 d-block"></i>
            <h3 class="text-body-secondary">"Games"</h3>
            <p class="text-body-tertiary">"Coming soon — browser games and experiments."</p>
        </div>
    }
}
