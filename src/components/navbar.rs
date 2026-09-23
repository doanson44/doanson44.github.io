use leptos::prelude::*;

use crate::i18n::*;
use crate::infrastructure::browser::toggle_theme_js;

#[component]
pub fn Navbar() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let i18n = use_i18n();

    let set_locale = move |locale| {
        i18n.set_locale(locale);
    };

    view! {
        <nav class="border-b border-[var(--border-color)] bg-[var(--surface)]" id="main-navbar">
            <div class="mx-auto flex max-w-screen-2xl flex-wrap items-center justify-between gap-2 px-4 py-2">
                <a class="flex items-center gap-2 text-lg font-bold text-[var(--text-primary)] no-underline" href="#/">
                    <span class="text-xl text-[var(--accent)]" aria-hidden="true">"⌘"</span>
                    <span>"doanson44"</span>
                </a>
                <button
                    class="inline-flex min-h-10 min-w-10 items-center justify-center rounded-md border border-[var(--border-color)] px-2 text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] md:hidden"
                    type="button"
                    aria-controls="platform-nav"
                    aria-label=move || t_string!(i18n, nav_toggle)
                    aria-expanded=move || menu_open.get().to_string()
                    on:click=move |_| menu_open.update(|open| *open = !*open)
                >
                    <span aria-hidden="true">"☰"</span>
                </button>
                <div
                    class=move || if menu_open.get() {
                        "order-3 w-full md:order-none md:flex md:w-auto"
                    } else {
                        "hidden w-full md:order-none md:flex md:w-auto"
                    }
                    id="platform-nav"
                >
                    <ul class="flex flex-col gap-1 py-2 md:flex-row md:items-center md:py-0">
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/" on:click=move |_| menu_open.set(false)>"⌂"<span class="ml-2">{move || t_string!(i18n, nav_home)}</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/tools" on:click=move |_| menu_open.set(false)>"⚒"<span class="ml-2">{move || t_string!(i18n, nav_tools)}</span></a></li>\n                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/tools/market" on:click=move |_| menu_open.set(false)>"◈"<span class="ml-2">{move || t_string!(i18n, nav_market)}</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/games" on:click=move |_| menu_open.set(false)>"♟"<span class="ml-2">{move || t_string!(i18n, nav_games)}</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/cv" on:click=move |_| menu_open.set(false)>"●"<span class="ml-2">{move || t_string!(i18n, nav_cv)}</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/socket" on:click=move |_| menu_open.set(false)>"↔"<span class="ml-2">{move || t_string!(i18n, nav_socket)}</span></a></li>
                    </ul>
                </div>
                <div class="ml-auto flex items-center gap-2">
                    <div class="flex items-center rounded-md border border-[var(--border-color)] p-0.5" role="group" aria-label=move || t_string!(i18n, language)>
                        <button type="button" class=move || locale_button_class(i18n.get_locale() == Locale::vi) aria-pressed=move || (i18n.get_locale() == Locale::vi).to_string() on:click=move |_| set_locale(Locale::vi) title=move || t_string!(i18n, language_vietnamese)>"VI"</button>
                        <button type="button" class=move || locale_button_class(i18n.get_locale() == Locale::en) aria-pressed=move || (i18n.get_locale() == Locale::en).to_string() on:click=move |_| set_locale(Locale::en) title=move || t_string!(i18n, language_english)>"EN"</button>
                    </div>
                    <button type="button" class="inline-flex min-h-9 min-w-9 items-center justify-center rounded-md border border-[var(--border-color)] px-2 text-sm text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" title=move || t_string!(i18n, theme_toggle) aria-label=move || t_string!(i18n, theme_toggle) on:click=move |_| { toggle_theme_js(); }>
                        <span aria-hidden="true">"☼/☾"</span>
                    </button>
                    <span class="rounded-full border border-[color-mix(in_srgb,var(--accent)_35%,transparent)] bg-[color-mix(in_srgb,var(--accent)_12%,transparent)] px-2 py-1 text-xs font-semibold text-[var(--accent)]">"WASM"</span>
                    <a href="https://github.com/doanson44/doanson44.github.io" target="_blank" rel="noopener noreferrer" class="hidden items-center gap-1 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-secondary)] no-underline transition hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] md:flex" title=move || t_string!(i18n, nav_github)>
                        <span aria-hidden="true">"◈"</span><span>{move || t_string!(i18n, nav_github)}</span>
                    </a>
                </div>
            </div>
        </nav>
    }
}

fn locale_button_class(active: bool) -> &'static str {
    if active {
        "rounded px-2 py-1 text-xs font-semibold text-[var(--accent)] bg-[var(--surface-hover)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
    } else {
        "rounded px-2 py-1 text-xs font-medium text-[var(--text-secondary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
    }
}
