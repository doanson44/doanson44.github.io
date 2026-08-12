use crate::application::ports::TimeProvider;
use crate::domain::time::{Countdown, Stopwatch, TimestampUnit};

pub struct TimeService;

impl TimeService {
    pub fn timestamp_to_datetime<P: TimeProvider>(provider: &P, value: &str, unit: TimestampUnit, timezone: &str) -> Result<String, String> {
        provider.timestamp_to_datetime(value, unit, timezone)
    }

    pub fn datetime_to_timestamp<P: TimeProvider>(provider: &P, value: &str, unit: TimestampUnit, timezone: &str) -> Result<String, String> {
        provider.datetime_to_timestamp(value, unit, timezone)
    }

    pub fn remaining<P: TimeProvider>(provider: &P, timer: &mut Countdown) -> u64 {
        timer.remaining_ms(provider.now_ms())
    }

    pub fn elapsed<P: TimeProvider>(provider: &P, stopwatch: &Stopwatch) -> std::time::Duration {
        stopwatch.elapsed(provider.now_ms())
    }
}
