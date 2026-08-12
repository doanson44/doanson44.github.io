use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopwatchState {
    Idle,
    Running,
    Paused,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stopwatch {
    state: StopwatchState,
    started_at_ms: Option<f64>,
    accumulated_ms: u64,
    laps: Vec<u64>,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            state: StopwatchState::Idle,
            started_at_ms: None,
            accumulated_ms: 0,
            laps: Vec::new(),
        }
    }

    pub fn state(&self) -> StopwatchState {
        self.state
    }

    pub fn elapsed(&self, now_ms: f64) -> Duration {
        Duration::from_millis(self.elapsed_ms(now_ms))
    }

    pub fn start(&mut self, now_ms: f64) {
        if self.state != StopwatchState::Running {
            self.started_at_ms = Some(now_ms);
            self.state = StopwatchState::Running;
        }
    }

    pub fn pause(&mut self, now_ms: f64) {
        if self.state == StopwatchState::Running {
            self.accumulated_ms = self.elapsed_ms(now_ms);
            self.started_at_ms = None;
            self.state = StopwatchState::Paused;
        }
    }

    pub fn lap(&mut self, now_ms: f64) -> Option<Duration> {
        if self.state != StopwatchState::Running {
            return None;
        }
        let elapsed = self.elapsed_ms(now_ms);
        let previous = self.laps.last().copied().unwrap_or(0);
        let split = elapsed.saturating_sub(previous);
        self.laps.push(elapsed);
        Some(Duration::from_millis(split))
    }

    pub fn laps(&self) -> &[u64] {
        &self.laps
    }

    pub fn reset(&mut self) {
        self.state = StopwatchState::Idle;
        self.started_at_ms = None;
        self.accumulated_ms = 0;
        self.laps.clear();
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
    fn stopwatch_tracks_elapsed_time() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start(1_000.0);
        assert_eq!(stopwatch.elapsed(4_500.0), Duration::from_millis(3_500));
    }

    #[test]
    fn laps_store_split_times() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start(0.0);
        assert_eq!(stopwatch.lap(2_000.0), Some(Duration::from_secs(2)));
        assert_eq!(stopwatch.lap(5_000.0), Some(Duration::from_secs(3)));
    }
}
