use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::application::ports::TimeProvider;
use crate::application::services::time::TimeService;
use crate::domain::time::{CountdownState, StopwatchState, TimestampDirection, TimestampUnit};
use crate::features::tools::time::state::{ClockEntry, TimeState, TimeTab};
use crate::infrastructure::time::{local_timezone, BrowserTimeProvider};

#[component]
pub fn TimePage() -> impl IntoView {
    let state = TimeState::new();
    let provider = BrowserTimeProvider;
    state.tick.set(provider.now_ms());
    let tick = state.tick;
    let interval_id = window().and_then(|win| {
        let callback =
            Closure::wrap(Box::new(move || tick.set(provider.now_ms())) as Box<dyn FnMut()>);
        let result = win.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            100,
        );
        callback.forget();
        result.ok()
    });
    on_cleanup(move || {
        if let (Some(win), Some(id)) = (window(), interval_id) {
            win.clear_interval_with_handle(id);
        }
    });

    view! {
        <div class="flex flex-grow flex-col overflow-hidden">
            <div class="flex shrink-0 flex-col border-b border-[var(--border-color)] bg-[var(--surface)] px-3 py-2">
                <strong>"Time & Utilities"</strong>
                <div class="text-sm text-[var(--text-secondary)]">"World clock, countdown, stopwatch, ruler, and timestamp conversion."</div>
            </div>
            <div class="flex flex-grow flex-col overflow-hidden lg:flex-row">
                <TimeNavigation state=state />
                <main class="flex-grow overflow-auto p-3 lg:p-4">
                    {move || match state.tab.get() {
                        TimeTab::WorldClock => view! { <WorldClock state=state provider=provider /> }.into_any(),
                        TimeTab::Timer => view! { <Timer state=state /> }.into_any(),
                        TimeTab::Stopwatch => view! { <Stopwatch state=state /> }.into_any(),
                        TimeTab::Ruler => view! { <Ruler state=state /> }.into_any(),
                        TimeTab::Timestamp => view! { <Timestamp state=state provider=provider /> }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }
}

#[component]
fn TimeNavigation(state: TimeState) -> impl IntoView {
    view! {
        <nav class="shrink-0" aria-label="Time utilities">
            <div class="flex flex-row gap-1 overflow-auto p-2 lg:flex-col">
                {[TimeTab::WorldClock, TimeTab::Timer, TimeTab::Stopwatch, TimeTab::Ruler, TimeTab::Timestamp]
                    .into_iter()
                    .map(|item| view! {
                        <button
                            type="button"
                            class=move || if state.tab.get() == item {
                                "rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            } else {
                                "rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                            }
                            on:click=move |_| state.tab.set(item)
                            aria-pressed=move || (state.tab.get() == item).to_string()
                        >
                            {item.label()}
                        </button>
                    })
                    .collect_view()}
            </div>
        </nav>
    }
}

#[component]
fn WorldClock(state: TimeState, provider: BrowserTimeProvider) -> impl IntoView {
    let choices = [
        ("Singapore", "Asia/Singapore"),
        ("San Francisco", "America/Los_Angeles"),
        ("Sydney", "Australia/Sydney"),
        ("Berlin", "Europe/Berlin"),
        ("Dubai", "Asia/Dubai"),
        ("Seoul", "Asia/Seoul"),
    ];
    let selected = RwSignal::new("Asia/Singapore".to_string());

    view! {
        <section>
            <div class="mb-3 flex flex-wrap items-end justify-between gap-3">
                <div>
                    <h2 class="mb-1 text-xl font-semibold">"World Clock"</h2>
                    <p class="mb-0 text-sm text-[var(--text-secondary)]">"Track multiple cities with IANA timezone data."</p>
                </div>
                <div class="flex gap-2">
                    <select class="rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-sm text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" aria-label="Timezone to add" prop:value=move || selected.get() on:change=move |ev| selected.set(event_target_value(&ev))>
                        {choices.into_iter().map(|(city, zone)| view! { <option value=zone>{city}</option> }).collect_view()}
                    </select>
                    <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white shadow-sm hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| {
                        if let Some((city, zone)) = choices.into_iter().find(|(_, zone)| *zone == selected.get_untracked()) {
                            state.clocks.update(|clocks| {
                                if !clocks.iter().any(|item| item.timezone == zone) {
                                    clocks.push(ClockEntry { city: city.into(), timezone: zone.into() });
                                }
                            });
                        }
                    }>
                        "+"
                        " Add"
                    </button>
                </div>
            </div>
            <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
                {move || state.clocks.get().into_iter().enumerate().map(|(index, clock)| {
                    let timezone = clock.timezone.clone();
                    let city = clock.city.clone();
                    view! {
                        <div class="min-w-0 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4 shadow-sm">
                            <div class="flex items-start justify-between gap-2">
                                <div>
                                    <h3 class="mb-1 text-sm font-semibold">{city.clone()}</h3>
                                    <span class="text-xs text-[var(--text-secondary)]">{timezone.clone()}</span>
                                </div>
                                <button type="button" class="rounded-md border border-[var(--border-color)] px-2 py-1 text-sm text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" title="Remove clock" aria-label=format!("Remove {city}") on:click=move |_| state.clocks.update(|clocks| if clocks.len() > 1 { clocks.remove(index); })>
                                    "×"
                                </button>
                            </div>
                            <div class="mt-3 font-mono text-2xl" aria-live="polite">{move || provider.format_datetime(state.tick.get(), &timezone).unwrap_or_else(|_| "Unavailable".into())}</div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn Timer(state: TimeState) -> impl IntoView {
    let error = RwSignal::new(None::<String>);
    let remaining = move || {
        let mut timer = state.countdown.get_untracked();
        let before = timer.state();
        let value = timer.remaining_ms(state.tick.get());
        if timer.state() != before {
            state.countdown.set(timer);
        }
        value
    };
    let start = move || match state.set_timer_from_inputs() {
        Ok(()) => {
            error.set(None);
            state
                .countdown
                .update(|timer| timer.start(state.tick.get_untracked()));
        }
        Err(message) => error.set(Some(message)),
    };

    view! {
        <section class="text-center">
            <h2 class="text-xl font-semibold">"Countdown"</h2>
            <p class="text-sm text-[var(--text-secondary)]">"Uses timestamps as the source of truth."</p>
            {move || error.get().map(|message| view! { <div class="my-3 rounded-md border border-[var(--danger)]/40 bg-[var(--danger)]/10 px-3 py-2 text-left text-sm text-[var(--danger)]" role="alert">{message}</div> })}
            <div class="my-6 font-mono text-5xl" aria-live="polite">{move || format_duration(remaining())}</div>
            <div class="mb-3 flex flex-wrap justify-center gap-2">
                {move || match state.countdown.get().state() {
                    CountdownState::Running => view! { <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.countdown.update(|timer| timer.pause(state.tick.get_untracked()))>"Pause"</button> }.into_any(),
                    CountdownState::Paused => view! { <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.countdown.update(|timer| timer.resume(state.tick.get_untracked()))>"Resume"</button> }.into_any(),
                    _ => view! { <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| start()>"Start"</button> }.into_any(),
                }}
                <button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.countdown.update(|timer| timer.reset())>"Reset"</button>
            </div>
            <div class="mb-3 flex flex-wrap justify-center gap-2">
                {[1u64, 5, 10, 25].into_iter().map(|minutes| view! {
                    <button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| {
                        state.timer_hours.set("00".into());
                        state.timer_minutes.set(format!("{minutes:02}"));
                        state.timer_seconds.set("00".into());
                        start();
                    }>{format!("{minutes} min")}</button>
                }).collect_view()}
            </div>
            <div class="mx-auto grid max-w-md grid-cols-3 gap-2 time-input-row">
                <TimeInput label="Hours" value=state.timer_hours />
                <TimeInput label="Minutes" value=state.timer_minutes />
                <TimeInput label="Seconds" value=state.timer_seconds />
            </div>
        </section>
    }
}

#[component]
fn TimeInput(label: &'static str, value: RwSignal<String>) -> impl IntoView {
    let id = format!("time-{}", label.to_ascii_lowercase());
    view! {
        <div>
            <label class="mb-1 block text-xs text-[var(--text-secondary)]" for=id.clone()>{label}</label>
            <input id=id type="number" min="0" max=if label == "Hours" { "99" } else { "59" } class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-center font-mono text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || value.get() on:input=move |ev| value.set(event_target_value(&ev)) />
        </div>
    }
}

#[component]
fn Stopwatch(state: TimeState) -> impl IntoView {
    let elapsed = move || state.stopwatch.get().elapsed(state.tick.get()).as_millis() as u64;
    view! {
        <section class="text-center">
            <h2 class="text-xl font-semibold">"Stopwatch"</h2>
            <p class="text-sm text-[var(--text-secondary)]">"Measure elapsed time with lap splits."</p>
            <div class="my-6 font-mono text-5xl" aria-live="polite">{move || format_stopwatch(elapsed())}</div>
            <div class="mb-4 flex flex-wrap justify-center gap-2">
                {move || if state.stopwatch.get().state() == StopwatchState::Running {
                    view! { <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.stopwatch.update(|watch| watch.pause(state.tick.get_untracked()))>"Pause"</button> }.into_any()
                } else {
                    view! { <button type="button" class="rounded-md bg-[var(--accent)] px-3 py-2 text-sm font-medium text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.stopwatch.update(|watch| watch.start(state.tick.get_untracked()))>"Start / Resume"</button> }.into_any()
                }}
                <button type="button" class="rounded-md border border-[var(--accent)] px-3 py-2 text-sm font-medium text-[var(--accent)] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" disabled=move || state.stopwatch.get().state() != StopwatchState::Running on:click=move |_| state.stopwatch.update(|watch| { watch.lap(state.tick.get_untracked()); })>"Lap"</button>
                <button type="button" class="rounded-md border border-[var(--border-color)] px-3 py-2 text-sm font-medium text-[var(--text-primary)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]" on:click=move |_| state.stopwatch.update(|watch| watch.reset())>"Reset"</button>
            </div>
            <div class="mx-auto max-w-2xl overflow-x-auto time-laps-table">
                <table class="w-full border-collapse text-left text-sm">
                    <thead><tr class="border-b border-[var(--border-color)]"><th scope="col" class="px-3 py-2">"Lap"</th><th scope="col" class="px-3 py-2">"Split"</th><th scope="col" class="px-3 py-2">"Total"</th></tr></thead>
                    <tbody>
                        {move || state.stopwatch.get().laps().iter().enumerate().map(|(index, total)| {
                            let previous = if index == 0 { 0 } else { state.stopwatch.get().laps()[index - 1] };
                            view! { <tr class="border-b border-[var(--border-color)]"><th scope="row" class="px-3 py-2 font-normal">{index + 1}</th><td class="px-3 py-2 font-mono">{format_stopwatch(total.saturating_sub(previous))}</td><td class="px-3 py-2 font-mono">{format_stopwatch(*total)}</td></tr> }
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </section>
    }
}

#[component]
fn Ruler(state: TimeState) -> impl IntoView {
    let _ = state;
    view! {
        <section class="w-full">
            <div class="mb-4">
                <h2 class="text-xl font-semibold">"Screen Ruler"</h2>
                <p class="mb-0 text-sm text-[var(--text-secondary)]">"Two responsive scales rendered across the full available width."</p>
            </div>
            <div class="w-full overflow-hidden rounded-md border border-[var(--border-color)]" role="img" aria-label="Responsive screen ruler with inches above centimeters">
                <div class="flex w-full flex-col">
                    <div class="flex w-full border-b border-[var(--border-color)]" aria-label="Inches">
                        {(0..=10).map(|value| view! {
                            <div class="flex-1 border-l border-[var(--border-color)] py-2 text-center first:border-l-0">
                                <div class="ruler-tick ruler-tick-inch mx-auto" aria-hidden="true"></div>
                                <span class="font-mono text-xs">{value}</span>
                            </div>
                        }).collect_view()}
                    </div>
                    <div class="flex w-full" aria-label="Centimeters">
                        {(0..=30).map(|value| view! {
                            <div class="flex-1 border-l border-[var(--border-color)] py-2 text-center first:border-l-0">
                                <div class="ruler-tick ruler-tick-cm mx-auto" aria-hidden="true"></div>
                                <span class="font-mono text-xs">{value}</span>
                            </div>
                        }).collect_view()}
                    </div>
                </div>
            </div>
            <div class="mt-3 flex justify-between gap-2">
                <span class="text-xs text-[var(--text-secondary)]">"Top: inch · Bottom: cm"</span>
                <span class="text-xs text-[var(--text-secondary)]">"Screen scale"</span>
            </div>
        </section>
    }
}

#[component]
fn Timestamp(state: TimeState, provider: BrowserTimeProvider) -> impl IntoView {
    let result = move || {
        let direction = state.timestamp_direction.get();
        let unit = state.timestamp_unit.get();
        let timezone = state.timestamp_timezone.get();
        let zone = if timezone == "Local" {
            local_timezone()
        } else {
            timezone
        };
        match direction {
            TimestampDirection::TimestampToDateTime => TimeService::timestamp_to_datetime(
                &provider,
                &state.timestamp_input.get(),
                unit,
                &zone,
            ),
            TimestampDirection::DateTimeToTimestamp => TimeService::datetime_to_timestamp(
                &provider,
                &state.timestamp_input.get(),
                unit,
                &zone,
            ),
        }
    };
    view! {
        <section>
            <h2 class="text-xl font-semibold">"Timestamp Converter"</h2>
            <p class="text-sm text-[var(--text-secondary)]">"Select the conversion direction, unit, and timezone."</p>
            <div class="grid grid-cols-1 gap-3 lg:grid-cols-3">
                <div>
                    <label class="mb-1 block text-sm font-medium" for="timestamp-direction">"Conversion"</label>
                    <select id="timestamp-direction" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { "to-date" } else { "to-timestamp" } on:change=move |ev| state.timestamp_direction.set(if event_target_value(&ev) == "to-date" { TimestampDirection::TimestampToDateTime } else { TimestampDirection::DateTimeToTimestamp })>
                        <option value="to-date">"Unix Timestamp → Date/Time"</option><option value="to-timestamp">"Date/Time → Unix Timestamp"</option>
                    </select>
                </div>
                <div>
                    <label class="mb-1 block text-sm font-medium" for="timestamp-unit">"Unit"</label>
                    <select id="timestamp-unit" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || if state.timestamp_unit.get() == TimestampUnit::Seconds { "seconds" } else { "milliseconds" } on:change=move |ev| state.timestamp_unit.set(if event_target_value(&ev) == "seconds" { TimestampUnit::Seconds } else { TimestampUnit::Milliseconds })>
                        <option value="seconds">"Seconds"</option><option value="milliseconds">"Milliseconds"</option>
                    </select>
                </div>
                <div>
                    <label class="mb-1 block text-sm font-medium" for="timestamp-timezone">"Timezone"</label>
                    <select id="timestamp-timezone" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || state.timestamp_timezone.get() on:change=move |ev| state.timestamp_timezone.set(event_target_value(&ev))>
                        <option value="Local">"Local"</option><option value="UTC">"UTC"</option>
                    </select>
                </div>
                <div class="lg:col-span-3">
                    <label class="mb-1 block text-sm font-medium" for="timestamp-input">"Value"</label>
                    <input id="timestamp-input" class="w-full rounded-md border border-[var(--border-color)] bg-[var(--surface)] px-3 py-2 font-mono text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none focus:ring-2 focus:ring-[var(--accent)]/25" prop:value=move || state.timestamp_input.get() on:input=move |ev| state.timestamp_input.set(event_target_value(&ev)) />
                </div>
            </div>
            <div class="mt-4 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] p-4">
                <h3 class="mb-2 text-sm font-semibold">"Result"</h3>
                <div class="break-words font-mono text-sm">{move || match result() { Ok(value) => value, Err(message) => message }}</div>
            </div>
        </section>
    }
}

fn format_duration(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1_000;
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn format_stopwatch(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1_000;
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;
    let millis = milliseconds % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}
