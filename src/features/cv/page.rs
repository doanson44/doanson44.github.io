use leptos::prelude::*;

use super::components::{CvSection, EducationCard, ExperienceCard, SkillGroup};
use super::data::{competencies, education, experiences, highlights, profile, skill_categories};
use crate::i18n::*;
use crate::infrastructure::browser::print_page;

/// Public CV and technical portfolio page.
#[component]
pub fn CvPage() -> impl IntoView {
    let profile = profile();
    let competencies = competencies();
    let skills = skill_categories();
    let experiences = experiences();
    let highlights = highlights();
    let education = education();
    let show_phone = RwSignal::new(false);
    let show_email = RwSignal::new(false);
    let show_cover_letter = RwSignal::new(false);
    let cover_letter_copied = RwSignal::new(false);
    let cover_letter_copy_error = RwSignal::new(false);
    let i18n = use_i18n();

    let copy_cover_letter = move |_| {
        let content = t_string!(i18n, cv_cover_letter_body).to_string();
        cover_letter_copied.set(false);
        cover_letter_copy_error.set(false);

        leptos::task::spawn_local(async move {
            if crate::infrastructure::browser::copy_to_clipboard(&content)
                .await
                .is_ok()
            {
                cover_letter_copied.set(true);
                gloo_timers::future::TimeoutFuture::new(1800).await;
                cover_letter_copied.set(false);
            } else {
                cover_letter_copy_error.set(true);
            }
        });
    };

    view! {
        <main class="flex flex-1 flex-col">
            <div class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 sm:py-12 lg:px-8">
                <header class="cv-hero rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-6 shadow-sm sm:p-8 lg:p-10">
                    <div class="grid gap-8 lg:grid-cols-[1fr_auto] lg:items-end">
                        <div>
                            <p class="mb-3 text-xs font-semibold uppercase tracking-[0.2em] text-[var(--accent)]">"Public Technical Portfolio"</p>
                            <h1 class="text-4xl font-bold tracking-tight text-[var(--text-primary)] sm:text-5xl">{profile.name}</h1>
                            <p class="mt-3 text-xl font-semibold text-[var(--accent)] sm:text-2xl">{profile.title}</p>
                            <p class="mt-5 max-w-3xl text-base leading-7 text-[var(--text-secondary)]">{profile.summary}</p>
                        </div>
                        <div class="flex flex-col gap-3 text-sm text-[var(--text-secondary)] lg:min-w-52 lg:text-right">
                            <span>{profile.location}</span>

                            <div class="flex items-center justify-start gap-2 lg:justify-end">
                                <Show
                                    when=move || show_phone.get()
                                    fallback=move || view! {
                                        <button
                                            type="button"
                                            class="rounded-md px-2 py-1 text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                            aria-label="Show phone number"
                                            on:click=move |_| show_phone.set(true)
                                        >
                                            {move || if i18n.get_locale() == Locale::vi { "Hiện số điện thoại" } else { "Show phone" }}
                                        </button>
                                    }
                                >
                                    <a
                                        class="break-all text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                        href=format!("tel:{}", profile.phone)
                                    >
                                        {profile.phone}
                                    </a>
                                </Show>
                            </div>

                            <div class="flex items-center justify-start gap-2 lg:justify-end">
                                <button
                                    type="button"
                                    class="rounded-md px-2 py-1 text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                    aria-label=move || t_string!(i18n, cv_show_cover_letter)
                                    on:click=move |_| {
                                        cover_letter_copied.set(false);
                                        cover_letter_copy_error.set(false);
                                        show_cover_letter.set(true);
                                    }
                                >
                                    {move || t_string!(i18n, cv_show_cover_letter)}
                                </button>
                            </div>

                            <div class="flex items-center justify-start gap-2 lg:justify-end">
                                <Show
                                    when=move || show_email.get()
                                    fallback=move || view! {
                                        <button
                                            type="button"
                                            class="rounded-md px-2 py-1 text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                            aria-label=move || t_string!(i18n, cv_show_email)
                                            on:click=move |_| show_email.set(true)
                                        >
                                            {move || t_string!(i18n, cv_show_email)}
                                        </button>
                                    }
                                >
                                    <a
                                        class="break-all text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                        href=format!("mailto:{}", profile.email)
                                    >
                                        {profile.email}
                                    </a>
                                </Show>
                            </div>
                        </div>
                    </div>

                    <div class="mt-8 flex flex-wrap items-center justify-between gap-4 border-t border-[var(--border-color)] pt-4">
                        <nav aria-label="CV sections">
                            <ul class="flex flex-wrap gap-2 text-sm">
                                <li><a class="cv-nav-link" href="#cv-about">{move || if i18n.get_locale() == Locale::vi { "Giới thiệu" } else { "About" }}</a></li>
                                <li><a class="cv-nav-link" href="#cv-skills">{move || if i18n.get_locale() == Locale::vi { "Kỹ năng" } else { "Skills" }}</a></li>
                                <li><a class="cv-nav-link" href="#cv-experience">{move || if i18n.get_locale() == Locale::vi { "Kinh nghiệm" } else { "Experience" }}</a></li>
                                <li><a class="cv-nav-link" href="#cv-highlights">{move || if i18n.get_locale() == Locale::vi { "Điểm nổi bật" } else { "Highlights" }}</a></li>
                                <li><a class="cv-nav-link" href="#cv-education">{move || t!(i18n, cv_education)}</a></li>
                            </ul>
                        </nav>

                        <button
                            type="button"
                            class="cv-print-button inline-flex items-center gap-2 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] shadow-sm transition hover:border-[var(--accent)] hover:text-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)] focus:ring-offset-2 focus:ring-offset-[var(--surface)]"
                            title="Open print dialog to save this CV as PDF"
                            aria-label="Download CV as PDF"
                            on:click=move |_| print_page()
                        >
                            <span aria-hidden="true">"↓"</span>
                            {move || if i18n.get_locale() == Locale::vi { "Tải PDF" } else { "Download PDF" }}
                        </button>
                    </div>
                </header>

                <div class="mt-12 space-y-12 sm:mt-16 sm:space-y-16">
                    <CvSection id="cv-about" title_key="competencies" eyebrow_key="what_i_do">
                        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                            {competencies.into_iter().map(|item| view! {
                                <div class="flex gap-3 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-3">
                                    <span class="mt-2 h-2 w-2 shrink-0 rounded-full bg-[var(--accent)]" aria-hidden="true"></span>
                                    <span class="text-sm leading-6 text-[var(--text-secondary)]">{item.name}</span>
                                </div>
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-skills" title_key="skills" eyebrow_key="technology">
                        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                            {skills.into_iter().map(|category| view! {
                                <SkillGroup category=category />
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-experience" title_key="experience" eyebrow_key="career">
                        <div class="space-y-6">
                            {experiences.into_iter().map(|experience| view! {
                                <ExperienceCard experience=experience />
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-highlights" title_key="highlights" eyebrow_key="engineering_focus">
                        <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                            <ul class="grid gap-3 sm:grid-cols-2">
                                {highlights.into_iter().map(|highlight| view! {
                                    <li class="flex gap-3 text-sm leading-7 text-[var(--text-secondary)]">
                                        <span class="mt-3 h-1.5 w-1.5 shrink-0 rounded-full bg-[var(--accent)]" aria-hidden="true"></span>
                                        <span>{highlight}</span>
                                    </li>
                                }).collect_view()}
                            </ul>
                        </div>
                    </CvSection>

                    <CvSection id="cv-education" title_key="education" eyebrow_key="academic_background">
                        <div class="space-y-4">
                            {education.into_iter().map(|value| view! {
                                <EducationCard value=value />
                            }).collect_view()}
                        </div>
                    </CvSection>
                </div>
            </div>
        </main>

        {move || if show_cover_letter.get() {
            view! {
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-[var(--overlay)] p-3" role="presentation">
                    <div
                        class="flex max-h-[90vh] w-full max-w-3xl flex-col overflow-hidden rounded-xl border border-[var(--border-color)] bg-[var(--surface)] shadow-xl"
                        role="dialog"
                        aria-modal="true"
                        aria-labelledby="cv-cover-letter-title"
                    >
                        <header class="flex shrink-0 items-start justify-between gap-3 border-b border-[var(--border-color)] px-4 py-3 sm:px-5">
                            <div>
                                <h2 id="cv-cover-letter-title" class="text-lg font-semibold text-[var(--text-primary)]">
                                    {move || t_string!(i18n, cv_cover_letter_title)}
                                </h2>
                                <p class="mt-1 text-xs text-[var(--text-secondary)]">
                                    {move || t_string!(i18n, cv_cover_letter_hint)}
                                </p>
                            </div>
                            <button
                                type="button"
                                class="min-h-10 min-w-10 rounded-md border border-[var(--border-color)] px-2 text-sm text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                                aria-label=move || t_string!(i18n, common_close)
                                title=move || t_string!(i18n, common_close)
                                on:click=move |_| {
                                    show_cover_letter.set(false);
                                    cover_letter_copied.set(false);
                                    cover_letter_copy_error.set(false);
                                }
                            >
                                "×"
                            </button>
                        </header>

                        <div class="min-h-0 overflow-y-auto px-4 py-5 sm:px-6">
                            <div class="whitespace-pre-line text-sm leading-7 text-[var(--text-secondary)] sm:text-base">
                                {move || t_string!(i18n, cv_cover_letter_body)}
                            </div>
                        </div>

                        <footer class="flex shrink-0 flex-col gap-2 border-t border-[var(--border-color)] px-4 py-3 sm:flex-row sm:items-center sm:justify-between sm:px-5">
                            <div class="min-h-5 text-xs" aria-live="polite">
                                <Show when=move || cover_letter_copy_error.get()>
                                    <span class="text-[var(--danger)]">{move || t_string!(i18n, cv_cover_letter_copy_error)}</span>
                                </Show>
                            </div>
                            <div class="flex flex-wrap justify-end gap-2">
                                <button
                                    type="button"
                                    class="min-h-10 rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                                    on:click=move |_| {
                                        show_cover_letter.set(false);
                                        cover_letter_copied.set(false);
                                        cover_letter_copy_error.set(false);
                                    }
                                >
                                    {move || t_string!(i18n, common_close)}
                                </button>
                                <button
                                    type="button"
                                    class="min-h-10 rounded-md border border-[var(--accent)] px-3 py-2 text-sm font-semibold text-[var(--accent)] transition hover:bg-[var(--accent)]/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                                    on:click=copy_cover_letter
                                >
                                    {move || if cover_letter_copied.get() {
                                        t_string!(i18n, common_copied)
                                    } else {
                                        t_string!(i18n, common_copy)
                                    }}
                                </button>
                            </div>
                        </footer>
                    </div>
                </div>
            }.into_any()
        } else {
            ().into_any()
        }}
    }
}
