use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CountdownState {
    Idle,
    Running,
    Paused,
    Finished,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Countdown {
    duration_ms: u64,
    state: CountdownState,
    started_at_ms: Option<f64>,
    paused_at_ms: Option<f64>,
    accumulated_ms: u64,
}

impl Countdown {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration_ms: duration.as_millis().min(u64::MAX as u128) as u64,
            state: CountdownState::Idle,
            started_at_ms: None,
            paused_at_ms: None,
            accumulated_ms: 0,
        }
    }

    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    pub fn state(&self) -> CountdownState {
        self.state
    }

    pub fn start(&mut self, now_ms: f64) {
        if self.duration_ms == 0 {
            self.state = CountdownState::Finished;
            return;
        }
        if matches!(self.state, CountdownState::Idle | CountdownState::Finished) {
            self.accumulated_ms = 0;
            self.paused_at_ms = None;
            self.started_at_ms = Some(now_ms);
            self.state = CountdownState::Running;
        }
    }

    pub fn pause(&mut self, now_ms: f64) {
        if self.state == CountdownState::Running {
            self.accumulated_ms = self.elapsed_ms(now_ms);
            self.paused_at_ms = Some(now_ms);
            self.started_at_ms = None;
            self.state = CountdownState::Paused;
        }
    }

    pub fn resume(&mut self, now_ms: f64) {
        if self.state == CountdownState::Paused {
            self.started_at_ms = Some(now_ms);
            self.paused_at_ms = None;
            self.state = CountdownState::Running;
        }
    }

    pub fn reset(&mut self) {
        self.state = CountdownState::Idle;
        self.started_at_ms = None;
        self.paused_at_ms = None;
        self.accumulated_ms = 0;
    }

    pub fn remaining_ms(&mut self, now_ms: f64) -> u64 {
        let elapsed = self.elapsed_ms(now_ms);
        let remaining = self.duration_ms.saturating_sub(elapsed);
        if remaining == 0 && self.state == CountdownState::Running {
            self.state = CountdownState::Finished;
            self.started_at_ms = None;
        }
        remaining
    }

    fn elapsed_ms(&self, now_ms: f64) -> u64 {
        let running = self
            .started_at_ms
            .map(|started| (now_ms - started).max(0.0) as u64)
            .unwrap_or(0);
        self.accumulated_ms.saturating_add(running)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_uses_time_difference() {
        let mut timer = Countdown::new(Duration::from_secs(10));
        timer.start(1_000.0);
        assert_eq!(timer.remaining_ms(4_000.0), 7_000);
    }

    #[test]
    fn pause_and_resume_preserve_elapsed_time() {
        let mut timer = Countdown::new(Duration::from_secs(10));
        timer.start(1_000.0);
        timer.pause(4_000.0);
        assert_eq!(timer.remaining_ms(9_000.0), 7_000);
        timer.resume(9_000.0);
        assert_eq!(timer.remaining_ms(10_000.0), 6_000);
    }

    #[test]
    fn countdown_finishes_at_zero() {
        let mut timer = Countdown::new(Duration::from_secs(1));
        timer.start(0.0);
        assert_eq!(timer.remaining_ms(1_000.0), 0);
        assert_eq!(timer.state(), CountdownState::Finished);
    }
}
