use leptos::prelude::*;

/// CV / Portfolio page — placeholder.
#[component]
pub fn CvPage() -> impl IntoView {
    view! {
        <main class="flex flex-1 items-center justify-center px-4 py-12 text-center">
            <div>
                <div class="mb-4 text-4xl text-[var(--text-secondary)]" aria-hidden="true">"●"</div>
                <h1 class="text-xl font-semibold text-[var(--text-primary)]">"CV / Portfolio"</h1>
                <p class="mt-2 text-sm text-[var(--text-secondary)]">"Coming soon — public CV and portfolio."</p>
            </div>
        </main>
    }
}
