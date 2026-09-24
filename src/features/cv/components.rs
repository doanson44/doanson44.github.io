use leptos::prelude::*;

use super::data::{Education, Experience, SkillCategory};
use crate::i18n::*;

/// Renders a labelled CV section with semantic heading structure.
#[component]
pub fn CvSection(
    id: &'static str,
    title_key: &'static str,
    #[prop(optional)] eyebrow_key: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <section id=id class="cv-section scroll-mt-24">
            <div class="mb-6 flex items-end justify-between gap-4 border-b border-[var(--border-color)] pb-3">
                <div>
                    {eyebrow_key.map(|key| view! {
                        <p class="mb-1 text-xs font-semibold uppercase tracking-[0.18em] text-[var(--accent)]">{move || localized_cv_text(key)}</p>
                    })}
                    <h2 class="text-2xl font-semibold tracking-tight text-[var(--text-primary)]">{move || localized_cv_text(title_key)}</h2>
                </div>
            </div>
            {children()}
        </section>
    }
}

/// Renders a professional experience entry.
#[component]
pub fn ExperienceCard(experience: Experience) -> impl IntoView {
    let is_current = experience.period.ends_with("Present");

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
                            {is_current.then(|| view! {
                                <span class="rounded-full border border-[color-mix(in_srgb,var(--accent)_35%,transparent)] bg-[color-mix(in_srgb,var(--accent)_12%,transparent)] px-2 py-0.5 text-xs font-semibold text-[var(--accent)]">{move || localized_cv_text("current")}</span>
                            })}
                        </div>
                        <p class="mt-1 font-medium text-[var(--accent)]">{experience.role}</p>
                    </div>
                    <time class="shrink-0 text-sm text-[var(--text-secondary)]">{experience.period}</time>
                </div>

                <p class="mt-4 text-sm leading-7 text-[var(--text-secondary)]">{experience.description}</p>

                {(!experience.projects.is_empty()).then(|| view! {
                    <div class="mt-5">
                        <h4 class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">{move || localized_cv_text("representative_projects")}</h4>
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
                    <h4 class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">{move || localized_cv_text("key_contributions")}</h4>
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
                    <div class="mt-5 flex flex-wrap gap-2" aria-label=move || localized_cv_text("technologies")>
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
            <p class="mt-3 text-sm text-[var(--text-secondary)]">{move || format_major_label(value.major)}</p>
        </article>
    }
}

fn localized_cv_text(key: &str) -> String {
    let i18n = use_i18n();
    match key {
        "competencies" => t_string!(i18n, cv_competencies).to_string(),
        "skills" => t_string!(i18n, cv_skills).to_string(),
        "experience" => t_string!(i18n, cv_experience).to_string(),
        "highlights" => t_string!(i18n, cv_highlights).to_string(),
        "education" => t_string!(i18n, cv_education).to_string(),
        "what_i_do" => {
            if i18n.get_locale() == Locale::vi {
                "Tôi làm gì".into()
            } else {
                "What I do".into()
            }
        }
        "technology" => {
            if i18n.get_locale() == Locale::vi {
                "Công nghệ".into()
            } else {
                "Technology".into()
            }
        }
        "career" => {
            if i18n.get_locale() == Locale::vi {
                "Sự nghiệp".into()
            } else {
                "Career".into()
            }
        }
        "engineering_focus" => {
            if i18n.get_locale() == Locale::vi {
                "Trọng tâm kỹ thuật".into()
            } else {
                "Engineering focus".into()
            }
        }
        "current" => {
            if i18n.get_locale() == Locale::vi { "Hiện tại".into() } else { "Current".into() }
        }
        "representative_projects" => {
            if i18n.get_locale() == Locale::vi { "Dự án tiêu biểu".into() } else { "Representative Projects".into() }
        }
        "key_contributions" => {
            if i18n.get_locale() == Locale::vi { "Đóng góp chính".into() } else { "Key Contributions".into() }
        }
        "technologies" => {
            if i18n.get_locale() == Locale::vi { "Công nghệ".into() } else { "Technologies".into() }
        }
        "current" => {
            if i18n.get_locale() == Locale::vi { "Hiện tại".into() } else { "Current".into() }
        }
        "representative_projects" => {
            if i18n.get_locale() == Locale::vi { "Dự án tiêu biểu".into() } else { "Representative Projects".into() }
        }
        "key_contributions" => {
            if i18n.get_locale() == Locale::vi { "Đóng góp chính".into() } else { "Key Contributions".into() }
        }
        "technologies" => {
            if i18n.get_locale() == Locale::vi { "Công nghệ".into() } else { "Technologies".into() }
        }
        "academic_background" => {
            if i18n.get_locale() == Locale::vi {
                "Học vấn".into()
            } else {
                "Academic background".into()
            }
        }
        _ => String::new(),
    }
}


fn format_major_label(major: &str) -> String {
    let i18n = use_i18n();
    if i18n.get_locale() == Locale::vi {
        format!("Chuyên ngành: {major}")
    } else {
        format!("Major: {major}")
    }
}


fn format_major_label(major: &str) -> String {
    let i18n = use_i18n();
    if i18n.get_locale() == Locale::vi {
        format!("Chuyên ngành: {major}")
    } else {
        format!("Major: {major}")
    }
}
