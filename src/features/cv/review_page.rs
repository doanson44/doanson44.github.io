use leptos::prelude::*;

use super::review::{review_cards, ReviewCard};
use crate::i18n::*;
use crate::infrastructure::browser::scroll_to_top;

#[component]
pub fn CvReviewPage() -> impl IntoView {
    let i18n = use_i18n();
    let cards = Memo::new(move |_| review_cards(i18n.get_locale()));
    let current_index = RwSignal::new(0usize);
    let flipped = RwSignal::new(false);

    Effect::new(move |_| {
        let _ = i18n.get_locale();
        current_index.set(0);
        flipped.set(false);
    });

    let current_card = Memo::new(move |_| {
        cards
            .get()
            .get(current_index.get())
            .cloned()
            .unwrap_or_else(|| ReviewCard {
                section: String::new(),
                title: String::new(),
                content: String::new(),
            })
    });

    let flip = move |_| flipped.update(|value| *value = !*value);

    let previous = move |_| {
        let total = cards.get().len();
        if total == 0 {
            return;
        }

        current_index.update(|index| {
            *index = if *index == 0 {
                total - 1
            } else {
                *index - 1
            };
        });
        flipped.set(false);
        scroll_to_top();
    };

    let next = move |_| {
        let total = cards.get().len();
        if total == 0 {
            return;
        }

        current_index.update(|index| {
            *index = (*index + 1) % total;
        });
        flipped.set(false);
        scroll_to_top();
    };

    view! {
        <main class="flex flex-1 flex-col">
            <div class="mx-auto w-full max-w-5xl px-4 py-8 sm:px-6 sm:py-12 lg:px-8">
                <header class="mb-8 flex flex-col gap-5 sm:flex-row sm:items-end sm:justify-between">
                    <div>
                        <a
                            href="#/cv"
                            class="inline-flex rounded-md text-sm font-medium text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        >
                            "← " {move || t_string!(i18n, cv_review_back)}
                        </a>
                        <p class="mt-5 text-xs font-semibold uppercase tracking-[0.2em] text-[var(--accent)]">
                            {move || t_string!(i18n, cv_review_label)}
                        </p>
                        <h1 class="mt-2 text-3xl font-bold tracking-tight text-[var(--text-primary)] sm:text-4xl">
                            {move || t_string!(i18n, cv_review_title)}
                        </h1>
                        <p class="mt-3 max-w-3xl text-sm leading-6 text-[var(--text-secondary)] sm:text-base">
                            {move || t_string!(i18n, cv_review_description)}
                        </p>
                    </div>
                    <div class="shrink-0 text-sm text-[var(--text-secondary)]">
                        {move || {
                            let total = cards.get().len();
                            if total == 0 {
                                t_string!(i18n, cv_review_empty)
                            } else {
                                format!(
                                    "{} / {}",
                                    current_index.get() + 1,
                                    total
                                )
                            }
                        }}
                    </div>
                </header>

                <Show
                    when=move || !cards.get().is_empty()
                    fallback=move || view! {
                        <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-8 text-center text-sm text-[var(--text-secondary)]">
                            {move || t_string!(i18n, cv_review_empty)}
                        </div>
                    }
                >
                    <article
                        class="group min-h-[28rem] cursor-pointer rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-6 shadow-sm transition hover:border-[var(--accent)] sm:min-h-[32rem] sm:p-10"
                        role="button"
                        tabindex="0"
                        aria-live="polite"
                        on:click=flip
                        on:keydown=move |event: web_sys::KeyboardEvent| {
                            if event.key() == "Enter" || event.key() == " " {
                                event.prevent_default();
                                flip(());
                            }
                        }
                    >
                        <div class="flex h-full min-h-[25rem] flex-col">
                            <div class="flex items-center justify-between gap-4">
                                <span class="rounded-full border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-1 text-xs font-semibold text-[var(--accent)]">
                                    {move || current_card.get().section}
                                </span>
                                <span class="text-xs font-medium uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                                    {move || if flipped.get() {
                                        t_string!(i18n, cv_review_answer)
                                    } else {
                                        t_string!(i18n, cv_review_question)
                                    }}
                                </span>
                            </div>

                            <div class="flex flex-1 flex-col justify-center py-8">
                                <Show
                                    when=move || !flipped.get()
                                    fallback=move || view! {
                                        <div>
                                            <h2 class="text-xl font-semibold leading-8 text-[var(--text-primary)] sm:text-2xl">
                                                {move || current_card.get().title}
                                            </h2>
                                            <div class="mt-6 whitespace-pre-wrap break-words text-sm leading-7 text-[var(--text-secondary)] sm:text-base">
                                                {move || current_card.get().content}
                                            </div>
                                        </div>
                                    }
                                >
                                    <div class="text-center">
                                        <p class="text-3xl font-bold tracking-tight text-[var(--text-primary)] sm:text-4xl">
                                            {move || current_card.get().title}
                                        </p>
                                        <p class="mt-5 text-sm text-[var(--text-secondary)]">
                                            {move || t_string!(i18n, cv_review_reveal)}
                                        </p>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </article>

                    <div class="mt-5 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                        <button
                            type="button"
                            class="min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            on:click=previous
                        >
                            "← " {move || t_string!(i18n, cv_review_previous)}
                        </button>

                        <button
                            type="button"
                            class="min-h-11 rounded-lg border border-[var(--accent)] px-5 py-2 text-sm font-semibold text-[var(--accent)] transition hover:bg-[var(--accent)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            on:click=flip
                        >
                            {move || if flipped.get() {
                                t_string!(i18n, cv_review_question)
                            } else {
                                t_string!(i18n, cv_review_reveal)
                            }}
                        </button>

                        <button
                            type="button"
                            class="min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            on:click=next
                        >
                            {move || t_string!(i18n, cv_review_next)} "→"
                        </button>
                    </div>
                </Show>
            </div>
        </main>
    }
}
