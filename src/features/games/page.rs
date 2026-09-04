use leptos::prelude::*;

/// Games page — placeholder.
#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <main class="flex flex-1 items-center justify-center px-4 py-12 text-center">
            <div>
                <div class="mb-4 text-4xl text-[var(--text-secondary)]" aria-hidden="true">"♟"</div>
                <h1 class="text-xl font-semibold text-[var(--text-primary)]">"Games"</h1>
                <p class="mt-2 text-sm text-[var(--text-secondary)]">"Coming soon — browser games and experiments."</p>
            </div>
        </main>
    }
}
