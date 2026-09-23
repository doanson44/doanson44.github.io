use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::application::{
    ports::{FundingRateProvider, FuturesConnectionStatus, FuturesMarketStream},
    services::FuturesMarketService,
};
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::TrackedFuturesTicker;

const UI_FLUSH_MS: i32 = 75;
const TICKER_CACHE_KEY: &str = "socket.tickers-cache";
const PINNED_SYMBOLS_KEY: &str = "socket.pinned-symbols";

type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CachedTickerMomentum {
    symbol: String,
    up_ticks: u64,
    down_ticks: u64,
}

/// Socket ticker view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketViewMode {
    All,
    PinnedOnly,
}

/// Additional market filter for burst detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketFilter {
    All,
    Burst,
}

/// Socket ticker sort mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketSortMode {
    Momentum,
    Price,
    TotalTicks,
    Funding,
    Change24h,
    Volume24h,
}

/// Socket ticker sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketSortDirection {
    Ascending,
    Descending,
}

/// Reactive state for the realtime Futures ticker grid.
#[derive(Clone, Copy)]
pub struct SocketState {
    pub tickers: RwSignal<MarketSnapshot, LocalStorage>,
    pub funding_rates: RwSignal<Option<FundingRateSnapshot>, LocalStorage>,
    pub view_mode: RwSignal<SocketViewMode>,
    pub filter: RwSignal<SocketFilter>,
    pub sort_mode: RwSignal<SocketSortMode>,
    pub sort_direction: RwSignal<SocketSortDirection>,
    pub search_query: RwSignal<String>,
    pub pinned_symbols: RwSignal<Vec<String>>,
    pub connection_status: RwSignal<FuturesConnectionStatus>,
}

impl SocketState {
    /// Creates the socket feature state and starts market/funding data loading.
    pub fn new(
        stream: Rc<dyn FuturesMarketStream>,
        funding_provider: Rc<dyn FundingRateProvider>,
    ) -> Self {
        let tickers = RwSignal::new_local(Rc::new(HashMap::new()));
        let funding_rates = RwSignal::new_local(None);
        let view_mode = RwSignal::new(SocketViewMode::All);
        let filter = RwSignal::new(SocketFilter::All);
        let sort_mode = RwSignal::new(SocketSortMode::Momentum);
        let sort_direction = RwSignal::new(SocketSortDirection::Descending);
        let search_query = RwSignal::new(String::new());
        let pinned_symbols = RwSignal::new(load_pinned_symbols());
        let connection_status = RwSignal::new(FuturesConnectionStatus::Connecting);
        let service = Rc::new(RefCell::new(FuturesMarketService::new()));

        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Some(raw) = storage.get_item(TICKER_CACHE_KEY).ok().flatten() {
                if let Ok(snapshot) = serde_json::from_str::<Vec<CachedTickerMomentum>>(&raw) {
                    let cached = snapshot
                        .into_iter()
                        .map(|item| (item.symbol, item.up_ticks, item.down_ticks));
                    service.borrow_mut().restore_momentum(cached);
                } else if let Ok(old_snapshot) =
                    serde_json::from_str::<HashMap<String, TrackedFuturesTicker>>(&raw)
                {
                    let cached = old_snapshot.into_iter().map(|(symbol, tracked)| {
                        (
                            symbol,
                            tracked.momentum.up_ticks,
                            tracked.momentum.down_ticks,
                        )
                    });
                    service.borrow_mut().restore_momentum(cached);
                }
            }
        }

        let flush_pending = Rc::new(Cell::new(false));

        let funding_signal = funding_rates;
        funding_provider.load_cached_or_fetch(Rc::new(move |result| {
            if let Ok(snapshot) = result {
                funding_signal.set(Some(snapshot));
            }
        }));

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

        let save_service = service.clone();
        let save_callback = Closure::wrap(Box::new(move || {
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten())
            {
                let snapshot = save_service
                    .borrow()
                    .export_momentum()
                    .map(|(symbol, momentum)| CachedTickerMomentum {
                        symbol: symbol.clone(),
                        up_ticks: momentum.up_ticks,
                        down_ticks: momentum.down_ticks,
                    })
                    .collect::<Vec<_>>();
                if !snapshot.is_empty() {
                    if let Ok(raw) = serde_json::to_string(&snapshot) {
                        let _ = storage.set_item(TICKER_CACHE_KEY, &raw);
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        if let Some(window) = web_sys::window() {
            if let Ok(handle) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                save_callback.as_ref().unchecked_ref(),
                5000,
            ) {
                on_cleanup(move || {
                    let _ = save_callback; // take ownership to keep alive until cleanup
                    if let Some(window) = web_sys::window() {
                        window.clear_interval_with_handle(handle);
                    }
                });
            }
        }

        Self {
            tickers,
            funding_rates,
            view_mode,
            filter,
            sort_mode,
            sort_direction,
            search_query,
            pinned_symbols,
            connection_status,
        }
    }

    /// Toggles a ticker pin.
    pub fn toggle_pin(&self, symbol: &str) {
        let mut symbols = self.pinned_symbols.get_untracked();
        if let Some(index) = symbols.iter().position(|item| item == symbol) {
            symbols.remove(index);
        } else {
            symbols.push(symbol.to_owned());
        }
        self.pinned_symbols.set(symbols.clone());
        save_pinned_symbols(&symbols);
    }
}

fn load_pinned_symbols() -> Vec<String> {
    let raw = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(PINNED_SYMBOLS_KEY).ok().flatten());

    if let Some(raw) = raw {
        if let Ok(symbols) = serde_json::from_str::<Vec<String>>(&raw) {
            return symbols;
        }
    }

    // Migrate the previous slot-based format once.
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("socket.pinned-slots").ok().flatten())
        .and_then(|raw| serde_json::from_str::<Vec<Option<String>>>(&raw).ok())
        .map(|slots| slots.into_iter().flatten().collect())
        .unwrap_or_default()
}

fn save_pinned_symbols(symbols: &[String]) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        if let Ok(raw) = serde_json::to_string(symbols) {
            let _ = storage.set_item(PINNED_SYMBOLS_KEY, &raw);
        }
    }
}
