use std::rc::Rc;

use leptos::prelude::*;

use crate::application::ports::MarketPinStore;
use crate::application::services::market::MarketService;
use crate::domain::market::MarketStock;
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
        }
    }

    pub fn load(&self) {
        self.loading.set(true);
        self.error.set(None);
        let stocks = self.stocks;
        let total_items = self.total_items;
        let displayed_items = self.displayed_items;
        let error = self.error;
        let loading = self.loading;
        self.service.load(Rc::new(move |result| {
            loading.set(false);
            match result {
                Ok(response) => {
                    stocks.set(response.data);
                    total_items.set(response.total_items);
                    displayed_items.set(response.displayed_items);
                    error.set(None);
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
