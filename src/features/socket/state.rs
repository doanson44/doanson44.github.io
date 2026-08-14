use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;

use crate::application::services::FuturesMarketService;
use crate::domain::futures::TrackedFuturesTicker;
use crate::infrastructure::mexc_futures::{
    connect_tickers, MexcFuturesConnectionStatus, MexcFuturesWsHandle,
};

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
    /// Creates the socket feature state and starts the all-market ticker stream.
    pub fn new() -> Self {
        let tickers = RwSignal::new(Vec::new());
        let view_mode = RwSignal::new(SocketViewMode::All);
        let ticker_limit = RwSignal::new(DEFAULT_LIMIT);
        let pinned_slots = RwSignal::new(Vec::<Option<String>>::new());
        let connection_status = RwSignal::new(MexcFuturesConnectionStatus::Connecting);
        let service = Rc::new(RefCell::new(FuturesMarketService::new()));

        let service_for_stream = service.clone();
        let tickers_signal = tickers;
        let on_batch = Rc::new(move |updates| {
            let snapshot = service_for_stream.borrow_mut().apply_batch(updates);
            tickers_signal.set(snapshot);
        });

        let service_for_status = service.clone();
        let status_signal = connection_status;
        let on_status = Rc::new(move |status| {
            if status == MexcFuturesConnectionStatus::Reconnecting {
                service_for_status.borrow_mut().rebaseline();
            }
            status_signal.set(status);
        });

        match connect_tickers(on_batch, on_status) {
            Ok(handle) => register_cleanup(handle),
            Err(error) => connection_status.set(MexcFuturesConnectionStatus::Error(error)),
        }

        Self {
            tickers,
            view_mode,
            ticker_limit,
            pinned_slots,
            connection_status,
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

fn register_cleanup(mut handle: MexcFuturesWsHandle) {
    on_cleanup(move || handle.close());
}
