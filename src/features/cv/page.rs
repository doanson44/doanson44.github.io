use leptos::prelude::*;

use crate::infrastructure::browser::print_page;
use super::components::{CvSection, EducationCard, ExperienceCard, SkillGroup};
use super::data::{competencies, education, experiences, highlights, profile, skill_categories};

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
                                            "Show phone"
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
                                <Show
                                    when=move || show_email.get()
                                    fallback=move || view! {
                                        <button
                                            type="button"
                                            class="rounded-md px-2 py-1 text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus:outline-none focus:ring-2 focus:ring-[var(--accent)]"
                                            aria-label="Show email address"
                                            on:click=move |_| show_email.set(true)
                                        >
                                            "Show email"
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
                                <li><a class="cv-nav-link" href="#cv-about">"About"</a></li>
                                <li><a class="cv-nav-link" href="#cv-skills">"Skills"</a></li>
                                <li><a class="cv-nav-link" href="#cv-experience">"Experience"</a></li>
                                <li><a class="cv-nav-link" href="#cv-highlights">"Highlights"</a></li>
                                <li><a class="cv-nav-link" href="#cv-education">"Education"</a></li>
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
                            "Download PDF"
                        </button>
                    </div>
                </header>

                <div class="mt-12 space-y-12 sm:mt-16 sm:space-y-16">
                    <CvSection id="cv-about" title="Core Competencies" eyebrow="What I do">
                        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                            {competencies.into_iter().map(|item| view! {
                                <div class="flex gap-3 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-3">
                                    <span class="mt-2 h-2 w-2 shrink-0 rounded-full bg-[var(--accent)]" aria-hidden="true"></span>
                                    <span class="text-sm leading-6 text-[var(--text-secondary)]">{item.name}</span>
                                </div>
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-skills" title="Technical Skills" eyebrow="Technology">
                        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                            {skills.into_iter().map(|category| view! {
                                <SkillGroup category=category />
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-experience" title="Professional Experience" eyebrow="Career">
                        <div class="space-y-6">
                            {experiences.into_iter().map(|experience| view! {
                                <ExperienceCard experience=experience />
                            }).collect_view()}
                        </div>
                    </CvSection>

                    <CvSection id="cv-highlights" title="Selected Technical Highlights" eyebrow="Engineering focus">
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

                    <CvSection id="cv-education" title="Education" eyebrow="Academic background">
                        <div class="space-y-4">
                            {education.into_iter().map(|value| view! {
                                <EducationCard value=value />
                            }).collect_view()}
                        </div>
                    </CvSection>
                </div>
            </div>
        </main>
    }
}
