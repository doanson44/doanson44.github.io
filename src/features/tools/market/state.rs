use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use leptos::prelude::*;

use crate::application::ports::MarketPinStore;
use crate::application::services::market::MarketService;
use crate::application::services::technical_analysis::TechnicalAnalysisService;
use crate::domain::market::{MarketPriceExtremes, MarketStock};
use crate::domain::technical_analysis::AnalysisResult;
use crate::infrastructure::browser;
use crate::infrastructure::market::MarketApi;

#[derive(Clone, Copy)]
pub struct MarketState {
    service: MarketService<MarketApi>,
    pub stocks: RwSignal<Vec<MarketStock>>,
    pub total_items: RwSignal<usize>,
    pub displayed_items: RwSignal<usize>,
    pub error: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
    pub pinned_symbols: RwSignal<Vec<String>>,
    pub history: RwSignal<HashMap<String, MarketPriceExtremes>>,
    pub history_loading: RwSignal<HashSet<String>>,
    pub history_errors: RwSignal<HashMap<String, String>>,
    pub analysis_loading: RwSignal<bool>,
    pub analysis_json: RwSignal<Option<String>>,
    pub analysis_result: RwSignal<Option<AnalysisResult>>,
    pub analysis_symbol: RwSignal<Option<String>>,
    pub analysis_modal_open: RwSignal<bool>,
    pub analysis_error: RwSignal<Option<String>>,
    pub analysis_copied: RwSignal<bool>,
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketState {
    pub fn new() -> Self {
        Self {
            service: MarketService::new(MarketApi),
            stocks: RwSignal::new(Vec::new()),
            total_items: RwSignal::new(0),
            displayed_items: RwSignal::new(0),
            error: RwSignal::new(None),
            loading: RwSignal::new(false),
            pinned_symbols: RwSignal::new(Vec::new()),
            history: RwSignal::new(HashMap::new()),
            history_loading: RwSignal::new(HashSet::new()),
            history_errors: RwSignal::new(HashMap::new()),
            analysis_loading: RwSignal::new(false),
            analysis_json: RwSignal::new(None),
            analysis_result: RwSignal::new(None),
            analysis_symbol: RwSignal::new(None),
            analysis_modal_open: RwSignal::new(false),
            analysis_error: RwSignal::new(None),
            analysis_copied: RwSignal::new(false),
        }
    }

    pub fn load_pins(&self) {
        match MarketApi.load() {
            Ok(symbols) => self.pinned_symbols.set(symbols),
            Err(message) => self.error.set(Some(message)),
        }
    }

    fn load_history_for_symbol(&self, symbol: &str) {
        let symbol = symbol.trim().to_ascii_uppercase();
        if symbol.is_empty() {
            return;
        }

        let mut loading = self.history_loading.get_untracked();
        loading.insert(symbol.clone());
        self.history_loading.set(loading);

        let mut errors = self.history_errors.get_untracked();
        errors.remove(&symbol);
        self.history_errors.set(errors);

        let history = self.history;
        let history_loading = self.history_loading;
        let history_errors = self.history_errors;
        let cutoff = browser::days_ago_iso8601(364);

        self.service.load_history(
            &symbol,
            &cutoff,
            Rc::new(move |result| {
                let mut loading = history_loading.get_untracked();
                loading.remove(&symbol);
                history_loading.set(loading);

                match result {
                    Ok(extremes) => {
                        let mut values = history.get_untracked();
                        values.insert(symbol.clone(), extremes);
                        history.set(values);
                    }
                    Err(message) => {
                        let mut errors = history_errors.get_untracked();
                        errors.insert(symbol.clone(), message);
                        history_errors.set(errors);
                    }
                }
            }),
        );
    }

    fn load_history_sequence(&self, symbols: Rc<Vec<String>>, index: usize) {
        if index >= symbols.len() {
            return;
        }

        let symbol = symbols[index].clone();
        let next_state = *self;
        let symbols_for_next = Rc::clone(&symbols);
        let mut loading = self.history_loading.get_untracked();
        loading.insert(symbol.clone());
        self.history_loading.set(loading);

        let mut errors = self.history_errors.get_untracked();
        errors.remove(&symbol);
        self.history_errors.set(errors);

        let cutoff = browser::days_ago_iso8601(364);
        self.service.load_history(
            &symbol,
            &cutoff,
            Rc::new(move |result| {
                let mut loading = next_state.history_loading.get_untracked();
                loading.remove(&symbol);
                next_state.history_loading.set(loading);

                match result {
                    Ok(extremes) => {
                        let mut values = next_state.history.get_untracked();
                        values.insert(symbol.clone(), extremes);
                        next_state.history.set(values);
                    }
                    Err(message) => {
                        let mut errors = next_state.history_errors.get_untracked();
                        errors.insert(symbol.clone(), message);
                        next_state.history_errors.set(errors);
                    }
                }

                next_state.load_history_sequence(symbols_for_next, index + 1);
            }),
        );
    }

    fn load_pinned_history(&self) {
        let symbols = self
            .pinned_symbols
            .get_untracked()
            .into_iter()
            .map(|symbol| symbol.trim().to_ascii_uppercase())
            .filter(|symbol| !symbol.is_empty())
            .collect::<Vec<_>>();

        self.load_history_sequence(Rc::new(symbols), 0);
    }

    pub fn toggle_pin(&self, symbol: &str) {
        let mut symbols = self.pinned_symbols.get_untracked();
        let symbol = symbol.trim().to_ascii_uppercase();
        let was_pinned = symbols.iter().any(|item| item == &symbol);

        if was_pinned {
            symbols.retain(|item| item != &symbol);
        } else {
            symbols.push(symbol.clone());
        }

        if let Err(message) = MarketApi.save(&symbols) {
            self.error.set(Some(message));
            return;
        }

        self.pinned_symbols.set(symbols);

        if was_pinned {
            let mut history = self.history.get_untracked();
            history.remove(&symbol);
            self.history.set(history);

            let mut errors = self.history_errors.get_untracked();
            errors.remove(&symbol);
            self.history_errors.set(errors);
        } else {
            self.load_history_for_symbol(&symbol);
        }
    }

    pub fn analyze_symbol(&self, symbol: &str) {
        self.run_analysis(symbol, false);
    }

    pub fn copy_symbol_analysis(&self, symbol: &str) {
        self.run_analysis(symbol, true);
    }

    fn run_analysis(&self, symbol: &str, copy_result: bool) {
        let symbol = symbol.trim().to_ascii_uppercase();
        if symbol.is_empty() {
            self.analysis_error.set(Some(
                "A market symbol is required for analysis.".to_string(),
            ));
            return;
        }

        self.analysis_loading.set(true);
        self.analysis_error.set(None);
        self.analysis_copied.set(false);
        self.analysis_modal_open.set(false);
        self.analysis_symbol.set(Some(symbol.clone()));

        let analysis_loading = self.analysis_loading;
        let analysis_json = self.analysis_json;
        let analysis_result = self.analysis_result;
        let analysis_modal_open = self.analysis_modal_open;
        let analysis_error = self.analysis_error;
        let analysis_copied = self.analysis_copied;
        let url = format!(
            "https://cafefnew.mediacdn.vn/Images/Uploaded/DuLieuDownload/Liveboard/{}_PriceHistory.json",
            symbol
        );

        MarketService::new(MarketApi).fetch_url(
            &url,
            Rc::new(move |result| {
                analysis_loading.set(false);
                match result.and_then(|raw| {
                    let input = TechnicalAnalysisService::price_history_input(&raw, &symbol)?;
                    let config = TechnicalAnalysisService::stock_daily_config_for_candles(
                        input.market_data.candles.len(),
                    );
                    TechnicalAnalysisService::analyze(&input, &config, browser::now_iso8601())
                }) {
                    Ok(result) => {
                        let json = match serde_json::to_string_pretty(&result) {
                            Ok(json) => json,
                            Err(error) => {
                                analysis_json.set(None);
                                analysis_result.set(None);
                                analysis_modal_open.set(false);
                                analysis_error.set(Some(format!(
                                    "Failed to serialize analysis result: {error}"
                                )));
                                return;
                            }
                        };
                        analysis_json.set(Some(json.clone()));
                        analysis_result.set(Some(result));
                        analysis_modal_open.set(!copy_result);
                        analysis_error.set(None);
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

    pub fn load(&self) {
        self.loading.set(true);
        self.error.set(None);
        let stocks = self.stocks;
        let total_items = self.total_items;
        let displayed_items = self.displayed_items;
        let error = self.error;
        let loading = self.loading;
        let state = *self;
        self.service.load(Rc::new(move |result| {
            loading.set(false);
            match result {
                Ok(response) => {
                    stocks.set(response.data);
                    total_items.set(response.total_items);
                    displayed_items.set(response.displayed_items);
                    error.set(None);
                    state.load_pinned_history();
                }
                Err(message) => {
                    stocks.set(Vec::new());
                    total_items.set(0);
                    displayed_items.set(0);
                    error.set(Some(message));
                }
            }
        }));
    }
}
