use leptos::prelude::*;

use super::data::{Education, Experience, SkillCategory};

/// Renders a labelled CV section with semantic heading structure.
#[component]
pub fn CvSection(
    id: &'static str,
    title: &'static str,
    #[prop(optional)] eyebrow: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <section id=id class="cv-section scroll-mt-24">
            <div class="mb-6 flex items-end justify-between gap-4 border-b border-[var(--border-color)] pb-3">
                <div>
                    {eyebrow.map(|value| view! {
                        <p class="mb-1 text-xs font-semibold uppercase tracking-[0.18em] text-[var(--accent)]">{value}</p>
                    })}
                    <h2 class="text-2xl font-semibold tracking-tight text-[var(--text-primary)]">{title}</h2>
                </div>
            </div>
            {children()}
        </section>
    }
}

/// Renders a professional experience entry.
#[component]
pub fn ExperienceCard(experience: Experience, index: usize) -> impl IntoView {
    let marker = if index == 0 { "Current" } else { "" };

    view! {
        <article class="relative pl-8 sm:pl-10">
            <div class="absolute left-0 top-1 flex h-5 w-5 items-center justify-center rounded-full border-2 border-[var(--accent)] bg-[var(--bg-primary)]" aria-hidden="true">
                <span class="h-2 w-2 rounded-full bg-[var(--accent)]"></span>
            </div>
            <div class="absolute bottom-0 left-[9px] top-6 w-px bg-[var(--border-color)]" aria-hidden="true"></div>

            <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 shadow-sm transition duration-150 hover:-translate-y-0.5 hover:bg-[var(--surface-hover)]">
                <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <div class="flex flex-wrap items-center gap-2">
                            <h3 class="text-lg font-semibold text-[var(--text-primary)]">{experience.company}</h3>
                            {(!marker.is_empty()).then(|| view! {
                                <span class="rounded-full border border-[color-mix(in_srgb,var(--accent)_35%,transparent)] bg-[color-mix(in_srgb,var(--accent)_12%,transparent)] px-2 py-0.5 text-xs font-semibold text-[var(--accent)]">"Current"</span>
                            })}
                        </div>
                        <p class="mt-1 font-medium text-[var(--accent)]">{experience.role}</p>
                    </div>
                    <time class="shrink-0 text-sm text-[var(--text-secondary)]">{experience.period}</time>
                </div>

                <p class="mt-4 text-sm leading-7 text-[var(--text-secondary)]">{experience.description}</p>

                {(!experience.projects.is_empty()).then(|| view! {
                    <div class="mt-5">
                        <h4 class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">"Representative Projects"</h4>
                        <ul class="mt-2 grid gap-1 text-sm text-[var(--text-secondary)] sm:grid-cols-2">
                            {experience.projects.into_iter().map(|project| view! {
                                <li class="flex gap-2">
                                    <span class="text-[var(--accent)]" aria-hidden="true">"•"</span>
                                    <span>{project}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>
                })}

                <div class="mt-5">
                    <h4 class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">"Key Contributions"</h4>
                    <ul class="mt-2 space-y-2 text-sm leading-6 text-[var(--text-secondary)]">
                        {experience.contributions.into_iter().map(|contribution| view! {
                            <li class="flex gap-2">
                                <span class="mt-2 h-1.5 w-1.5 shrink-0 rounded-full bg-[var(--accent)]" aria-hidden="true"></span>
                                <span>{contribution}</span>
                            </li>
                        }).collect_view()}
                    </ul>
                </div>

                {(!experience.technologies.is_empty()).then(|| view! {
                    <div class="mt-5 flex flex-wrap gap-2" aria-label="Technologies">
                        {experience.technologies.into_iter().map(|technology| view! {
                            <span class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-2.5 py-1 text-xs font-medium text-[var(--text-secondary)]">{technology}</span>
                        }).collect_view()}
                    </div>
                })}
            </div>
        </article>
    }
}

/// Renders one technical skill category.
#[component]
pub fn SkillGroup(category: SkillCategory) -> impl IntoView {
    view! {
        <article class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">{category.name}</h3>
            <div class="mt-3 flex flex-wrap gap-2">
                {category.skills.into_iter().map(|skill| view! {
                    <span class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-2.5 py-1 text-xs text-[var(--text-secondary)]">{skill}</span>
                }).collect_view()}
            </div>
        </article>
    }
}

/// Renders an education entry.
#[component]
pub fn EducationCard(value: Education) -> impl IntoView {
    view! {
        <article class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5">
            <div class="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                <div>
                    <h3 class="text-lg font-semibold text-[var(--text-primary)]">{value.institution}</h3>
                    <p class="mt-1 text-sm font-medium text-[var(--accent)]">{value.degree}</p>
                </div>
                <span class="w-fit rounded-md border border-[var(--border-color)] px-2.5 py-1 text-xs text-[var(--text-secondary)]">{value.classification}</span>
            </div>
            <p class="mt-3 text-sm text-[var(--text-secondary)]">"Major: "{value.major}</p>
        </article>
    }
}
