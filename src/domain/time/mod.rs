//! Pure domain logic for time utilities.

pub mod countdown;
pub mod stopwatch;
pub mod timestamp;

pub use countdown::{Countdown, CountdownState};
pub use stopwatch::{Stopwatch, StopwatchState};
pub use timestamp::{millis_to_timestamp, timestamp_to_millis, TimestampDirection, TimestampUnit};
