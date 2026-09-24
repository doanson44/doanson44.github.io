use std::collections::HashSet;

use leptos::prelude::*;

use super::review::{review_cards, ReviewCard};
use crate::i18n::*;
use crate::infrastructure::browser::{scroll_to_top, storage_get, storage_set};
use leptos_i18n::I18nContext;

const PROGRESS_STORAGE_KEY: &str = "cv.review.understood.v1";

#[component]
pub fn CvReviewPage(card_id: Option<String>) -> impl IntoView {
    let i18n = use_i18n();
    let cards = Memo::new(move |_| review_cards(i18n.get_locale()));
    let understood = RwSignal::new(load_understood());

    let save_progress = move |next: HashSet<String>| {
        if let Ok(value) = serde_json::to_string(&next) {
            let _ = storage_set(PROGRESS_STORAGE_KEY, &value);
        }
        understood.set(next);
    };

    let clear_progress = move |_| {
        let empty = HashSet::new();
        let _ = storage_set(PROGRESS_STORAGE_KEY, "[]");
        understood.set(empty);
    };

    let selected_card = Memo::new(move |_| {
        let id = card_id.as_deref()?;
        cards.get().iter().find(|card| card.id == id).copied()
    });

    let toggle_understood = move |id: String| {
        let mut next = understood.get();
        if next.contains(&id) {
            next.remove(&id);
        } else {
            next.insert(id);
        }
        save_progress(next);
    };

    view! {
        <main class="flex flex-1 flex-col">
            <div class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 sm:py-12 lg:px-8">
                {move || {
                    if card_id.is_some() {
                        detail_view(
                            i18n,
                            selected_card.get(),
                            understood.get(),
                            toggle_understood,
                        )
                        .into_any()
                    } else {
                        list_view(
                            i18n,
                            cards.get(),
                            understood.get(),
                            clear_progress,
                        )
                        .into_any()
                    }
                }}
            </div>
        </main>
    }
}

fn list_view(
    i18n: I18nContext<crate::i18n::Locale>,
    cards: &'static [ReviewCard],
    understood: HashSet<String>,
    clear_progress: impl FnMut(leptos::ev::MouseEvent) + Copy + 'static,
) -> impl IntoView {
    let total = cards.len();
    let completed = cards
        .iter()
        .filter(|card| understood.contains(&card.id))
        .count();

    view! {
        <header class="mb-8 flex flex-col gap-5 sm:flex-row sm:items-end sm:justify-between">
            <div>
                <a
                    href="#/cv"
                    class="inline-flex rounded-md text-sm font-medium text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                >
                    "← " {t_string!(i18n, cv_review_back)}
                </a>
                <p class="mt-5 text-xs font-semibold uppercase tracking-[0.2em] text-[var(--accent)]">
                    {t_string!(i18n, cv_review_label)}
                </p>
                <h1 class="mt-2 text-3xl font-bold tracking-tight text-[var(--text-primary)] sm:text-4xl">
                    {t_string!(i18n, cv_review_title)}
                </h1>
                <p class="mt-3 max-w-3xl text-sm leading-6 text-[var(--text-secondary)] sm:text-base">
                    {t_string!(i18n, cv_review_description)}
                </p>
            </div>

            <div class="flex shrink-0 flex-col items-start gap-3 sm:items-end">
                <span class="text-sm font-medium text-[var(--text-secondary)]">
                    {format!("{} / {}", completed, total)}
                </span>
                <button
                    type="button"
                    disabled=completed == 0
                    class="min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=clear_progress
                >
                    {t_string!(i18n, cv_review_clear)}
                </button>
            </div>
        </header>

        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {cards
                .into_iter()
                .enumerate()
                .map(|(index, card)| {
                    let id = card.id.clone();
                    let is_understood = understood.contains(id.as_str());
                    view! {
                        <a
                            href=format!("#/cv/review/{id}")
                            class="group rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 transition hover:-translate-y-0.5 hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        >
                            <div class="flex items-start justify-between gap-3">
                                <span class="text-xs font-semibold uppercase tracking-[0.12em] text-[var(--text-tertiary)]">
                                    {format!("#{}", index + 1)}
                                </span>
                                <span class=if is_understood {
                                    "rounded-full border border-[var(--accent)]/40 bg-[var(--accent)]/10 px-2 py-1 text-xs font-semibold text-[var(--accent)]"
                                } else {
                                    "rounded-full border border-[var(--border-color)] px-2 py-1 text-xs font-semibold text-[var(--text-tertiary)]"
                                }>
                                    {if is_understood {
                                        t_string!(i18n, cv_review_understood)
                                    } else {
                                        t_string!(i18n, cv_review_not_understood)
                                    }}
                                </span>
                            </div>
                            <p class="mt-4 text-xs font-medium text-[var(--accent)]">{card.section}</p>
                            <h2 class="mt-2 text-lg font-semibold leading-7 text-[var(--text-primary)] group-hover:text-[var(--accent)]">
                                {card.title}
                            </h2>
                            <p class="mt-3 text-sm leading-6 text-[var(--text-secondary)]">
                                {card.question}
                            </p>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

fn detail_view(
    i18n: I18nContext,
    card: Option<ReviewCard>,
    understood: HashSet<String>,
    toggle_understood: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    let Some(card) = card else {
        return view! {
            <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-8 text-center">
                <p class="text-sm text-[var(--text-secondary)]">{t_string!(i18n, cv_review_empty)}</p>
                <a
                    href="#/cv/review"
                    class="mt-5 inline-flex min-h-11 items-center rounded-lg border border-[var(--border-color)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                >
                    {t_string!(i18n, cv_review_back_to_list)}
                </a>
            </div>
        }.into_any();
    };

    let id = card.id.clone();
    let is_understood = understood.contains(&id);

    view! {
        <div>
            <a
                href="#/cv/review"
                class="inline-flex rounded-md text-sm font-medium text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
            >
                "← " {t_string!(i18n, cv_review_back_to_list)}
            </a>

            <article class="mt-6 overflow-hidden rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] shadow-sm">
                <div class="border-b border-[var(--border-color)] px-5 py-5 sm:px-8">
                    <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--accent)]">
                        {card.section}
                    </p>
                    <h1 class="mt-2 text-2xl font-bold tracking-tight text-[var(--text-primary)] sm:text-3xl">
                        {card.title}
                    </h1>
                </div>

                <div class="grid gap-0 lg:grid-cols-2">
                    <section class="border-b border-[var(--border-color)] p-5 sm:p-8 lg:border-b-0 lg:border-r">
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                            {t_string!(i18n, cv_review_question)}
                        </p>
                        <h2 class="mt-4 text-xl font-semibold leading-8 text-[var(--text-primary)]">
                            {card.question}
                        </h2>
                    </section>

                    <section class="p-5 sm:p-8">
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                            {t_string!(i18n, cv_review_answer)}
                        </p>
                        <div class="mt-4 whitespace-pre-wrap break-words text-sm leading-7 text-[var(--text-secondary)] sm:text-base">
                            {card.answer}
                        </div>
                    </section>
                </div>
            </article>

            <div class="mt-5 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <span class="text-sm text-[var(--text-secondary)]">
                    {if is_understood {
                        t_string!(i18n, cv_review_understood)
                    } else {
                        t_string!(i18n, cv_review_not_understood)
                    }}
                </span>

                <button
                    type="button"
                    class=if is_understood {
                        "min-h-11 rounded-lg border border-[var(--accent)] bg-[var(--accent)]/10 px-5 py-2 text-sm font-semibold text-[var(--accent)] transition hover:bg-[var(--accent)]/15 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    } else {
                        "min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-5 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    }
                    on:click=move |_| {
                        toggle_understood(id.to_string());
                        scroll_to_top();
                    }
                >
                    {if is_understood {
                        t_string!(i18n, cv_review_mark_not_understood)
                    } else {
                        t_string!(i18n, cv_review_mark_understood)
                    }}
                </button>
            </div>
        </div>
    }
}

fn load_understood() -> HashSet<String> {
    storage_get(PROGRESS_STORAGE_KEY)
        .and_then(|value| serde_json::from_str::<HashSet<String>>(&value).ok())
        .unwrap_or_default()
}
