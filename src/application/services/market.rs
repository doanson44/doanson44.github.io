use std::rc::Rc;

use crate::application::ports::MarketClient;
use crate::domain::market::{parse_market_response, MarketPriceExtremes};

/// Application service for retrieving CafeF market data directly from CafeF.
#[derive(Debug, Clone, Copy)]
pub struct MarketService<C> {
    client: C,
}

impl<C> MarketService<C>
where
    C: MarketClient + Clone + 'static,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn load(
        &self,
        on_result: Rc<dyn Fn(Result<crate::domain::market::MarketResponse, String>)>,
    ) {
        self.client.fetch(Rc::new(move |result| {
            on_result(result.and_then(|raw| parse_market_response(&raw)));
        }));
    }

    pub fn load_history(
        &self,
        symbol: &str,
        week_52_cutoff: &str,
        on_result: Rc<dyn Fn(Result<MarketPriceExtremes, String>)>,
    ) {
        let symbol = symbol.trim().to_ascii_uppercase();
        let callback_symbol = symbol.clone();
        let cutoff = week_52_cutoff.to_string();
        self.client.fetch_history(
            &symbol,
            Rc::new(move |result| {
                let parsed = result.and_then(|raw| {
                    let history =
                        crate::domain::market::parse_price_history_response(&raw, &callback_symbol)?;
                    Ok(history.extremes(&cutoff))
                });
                on_result(parsed);
            }),
        );
    }

    pub fn fetch_url(&self, target_url: &str, on_result: Rc<dyn Fn(Result<String, String>)>) {
        self.client.fetch_url(target_url, on_result);
    }
}
