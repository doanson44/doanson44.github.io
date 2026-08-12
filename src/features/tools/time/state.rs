use leptos::prelude::*;

use crate::domain::time::{Countdown, Stopwatch, TimestampDirection, TimestampUnit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTab {
    WorldClock,
    Timer,
    Stopwatch,
    Ruler,
    Timestamp,
}

impl TimeTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::WorldClock => "World Clock",
            Self::Timer => "Timer",
            Self::Stopwatch => "Stopwatch",
            Self::Ruler => "Ruler",
            Self::Timestamp => "Timestamp",
        }
    }
}

#[derive(Clone, Copy)]
pub struct TimeState {
    pub tab: RwSignal<TimeTab>,
    pub tick: RwSignal<f64>,
    pub countdown: RwSignal<Countdown>,
    pub timer_hours: RwSignal<String>,
    pub timer_minutes: RwSignal<String>,
    pub timer_seconds: RwSignal<String>,
    pub stopwatch: RwSignal<Stopwatch>,
    pub timestamp_direction: RwSignal<TimestampDirection>,
    pub timestamp_unit: RwSignal<TimestampUnit>,
    pub timestamp_timezone: RwSignal<String>,
    pub timestamp_input: RwSignal<String>,
    pub ruler_unit: RwSignal<String>,
    pub ruler_orientation: RwSignal<String>,
    pub ruler_calibrated: RwSignal<bool>,
}

impl Default for TimeState {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeState {
    pub fn new() -> Self {
        Self {
            tab: RwSignal::new(TimeTab::WorldClock),
            tick: RwSignal::new(0.0),
            countdown: RwSignal::new(Countdown::new(std::time::Duration::from_secs(25 * 60))),
            timer_hours: RwSignal::new("00".into()),
            timer_minutes: RwSignal::new("25".into()),
            timer_seconds: RwSignal::new("00".into()),
            stopwatch: RwSignal::new(Stopwatch::new()),
            timestamp_direction: RwSignal::new(TimestampDirection::TimestampToDateTime),
            timestamp_unit: RwSignal::new(TimestampUnit::Seconds),
            timestamp_timezone: RwSignal::new("Local".into()),
            timestamp_input: RwSignal::new("0".into()),
            ruler_unit: RwSignal::new("px".into()),
            ruler_orientation: RwSignal::new("horizontal".into()),
            ruler_calibrated: RwSignal::new(false),
        }
    }

    pub fn set_timer_from_inputs(&self) -> Result<(), String> {
        let hours = parse_component(&self.timer_hours.get_untracked(), "hours")?;
        let minutes = parse_component(&self.timer_minutes.get_untracked(), "minutes")?;
        let seconds = parse_component(&self.timer_seconds.get_untracked(), "seconds")?;
        if minutes > 59 || seconds > 59 {
            return Err("Minutes and seconds must be between 0 and 59.".into());
        }
        let total = hours
            .saturating_mul(3_600)
            .saturating_add(minutes.saturating_mul(60))
            .saturating_add(seconds);
        self.countdown.set(Countdown::new(std::time::Duration::from_secs(total)));
        Ok(())
    }
}

fn parse_component(value: &str, label: &str) -> Result<u64, String> {
    value.trim().parse::<u64>().map_err(|_| format!("Enter a valid {label} value."))
}
