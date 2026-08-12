use crate::application::ports::TimeProvider;
use crate::domain::time::TimestampUnit;

#[derive(Clone, Copy, Default)]
pub struct BrowserTimeProvider;

impl TimeProvider for BrowserTimeProvider {
    fn now_ms(&self) -> f64 {
        super::now_ms()
    }

    fn format_datetime(&self, millis: f64, timezone: &str) -> Result<String, String> {
        super::format_clock(millis, timezone)
    }

    fn timestamp_to_datetime(
        &self,
        value: &str,
        unit: TimestampUnit,
        timezone: &str,
    ) -> Result<String, String> {
        super::timestamp_to_datetime(value, unit, timezone)
    }

    fn datetime_to_timestamp(
        &self,
        value: &str,
        unit: TimestampUnit,
        timezone: &str,
    ) -> Result<String, String> {
        super::datetime_to_timestamp(value, unit, timezone)
    }
}
