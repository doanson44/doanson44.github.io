use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::application::ports::TimeProvider;
use crate::domain::time::{CountdownState, TimestampDirection, TimestampUnit, StopwatchState};
use crate::infrastructure::time::BrowserTimeProvider;
use crate::features::tools::time::state::{ClockEntry, TimeState, TimeTab};

#[component]
pub fn TimePage() -> impl IntoView {
    let state = TimeState::new();
    let provider = BrowserTimeProvider;
    state.tick.set(provider.now_ms());

    let tick = state.tick;
    let interval_id = window().and_then(|win| {
        let callback = Closure::wrap(Box::new(move || tick.set(provider.now_ms())) as Box<dyn FnMut()>);
        let result = win
            .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 100);
        callback.forget();
        result.ok()
    });
    on_cleanup(move || {
        if let (Some(win), Some(id)) = (window(), interval_id) {
            win.clear_interval_with_handle(id);
        }
    });

    let error = RwSignal::new(Option::<String>::None);
    let active_tab = state.tab;

    view! {
        <div class="time-page d-flex flex-column flex-grow-1 overflow-hidden">
            <div class="px-3 py-2 border-bottom border-secondary bg-body-tertiary flex-shrink-0">
                <div class="d-flex align-items-center gap-2"><i class="bi bi-clock-history text-primary"></i><strong>"Time & Utilities"</strong></div>
                <div class="small text-body-secondary">"World clock, countdown timer, stopwatch, ruler, and timestamp conversion."</div>
            </div>

            <div class="time-tool-shell d-flex flex-column flex-lg-row flex-grow-1 overflow-hidden">
                <nav class="time-tool-nav border-end-lg border-secondary flex-shrink-0" aria-label="Time utilities">
                    <div class="d-flex flex-row flex-lg-column overflow-auto p-2 gap-1">
                        {[
                            TimeTab::WorldClock,
                            TimeTab::Timer,
                            TimeTab::Stopwatch,
                            TimeTab::Ruler,
                            TimeTab::Timestamp,
                        ].into_iter().map(|tab| {
                            view! {
                                <button
                                    type="button"
                                    class=move || if active_tab.get() == tab { "btn btn-primary text-start time-tool-tab" } else { "btn btn-outline-secondary text-start time-tool-tab" }
                                    on:click=move |_| active_tab.set(tab)
                                    aria-pressed=move || (active_tab.get() == tab).to_string()
                                >
                                    {tab.label()}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </nav>

                <main class="flex-grow-1 overflow-auto p-3 p-lg-4">
                    {move || match active_tab.get() {
                        TimeTab::WorldClock => view! { <WorldClock state=state provider=provider /> }.into_any(),
                        TimeTab::Timer => view! { <CountdownTimer state=state error=error /> }.into_any(),
                        TimeTab::Stopwatch => view! { <StopwatchView state=state /> }.into_any(),
                        TimeTab::Ruler => view! { <RulerView state=state /> }.into_any(),
                        TimeTab::Timestamp => view! { <TimestampView state=state provider=provider /> }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }
}

#[component]
fn WorldClock(state: TimeState, provider: BrowserTimeProvider) -> impl IntoView {
    let add_timezone = RwSignal::new("Asia/Singapore".to_string());
    let choices = [
        ("Singapore", "Asia/Singapore"),
        ("San Francisco", "America/Los_Angeles"),
        ("Sydney", "Australia/Sydney"),
        ("Berlin", "Europe/Berlin"),
        ("Dubai", "Asia/Dubai"),
        ("Seoul", "Asia/Seoul"),
    ];

    view! {
        <section class="time-section">
            <div class="d-flex flex-wrap align-items-start justify-content-between gap-3 mb-3">
                <div><h2 class="h4 mb-1">"World Clock"</h2><p class="text-body-secondary mb-0">"Track multiple cities using the browser's IANA timezone data."</p></div>
                <div class="d-flex gap-2 align-items-center">
                    <label class="visually-hidden" for="add-timezone">"Timezone"</label>
                    <select id="add-timezone" class="form-select form-select-sm" prop:value=move || add_timezone.get() on:change=move |ev| add_timezone.set(event_target_value(&ev))>
                        {choices.into_iter().map(|(city, tz)| view! { <option value=tz>{city}</option> }).collect_view()}
                    </select>
                    <button type="button" class="btn btn-primary btn-sm text-nowrap" title="Add timezone" on:click=move |_| {
                        if let Some((city, tz)) = choices.into_iter().find(|(_, tz)| *tz == add_timezone.get_untracked()) {
                            state.clocks.update(|clocks| {
                                if !clocks.iter().any(|entry| entry.timezone == tz) {
                                    clocks.push(ClockEntry { city: city.into(), timezone: tz.into() });
                                }
                            });
                        }
                    }><i class="bi bi-plus-lg me-1"></i>"Add"</button>
                </div>
            </div>
            <div class="row g-3">
                {move || state.clocks.get().into_iter().enumerate().map(|(index, clock)| {
                    let city = clock.city.clone();
                    let timezone = clock.timezone.clone();
                    view! {
                        <div class="col-12 col-md-6">
                            <div class="card bg-body-tertiary border-secondary h-100">
                                <div class="card-body">
                                    <div class="d-flex justify-content-between gap-2">
                                        <div><h3 class="h6 mb-1">{city}</h3><div class="small text-body-secondary">{timezone.clone()}</div></div>
                                        <button type="button" class="btn btn-outline-danger btn-sm" title=format!("Remove {}", clock.city) aria-label=format!("Remove {}", clock.city) on:click=move |_| state.clocks.update(|clocks| { if clocks.len() > 1 { clocks.remove(index); } })><i class="bi bi-x-lg"></i></button>
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
fn CountdownTimer(state: TimeState, error: RwSignal<Option<String>>) -> impl IntoView {
    let set_duration = move || {
        if let Err(message) = state.set_timer_from_inputs() {
            error.set(Some(message));
        } else {
            error.set(None);
        }
    };
    let start = move || {
        set_duration();
        let now = state.tick.get_untracked();
        state.countdown.update(|timer| timer.start(now));
    };
    let remaining = move || {
        let now = state.tick.get();
        let mut timer = state.countdown.get();
        let value = timer.remaining_ms(now);
        state.countdown.set(timer);
        value
    };

    view! {
        <section class="time-section time-centered-section">
            <h2 class="h4 mb-1">"Countdown"</h2>
            <p class="text-body-secondary mb-4">"Set a duration and run it without losing accuracy when the browser throttles timers."</p>
            {move || error.get().map(|message| view! { <div class="alert alert-danger" role="alert">{message}</div> })}
            <div class="time-display font-monospace" aria-live="polite">{move || format_duration(remaining())}</div>
            <div class="d-flex justify-content-center gap-2 mb-4">
                {move || match state.countdown.get().state() {
                    CountdownState::Running => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.countdown.update(|timer| timer.pause(state.tick.get_untracked()))><i class="bi bi-pause-fill me-1"></i>"Pause"</button> }.into_any(),
                    CountdownState::Paused => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.countdown.update(|timer| timer.resume(state.tick.get_untracked()))><i class="bi bi-play-fill me-1"></i>"Resume"</button> }.into_any(),
                    _ => view! { <button type="button" class="btn btn-primary" on:click=move |_| start()><i class="bi bi-play-fill me-1"></i>"Start"</button> }.into_any(),
                }}
                <button type="button" class="btn btn-outline-secondary" on:click=move |_| state.countdown.update(|timer| timer.reset())>"Reset"</button>
            </div>
            <div class="row g-2 justify-content-center mb-3">
                {[("1 min", 1u64), ("5 min", 5), ("10 min", 10), ("25 min", 25)].into_iter().map(|(label, minutes)| view! { <div class="col-6 col-sm-auto"><button type="button" class="btn btn-outline-secondary w-100" on:click=move |_| { state.timer_hours.set("00".into()); state.timer_minutes.set(format!("{minutes:02}")); state.timer_seconds.set("00".into()); set_duration(); }>{label}</button></div> }).collect_view()}
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
fn StopwatchView(state: TimeState) -> impl IntoView {
    let elapsed = move || {
        let now = state.tick.get();
        state.stopwatch.get().elapsed(now).as_millis() as u64
    };
    view! {
        <section class="time-section time-centered-section">
            <h2 class="h4 mb-1">"Stopwatch"</h2>
            <p class="text-body-secondary mb-4">"Measure elapsed time with lap splits."</p>
            <div class="time-display font-monospace" aria-live="polite">{move || format_stopwatch(elapsed())}</div>
            <div class="d-flex justify-content-center gap-2 mb-4">
                {move || match state.stopwatch.get().state() {
                    StopwatchState::Running => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.stopwatch.update(|watch| watch.pause(state.tick.get_untracked()))>"Pause"</button> }.into_any(),
                    _ => view! { <button type="button" class="btn btn-primary" on:click=move |_| state.stopwatch.update(|watch| watch.start(state.tick.get_untracked()))>"Start / Resume"</button> }.into_any(),
                }}
                <button type="button" class="btn btn-outline-primary" disabled=move || state.stopwatch.get().state() != StopwatchState::Running on:click=move |_| state.stopwatch.update(|watch| { watch.lap(state.tick.get_untracked()); })>"Lap"</button>
                <button type="button" class="btn btn-outline-secondary" on:click=move |_| state.stopwatch.update(|watch| watch.reset())>"Reset"</button>
            </div>
            <div class="table-responsive mx-auto time-laps-table">
                <table class="table table-sm align-middle mb-0"><thead><tr><th scope="col">"Lap"</th><th scope="col">"Split"</th><th scope="col">"Total"</th></tr></thead>
                    <tbody>{move || state.stopwatch.get().laps().iter().enumerate().map(|(index, total)| {
                        let previous = if index == 0 { 0 } else { state.stopwatch.get().laps()[index - 1] };
                        view! { <tr><th scope="row">{index + 1}</th><td class="font-monospace">{format_stopwatch(total.saturating_sub(previous))}</td><td class="font-monospace">{format_stopwatch(*total)}</td></tr> }
                    }).collect_view()}</tbody>
                </table>
            </div>
        </section>
    }
}

#[component]
fn RulerView(state: TimeState) -> impl IntoView {
    view! {
        <section class="time-section">
            <div class="d-flex flex-wrap justify-content-between gap-3 mb-4">
                <div><h2 class="h4 mb-1">"Screen Ruler"</h2><p class="text-body-secondary mb-0">"CSS-pixel ruler for screen and layout measurements. Physical units require calibration."</p></div>
                <div class="d-flex gap-2">
                    <select class="form-select form-select-sm" aria-label="Ruler unit" prop:value=move || state.ruler_unit.get() on:change=move |ev| state.ruler_unit.set(event_target_value(&ev))>
                        <option value="px">"px"</option><option value="cm">"cm"</option><option value="inch">"inch"</option>
                    </select>
                    <select class="form-select form-select-sm" aria-label="Ruler orientation" prop:value=move || state.ruler_orientation.get() on:change=move |ev| state.ruler_orientation.set(event_target_value(&ev))>
                        <option value="horizontal">"Horizontal"</option><option value="vertical">"Vertical"</option>
                    </select>
                </div>
            </div>
            <div class=move || if state.ruler_orientation.get() == "vertical" { "screen-ruler screen-ruler-vertical" } else { "screen-ruler" } role="img" aria-label="Screen ruler with ten equal divisions">
                {(0..=10).map(|value| view! { <div class="ruler-mark"><span>{value}</span></div> }).collect_view()}
            </div>
            <div class="d-flex flex-wrap align-items-center justify-content-between gap-2 mt-3">
                <span class="small text-body-secondary">{move || if state.ruler_calibrated.get() { "Calibrated scale" } else { "Screen scale; physical size is approximate" }}</span>
                <button type="button" class="btn btn-outline-secondary btn-sm" on:click=move |_| state.ruler_calibrated.update(|value| *value = !*value)>{move || if state.ruler_calibrated.get() { "Clear calibration" } else { "Calibrate" }}</button>
            </div>
        </section>
    }
}

#[component]
fn TimestampView(state: TimeState, provider: BrowserTimeProvider) -> impl IntoView {
    let result = move || {
        let direction = state.timestamp_direction.get();
        let unit = state.timestamp_unit.get();
        let timezone = state.timestamp_timezone.get();
        match direction {
            TimestampDirection::TimestampToDateTime => provider.timestamp_to_datetime(&state.timestamp_input.get(), unit, if timezone == "Local" { local_timezone() } else { &timezone }),
            TimestampDirection::DateTimeToTimestamp => provider.datetime_to_timestamp(&state.timestamp_input.get(), unit, if timezone == "Local" { local_timezone() } else { &timezone }),
        }
    };

    view! {
        <section class="time-section">
            <h2 class="h4 mb-1">"Timestamp Converter"</h2>
            <p class="text-body-secondary mb-4">"Convert Unix timestamps and date/time values with explicit direction, unit, and timezone selection."</p>
            <div class="row g-3">
                <div class="col-12 col-lg-4">
                    <label class="form-label" for="timestamp-direction">"Conversion"</label>
                    <select id="timestamp-direction" class="form-select" prop:value=move || match state.timestamp_direction.get() { TimestampDirection::TimestampToDateTime => "to-date", TimestampDirection::DateTimeToTimestamp => "to-timestamp" } on:change=move |ev| state.timestamp_direction.set(if event_target_value(&ev) == "to-date" { TimestampDirection::TimestampToDateTime } else { TimestampDirection::DateTimeToTimestamp })>
                        <option value="to-date">"Unix Timestamp → Date/Time"</option>
                        <option value="to-timestamp">"Date/Time → Unix Timestamp"</option>
                    </select>
                </div>
                <div class="col-12 col-sm-6 col-lg-4">
                    <label class="form-label" for="timestamp-unit">"Unit"</label>
                    <select id="timestamp-unit" class="form-select" prop:value=move || match state.timestamp_unit.get() { TimestampUnit::Seconds => "seconds", TimestampUnit::Milliseconds => "milliseconds" } on:change=move |ev| state.timestamp_unit.set(if event_target_value(&ev) == "seconds" { TimestampUnit::Seconds } else { TimestampUnit::Milliseconds })>
                        <option value="seconds">"Seconds"</option><option value="milliseconds">"Milliseconds"</option>
                    </select>
                </div>
                <div class="col-12 col-sm-6 col-lg-4">
                    <label class="form-label" for="timestamp-timezone">"Timezone"</label>
                    <select id="timestamp-timezone" class="form-select" prop:value=move || state.timestamp_timezone.get() on:change=move |ev| state.timestamp_timezone.set(event_target_value(&ev))>
                        <option value="Local">"Local"</option><option value="UTC">"UTC"</option>
                    </select>
                </div>
                <div class="col-12">
                    <label class="form-label" for="timestamp-input">{move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { "Timestamp" } else { "Date / Time" }}</label>
                    <input id="timestamp-input" class="form-control font-monospace" type=move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { "text" } else { "datetime-local" } placeholder=move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { "1786546112" } else { "" } prop:value=move || state.timestamp_input.get() on:input=move |ev| state.timestamp_input.set(event_target_value(&ev)) />
                    <div class="form-text">{move || if state.timestamp_direction.get() == TimestampDirection::TimestampToDateTime { format!("Unix {}", state.timestamp_unit.get().label()) } else { "Use your local date/time or UTC selection above.".into() }}</div>
                </div>
            </div>
            <div class="card bg-body-tertiary border-secondary mt-4">
                <div class="card-body">
                    <div class="text-body-secondary small mb-1">"Result"</div>
                    <div class="font-monospace fs-5" aria-live="polite">{move || result().unwrap_or_else(|message| message)}</div>
                </div>
            </div>
        </section>
    }
}

fn local_timezone() -> String {
    js_sys::global()
        .unchecked_into::<js_sys::Object>();
    let date = js_sys::Date::new_0();
    let _ = date;
    "Asia/Ho_Chi_Minh".into()
}

fn format_duration(ms: u64) -> String {
    let total_seconds = ms / 1_000;
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn format_stopwatch(ms: u64) -> String {
    let minutes = ms / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let millis = ms % 1_000;
    format!("{minutes:02}:{seconds:02}.{millis:03}")
}
