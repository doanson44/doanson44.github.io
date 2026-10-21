use gloo_timers::callback::{Interval, Timeout};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::domain::account_manager::{generate_totp, Account, Environment, OtpMode, Project};
use crate::features::tools::accounts::state::AccountManagerState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn AccountManagerPage() -> impl IntoView {
    let state = AccountManagerState::new();
    let modal = RwSignal::new(None::<&'static str>);
    let modal_snapshot = RwSignal::new(None::<AccountManagerSnapshot>);
    let copy_toast = RwSignal::new(None::<String>);
    let copy_toast_visible = RwSignal::new(false);
    let copy_toast_generation = RwSignal::new(0_u64);
    let now = state.now_seconds;
    Interval::new(1_000, move || {
        now.set((js_sys::Date::now() / 1000.0) as u64);
    })
    .forget();

    view! {
        <div class="mx-auto flex w-full max-w-7xl flex-1 flex-col gap-4 px-4 py-6 sm:px-6 lg:px-8">
            <header class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <div>
                    <h1 class="text-2xl font-bold text-[var(--text-primary)]">"Project & Account Manager"</h1>
                    <p class="mt-1 text-sm text-[var(--text-secondary)]">"Manage project environments, credentials, and OTPs locally in this browser."</p>
                </div>
                <div class="flex flex-wrap gap-2">
                    <button type="button" class=button_class() on:click=move |_| export_json_file(&state)>"Export JSON"</button>
                    <button type="button" class=button_class() on:click=move |_| {
                        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                            if let Some(input) = document.get_element_by_id("accounts-json-import")
                                .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                            {
                                input.click();
                            }
                        }
                    }>"Import JSON"</button>
                    <input id="accounts-json-import" type="file" accept=".json,application/json" class="hidden"
                        on:change=move |event: web_sys::Event| {
                            let input = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok());
                            let file = input.as_ref().and_then(|element| element.files()).and_then(|files| files.get(0));
                            let input_for_reset = input;
                            if let Some(file) = file {
                                leptos::task::spawn_local(async move {
                                    match JsFuture::from(file.text()).await {
                                        Ok(contents) => match contents.as_string() {
                                            Some(json) => state.import_json(&json),
                                            None => state.message.set(Some("Could not read the selected JSON file.".into())),
                                        },
                                        Err(_) => state.message.set(Some("Could not read the selected JSON file.".into())),
                                    }
                                    if let Some(input) = input_for_reset {
                                        input.set_value("");
                                    }
                                });
                            }
                        } />
                    <button type="button" class=primary_button_class() on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.add_project(); modal.set(Some("project")); }>"+ New Project"</button>
                </div>
            </header>

            {move || state.message.get().map(|message| view! {
                <div class="flex items-start justify-between gap-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-secondary)]" role="status">
                    <span>{message}</span>
                    <button type="button" class="font-semibold text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.message.set(None) aria-label="Dismiss message">"×"</button>
                </div>
            })}

            {move || copy_toast.get().map(|message| view! {
                <div class=move || if copy_toast_visible.get() {
                    "pointer-events-none fixed bottom-5 right-5 z-[100] flex max-w-[calc(100vw-2.5rem)] items-center gap-2 rounded-lg border border-[var(--success)]/40 bg-[var(--surface)] px-4 py-3 text-sm font-medium text-[var(--text-primary)] shadow-xl opacity-100 translate-y-0 transition-all duration-300"
                } else {
                    "pointer-events-none fixed bottom-5 right-5 z-[100] flex max-w-[calc(100vw-2.5rem)] items-center gap-2 rounded-lg border border-[var(--success)]/40 bg-[var(--surface)] px-4 py-3 text-sm font-medium text-[var(--text-primary)] shadow-xl opacity-0 translate-y-2 transition-all duration-300"
                } role="status" aria-live="polite">
                    <span class="flex h-5 w-5 items-center justify-center rounded-full bg-[var(--success)]/15 text-[var(--success)]" aria-hidden="true">"✓"</span>
                    <span>{message}</span>
                </div>
            })}

            <div class="grid min-h-0 flex-1 gap-4 lg:grid-cols-[220px_minmax(0,1fr)]">
                <aside class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3">
                    <div class="mb-3 flex items-center justify-between gap-2">
                        <h2 class="text-sm font-semibold uppercase tracking-wide text-[var(--text-secondary)]">"Projects"</h2>
                        <button type="button" class="rounded px-2 py-1 text-sm font-semibold text-[var(--accent)] hover:bg-[var(--surface-hover)]" on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.add_project(); modal.set(Some("project")); } aria-label="Add project">"+" </button>
                    </div>
                    <div class="flex flex-col gap-1">
                        <For
                            each=move || state.projects.get()
                            key=|project| project.id.clone()
                            children=move |project| {
                                let id = project.id.clone();
                                let id_for_class = id.clone();
                                let id_for_select = id.clone();
                                let id_for_delete = id.clone();
                                let id_for_edit = id.clone();
                                let id_for_name = id.clone();
                                let id_for_environment_count = id.clone();
                                let id_for_delete_label = id.clone();
                                let id_for_edit_label = id.clone();
                                let id_for_move_up = id.clone();
                                let id_for_move_down = id.clone();
                                let id_for_move_up_label = id.clone();
                                let id_for_move_down_label = id.clone();
                                let id_for_move_up_disabled = id.clone();
                                let id_for_move_down_disabled = id.clone();
                                view! {
                                    <div class=move || if state.selected_project.get().as_deref() == Some(id_for_class.as_str()) { "flex items-start gap-1 rounded-md bg-[var(--accent)]/10 p-1 ring-1 ring-[var(--accent)]/40" } else { "flex items-start gap-1 rounded-md p-1 hover:bg-[var(--surface-hover)]" }>
                                        <div class="flex shrink-0 flex-col items-stretch gap-1">
                                        <button type="button" class="rounded px-2 py-1 text-xs text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-30" aria-label=move || format!("Move project {} up", project_by_id(&state, &id_for_move_up_label).map(|item| item.name).unwrap_or_default()) title="Move project up" disabled=move || state.projects.get().iter().position(|item| item.id == id_for_move_up_disabled).is_none_or(|index| index == 0) on:click=move |_| state.move_project(id_for_move_up.clone(), -1)>"↑"</button>
                                        <button type="button" class="rounded px-2 py-1 text-xs text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-30" aria-label=move || format!("Move project {} down", project_by_id(&state, &id_for_move_down_label).map(|item| item.name).unwrap_or_default()) title="Move project down" disabled=move || state.projects.get().iter().position(|item| item.id == id_for_move_down_disabled).is_none_or(|index| index + 1 >= state.projects.get().len()) on:click=move |_| state.move_project(id_for_move_down.clone(), 1)>"↓"</button>
                                        </div>
                                        <button type="button" class="min-w-0 flex-1 rounded px-2 py-1 text-left text-sm text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.select_project(id_for_select.clone())>
                                            <span class="block truncate font-semibold">{move || project_by_id(&state, &id_for_name).map(|item| item.name).unwrap_or_default()}</span>
                                            <span class="mt-1 block text-xs text-[var(--text-secondary)]">{move || project_by_id(&state, &id_for_environment_count).map(|item| format!("{} environments", item.environments.len())).unwrap_or_else(|| "0 environments".into())}</span>
                                        </button>
                                        <div class="flex shrink-0 flex-col items-stretch gap-1">
                                        <button type="button" class="shrink-0 rounded px-2 py-1 text-xs font-semibold text-[var(--accent)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label=move || format!("Edit project {}", project_by_id(&state, &id_for_edit_label).map(|item| item.name).unwrap_or_default()) title="Edit project" on:click=move |_| {
                                            modal_snapshot.set(Some(snapshot_state(&state)));
                                            state.defer_persistence.set(true);
                                            state.select_project(id_for_edit.clone());
                                            modal.set(Some("project"));
                                        }>"Edit"</button>
                                        <button type="button" class="shrink-0 rounded px-2 py-1 text-xs font-semibold text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label=move || format!("Delete project {}", project_by_id(&state, &id_for_delete_label).map(|item| item.name).unwrap_or_default()) title="Delete project" on:click=move |_| {
                                            if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this project and all its environments/accounts?").unwrap_or(false)) {
                                                state.select_project(id_for_delete.clone()); state.delete_selected_project();
                                                if modal.get() == Some("project") { modal.set(None); modal_snapshot.set(None); }
                                            }
                                        }>"Delete"</button>
                                        </div>
                                    </div>
                                }
                            }
                        />
                        <Show when=move || state.projects.get().is_empty()>
                            <p class="px-2 py-4 text-sm text-[var(--text-secondary)]">"No projects yet. Add one to begin."</p>
                        </Show>
                    </div>
                </aside>

                <main class="flex min-w-0 flex-col gap-4">
                    <Show when=move || modal.get() == Some("project")>
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/60 p-4" on:keydown=move |event: web_sys::KeyboardEvent| { if event.key() == "Escape" { cancel_modal(&state, modal, modal_snapshot); } }>
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Project settings">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                        <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                            <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Project settings"</h2>
                        </div>
                        <label class=label_class() for="account-project-name">"Project name"</label>
                        <input id="account-project-name" class=input_class() prop:value=move || selected_project(&state).map(|p| p.name).unwrap_or_default()
                            on:input=move |event| state.update_project("name", event_target_value(&event))
                            disabled=move || state.selected_project.get().is_none() />
                        <label class=label_class() for="account-project-description">"Description"</label>
                        <input id="account-project-description" class=input_class() prop:value=move || selected_project(&state).map(|p| p.description).unwrap_or_default()
                            on:input=move |event| state.update_project("description", event_target_value(&event))
                            disabled=move || state.selected_project.get().is_none() />
                        <div class="mt-5 flex justify-end gap-2 border-t border-[var(--border-color)] pt-4">
                            <button type="button" class=button_class() on:click=move |_| cancel_modal(&state, modal, modal_snapshot)>"Cancel"</button>
                            <button type="button" class=primary_button_class() on:click=move |_| { state.defer_persistence.set(false); state.save(); modal_snapshot.set(None); modal.set(None); }>"Save"</button>
                        </div>
                    </section>
                                </div>
                            </div>
                        </Show>

                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                        <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                            <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Environments"</h2>
                            <button type="button" class=primary_button_class() on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.add_environment(); modal.set(Some("environment")); } disabled=move || state.selected_project.get().is_none()>"+ Add Environment"</button>
                        </div>
                        <div class="grid gap-2 sm:grid-cols-2 xl:grid-cols-3">
                            <For
                                each=move || selected_project(&state).map(|p| p.environments).unwrap_or_default()
                                key=|environment| environment.id.clone()
                                children=move |environment| {
                                    let id = environment.id.clone();
                                    let id_for_class = id.clone();
                                    let id_for_select = id.clone();
                                    let id_for_card_click = id.clone();
                                    let id_for_edit_environment = id.clone();
                                    let id_for_delete_environment = id.clone();
                                    let id_for_name = id.clone();
                                    let id_for_account_count = id.clone();
                                    let id_for_url = id.clone();
                                    let id_for_url_display = id.clone();
                                    let id_for_url_title = id.clone();
                                    let id_for_delete_label = id.clone();
                                    view! {
                                        <div class=move || if state.selected_environment.get().as_deref() == Some(id_for_class.as_str()) {
                                            "cursor-pointer rounded-md border border-[var(--accent)] bg-[var(--accent)]/5 p-3"
                                        } else {
                                            "cursor-pointer rounded-md border border-[var(--border-color)] p-3 hover:bg-[var(--surface-hover)]"
                                        }
                                            on:click=move |_| state.select_environment(id_for_card_click.clone())>
                                            <div class="flex items-start justify-between gap-2">
                                                <button type="button" class="min-w-0 text-left font-semibold text-[var(--text-primary)] hover:text-[var(--accent)]"
                                                    on:click=move |_| state.select_environment(id_for_select.clone())>{move || environment_by_id(&state, &id_for_name).map(|item| item.name).unwrap_or_default()}</button>
                                                <div class="flex items-center gap-1">
                                                    <span class="shrink-0 rounded bg-[var(--surface-hover)] px-2 py-0.5 text-xs text-[var(--text-secondary)]">{move || environment_by_id(&state, &id_for_account_count).map(|item| item.accounts.len()).unwrap_or_default()}</span>
                                                    <button type="button" class="rounded px-2 py-1 text-xs text-[var(--accent)] hover:bg-[var(--surface-hover)]" on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.select_environment(id_for_edit_environment.clone()); modal.set(Some("environment")); }>"Edit"</button>
                                                    <button type="button" class="rounded px-2 py-1 text-xs text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label=move || format!("Delete environment {}", environment_by_id(&state, &id_for_delete_label).map(|item| item.name).unwrap_or_default()) on:click=move |event: web_sys::MouseEvent| { event.stop_propagation(); if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this environment and all its accounts?").unwrap_or(false)) { state.select_environment(id_for_delete_environment.clone()); state.delete_selected_environment(); } }>"Delete"</button>
                                                </div>
                                            </div>
                                            <button type="button" class="mt-2 block max-w-full truncate text-left text-xs text-[var(--accent)] underline underline-offset-2"
                                                title=move || environment_by_id(&state, &id_for_url_title).map(|item| item.url).unwrap_or_default()
                                                on:click=move |_| {
                                                    if let Some(item) = environment_by_id(&state, &id_for_url) {
                                                        open_environment_url(&item.url, state.message);
                                                    }
                                                }>{move || environment_by_id(&state, &id_for_url_display).map(|item| if item.url.is_empty() { "Set environment URL".to_string() } else { item.url }).unwrap_or_default()}</button>
                                        </div>
                                    }
                                }
                            />
                        </div>
                        <Show when=move || selected_project(&state).is_some_and(|project| project.environments.is_empty())>
                            <p class="py-3 text-sm text-[var(--text-secondary)]">"This project has no environments."</p>
                        </Show>
                    </section>

                    <Show when=move || selected_environment(&state).is_some()>
                        <Show when=move || modal.get() == Some("environment")>
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/60 p-4" on:keydown=move |event: web_sys::KeyboardEvent| { if event.key() == "Escape" { cancel_modal(&state, modal, modal_snapshot); } }>
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Environment settings">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Environment settings"</h2>
                                <span class="text-sm text-[var(--text-secondary)]">"Edit environment details"</span>
                            </div>
                            <div class="grid gap-3 md:grid-cols-2">
                                <div>
                                    <label class=label_class() for="account-environment-name">"Environment name"</label>
                                    <input id="account-environment-name" class=input_class() prop:value=move || selected_environment(&state).map(|e| e.name).unwrap_or_default()
                                        on:input=move |event| state.update_environment("name", event_target_value(&event)) />
                                </div>
                                <div>
                                    <label class=label_class() for="account-environment-url">"Environment URL"</label>
                                    <div class="flex gap-2">
                                        <input id="account-environment-url" type="url" class=input_class() prop:value=move || selected_environment(&state).map(|e| e.url).unwrap_or_default()
                                            on:input=move |event| state.update_environment("url", event_target_value(&event)) />
                                        <button type="button" class=button_class() on:click=move |_| {
                                            if let Some(environment) = selected_environment(&state) { open_environment_url(&environment.url, state.message); }
                                        }>"Open ↗"</button>
                                    </div>
                                </div>
                                <div class="md:col-span-2">
                                    <label class=label_class() for="account-environment-description">"Description"</label>
                                    <input id="account-environment-description" class=input_class() prop:value=move || selected_environment(&state).map(|e| e.description).unwrap_or_default()
                                        on:input=move |event| state.update_environment("description", event_target_value(&event)) />
                                </div>
                            </div>
                            <div class="mt-5 flex justify-end gap-2 border-t border-[var(--border-color)] pt-4">
                                <button type="button" class=button_class() on:click=move |_| cancel_modal(&state, modal, modal_snapshot)>"Cancel"</button>
                                <button type="button" class=primary_button_class() on:click=move |_| { state.defer_persistence.set(false); state.save(); modal_snapshot.set(None); modal.set(None); }>"Save"</button>
                            </div>
                        </section>
                                </div>
                            </div>
                        </Show>
                    </Show>

                    <Show when=move || selected_environment(&state).is_some()>
                        <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Accounts"</h2>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm text-[var(--text-secondary)]">{move || selected_environment(&state).map(|e| format!("{} accounts", e.accounts.len())).unwrap_or_default()}</span>
                                    <button type="button" class=primary_button_class() on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.add_account(); modal.set(Some("account")); }>"+ Add Account"</button>
                                </div>
                            </div>
                            <div class="grid gap-3 xl:grid-cols-2">
                                <For
                                    each=move || selected_environment(&state).map(|e| e.accounts).unwrap_or_default()
                                    key=|account| account.id.clone()
                                    children=move |account| {
                                        let id = account.id.clone();
                                        let id_for_edit = id.clone();
                                        let id_for_delete = id.clone();
                                        let id_for_name = id.clone();
                                        let id_for_name_select = id.clone();
                                        let id_for_username = id.clone();
                                        let id_for_username_title = id.clone();
                                        let id_for_username_copy = id.clone();
                                        let id_for_username_copy_disabled = id.clone();
                                        let id_for_password_copy = id.clone();
                                        let id_for_password_copy_disabled = id.clone();
                                        let id_for_password_display = id.clone();
                                        let id_for_otp = id.clone();
                                        let id_for_otp_copy = id.clone();
                                        let id_for_otp_disabled = id.clone();
                                        let id_for_totp = id.clone();
                                        let id_for_delete_label = id.clone();
                                        view! {
                                            <article class="min-w-0 rounded-md border border-[var(--border-color)] p-3">
                                                <div class="mb-3 flex items-center justify-between gap-2">
                                                    <button type="button" class="truncate text-left font-semibold text-[var(--text-primary)] hover:text-[var(--accent)]"
                                                        on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.selected_account.set(Some(id_for_name_select.clone())); modal.set(Some("account")); }>{move || account_by_id(&state, &id_for_name).map(|item| item.name).unwrap_or_default()}</button>
                                                    <div class="flex shrink-0 items-center gap-1">
                                                        <button type="button" class="rounded px-2 py-1 text-xs text-[var(--accent)] hover:bg-[var(--surface-hover)]" on:click=move |_| { modal_snapshot.set(Some(snapshot_state(&state))); state.defer_persistence.set(true); state.selected_account.set(Some(id_for_edit.clone())); modal.set(Some("account")); }>"Edit"</button>
                                                        <button type="button" class="rounded px-2 py-1 text-xs text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label=move || format!("Delete account {}", account_by_id(&state, &id_for_delete_label).map(|item| item.name).unwrap_or_default()) on:click=move |_| { if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this account?").unwrap_or(false)) { state.selected_account.set(Some(id_for_delete.clone())); state.delete_selected_account(); } }>"Delete"</button>
                                                    </div>
                                                </div>
                                                <div class="grid grid-cols-[5rem_minmax(0,1fr)_auto] items-center gap-2 py-1">
                                                    <span class="text-xs text-[var(--text-secondary)]">"Username"</span>
                                                    <span class="truncate font-mono text-xs text-[var(--text-primary)]" title=move || account_by_id(&state, &id_for_username_title).map(|item| item.username).unwrap_or_default()>{move || account_by_id(&state, &id_for_username).map(|item| if item.username.is_empty() { "—".into() } else { item.username }).unwrap_or_else(|| "—".into())}</span>
                                                    <button type="button" class=button_class() disabled=move || account_by_id(&state, &id_for_username_copy_disabled).is_none_or(|item| item.username.is_empty())
                                                        on:click=move |_| { if let Some(item) = account_by_id(&state, &id_for_username_copy) { copy_value(item.username, "Username", state.message, copy_toast, copy_toast_visible, copy_toast_generation); } }>"Copy"</button>
                                                </div>
                                                <div class="grid grid-cols-[5rem_minmax(0,1fr)_auto] items-center gap-2 py-1">
                                                    <span class="text-xs text-[var(--text-secondary)]">"Password"</span>
                                                    <span class="truncate font-mono text-xs text-[var(--text-primary)]" title="Password">{move || account_by_id(&state, &id_for_password_display).map(|item| if item.password.is_empty() { "—".to_string() } else { "••••••••".to_string() }).unwrap_or_else(|| "—".into())}</span>
                                                    <button type="button" class=button_class() disabled=move || account_by_id(&state, &id_for_password_copy_disabled).is_none_or(|item| item.password.is_empty())
                                                        on:click=move |_| { if let Some(item) = account_by_id(&state, &id_for_password_copy) { copy_value(item.password, "Password", state.message, copy_toast, copy_toast_visible, copy_toast_generation); } }>"Copy"</button>
                                                </div>
                                                <div class="grid grid-cols-[5rem_minmax(0,1fr)_auto] items-center gap-2 py-1">
                                                    <span class="text-xs text-[var(--text-secondary)]">"OTP"</span>
                                                    <span class="truncate font-mono text-xs text-[var(--text-primary)]">{move || account_by_id(&state, &id_for_otp).map(|item| account_otp(&item, state.now_seconds.get())).unwrap_or_else(|| "—".into())}</span>
                                                    <button type="button" class=button_class() disabled=move || account_by_id(&state, &id_for_otp_disabled).is_none_or(|item| item.otp_mode == OtpMode::None)
                                                        on:click=move |_| { if let Some(item) = account_by_id(&state, &id_for_otp_copy) { copy_value(account_otp(&item, state.now_seconds.get()), "OTP", state.message, copy_toast, copy_toast_visible, copy_toast_generation); } }>"Copy"</button>
                                                </div>
                                                <Show when=move || account_by_id(&state, &id_for_totp).is_some_and(|item| item.otp_mode == OtpMode::Totp)>
                                                    <p class="mt-1 text-right text-xs text-[var(--text-secondary)]">{move || format!("Refreshes in {}s", 30 - (state.now_seconds.get() % 30))}</p>
                                                </Show>
                                            </article>
                                        }
                                    }
                                />
                            </div>
                            <Show when=move || selected_environment(&state).is_some_and(|environment| environment.accounts.is_empty())>
                                <p class="py-4 text-sm text-[var(--text-secondary)]">"No accounts in this environment yet."</p>
                            </Show>
                        </section>
                    </Show>

                    <Show when=move || selected_account(&state).is_some()>
                        <Show when=move || modal.get() == Some("account")>
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/60 p-4" on:keydown=move |event: web_sys::KeyboardEvent| { if event.key() == "Escape" { cancel_modal(&state, modal, modal_snapshot); } }>
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Account details">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-4 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Account details"</h2>
                            </div>
                            <div class="grid gap-3 md:grid-cols-2">
                                <div>
                                    <label class=label_class() for="account-name">"Account name"</label>
                                    <input id="account-name" class=input_class() prop:value=move || selected_account(&state).map(|a| a.name).unwrap_or_default()
                                        on:input=move |event| state.update_account("name", event_target_value(&event)) />
                                </div>
                                <div>
                                    <label class=label_class() for="account-username">"Username"</label>
                                    <div class="flex gap-2">
                                        <input id="account-username" class=input_class() prop:value=move || selected_account(&state).map(|a| a.username).unwrap_or_default()
                                            on:input=move |event| state.update_account("username", event_target_value(&event)) />
                                        <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.username).unwrap_or_default(), "Username", state.message, copy_toast, copy_toast_visible, copy_toast_generation)>"Copy"</button>
                                    </div>
                                </div>
                                <div>
                                    <label class=label_class() for="account-password">"Password"</label>
                                    <div class="flex gap-2">
                                        <input id="account-password" type="password" class=input_class() prop:value=move || selected_account(&state).map(|a| a.password).unwrap_or_default()
                                            on:input=move |event| state.update_account("password", event_target_value(&event)) />
                                        <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.password).unwrap_or_default(), "Password", state.message, copy_toast, copy_toast_visible, copy_toast_generation)>"Copy"</button>
                                    </div>
                                </div>
                                <div>
                                    <label class=label_class() for="account-otp-mode">"OTP mode"</label>
                                    <select id="account-otp-mode" class=input_class()
                                        prop:value=move || selected_account(&state).map(|a| match a.otp_mode { OtpMode::Totp => "totp", OtpMode::Default => "default", OtpMode::None => "none" }.to_string()).unwrap_or_else(|| "none".into())
                                        on:change=move |event| state.update_account("otp_mode", event_target_value(&event))>
                                        <option value="totp">"Manual secret key / TOTP"</option>
                                        <option value="default">"Default OTP"</option>
                                        <option value="none">"None"</option>
                                    </select>
                                </div>
                                <Show when=move || selected_account(&state).is_some_and(|a| a.otp_mode == OtpMode::Totp)>
                                    <div class="md:col-span-2">
                                        <label class=label_class() for="account-otp-secret">"Base32 secret key"</label>
                                        <div class="flex gap-2">
                                            <input id="account-otp-secret" class=input_class() prop:value=move || selected_account(&state).map(|a| a.otp_secret).unwrap_or_default()
                                                on:input=move |event| state.update_account("secret", event_target_value(&event)) />
                                            <button type="button" class=button_class() on:click=move |_| {
                                                let bytes = *uuid::Uuid::new_v4().as_bytes();
                                                state.update_account("secret", crate::domain::account_manager::encode_base32(&bytes));
                                            }>"Generate Key"</button>
                                        </div>
                                    </div>
                                </Show>
                                <Show when=move || selected_account(&state).is_some_and(|a| a.otp_mode == OtpMode::Default)>
                                    <div>
                                        <label class=label_class() for="account-default-otp">"Default OTP"</label>
                                        <div class="flex gap-2">
                                            <input id="account-default-otp" class=input_class() prop:value=move || selected_account(&state).map(|a| a.default_otp).unwrap_or_default()
                                                on:input=move |event| state.update_account("default_otp", event_target_value(&event)) />
                                            <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.default_otp).unwrap_or_default(), "OTP", state.message, copy_toast, copy_toast_visible, copy_toast_generation)>"Copy"</button>
                                        </div>
                                    </div>
                                </Show>
                                <div class="md:col-span-2">
                                    <label class=label_class() for="account-current-otp">"Current OTP"</label>
                                    <div class="flex flex-wrap items-center gap-2">
                                        <output id="account-current-otp" class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2 font-mono text-lg font-semibold tracking-widest text-[var(--text-primary)]">
                                            {move || selected_account(&state).map(|account| account_otp(&account, state.now_seconds.get())).unwrap_or_default()}
                                        </output>
                                        <button type="button" class=button_class() disabled=move || selected_account(&state).map(|account| account.otp_mode == OtpMode::None).unwrap_or(true) on:click=move |_| copy_value(selected_account(&state).map(|account| account_otp(&account, state.now_seconds.get())).unwrap_or_default(), "OTP", state.message, copy_toast, copy_toast_visible, copy_toast_generation)>"Copy OTP"</button>
                                        <span class="text-xs text-[var(--text-secondary)]">{move || format!("Refreshes in {}s", 30 - (state.now_seconds.get() % 30))}</span>
                                    </div>
                                </div>
                                <div class="md:col-span-2">
                                    <label class=label_class() for="account-notes">"Notes"</label>
                                    <textarea id="account-notes" rows="3" class=input_class() prop:value=move || selected_account(&state).map(|a| a.notes).unwrap_or_default()
                                        on:input=move |event| state.update_account("notes", event_target_value(&event))></textarea>
                                </div>
                            </div>
                            <div class="mt-5 flex justify-end gap-2 border-t border-[var(--border-color)] pt-4">
                                <button type="button" class=button_class() on:click=move |_| cancel_modal(&state, modal, modal_snapshot)>"Cancel"</button>
                                <button type="button" class=primary_button_class() on:click=move |_| { state.defer_persistence.set(false); state.save(); modal_snapshot.set(None); modal.set(None); }>"Save"</button>
                            </div>
                        </section>
                                </div>
                            </div>
                        </Show>
                    </Show>

                </main>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct AccountManagerSnapshot { projects: Vec<Project>, selected_project: Option<String>, selected_environment: Option<String>, selected_account: Option<String> }

fn snapshot_state(state: &AccountManagerState) -> AccountManagerSnapshot {
    AccountManagerSnapshot { projects: state.projects.get_untracked(), selected_project: state.selected_project.get_untracked(), selected_environment: state.selected_environment.get_untracked(), selected_account: state.selected_account.get_untracked() }
}

fn cancel_modal(state: &AccountManagerState, modal: RwSignal<Option<&'static str>>, modal_snapshot: RwSignal<Option<AccountManagerSnapshot>>) {
    if let Some(snapshot) = modal_snapshot.get_untracked() {
        state.projects.set(snapshot.projects);
        state.selected_project.set(snapshot.selected_project);
        state.selected_environment.set(snapshot.selected_environment);
        state.selected_account.set(snapshot.selected_account);
    }
    state.defer_persistence.set(false);
    state.save();
    modal_snapshot.set(None);
    modal.set(None);
}

fn export_json_file(state: &AccountManagerState) {
    let json = match state.export_json() {
        Ok(json) => json,
        Err(error) => {
            state.message.set(Some(error));
            return;
        }
    };

    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(&json));
    let blob = match web_sys::Blob::new_with_str_sequence(&parts) {
        Ok(blob) => blob,
        Err(_) => {
            state.message.set(Some("Could not prepare the JSON file for download.".into()));
            return;
        }
    };
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(url) => url,
        Err(_) => {
            state.message.set(Some("Could not create a download URL for the JSON file.".into()));
            return;
        }
    };

    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        state.message.set(Some("Browser document is unavailable.".into()));
        let _ = web_sys::Url::revoke_object_url(&url);
        return;
    };
    let anchor = match document.create_element("a").and_then(|element| element.dyn_into::<web_sys::HtmlAnchorElement>().map_err(|_| {
        wasm_bindgen::JsValue::from_str("Could not create a download link.")
    })) {
        Ok(anchor) => anchor,
        Err(_) => {
            state.message.set(Some("Could not create a download link.".into()));
            let _ = web_sys::Url::revoke_object_url(&url);
            return;
        }
    };
    anchor.set_href(&url);
    anchor.set_download("accounts.json");
    anchor.click();
    Timeout::new(1_000, move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    })
    .forget();
    state.message.set(Some("JSON export downloaded.".into()));
}

fn copy_value(
    value: String,
    label: &'static str,
    message: RwSignal<Option<String>>,
    toast: RwSignal<Option<String>>,
    toast_visible: RwSignal<bool>,
    toast_generation: RwSignal<u64>,
) {
    leptos::task::spawn_local(async move {
        match copy_to_clipboard(&value).await {
            Ok(()) => {
                let generation = toast_generation.get_untracked().wrapping_add(1);
                toast_generation.set(generation);
                toast.set(Some(format!("{label} copied.")));
                toast_visible.set(true);

                Timeout::new(1_800, move || {
                    if toast_generation.get_untracked() != generation {
                        return;
                    }
                    toast_visible.set(false);
                    Timeout::new(300, move || {
                        if toast_generation.get_untracked() == generation {
                            toast.set(None);
                        }
                    })
                    .forget();
                })
                .forget();
            }
            Err(error) => message.set(Some(error)),
        }
    });
}

fn project_by_id(state: &AccountManagerState, id: &str) -> Option<Project> {
    state.projects.get().into_iter().find(|project| project.id == id)
}

fn environment_by_id(state: &AccountManagerState, id: &str) -> Option<Environment> {
    state.projects.get().into_iter()
        .flat_map(|project| project.environments)
        .find(|environment| environment.id == id)
}

fn account_by_id(state: &AccountManagerState, id: &str) -> Option<Account> {
    state.projects.get().into_iter()
        .flat_map(|project| project.environments)
        .flat_map(|environment| environment.accounts)
        .find(|account| account.id == id)
}

fn selected_project(state: &AccountManagerState) -> Option<Project> {
    let id = state.selected_project.get()?;
    state.projects.get().into_iter().find(|project| project.id == id)
}

fn selected_environment(state: &AccountManagerState) -> Option<Environment> {
    let project_id = state.selected_project.get()?;
    let environment_id = state.selected_environment.get()?;
    state.projects.get().into_iter()
        .find(|project| project.id == project_id)?
        .environments.into_iter().find(|environment| environment.id == environment_id)
}

fn selected_account(state: &AccountManagerState) -> Option<Account> {
    let project_id = state.selected_project.get()?;
    let environment_id = state.selected_environment.get()?;
    let account_id = state.selected_account.get()?;
    state.projects.get().into_iter()
        .find(|project| project.id == project_id)?
        .environments.into_iter().find(|environment| environment.id == environment_id)?
        .accounts.into_iter().find(|account| account.id == account_id)
}

fn account_otp(account: &Account, now_seconds: u64) -> String {
    match &account.otp_mode {
        OtpMode::Totp => generate_totp(&account.otp_secret, now_seconds).unwrap_or_else(|_| "Invalid secret".into()),
        OtpMode::Default => account.default_otp.clone(),
        OtpMode::None => "—".into(),
    }
}

fn open_environment_url(url: &str, message: RwSignal<Option<String>>) {
    let url = url.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        message.set(Some("Enter a valid http:// or https:// environment URL first.".into()));
        return;
    }
    let Some(window) = web_sys::window() else {
        message.set(Some("Browser window is unavailable.".into()));
        return;
    };
    // A null return value is expected with noopener in some browsers, so do not
    // mistake it for a blocked popup.
    if window.open_with_url_and_target_and_features(url, "_blank", "noopener,noreferrer").is_err() {
        message.set(Some("Could not open the environment URL.".into()));
    }
}

fn button_class() -> &'static str {
    "inline-flex items-center justify-center rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50"
}

fn primary_button_class() -> &'static str {
    "inline-flex items-center justify-center rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-semibold text-white hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50"
}

fn label_class() -> &'static str {
    "mb-1 mt-3 block text-sm font-medium text-[var(--text-secondary)]"
}

fn input_class() -> &'static str {
    "w-full min-w-0 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/25 disabled:opacity-50"
}
