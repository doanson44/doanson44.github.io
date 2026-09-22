use leptos::prelude::*;

use crate::i18n::*;

/// Home page — platform landing page.
///
/// Serves as the entry point for the doanson44.github.io platform,
/// providing navigation to all feature areas.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="flex flex-1 flex-col">
            <div class="mx-auto w-full max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
                <div class="mx-auto max-w-3xl text-center">
                    <h1 class="mb-3 text-4xl font-bold tracking-tight text-[var(--text-primary)] sm:text-5xl">
                        <span class="mr-2 text-[var(--accent)]" aria-hidden="true">">_"</span>
                        "doanson44.github.io"
                    </h1>
                    <p class="mb-8 text-lg text-[var(--text-secondary)]">
                        {move || t!(use_i18n(), home_tagline)}
                    </p>
                </div>

                <div class="mx-auto grid max-w-5xl grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
                    <HomeCard href="#/tools" icon="⚒" title=move || t!(use_i18n(), nav_tools) description=move || t!(use_i18n(), home_tools_desc) />
                    <HomeCard href="#/games" icon="♟" title=move || t!(use_i18n(), nav_games) description=move || t!(use_i18n(), home_games_desc) />
                    <HomeCard href="#/cv" icon="●" title=move || t!(use_i18n(), nav_cv) description=move || t!(use_i18n(), home_cv_desc) />
                    <HomeCard href="#/socket" icon="↔" title=move || t!(use_i18n(), nav_socket) description=move || t!(use_i18n(), home_socket_desc) />
                </div>
            </div>
        </main>
    }
}

#[component]
fn HomeCard(
    #[prop(into)] href: String,
    icon: &'static str,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            class="group rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-6 text-center no-underline shadow-sm transition hover:-translate-y-0.5 hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
        >
            <div class="mb-4 text-4xl text-[var(--accent)]" aria-hidden="true">{icon}</div>
            <h2 class="mb-2 text-lg font-semibold text-[var(--text-primary)]">{title}</h2>
            <p class="text-sm leading-6 text-[var(--text-secondary)]">{description}</p>
        </a>
    }
}
