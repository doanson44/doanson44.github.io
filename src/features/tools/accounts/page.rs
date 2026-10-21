use gloo_timers::callback::Interval;
use leptos::prelude::*;

use crate::domain::account_manager::{generate_totp, Account, Environment, OtpMode, Project};
use crate::features::tools::accounts::state::AccountManagerState;
use crate::infrastructure::browser::copy_to_clipboard;

#[component]
pub fn AccountManagerPage() -> impl IntoView {
    let state = AccountManagerState::new();
    let modal = RwSignal::new(None::<&'static str>);
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
                    <button type="button" class=button_class() on:click=move |_| state.export_json()>"Export JSON"</button>
                    <button type="button" class=button_class() on:click=move |_| state.import_json()>"Import JSON"</button>
                    <button type="button" class=primary_button_class() on:click=move |_| { state.add_project(); modal.set(Some("project")); }>"+ New Project"</button>
                </div>
            </header>

            {move || state.message.get().map(|message| view! {
                <div class="flex items-start justify-between gap-3 rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-secondary)]" role="status">
                    <span>{message}</span>
                    <button type="button" class="font-semibold text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.message.set(None) aria-label="Dismiss message">"×"</button>
                </div>
            })}

            <div class="grid min-h-0 flex-1 gap-4 lg:grid-cols-[220px_minmax(0,1fr)]">
                <aside class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-3">
                    <div class="mb-3 flex items-center justify-between gap-2">
                        <h2 class="text-sm font-semibold uppercase tracking-wide text-[var(--text-secondary)]">"Projects"</h2>
                        <button type="button" class="rounded px-2 py-1 text-xs font-semibold text-[var(--accent)] hover:bg-[var(--surface-hover)] disabled:opacity-50" on:click=move |_| modal.set(Some("project")) disabled=move || state.selected_project.get().is_none()>"Edit"</button>
                        <button type="button" class="rounded px-2 py-1 text-sm font-semibold text-[var(--accent)] hover:bg-[var(--surface-hover)]" on:click=move |_| { state.add_project(); modal.set(Some("project")); } aria-label="Add project">"+" </button>
                    </div>
                    <div class="flex flex-col gap-1">
                        <For
                            each=move || state.projects.get()
                            key=|project| project.id.clone()
                            children=move |project| {
                                let id = project.id.clone();
                                let id_for_class = id.clone();
                                let id_for_select = id.clone();
                                view! {
                                    <button type="button"
                                        class=move || if state.selected_project.get().as_deref() == Some(id_for_class.as_str()) {
                                            "w-full rounded-md bg-[var(--accent)]/10 px-3 py-2 text-left text-sm font-semibold text-[var(--text-primary)] ring-1 ring-[var(--accent)]/40"
                                        } else {
                                            "w-full rounded-md px-3 py-2 text-left text-sm text-[var(--text-secondary)] hover:bg-[var(--surface-hover)]"
                                        }
                                        on:click=move |_| state.select_project(id_for_select.clone())>
                                        <span class="block truncate">{project.name.clone()}</span>
                                        <span class="mt-1 block text-xs opacity-75">{format!("{} environments", project.environments.len())}</span>
                                    </button>
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
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Project settings">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                        <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                            <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Project settings"</h2><button type="button" class="rounded px-2 py-1 text-lg font-semibold text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label="Close dialog" on:click=move |_| modal.set(None)>"×"</button>
                            <button type="button" class=button_class() on:click=move |_| {
                                if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this project and all its environments/accounts?").unwrap_or(false)) {
                                    state.delete_selected_project();
                                }
                            } disabled=move || state.selected_project.get().is_none()>"Delete Project"</button>
                        </div>
                        <label class=label_class() for="account-project-name">"Project name"</label>
                        <input id="account-project-name" class=input_class() prop:value=move || selected_project(&state).map(|p| p.name).unwrap_or_default()
                            on:input=move |event| state.update_project("name", event_target_value(&event))
                            disabled=move || state.selected_project.get().is_none() />
                        <label class=label_class() for="account-project-description">"Description"</label>
                        <input id="account-project-description" class=input_class() prop:value=move || selected_project(&state).map(|p| p.description).unwrap_or_default()
                            on:input=move |event| state.update_project("description", event_target_value(&event))
                            disabled=move || state.selected_project.get().is_none() />
                    </section>
                                </div>
                            </div>
                        </Show>

                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                        <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                            <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Environments"</h2>
                            <button type="button" class=primary_button_class() on:click=move |_| { state.add_environment(); modal.set(Some("environment")); } disabled=move || state.selected_project.get().is_none()>"+ Add Environment"</button>
                        </div>
                        <div class="grid gap-2 sm:grid-cols-2 xl:grid-cols-3">
                            <For
                                each=move || selected_project(&state).map(|p| p.environments).unwrap_or_default()
                                key=|environment| environment.id.clone()
                                children=move |environment| {
                                    let id = environment.id.clone();
                                    let id_for_class = id.clone();
                                    let id_for_select = id.clone();
                                    let id_for_edit_environment = id.clone();
                                    let url = environment.url.clone();
                                    let url_for_click = url.clone();
                                    let url_for_display = url.clone();
                                    view! {
                                        <div class=move || if state.selected_environment.get().as_deref() == Some(id_for_class.as_str()) {
                                            "rounded-md border border-[var(--accent)] bg-[var(--accent)]/5 p-3"
                                        } else {
                                            "rounded-md border border-[var(--border-color)] p-3 hover:bg-[var(--surface-hover)]"
                                        }>
                                            <div class="flex items-start justify-between gap-2">
                                                <button type="button" class="min-w-0 text-left font-semibold text-[var(--text-primary)] hover:text-[var(--accent)]"
                                                    on:click=move |_| state.select_environment(id_for_select.clone())>{environment.name.clone()}</button>
                                                <div class="flex items-center gap-1"><span class="shrink-0 rounded bg-[var(--surface-hover)] px-2 py-0.5 text-xs text-[var(--text-secondary)]">{environment.accounts.len()}</span><button type="button" class="rounded px-2 py-1 text-xs text-[var(--accent)] hover:bg-[var(--surface-hover)]" on:click=move |_| { state.select_environment(id_for_edit_environment.clone()); modal.set(Some("environment")); }>"Edit"</button></div>
                                            </div>
                                            <button type="button" class="mt-2 block max-w-full truncate text-left text-xs text-[var(--accent)] underline underline-offset-2"
                                                title=url.clone()
                                                on:click=move |_| open_environment_url(&url_for_click, state.message)>{if url_for_display.is_empty() { "Set environment URL".to_string() } else { url_for_display.clone() }}</button>
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
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Environment settings">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Environment settings"</h2><button type="button" class="rounded px-2 py-1 text-lg font-semibold text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label="Close dialog" on:click=move |_| modal.set(None)>"×"</button>
                                <div class="flex gap-2">
                                    <button type="button" class=button_class() on:click=move |_| {
                                        if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this environment and its accounts?").unwrap_or(false)) {
                                            state.delete_selected_environment();
                                        }
                                    }>"Delete Environment"</button>
                                    <button type="button" class=primary_button_class() on:click=move |_| { state.add_account(); modal.set(Some("account")); }>"+ Add Account"</button>
                                </div>
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
                        </section>
                                </div>
                            </div>
                        </Show>
                    </Show>

                    <Show when=move || selected_environment(&state).is_some()>
                        <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Accounts"</h2>
                                <span class="text-sm text-[var(--text-secondary)]">{move || selected_environment(&state).map(|e| format!("{} accounts", e.accounts.len())).unwrap_or_default()}</span>
                            </div>
                            <div class="grid gap-3 xl:grid-cols-2">
                                <For
                                    each=move || selected_environment(&state).map(|e| e.accounts).unwrap_or_default()
                                    key=|account| account.id.clone()
                                    children=move |account| {
                                        let id = account.id.clone();
                                        let id_for_edit = id.clone();
                                        let username = account.username.clone();
                                        let password = account.password.clone();
                                        let otp_account = account.clone();
                                        let otp_copy_account = account.clone();
                                        let otp_disabled_account = account.clone();
                                        let is_totp = account.otp_mode == OtpMode::Totp;
                                        let has_otp = account.otp_mode != OtpMode::None;
                                        view! {
                                            <article class="min-w-0 rounded-md border border-[var(--border-color)] p-3">
                                                <div class="mb-3 flex items-center justify-between gap-2">
                                                    <button type="button" class="truncate text-left font-semibold text-[var(--text-primary)] hover:text-[var(--accent)]"
                                                        on:click=move |_| { state.selected_account.set(Some(id.clone())); modal.set(Some("account")); }>{account.name.clone()}</button>
                                                    <button type="button" class="rounded px-2 py-1 text-xs text-[var(--accent)] hover:bg-[var(--surface-hover)]"
                                                        on:click=move |_| { state.selected_account.set(Some(id_for_edit.clone())); modal.set(Some("account")); }>"Edit"</button>
                                                </div>
                                                <CredentialRow label="Username" value=username on_copy=move |value| copy_value(value, "Username", state.message) />
                                                <CredentialRow label="Password" value=password on_copy=move |value| copy_value(value, "Password", state.message) />
                                                <div class="grid grid-cols-[5rem_minmax(0,1fr)_auto] items-center gap-2 py-1">
                                                    <span class="text-xs text-[var(--text-secondary)]">"OTP"</span>
                                                    <span class="truncate font-mono text-xs text-[var(--text-primary)]">{move || account_otp(&otp_account, state.now_seconds.get())}</span>
                                                    <button type="button" class=button_class() disabled=move || !has_otp || account_otp(&otp_disabled_account, state.now_seconds.get()).is_empty()
                                                        on:click=move |_| copy_value(account_otp(&otp_copy_account, state.now_seconds.get()), "OTP", state.message)>"Copy"</button>
                                                </div>
                                                <Show when=move || is_totp>
                                                    <p class="mt-1 text-right text-xs text-[var(--text-secondary)]">{format!("Refreshes in {}s", 30 - (state.now_seconds.get() % 30))}</p>
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
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
                                <div class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-lg bg-[var(--surface)] shadow-xl" role="dialog" aria-modal="true" aria-label="Account details">
                                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                            <div class="mb-4 flex flex-wrap items-center justify-between gap-2">
                                <h2 class="text-lg font-semibold text-[var(--text-primary)]">"Account details"</h2><button type="button" class="rounded px-2 py-1 text-lg font-semibold text-[var(--text-secondary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-[var(--accent)]" aria-label="Close dialog" on:click=move |_| modal.set(None)>"×"</button>
                                <button type="button" class=button_class() on:click=move |_| {
                                    if web_sys::window().is_some_and(|window| window.confirm_with_message("Delete this account?").unwrap_or(false)) {
                                        state.delete_selected_account();
                                    }
                                }>"Delete Account"</button>
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
                                        <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.username).unwrap_or_default(), "Username", state.message)>"Copy"</button>
                                    </div>
                                </div>
                                <div>
                                    <label class=label_class() for="account-password">"Password"</label>
                                    <div class="flex gap-2">
                                        <input id="account-password" type="password" class=input_class() prop:value=move || selected_account(&state).map(|a| a.password).unwrap_or_default()
                                            on:input=move |event| state.update_account("password", event_target_value(&event)) />
                                        <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.password).unwrap_or_default(), "Password", state.message)>"Copy"</button>
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
                                            <button type="button" class=button_class() on:click=move |_| copy_value(selected_account(&state).map(|a| a.default_otp).unwrap_or_default(), "OTP", state.message)>"Copy"</button>
                                        </div>
                                    </div>
                                </Show>
                                <div class="md:col-span-2">
                                    <label class=label_class() for="account-current-otp">"Current OTP"</label>
                                    <div class="flex flex-wrap items-center gap-2">
                                        <output id="account-current-otp" class="rounded-md border border-[var(--border-color)] bg-[var(--surface-hover)] px-3 py-2 font-mono text-lg font-semibold tracking-widest text-[var(--text-primary)]">
                                            {move || selected_account(&state).map(|account| account_otp(&account, state.now_seconds.get())).unwrap_or_default()}
                                        </output>
                                        <button type="button" class=button_class() disabled=move || selected_account(&state).map(|account| account.otp_mode == OtpMode::None).unwrap_or(true) on:click=move |_| copy_value(selected_account(&state).map(|account| account_otp(&account, state.now_seconds.get())).unwrap_or_default(), "OTP", state.message)>"Copy OTP"</button>
                                        <span class="text-xs text-[var(--text-secondary)]">{move || format!("Refreshes in {}s", 30 - (state.now_seconds.get() % 30))}</span>
                                    </div>
                                </div>
                                <div class="md:col-span-2">
                                    <label class=label_class() for="account-notes">"Notes"</label>
                                    <textarea id="account-notes" rows="3" class=input_class() prop:value=move || selected_account(&state).map(|a| a.notes).unwrap_or_default()
                                        on:input=move |event| state.update_account("notes", event_target_value(&event))></textarea>
                                </div>
                            </div>
                        </section>
                                </div>
                            </div>
                        </Show>
                    </Show>

                    <section class="rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                        <div class="mb-2 flex items-center justify-between gap-2">
                            <h2 class="text-sm font-semibold text-[var(--text-primary)]">"Backup / restore JSON"</h2>
                            <button type="button" class=button_class() on:click=move |_| copy_value(state.transfer_json.get_untracked(), "JSON", state.message)>"Copy JSON"</button>
                        </div>
                        <textarea rows="4" class=input_class() placeholder="Export fills this field; paste JSON here to import." prop:value=move || state.transfer_json.get()
                            on:input=move |event| state.transfer_json.set(event_target_value(&event))></textarea>
                        <p class="mt-2 text-xs text-[var(--text-secondary)]">"Data is saved to this browser's localStorage. Export a backup before clearing browser data."</p>
                    </section>
                </main>
            </div>
        </div>
    }
}

#[component]
fn CredentialRow(
    label: &'static str,
    value: String,
    on_copy: impl Fn(String) + 'static + Clone,
) -> impl IntoView {
    let copy_value = value.clone();
    view! {
        <div class="grid grid-cols-[5rem_minmax(0,1fr)_auto] items-center gap-2 py-1">
            <span class="text-xs text-[var(--text-secondary)]">{label}</span>
            <span class="truncate font-mono text-xs text-[var(--text-primary)]" title=value.clone()>{if value.is_empty() { "—".into() } else if label == "Password" { "••••••••".into() } else { value.clone() }}</span>
            <button type="button" class=button_class() disabled=copy_value.is_empty() on:click=move |_| on_copy(copy_value.clone())>"Copy"</button>
        </div>
    }
}

fn copy_value(value: String, label: &'static str, message: RwSignal<Option<String>>) {
    leptos::task::spawn_local(async move {
        match copy_to_clipboard(&value).await {
            Ok(()) => message.set(Some(format!("{label} copied."))),
            Err(error) => message.set(Some(error)),
        }
    });
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
