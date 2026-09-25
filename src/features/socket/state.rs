use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::{closure::Closure, JsCast};

use crate::application::{
    ports::{
        FundingRateProvider, FuturesConnectionStatus, FuturesMarketStream, RealTradingStorage,
    },
    services::{
        mexc_account::MexcFuturesAccountService,
        mexc_trading::{MarketOrderRequest, MexcFuturesTradingService},
        proxy::ProxyService,
        technical_analysis::TechnicalAnalysisService,
        trading::TradingService,
        FuturesMarketService,
    },
};
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::TrackedFuturesTicker;
use crate::domain::technical_analysis::AnalysisResult;
use crate::domain::trading::{
    ExecutionMode, ExecutionSettings, HoldingSummary, PortfolioSummary, PositionSide,
    RealAccountSnapshot, RealPosition, RealTradingSettings, TradingSnapshot,
};
use crate::infrastructure::browser;
use crate::infrastructure::execution::LocalExecutionStorage;
use crate::infrastructure::proxy::ProxyApi;
use crate::infrastructure::trading::LocalTradingStorage;

const UI_FLUSH_MS: i32 = 75;
const PINNED_SYMBOLS_KEY: &str = "socket.pinned-symbols";
const POSITION_SIDE_CACHE_KEY: &str = "socket.position-side";
const DEFAULT_PAGE_SIZE: usize = 10;

type MarketSnapshot = Rc<HashMap<String, TrackedFuturesTicker>>;

/// Input values for saving socket trading settings.
#[derive(Clone)]
pub struct TradingSettingsInput {
    pub initial_capital: f64,
    pub fee_percent: f64,
    pub leverage: f64,
    pub trade_allocation_percent: f64,
    pub execution_mode: ExecutionMode,
    pub api_url: String,
    pub api_key: String,
    pub api_secret: String,
}

/// Socket ticker view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketViewMode {
    All,
    PinnedOnly,
}

/// Socket ticker direction filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketDirectionFilter {
    All,
    Long,
    Short,
}

/// Socket ticker sort mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketSortMode {
    Symbol,
    Ranking,
    Direction,
    Change15s,
    Change1m,
    Change5m,
    Price,
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
    pub direction_filter: RwSignal<SocketDirectionFilter>,
    pub sort_mode: RwSignal<SocketSortMode>,
    pub sort_direction: RwSignal<SocketSortDirection>,
    pub search_query: RwSignal<String>,
    pub pinned_symbols: RwSignal<Vec<String>>,
    pub page_size: RwSignal<usize>,
    pub current_page: RwSignal<usize>,
    pub connection_status: RwSignal<FuturesConnectionStatus>,
    pub analysis_loading: RwSignal<bool>,
    pub analysis_json: RwSignal<Option<String>>,
    pub analysis_result: RwSignal<Option<AnalysisResult>>,
    pub analysis_symbol: RwSignal<Option<String>>,
    pub analysis_timeframe: RwSignal<Option<String>>,
    pub analysis_modal_open: RwSignal<bool>,
    pub analysis_error: RwSignal<Option<String>>,
    pub analysis_copied: RwSignal<bool>,
    pub trading_snapshot: RwSignal<TradingSnapshot>,
    pub settings_open: RwSignal<bool>,
    pub trading_error: RwSignal<Option<String>>,
    pub trading_notice: RwSignal<Option<String>>,
    pub execution_mode: RwSignal<ExecutionMode>,
    pub api_url: RwSignal<String>,
    pub api_key: RwSignal<String>,
    pub api_secret: RwSignal<String>,
    pub real_account: RwSignal<Option<RealAccountSnapshot>>,
    pub real_positions: RwSignal<Vec<RealPosition>>,
    pub real_account_loading: RwSignal<bool>,
    pub reset_metrics_request: RwSignal<u64>,
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
        let direction_filter = RwSignal::new(SocketDirectionFilter::All);
        let sort_mode = RwSignal::new(SocketSortMode::Ranking);
        let sort_direction = RwSignal::new(SocketSortDirection::Descending);
        let search_query = RwSignal::new(String::new());
        let mut loaded_snapshot = TradingService::load(&LocalTradingStorage);
        let position_side = load_position_side().unwrap_or(loaded_snapshot.settings.position_side);
        save_position_side(position_side);
        loaded_snapshot.settings.position_side = position_side;
        let pinned_symbols = RwSignal::new(
            loaded_snapshot
                .portfolio
                .positions
                .iter()
                .map(|position| position.symbol.clone())
                .collect::<Vec<_>>(),
        );
        save_pinned_symbols(&pinned_symbols.get_untracked());
        let page_size = RwSignal::new(DEFAULT_PAGE_SIZE);
        let current_page = RwSignal::new(1usize);
        let connection_status = RwSignal::new(FuturesConnectionStatus::Connecting);
        let analysis_loading = RwSignal::new(false);
        let analysis_json = RwSignal::new(None);
        let analysis_result = RwSignal::new(None);
        let analysis_symbol = RwSignal::new(None);
        let analysis_timeframe = RwSignal::new(None);
        let analysis_modal_open = RwSignal::new(false);
        let analysis_error = RwSignal::new(None);
        let analysis_copied = RwSignal::new(false);
        let trading_snapshot = RwSignal::new(loaded_snapshot);
        let execution_settings = LocalExecutionStorage
            .load()
            .ok()
            .flatten()
            .unwrap_or_default();
        let settings_open = RwSignal::new(false);
        let trading_error = RwSignal::new(None);
        let trading_notice = RwSignal::new(None);
        let execution_mode = RwSignal::new(execution_settings.mode);
        let api_url = RwSignal::new(execution_settings.real.api_url.clone());
        let api_key = RwSignal::new(execution_settings.real.api_key.clone());
        let api_secret = RwSignal::new(execution_settings.real.api_secret.clone());
        let real_account = RwSignal::new(execution_settings.real.account.clone());
        let real_positions = RwSignal::new(Vec::new());
        let real_account_loading = RwSignal::new(false);
        let reset_metrics_request = RwSignal::new(0u64);
        let service = Rc::new(RefCell::new(FuturesMarketService::new()));

        let reset_request = reset_metrics_request;
        let reset_service = service.clone();
        let reset_tickers = tickers;
        let reset_initialized = Rc::new(Cell::new(false));
        Effect::new(move |_| {
            let _request = reset_request.get();
            if !reset_initialized.replace(true) {
                return;
            }

            reset_service.borrow_mut().reset_metrics();
            reset_tickers.set(Rc::new(reset_service.borrow().snapshot()));
        });

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

        let flush_for_status = schedule_flush.clone();
        let status_signal = connection_status;
        let on_status = Rc::new(move |status: FuturesConnectionStatus| {
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

        let state = Self {
            tickers,
            funding_rates,
            view_mode,
            direction_filter,
            sort_mode,
            sort_direction,
            search_query,
            pinned_symbols,
            page_size,
            current_page,
            connection_status,
            analysis_loading,
            analysis_json,
            analysis_result,
            analysis_symbol,
            analysis_timeframe,
            analysis_modal_open,
            analysis_error,
            analysis_copied,
            trading_snapshot,
            settings_open,
            trading_error,
            trading_notice,
            execution_mode,
            api_url,
            api_key,
            api_secret,
            real_account,
            real_positions,
            real_account_loading,
            reset_metrics_request,
        };
        if execution_settings.mode == ExecutionMode::Real {
            state.refresh_real_account();
        }
        state
    }

    /// Resets the short-term ranking history for all known tickers.
    pub fn reset_metrics(&self) {
        self.trading_error.set(None);
        self.trading_notice.set(None);
        self.reset_metrics_request
            .update(|request| *request = request.wrapping_add(1));
    }

    /// Sets the page size and returns to the first page.
    pub fn set_page_size(&self, size: usize) {
        self.page_size.set(size.max(DEFAULT_PAGE_SIZE));
        self.current_page.set(1);
    }

    /// Moves to a page within the available range.
    pub fn set_page(&self, page: usize) {
        self.current_page.set(page.max(1));
    }

    /// Sets the market direction filter.
    pub fn set_direction_filter(&self, filter: SocketDirectionFilter) {
        self.direction_filter.set(filter);
        self.current_page.set(1);
    }

    /// Selects a sort column, toggling direction when already selected.
    pub fn set_sort(&self, mode: SocketSortMode) {
        if self.sort_mode.get_untracked() == mode {
            self.sort_direction.update(|direction| {
                *direction = match direction {
                    SocketSortDirection::Ascending => SocketSortDirection::Descending,
                    SocketSortDirection::Descending => SocketSortDirection::Ascending,
                };
            });
        } else {
            self.sort_mode.set(mode);
            self.sort_direction.set(SocketSortDirection::Descending);
        }
    }

    /// Toggles a ticker pin without affecting its paper-trading position.
    pub fn toggle_pin(&self, symbol: &str) {
        let mut symbols = self.pinned_symbols.get_untracked();
        if let Some(index) = symbols.iter().position(|item| item == symbol) {
            symbols.remove(index);
            self.trading_notice.set(Some(format!("Unpinned {symbol}.")));
        } else {
            symbols.push(symbol.to_owned());
            self.trading_notice.set(Some(format!("Pinned {symbol}.")));
        }

        self.trading_error.set(None);
        self.pinned_symbols.set(symbols);
        save_pinned_symbols(&self.pinned_symbols.get_untracked());
    }

    /// Executes the selected trading mode. Real order execution is intentionally
    /// not wired here until the authenticated order executor is added.
    pub fn trade(&self, symbol: &str) {
        if self.execution_mode.get_untracked() == ExecutionMode::Real {
            self.trade_real(symbol);
            return;
        }
        let Some(price) = self
            .tickers
            .get_untracked()
            .get(symbol)
            .and_then(|ticker| ticker.ticker.last_price)
        else {
            self.trading_error.set(Some(
                "A live market price is required to trade.".to_string(),
            ));
            return;
        };

        self.trading_error.set(None);
        self.trading_notice.set(None);

        let snapshot = self.trading_snapshot.get_untracked();
        let timestamp_ms = js_sys::Date::now() as i64;
        let is_held = snapshot
            .portfolio
            .positions
            .iter()
            .any(|position| position.symbol == symbol);

        let next_snapshot = if is_held {
            TradingService::close(&snapshot, symbol, price, timestamp_ms)
        } else {
            TradingService::open(&snapshot, symbol, price, timestamp_ms)
        };

        let Ok(next_snapshot) = next_snapshot else {
            self.trading_error.set(Some(
                next_snapshot
                    .err()
                    .unwrap_or_else(|| "Paper trade failed.".to_string()),
            ));
            return;
        };

        if let Err(message) = TradingService::save(&LocalTradingStorage, &next_snapshot) {
            self.trading_error.set(Some(message));
            return;
        }

        let action = if is_held {
            match snapshot
                .portfolio
                .positions
                .iter()
                .find(|position| position.symbol == symbol)
                .map(|position| position.side)
            {
                Some(crate::domain::trading::PositionSide::Short) => "Bought",
                _ => "Sold",
            }
        } else {
            match snapshot.settings.position_side {
                crate::domain::trading::PositionSide::Short => "Sold",
                _ => "Bought",
            }
        };

        if is_held {
            let mut symbols = self.pinned_symbols.get_untracked();
            if let Some(index) = symbols.iter().position(|item| item == symbol) {
                symbols.remove(index);
                self.pinned_symbols.set(symbols);
                save_pinned_symbols(&self.pinned_symbols.get_untracked());
            }
        } else {
            let mut symbols = self.pinned_symbols.get_untracked();
            if !symbols.iter().any(|item| item == symbol) {
                symbols.push(symbol.to_owned());
                self.pinned_symbols.set(symbols);
                save_pinned_symbols(&self.pinned_symbols.get_untracked());
            }
        }

        self.trading_notice
            .set(Some(format!("{action} {symbol} at market price.")));
        self.trading_snapshot.set(next_snapshot);
    }

    /// Returns the current portfolio valuation using the latest socket prices.
    pub fn portfolio_summary(&self) -> PortfolioSummary {
        if self.execution_mode.get_untracked() == ExecutionMode::Real {
            let account = self.real_account.get_untracked().unwrap_or_default();
            let holdings = self
                .real_positions
                .get_untracked()
                .into_iter()
                .map(|position| HoldingSummary {
                    symbol: position.symbol,
                    side: position.side,
                    quantity: position.hold_volume,
                    market_value: position.initial_margin,
                    pnl: position.unrealized_pnl,
                })
                .collect::<Vec<_>>();
            let realized_pnl = self
                .real_positions
                .get_untracked()
                .iter()
                .map(|position| position.realized_pnl)
                .sum::<f64>();
            return PortfolioSummary {
                cash: account.available_balance,
                equity: account.equity,
                total_pnl: account.unrealized + realized_pnl,
                realized_pnl,
                unrealized_pnl: account.unrealized,
                holdings,
            };
        }

        let prices = self
            .tickers
            .get_untracked()
            .iter()
            .filter_map(|(symbol, ticker)| {
                ticker
                    .ticker
                    .last_price
                    .filter(|price| price.is_finite() && *price > 0.0)
                    .map(|price| (symbol.clone(), price))
            })
            .collect::<HashMap<_, _>>();

        TradingService::summarize(&self.trading_snapshot.get_untracked(), &prices)
    }

    /// Refreshes the live MEXC account and positions for the current Real Trading credentials.
    pub fn refresh_real_account(&self) {
        if self.execution_mode.get_untracked() != ExecutionMode::Real {
            return;
        }
        let api_url = self.api_url.get_untracked();
        let api_key = self.api_key.get_untracked();
        let api_secret = self.api_secret.get_untracked();
        if api_url.trim().is_empty() || api_key.trim().is_empty() || api_secret.trim().is_empty() {
            return;
        }

        let account_signal = self.real_account;
        let positions_signal = self.real_positions;
        let error_signal = self.trading_error;
        let api_url_for_positions = api_url.clone();
        let api_key_for_positions = api_key.clone();
        let api_secret_for_positions = api_secret.clone();
        MexcFuturesAccountService::new(ProxyApi).fetch_usdt_asset(
            &api_url,
            &api_key,
            &api_secret,
            js_sys::Date::now().max(0.0) as i64,
            Rc::new(move |account_result| match account_result {
                Ok(account) => {
                    account_signal.set(Some(account));
                    MexcFuturesTradingService::new(ProxyApi).fetch_positions(
                        &api_url_for_positions,
                        &api_key_for_positions,
                        &api_secret_for_positions,
                        js_sys::Date::now().max(0.0) as i64,
                        Rc::new(move |positions_result| match positions_result {
                            Ok(positions) => {
                                positions_signal.set(positions);
                                error_signal.set(None);
                            }
                            Err(message) => error_signal.set(Some(message)),
                        }),
                    );
                }
                Err(message) => error_signal.set(Some(message)),
            }),
        );
    }

    /// Refreshes the live MEXC positions for the current Real Trading credentials.
    pub fn refresh_real_positions(&self) {
        if self.execution_mode.get_untracked() != ExecutionMode::Real {
            return;
        }
        let api_url = self.api_url.get_untracked();
        let api_key = self.api_key.get_untracked();
        let api_secret = self.api_secret.get_untracked();
        if api_url.trim().is_empty() || api_key.trim().is_empty() || api_secret.trim().is_empty() {
            return;
        }

        let positions_signal = self.real_positions;
        let error_signal = self.trading_error;
        MexcFuturesTradingService::new(ProxyApi).fetch_positions(
            &api_url,
            &api_key,
            &api_secret,
            js_sys::Date::now().max(0.0) as i64,
            Rc::new(move |result| match result {
                Ok(positions) => {
                    positions_signal.set(positions);
                    error_signal.set(None);
                }
                Err(message) => error_signal.set(Some(message)),
            }),
        );
    }

    fn trade_real(&self, symbol: &str) {
        let Some(price) = self
            .tickers
            .get_untracked()
            .get(symbol)
            .and_then(|ticker| ticker.ticker.last_price)
            .filter(|price| price.is_finite() && *price > 0.0)
        else {
            self.trading_error.set(Some(
                "A live market price is required to trade.".to_string(),
            ));
            return;
        };

        let positions = self.real_positions.get_untracked();
        let existing = positions
            .iter()
            .find(|position| position.symbol == symbol)
            .cloned();
        let action = existing
            .as_ref()
            .map(|position| match position.side {
                PositionSide::Long => "close LONG",
                PositionSide::Short => "close SHORT",
            })
            .unwrap_or(
                match self.trading_snapshot.get_untracked().settings.position_side {
                    PositionSide::Long => "open LONG",
                    PositionSide::Short => "open SHORT",
                },
            );

        let message = format!(
            "REAL TRADING: {action} {symbol} at market price around {:.6}. Continue?",
            price
        );
        let confirmed = web_sys::window()
            .and_then(|window| window.confirm_with_message(&message).ok())
            .unwrap_or(false);
        if !confirmed {
            self.trading_notice
                .set(Some("Real order cancelled.".to_string()));
            return;
        }

        let api_url = self.api_url.get_untracked();
        let api_key = self.api_key.get_untracked();
        let api_secret = self.api_secret.get_untracked();
        let settings = self.trading_snapshot.get_untracked().settings;
        let account = self.real_account.get_untracked().unwrap_or_default();

        if api_key.trim().is_empty() || api_secret.trim().is_empty() {
            self.trading_error.set(Some(
                "Real trading API credentials are not configured.".to_string(),
            ));
            return;
        }

        self.trading_error.set(None);
        self.trading_notice.set(None);

        let service = Rc::new(MexcFuturesTradingService::new(ProxyApi));
        let error_signal = self.trading_error;
        let notice_signal = self.trading_notice;
        let positions_signal = self.real_positions;
        let account_signal = self.real_account;
        let api_url_for_refresh = api_url.clone();
        let api_key_for_refresh = api_key.clone();
        let api_secret_for_refresh = api_secret.clone();
        let symbol_owned = symbol.to_string();
        let order_api_url = api_url.clone();
        let order_api_key = api_key.clone();
        let order_api_secret = api_secret.clone();

        let submit: Rc<
            dyn Fn(crate::application::services::mexc_trading::ContractDetail),
        > = {
            let service = service.clone();
            Rc::new(move |contract: crate::application::services::mexc_trading::ContractDetail| {
                let (side, volume, position_id, reduce_only) =
                    if let Some(position) = existing.as_ref() {
                        (
                            match position.side {
                                PositionSide::Long => 4,
                                PositionSide::Short => 2,
                            },
                            position.hold_volume,
                            Some(position.position_id),
                            Some(true),
                        )
                    } else {
                        let allocation = settings.trade_allocation_percent / 100.0;
                        let margin = account.available_balance * allocation;
                        let notional = margin * settings.leverage;
                        let raw_volume = notional / (price * contract.contract_size);
                        let volume = (raw_volume / contract.vol_unit).floor() * contract.vol_unit;
                        (
                            match settings.position_side {
                                PositionSide::Long => 1,
                                PositionSide::Short => 3,
                            },
                            volume,
                            None,
                            None,
                        )
                    };

                if !volume.is_finite() || volume < contract.min_vol || volume > contract.max_vol {
                    error_signal.set(Some(
                        "Calculated order volume is outside the MEXC contract limits.".to_string(),
                    ));
                    return;
                }

                service.submit_market_order(
                    &order_api_url,
                    &order_api_key,
                    &order_api_secret,
                    js_sys::Date::now().max(0.0) as i64,
                    MarketOrderRequest {
                        symbol: symbol_owned.clone(),
                        price,
                        vol: volume,
                        leverage: settings.leverage.max(1.0) as u32,
                        side,
                        order_type: 5,
                        open_type: 1,
                        position_id,
                        reduce_only,
                    },
                    Rc::new(move |result| match result {
                        Ok(order_id) => {
                            notice_signal.set(Some(format!("MEXC order {order_id} submitted.")));
                            let positions_signal = positions_signal;
                            let account_signal = account_signal;
                            let api_url = api_url_for_refresh.clone();
                            let api_key = api_key_for_refresh.clone();
                            let api_secret = api_secret_for_refresh.clone();
                            let positions_api_url = api_url.clone();
                            let positions_api_key = api_key.clone();
                            let positions_api_secret = api_secret.clone();
                            MexcFuturesAccountService::new(ProxyApi).fetch_usdt_asset(
                                &api_url,
                                &api_key,
                                &api_secret,
                                js_sys::Date::now().max(0.0) as i64,
                                Rc::new(move |account_result| {
                                    if let Ok(account) = account_result {
                                        account_signal.set(Some(account));
                                    }
                                    MexcFuturesTradingService::new(ProxyApi).fetch_positions(
                                        &positions_api_url,
                                        &positions_api_key,
                                        &positions_api_secret,
                                        js_sys::Date::now().max(0.0) as i64,
                                        Rc::new(move |positions_result| {
                                            if let Ok(positions) = positions_result {
                                                positions_signal.set(positions);
                                            }
                                        }),
                                    );
                                }),
                            );
                        }
                        Err(message) => error_signal.set(Some(message)),
                    }),
                );
            })
        };
        let submit_for_callback = submit.clone();
        service.fetch_contract(
            &api_url,
            symbol,
            Rc::new(move |result| match result {
                Ok(contract) => submit_for_callback(contract),
                Err(message) => error_signal.set(Some(message)),
            }),
        );
    }

    /// Opens the paper-trading settings panel.
    pub fn open_settings(&self) {
        self.trading_error.set(None);
        self.trading_notice.set(None);
        self.settings_open.set(true);
    }

    /// Closes the paper-trading settings panel.
    pub fn close_settings(&self) {
        self.settings_open.set(false);
    }

    /// Saves execution settings. Real mode validates the MEXC account first and
    /// only persists the settings after the account request succeeds.
    pub fn save_settings(&self, input: TradingSettingsInput) {
        let TradingSettingsInput {
            initial_capital,
            fee_percent,
            leverage,
            trade_allocation_percent,
            execution_mode,
            api_url,
            api_key,
            api_secret,
        } = input;
        let fee_rate = fee_percent / 100.0;
        let position_side = self.trading_snapshot.get_untracked().settings.position_side;

        if execution_mode == ExecutionMode::Real {
            let api_url = api_url.trim().trim_end_matches('/').to_string();
            let api_key = api_key.trim().to_string();
            let api_secret = api_secret.trim().to_string();

            if api_url.is_empty() || api_key.is_empty() || api_secret.is_empty() {
                self.trading_error.set(Some(
                    "API URL, API key, and API secret are required for real trading.".to_string(),
                ));
                return;
            }

            self.real_account_loading.set(true);
            self.trading_error.set(None);
            self.trading_notice.set(None);

            let loading = self.real_account_loading;
            let error = self.trading_error;
            let notice = self.trading_notice;
            let mode = self.execution_mode;
            let url_signal = self.api_url;
            let key_signal = self.api_key;
            let secret_signal = self.api_secret;
            let account_signal = self.real_account;
            let real_positions_signal = self.real_positions;
            let settings_open = self.settings_open;
            let saved_api_url = api_url.clone();
            let saved_api_key = api_key.clone();
            let saved_api_secret = api_secret.clone();

            MexcFuturesAccountService::new(ProxyApi).fetch_usdt_asset(
                &api_url,
                &api_key,
                &api_secret,
                js_sys::Date::now().max(0.0) as i64,
                Rc::new(move |result| {
                    loading.set(false);
                    match result {
                        Ok(account) => {
                            let settings = ExecutionSettings {
                                mode: ExecutionMode::Real,
                                real: RealTradingSettings {
                                    api_url: saved_api_url.clone(),
                                    api_key: saved_api_key.clone(),
                                    api_secret: saved_api_secret.clone(),
                                    account: Some(account.clone()),
                                },
                            };

                            match LocalExecutionStorage.save(&settings) {
                                Ok(()) => {
                                    mode.set(ExecutionMode::Real);
                                    url_signal.set(saved_api_url.clone());
                                    key_signal.set(saved_api_key.clone());
                                    secret_signal.set(saved_api_secret.clone());
                                    account_signal.set(Some(account.clone()));
                                    error.set(None);
                                    settings_open.set(false);
                                    notice.set(Some(format!(
                                        "Real trading settings saved. MEXC Futures USDT equity: {:.2} USDT.",
                                        account.equity
                                    )));
                                    MexcFuturesTradingService::new(ProxyApi).fetch_positions(
                                        &saved_api_url,
                                        &saved_api_key,
                                        &saved_api_secret,
                                        js_sys::Date::now().max(0.0) as i64,
                                        Rc::new(move |positions_result| {
                                            if let Ok(positions) = positions_result {
                                                real_positions_signal.set(positions);
                                            }
                                        }),
                                    );
                                }
                                Err(message) => error.set(Some(message)),
                            }
                        }
                        Err(message) => error.set(Some(message)),
                    }
                }),
            );
            return;
        }

        match TradingService::reset_with_settings(
            initial_capital,
            fee_rate,
            leverage,
            trade_allocation_percent,
            position_side,
        ) {
            Ok(snapshot) => match TradingService::save(&LocalTradingStorage, &snapshot) {
                Ok(()) => {
                    let mut settings = LocalExecutionStorage
                        .load()
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    settings.mode = ExecutionMode::Paper;
                    if let Err(message) = LocalExecutionStorage.save(&settings) {
                        self.trading_error.set(Some(message));
                        return;
                    }

                    self.trading_snapshot.set(snapshot);
                    self.execution_mode.set(ExecutionMode::Paper);
                    self.pinned_symbols.set(Vec::new());
                    save_pinned_symbols(&[]);
                    self.trading_error.set(None);
                    self.trading_notice.set(Some(
                        "Trading settings saved. The paper portfolio was reset.".to_string(),
                    ));
                    self.settings_open.set(false);
                }
                Err(message) => self.trading_error.set(Some(message)),
            },
            Err(message) => self.trading_error.set(Some(message)),
        }
    }

    /// Sets the side used for new paper-trading positions and persists it locally.
    pub fn set_position_side(&self, side: PositionSide) {
        let mut snapshot = self.trading_snapshot.get_untracked();
        snapshot.settings.position_side = side;

        match TradingService::save(&LocalTradingStorage, &snapshot) {
            Ok(()) => {
                save_position_side(side);
                self.trading_snapshot.set(snapshot);
                self.trading_error.set(None);
                self.trading_notice.set(Some(format!(
                    "Order side set to {}.",
                    match side {
                        PositionSide::Long => "Long",
                        PositionSide::Short => "Short",
                    }
                )));
            }
            Err(message) => self.trading_error.set(Some(message)),
        }
    }

    /// Analyzes a MEXC Futures symbol at the requested timeframe.
    pub fn analyze_symbol(&self, symbol: &str, timeframe: &str) {
        self.run_analysis(symbol, timeframe, false);
    }

    /// Runs analysis and copies the resulting JSON to the clipboard.
    pub fn copy_symbol_analysis(&self, symbol: &str, timeframe: &str) {
        self.run_analysis(symbol, timeframe, true);
    }

    fn run_analysis(&self, symbol: &str, timeframe: &str, copy_result: bool) {
        let symbol = symbol.trim().to_ascii_uppercase().replace('_', "");
        let timeframe = timeframe.trim().to_ascii_uppercase();
        if symbol.is_empty() || timeframe.is_empty() {
            self.analysis_error.set(Some(
                "A valid symbol and timeframe are required for analysis.".to_string(),
            ));
            return;
        }

        self.analysis_loading.set(true);
        self.analysis_error.set(None);
        self.analysis_copied.set(false);
        self.analysis_modal_open.set(false);
        self.analysis_symbol.set(Some(symbol.clone()));
        self.analysis_timeframe.set(Some(timeframe.clone()));

        let analysis_loading = self.analysis_loading;
        let analysis_json = self.analysis_json;
        let analysis_result = self.analysis_result;
        let analysis_modal_open = self.analysis_modal_open;
        let analysis_error = self.analysis_error;
        let analysis_copied = self.analysis_copied;
        // MEXC uses the base asset symbol without the numeric multiplier prefix
        // for Kline requests (for example, 1000000BABYDOGE_USDT -> BABYDOGEUSDT).
        let api_symbol = symbol
            .trim_start_matches(|character: char| character.is_ascii_digit())
            .to_string();
        if api_symbol.is_empty() {
            self.analysis_error.set(Some(
                "The symbol does not contain a valid asset name.".to_string(),
            ));
            return;
        }
        let url = format!(
            "https://api.mexc.com/api/v3/klines?symbol={api_symbol}&interval={}&limit=500",
            match timeframe.as_str() {
                "4H" => "4h",
                "1D" => "1d",
                _ => "1d",
            }
        );

        ProxyService::new(ProxyApi).fetch_raw(
            &url,
            Rc::new(move |result| {
                analysis_loading.set(false);
                match result.and_then(|raw| {
                    let input =
                        TechnicalAnalysisService::mexc_klines_input(&raw, &api_symbol, &timeframe)?;
                    let config = TechnicalAnalysisService::default_crypto_config();
                    TechnicalAnalysisService::analyze(&input, &config, browser::now_iso8601())
                }) {
                    Ok(result) => {
                        let json = match serde_json::to_string_pretty(&result) {
                            Ok(json) => json,
                            Err(error) => {
                                analysis_error.set(Some(format!(
                                    "Failed to serialize analysis result: {error}"
                                )));
                                return;
                            }
                        };
                        analysis_json.set(Some(json.clone()));
                        analysis_result.set(Some(result));
                        analysis_error.set(None);
                        analysis_modal_open.set(!copy_result);
                        if copy_result {
                            wasm_bindgen_futures::spawn_local(async move {
                                match browser::copy_to_clipboard(&json).await {
                                    Ok(()) => analysis_copied.set(true),
                                    Err(message) => analysis_error.set(Some(message)),
                                }
                            });
                        }
                    }
                    Err(message) => {
                        analysis_json.set(None);
                        analysis_result.set(None);
                        analysis_modal_open.set(false);
                        analysis_error.set(Some(message));
                    }
                }
            }),
        );
    }

    pub fn close_analysis(&self) {
        self.analysis_modal_open.set(false);
    }

    pub fn copy_analysis(&self) {
        let Some(json) = self.analysis_json.get_untracked() else {
            return;
        };
        let copied = self.analysis_copied;
        let error = self.analysis_error;
        wasm_bindgen_futures::spawn_local(async move {
            match browser::copy_to_clipboard(&json).await {
                Ok(()) => {
                    copied.set(true);
                    error.set(None);
                }
                Err(message) => error.set(Some(message)),
            }
        });
    }
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

fn load_position_side() -> Option<PositionSide> {
    let storage = web_sys::window()?.local_storage().ok().flatten()?;
    match storage
        .get_item(POSITION_SIDE_CACHE_KEY)
        .ok()
        .flatten()?
        .as_str()
    {
        "short" => Some(PositionSide::Short),
        "long" => Some(PositionSide::Long),
        _ => None,
    }
}

fn save_position_side(side: PositionSide) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let value = match side {
            PositionSide::Long => "long",
            PositionSide::Short => "short",
        };
        let _ = storage.set_item(POSITION_SIDE_CACHE_KEY, value);
    }
}
