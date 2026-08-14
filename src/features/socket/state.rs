use leptos::prelude::*;

use crate::domain::futures::TrackedFuturesTicker;
use crate::infrastructure::mexc_futures::MexcFuturesConnectionStatus;

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
    pub connection_status: RwSignal<MexcFuturesConnectionStatus>,
}

impl SocketState {
    /// Creates the socket feature state.
    pub fn new() -> Self {
        Self {
            tickers: RwSignal::new(Vec::new()),
            view_mode: RwSignal::new(SocketViewMode::All),
            ticker_limit: RwSignal::new(10),
            connection_status: RwSignal::new(MexcFuturesConnectionStatus::Connecting),
        }
    }
}
