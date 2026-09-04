use leptos::prelude::*;

use crate::infrastructure::browser::toggle_theme_js;

#[component]
pub fn Navbar() -> impl IntoView {
    let menu_open = RwSignal::new(false);

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
                    aria-label="Toggle navigation"
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
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/" on:click=move |_| menu_open.set(false)>"⌂"<span class="ml-2">"Home"</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/tools" on:click=move |_| menu_open.set(false)>"⚒"<span class="ml-2">"Tools"</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/games" on:click=move |_| menu_open.set(false)>"♟"<span class="ml-2">"Games"</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/cv" on:click=move |_| menu_open.set(false)>"●"<span class="ml-2">"CV"</span></a></li>
                        <li><a class="flex items-center rounded-md px-3 py-2 text-sm text-[var(--text-secondary)] no-underline hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" href="#/socket" on:click=move |_| menu_open.set(false)>"↔"<span class="ml-2">"Socket"</span></a></li>
                    </ul>
                </div>
                <div class="ml-auto flex items-center gap-2">
                    <button type="button" class="inline-flex min-h-9 min-w-9 items-center justify-center rounded-md border border-[var(--border-color)] px-2 text-sm text-[var(--text-secondary)] transition hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]" title="Toggle dark/light mode" aria-label="Toggle dark/light mode" on:click=move |_| { toggle_theme_js(); }>
                        <span aria-hidden="true">"☼/☾"</span>
                    </button>
                    <span class="rounded-full border border-[color-mix(in_srgb,var(--accent)_35%,transparent)] bg-[color-mix(in_srgb,var(--accent)_12%,transparent)] px-2 py-1 text-xs font-semibold text-[var(--accent)]">"WASM"</span>
                    <a href="https://github.com/doanson44/doanson44.github.io" target="_blank" rel="noopener noreferrer" class="hidden items-center gap-1 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm text-[var(--text-secondary)] no-underline transition hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] md:flex" title="View on GitHub">
                        <span aria-hidden="true">"◈"</span><span>"GitHub"</span>
                    </a>
                </div>
            </div>
        </nav>
    }
}
