use leptos::prelude::*;

use crate::domain::futures::TrackedFuturesTicker;
use crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus;

const DEFAULT_LIMIT: usize = 10;
const LIMIT_OPTIONS: [usize; 5] = [10, 20, 30, 50, 100];

/// Socket ticker view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketViewMode {
    All,
    PinnedOnly,
}

/// Reactive state for the realtime Futures ticker grid.
#[derive(Clone, Copy)]
pub struct SocketState {
    pub tickers: RwSignal<Vec<TrackedFuturesTicker>>,
    pub view_mode: RwSignal<SocketViewMode>,
    pub ticker_limit: RwSignal<usize>,
    pub pinned_slots: RwSignal<Vec<Option<String>>>,
    pub connection_status: RwSignal<MexcFuturesConnectionStatus>,
}

impl Default for SocketState {
    fn default() -> Self {
        Self::new()
    }
}

impl SocketState {
    /// Creates the socket feature state.
    pub fn new() -> Self {
        Self {
            tickers: RwSignal::new(Vec::new()),
            view_mode: RwSignal::new(SocketViewMode::All),
            ticker_limit: RwSignal::new(DEFAULT_LIMIT),
            pinned_slots: RwSignal::new(Vec::new()),
            connection_status: RwSignal::new(MexcFuturesConnectionStatus::Connecting),
        }
    }

    /// Sets the number of dynamic ticker cards rendered in the All view.
    pub fn set_ticker_limit(&self, limit: usize) {
        if LIMIT_OPTIONS.contains(&limit) {
            self.ticker_limit.set(limit);
        }
    }

    /// Returns the available dynamic ticker limit options.
    pub fn limit_options() -> &'static [usize] {
        &LIMIT_OPTIONS
    }
}
