pub mod base64;
pub mod developer;
pub mod finance;
pub mod json;
pub mod jwt;
pub mod time;

use std::collections::HashMap;

use crate::domain::futures::{
    FuturesTickerMomentum, FuturesTickerRegistry, FuturesTickerUpdate, TrackedFuturesTicker,
};
use crate::domain::markdown::{render_markdown, RenderedMarkdown};

/// Application service that combines the live Futures registry with session-local momentum.
#[derive(Debug, Default)]
pub struct FuturesMarketService {
    registry: FuturesTickerRegistry,
    momentum: HashMap<String, FuturesTickerMomentum>,
}

impl FuturesMarketService {
    /// Creates an empty market service.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies an incremental market batch and returns the current tracked snapshot.
    pub fn apply_batch(&mut self, updates: Vec<FuturesTickerUpdate>) -> Vec<TrackedFuturesTicker> {
        for update in &updates {
            let momentum = self
                .momentum
                .entry(update.symbol.clone())
                .or_insert_with(|| FuturesTickerMomentum::baseline(None));
            momentum.observe(update.last_price);
        }

        let snapshot = self.registry.apply_batch(updates);
        snapshot
            .into_iter()
            .filter_map(|ticker| {
                self.momentum
                    .get(&ticker.symbol)
                    .cloned()
                    .map(|momentum| TrackedFuturesTicker { ticker, momentum })
            })
            .collect()
    }

    /// Re-baselines all known tickers after a reconnect without creating synthetic ticks.
    pub fn rebaseline(&mut self) {
        let snapshot = self.registry.apply_batch(Vec::new());
        for ticker in snapshot {
            if let Some(momentum) = self.momentum.get_mut(&ticker.symbol) {
                *momentum = FuturesTickerMomentum::baseline(None);
            }
        }
    }
}

/// Application service for Markdown rendering.
pub struct MarkdownService;

impl MarkdownService {
    /// Renders Markdown content into platform presentation segments.
    pub fn render(content: &str) -> RenderedMarkdown {
        render_markdown(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::markdown::RenderSegment;

    #[test]
    fn test_service_renders_markdown() {
        let result = MarkdownService::render("# Hello");
        assert!(!result.segments.is_empty());
        match &result.segments[0] {
            RenderSegment::Html(h) => assert!(h.contains("<h1>")),
            _ => panic!("Expected HTML segment"),
        }
    }

    fn update(symbol: &str, price: f64) -> FuturesTickerUpdate {
        FuturesTickerUpdate {
            symbol: symbol.into(),
            last_price: Some(price),
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: None,
        }
    }

    #[test]
    fn first_update_is_a_baseline() {
        let mut service = FuturesMarketService::new();
        let snapshot = service.apply_batch(vec![update("BTC_USDT", 100.0)]);

        assert_eq!(snapshot[0].momentum.up_ticks, 0);
        assert_eq!(snapshot[0].momentum.down_ticks, 0);
    }

    #[test]
    fn subsequent_updates_count_directional_ticks() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        service.apply_batch(vec![update("BTC_USDT", 101.0)]);
        let snapshot = service.apply_batch(vec![update("BTC_USDT", 100.0)]);

        assert_eq!(snapshot[0].momentum.up_ticks, 1);
        assert_eq!(snapshot[0].momentum.down_ticks, 1);
        assert_eq!(snapshot[0].momentum.progress(), 0);
    }

    #[test]
    fn reconnect_rebaseline_does_not_create_a_tick() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        service.apply_batch(vec![update("BTC_USDT", 101.0)]);
        service.rebaseline();
        let snapshot = service.apply_batch(vec![update("BTC_USDT", 102.0)]);

        assert_eq!(snapshot[0].momentum.up_ticks, 1);
        assert_eq!(snapshot[0].momentum.down_ticks, 0);
    }
}
