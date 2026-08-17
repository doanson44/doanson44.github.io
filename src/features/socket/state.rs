use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{closure::Closure, JsCast};

use crate::application::{
    ports::{FuturesConnectionStatus, FuturesMarketStream},
    services::FuturesMarketService,
};
use crate::domain::futures::TrackedFuturesTicker;

const DEFAULT_LIMIT: usize = 10;
const LIMIT_OPTIONS: [usize; 6] = [10, 20, 30, 50, 100, usize::MAX];
const UI_FLUSH_MS: i32 = 75;

type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

/// Socket ticker view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketViewMode {
    All,
    PinnedOnly,
}

/// Socket ticker sort mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketSortMode {
    Momentum,
    TotalTicks,
}

/// 24-hour percentage change filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketChangeFilter {
    #[default]
    All,
    Positive,
    Negative,
}

/// Rolling directional momentum bias filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketMomentumFilter {
    #[default]
    All,
    Bullish,
    Bearish,
}

/// Last-price versus fair-price filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketFairPriceFilter {
    #[default]
    All,
    Above,
    Below,
}

/// 24-hour volume rank filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketVolumeFilter {
    #[default]
    All,
    TopHalf,
    TopQuarter,
}

/// Combined market data filters used by the Socket ticker projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SocketFilters {
    pub change: SocketChangeFilter,
    pub momentum: SocketMomentumFilter,
    pub fair_price: SocketFairPriceFilter,
    pub volume: SocketVolumeFilter,
}

/// Reactive state for the realtime Futures ticker grid.
#[derive(Clone, Copy)]
pub struct SocketState {
    pub tickers: RwSignal<MarketSnapshot, LocalStorage>,
    pub view_mode: RwSignal<SocketViewMode>,
    pub sort_mode: RwSignal<SocketSortMode>,
    pub filters: RwSignal<SocketFilters>,
    pub ticker_limit: RwSignal<usize>,
    pub pinned_slots: RwSignal<Vec<Option<String>>>,
    pub connection_status: RwSignal<FuturesConnectionStatus>,
}

impl SocketState {
    /// Creates the socket feature state and starts the all-market ticker stream.
    pub fn new(stream: Rc<dyn FuturesMarketStream>) -> Self {
        let tickers = RwSignal::new_local(Rc::new(HashMap::new()));
        let view_mode = RwSignal::new(SocketViewMode::All);
        let sort_mode = RwSignal::new(SocketSortMode::Momentum);
        let filters = RwSignal::new(SocketFilters::default());
        let ticker_limit = RwSignal::new(DEFAULT_LIMIT);
        let pinned_slots = RwSignal::new(Vec::<Option<String>>::new());
        let connection_status = RwSignal::new(FuturesConnectionStatus::Connecting);
        let service = Rc::new(RefCell::new(FuturesMarketService::new()));
        let flush_pending = Rc::new(Cell::new(false));

        let schedule_flush = {
            let service = service.clone();
            let flush_pending = flush_pending.clone();
            Rc::new(move || {
                if flush_pending.replace(true) {
                    return;
                }
                let service = service.clone();
                let flush_pending_for_callback = flush_pending.clone();
                let callback = Closure::once_aborting(move || {
                    flush_pending_for_callback.set(false);
                    let snapshot = service.borrow().snapshot();
                    tickers.set(Rc::new(snapshot));
                })
                .into_js_value();
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.unchecked_ref(),
                        UI_FLUSH_MS,
                    );
                } else {
                    flush_pending.set(false);
                }
            })
        };

        let service_for_stream = service.clone();
        let flush_for_stream = schedule_flush.clone();
        let on_batch = Rc::new(move |updates| {
            service_for_stream.borrow_mut().apply_batch(updates);
            flush_for_stream();
        });

        let service_for_status = service.clone();
        let flush_for_status = schedule_flush.clone();
        let status_signal = connection_status;
        let on_status = Rc::new(move |status: FuturesConnectionStatus| {
            if status == FuturesConnectionStatus::Reconnecting {
                service_for_status.borrow_mut().rebaseline();
            }
            status_signal.set(status);
            flush_for_status();
        });

        match stream.connect(on_batch, on_status) {
            Ok(handle) => {
                let handle = SendWrapper::new(handle);
                on_cleanup(move || {
                    let mut handle = handle;
                    handle.close();
                });
            }
            Err(error) => connection_status.set(FuturesConnectionStatus::Error(error)),
        }

        Self {
            tickers,
            view_mode,
            sort_mode,
            filters,
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

    /// Toggles a ticker pin while preserving its current rendered slot.
    pub fn toggle_pin(&self, symbol: &str, current_index: usize) {
        let mut slots = self.pinned_slots.get_untracked();
        if let Some(index) = slots
            .iter()
            .position(|slot| slot.as_deref() == Some(symbol))
        {
            slots[index] = None;
            while slots.last().is_some_and(Option::is_none) {
                slots.pop();
            }
        } else {
            if slots.len() <= current_index {
                slots.resize(current_index + 1, None);
            }
            slots[current_index] = Some(symbol.to_owned());
        }
        self.pinned_slots.set(slots);
    }
}
