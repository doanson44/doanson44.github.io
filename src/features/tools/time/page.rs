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
        <div class="d-flex flex-column flex-grow-1 overflow-hidden">
            <div class="px-3 py-2 border-bottom border-secondary bg-body-tertiary flex-shrink-0">
                <strong><i class="bi bi-clock-history text-primary me-2"></i>"Time & Utilities"</strong>
                <div class="small text-body-secondary">"World clock, countdown, stopwatch, ruler, and timestamp conversion."</div>
            </div>
            <div class="d-flex flex-column flex-lg-row flex-grow-1 overflow-hidden">
                <TimeNavigation state=state />
                <main class="flex-grow-1 overflow-auto p-3 p-lg-4">
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
        <nav class="flex-shrink-0" aria-label="Time utilities">
            <div class="d-flex flex-row flex-lg-column gap-1 overflow-auto p-2">
                {[TimeTab::WorldClock, TimeTab::Timer, TimeTab::Stopwatch, TimeTab::Ruler, TimeTab::Timestamp]
                    .into_iter()
                    .map(|item| view! {
                        <button type="button" class=move || if state.tab.get() == item { "btn btn-primary" } else { "btn btn-outline-secondary" } on:click=move |_| state.tab.set(item) aria-pressed=move || (state.tab.get() == item).to_string()>
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
            <div class="d-flex flex-wrap justify-content-between gap-3 mb-3">
                <div>
                    <h2 class="h4 mb-1">"World Clock"</h2>
                    <p class="text-body-secondary mb-0">"Track multiple cities with IANA timezone data."</p>
                </div>
                <div class="d-flex gap-2">
                    <select class="form-select form-select-sm" aria-label="Timezone to add" prop:value=move || selected.get() on:change=move |ev| selected.set(event_target_value(&ev))>
                        {choices.into_iter().map(|(city, zone)| view! { <option value=zone>{city}</option> }).collect_view()}
                    </select>
                    <button type="button" class="btn btn-primary btn-sm" on:click=move |_| {
                        if let Some((city, zone)) = choices.into_iter().find(|(_, zone)| *zone == selected.get_untracked()) {
                            state.clocks.update(|clocks| {
                                if !clocks.iter().any(|item| item.timezone == zone) {
                                    clocks.push(ClockEntry { city: city.into(), timezone: zone.into() });
                                }
                            });
                        }
                    }><i class="bi bi-plus-lg me-1"></i>"Add"</button>
                </div>
            </div>
            <div class="row g-3">
                {move || state.clocks.get().into_iter().enumerate().map(|(index, clock)| {
                    let timezone = clock.timezone.clone();
                    let city = clock.city.clone();
                    view! {
                        <div class="col-12 col-md-6">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body">
                                    <div class="d-flex justify-content-between gap-2">
                                        <div><h3 class="h6 mb-1">{city.clone()}</h3><span class="small text-body-secondary">{timezone.clone()}</span></div>
                                        <button type="button" class="btn btn-outline-danger btn-sm" title="Remove clock" aria-label=format!("Remove {city}") on:click=move |_| state.clocks.update(|clocks| if clocks.len() > 1 { clocks.remove(index); })><i class="bi bi-x-lg"></i></button>
                                    </div>
                                    <div class="fs-4 font-monospace mt-3" aria-live="polite">{move || provider.format_datetime(state.tick.get(), &timezone).unwrap_or_else(|_| "Unavailable".into())}</div>
                                </div>
                            </div>
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
            <h2 class="h4">"Countdown"</h2>
            <p class="text-body-secondary">"Uses timestamps as the source of truth."</p>
            {move || error.get().map(|message| view! { <div class="alert alert-danger text-start" role="alert">{message}</div> })}
            <div class="display-3 font-monospace my-4" aria-live="polite">{move || format_duration(remaining())}</div>
            <div class="d-flex flex-wrap justify-content-center gap-2 mb-3">
                {move || match state.countdown.get().state() {
                    CountdownState::Running => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.countdown.update(|timer| timer.pause(state.tick.get_untracked()))>"Pause"</button> }.into_any(),
                    CountdownState::Paused => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.countdown.update(|timer| timer.resume(state.tick.get_untracked()))>"Resume"</button> }.into_any(),
                    _ => view! { <button type="button" class="btn btn-primary" on:click=move |_| start()>"Start"</button> }.into_any(),
                }}
                <button type="button" class="btn btn-outline-secondary" on:click=move |_| state.countdown.update(|timer| timer.reset())>"Reset"</button>
            </div>
            <div class="d-flex flex-wrap justify-content-center gap-2 mb-3">
                {[1u64, 5, 10, 25].into_iter().map(|minutes| view! {
                    <button type="button" class="btn btn-outline-secondary" on:click=move |_| {
                        state.timer_hours.set("00".into());
                        state.timer_minutes.set(format!("{minutes:02}"));
                        state.timer_seconds.set("00".into());
                        start();
                    }>{format!("{minutes} min")}</button>
                }).collect_view()}
            </div>
            <div class="row g-2 mx-auto time-input-row">
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
        <div class="col-4">
            <label class="form-label small text-body-secondary" for=id.clone()>{label}</label>
            <input id=id type="number" min="0" max=if label == "Hours" { "99" } else { "59" } class="form-control text-center font-monospace" prop:value=move || value.get() on:input=move |ev| value.set(event_target_value(&ev)) />
        </div>
    }
}

#[component]
fn Stopwatch(state: TimeState) -> impl IntoView {
    let elapsed = move || state.stopwatch.get().elapsed(state.tick.get()).as_millis() as u64;
    view! {
        <section class="text-center">
            <h2 class="h4">"Stopwatch"</h2>
            <p class="text-body-secondary">"Measure elapsed time with lap splits."</p>
            <div class="display-3 font-monospace my-4" aria-live="polite">{move || format_stopwatch(elapsed())}</div>
            <div class="d-flex flex-wrap justify-content-center gap-2 mb-4">
                {move || if state.stopwatch.get().state() == StopwatchState::Running {
                    view! { <button type="button" class="btn btn-primary" on:click=move |_| state.stopwatch.update(|watch| watch.pause(state.tick.get_untracked()))>"Pause"</button> }.into_any()
                } else {
                    view! { <button type="button" class="btn btn-primary" on:click=move |_| state.stopwatch.update(|watch| watch.start(state.tick.get_untracked()))>"Start / Resume"</button> }.into_any()
                }}
                <button type="button" class="btn btn-outline-primary" disabled=move || state.stopwatch.get().state() != StopwatchState::Running on:click=move |_| state.stopwatch.update(|watch| { watch.lap(state.tick.get_untracked()); })>"Lap"</button>
                <button type="button" class="btn btn-outline-secondary" on:click=move |_| state.stopwatch.update(|watch| watch.reset())>"Reset"</button>
            </div>
            <div class="table-responsive mx-auto time-laps-table">
                <table class="table table-sm"><thead><tr><th scope="col">"Lap"</th><th scope="col">"Split"</th><th scope="col">"Total"</th></tr></thead><tbody>
                    {move || state.stopwatch.get().laps().iter().enumerate().map(|(index, total)| {
                        let previous = if index == 0 { 0 } else { state.stopwatch.get().laps()[index - 1] };
                        view! { <tr><th scope="row">{index + 1}</th><td class="font-monospace">{format_stopwatch(total.saturating_sub(previous))}</td><td class="font-monospace">{format_stopwatch(*total)}</td></tr> }
                    }).collect_view()}
                </tbody></table>
            </div>
        </section>
    }
}

#[component]
fn Ruler(state: TimeState) -> impl IntoView {
    let _ = state;
    view! {
        <section class="w-100">
            <div class="mb-4">
                <h2 class="h4">"Screen Ruler"</h2>
                <p class="text-body-secondary mb-0">"Two responsive scales rendered across the full available width."</p>
            </div>

            <div class="w-100 border border-secondary rounded overflow-hidden" role="img" aria-label="Responsive screen ruler with inches above centimeters">
                <div class="d-flex flex-column w-100">
                    <div class="d-flex w-100 border-bottom border-secondary" aria-label="Inches">
                        {(0..=10).map(|value| view! {
                            <div class="flex-fill text-center border-start border-secondary py-2">
                                <div class="ruler-tick ruler-tick-inch mx-auto" aria-hidden="true"></div>
                                <span class="small font-monospace">{value}</span>
                            </div>
                        }).collect_view()}
                    </div>
                    <div class="d-flex w-100" aria-label="Centimeters">
                        {(0..=30).map(|value| view! {
                            <div class="flex-fill text-center border-start border-secondary py-2">
                                <div class="ruler-tick ruler-tick-cm mx-auto" aria-hidden="true"></div>
                                <span class="small font-monospace">{value}</span>
                            </div>
                        }).collect_view()}
                    </div>
                </div>
            </div>

            <div class="d-flex justify-content-between gap-2 mt-3">
                <span class="small text-body-secondary">"Top: inch · Bottom: cm"</span>
                <span class="small text-body-secondary">"Screen scale"</span>
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
            <h2 class="h4">"Timestamp Converter"</h2>
            <p class="text-body-secondary">"Select the conversion direction, unit, and timezone."</p>
            <div class="row g-3">
                <div class="col-12 col-lg-4"><label class="form-label" for="timestamp-direction">"Conversion"</label><select id="timestamp-direction" class="form-select" prop:value=move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { "to-date" } else { "to-timestamp" } on:change=move |ev| state.timestamp_direction.set(if event_target_value(&ev) == "to-date" { TimestampDirection::TimestampToDateTime } else { TimestampDirection::DateTimeToTimestamp })><option value="to-date">"Unix Timestamp → Date/Time"</option><option value="to-timestamp">"Date/Time → Unix Timestamp"</option></select></div>
                <div class="col-12 col-sm-6 col-lg-4"><label class="form-label" for="timestamp-unit">"Unit"</label><select id="timestamp-unit" class="form-select" prop:value=move || if state.timestamp_unit.get() == TimestampUnit::Seconds { "seconds" } else { "milliseconds" } on:change=move |ev| state.timestamp_unit.set(if event_target_value(&ev) == "seconds" { TimestampUnit::Seconds } else { TimestampUnit::Milliseconds })><option value="seconds">"Seconds"</option><option value="milliseconds">"Milliseconds"</option></select></div>
                <div class="col-12 col-sm-6 col-lg-4"><label class="form-label" for="timestamp-timezone">"Timezone"</label><select id="timestamp-timezone" class="form-select" prop:value=move || state.timestamp_timezone.get() on:change=move |ev| state.timestamp_timezone.set(event_target_value(&ev))><option value="Local">"Local"</option><option value="UTC">"UTC"</option></select></div>
                <div class="col-12"><label class="form-label" for="timestamp-input">"Value"</label><input id="timestamp-input" class="form-control font-monospace" prop:value=move || state.timestamp_input.get() on:input=move |ev| state.timestamp_input.set(event_target_value(&ev)) /></div>
            </div>
            <div class="card bg-body-tertiary border-secondary mt-4"><div class="card-body"><h3 class="h6">"Result"</h3><div class="font-monospace text-break">{move || match result() { Ok(value) => value, Err(message) => message }}</div></div></div>
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
