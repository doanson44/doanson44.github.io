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

/// Application service that owns live Futures market state and session-local momentum.
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

    /// Applies market updates immediately without publishing a UI snapshot.
    pub fn apply_batch(&mut self, updates: impl IntoIterator<Item = FuturesTickerUpdate>) {
        let updates = updates.into_iter().collect::<Vec<_>>();
        for update in &updates {
            let momentum = self
                .momentum
                .entry(update.symbol.clone())
                .or_insert_with(|| FuturesTickerMomentum::baseline(None));
            momentum.observe(update.last_price);
        }
        self.registry.apply_batch(updates);
    }

    /// Creates a point-in-time projection for the UI flush cycle.
    pub fn snapshot(&self) -> HashMap<String, TrackedFuturesTicker> {
        self.registry
            .snapshot()
            .map(|(symbol, ticker)| {
                let momentum = self
                    .momentum
                    .get(symbol)
                    .cloned()
                    .unwrap_or_else(|| FuturesTickerMomentum::baseline(ticker.last_price));
                (
                    symbol.clone(),
                    TrackedFuturesTicker {
                        ticker: ticker.clone(),
                        momentum,
                    },
                )
            })
            .collect()
    }

    /// Exports all session-local momentum data, including items not actively in the registry.
    pub fn export_momentum(&self) -> impl Iterator<Item = (&String, &FuturesTickerMomentum)> {
        self.momentum.iter()
    }

    /// Restores only cached directional counters; live ticker data is not restored.
    pub fn restore_momentum(&mut self, cached: impl IntoIterator<Item = (String, u64, u64)>) {
        for (symbol, up_ticks, down_ticks) in cached {
            self.momentum.insert(
                symbol,
                FuturesTickerMomentum::from_cached_counts(up_ticks, down_ticks),
            );
        }
    }

    /// Re-baselines known tickers after reconnect without creating synthetic ticks.
    pub fn rebaseline(&mut self) {
        for (symbol, ticker) in self.registry.snapshot() {
            if let Some(momentum) = self.momentum.get_mut(symbol) {
                *momentum = FuturesTickerMomentum::baseline(ticker.last_price);
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
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].momentum.up_ticks, 0);
        assert_eq!(snapshot["BTC_USDT"].momentum.down_ticks, 0);
    }

    #[test]
    fn subsequent_updates_count_directional_ticks() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        service.apply_batch(vec![update("BTC_USDT", 101.0)]);
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].momentum.up_ticks, 1);
        assert_eq!(snapshot["BTC_USDT"].momentum.down_ticks, 1);
        assert_eq!(snapshot["BTC_USDT"].momentum.progress(), 0);
    }

    #[test]
    fn cached_momentum_is_applied_before_the_first_live_price() {
        let mut service = FuturesMarketService::new();
        service.restore_momentum(vec![("BTC_USDT".into(), 4, 2)]);
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].momentum.up_ticks, 4);
        assert_eq!(snapshot["BTC_USDT"].momentum.down_ticks, 2);
        assert_eq!(snapshot["BTC_USDT"].momentum.previous_price, Some(100.0));
    }

    #[test]
    fn cached_momentum_continues_with_live_ticks() {
        let mut service = FuturesMarketService::new();
        service.restore_momentum(vec![("BTC_USDT".into(), 4, 2)]);
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        service.apply_batch(vec![update("BTC_USDT", 101.0)]);
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].momentum.up_ticks, 5);
        assert_eq!(snapshot["BTC_USDT"].momentum.down_ticks, 3);
    }

    #[test]
    fn reconnect_rebaseline_does_not_create_a_tick() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![update("BTC_USDT", 100.0)]);
        service.apply_batch(vec![update("BTC_USDT", 101.0)]);
        service.rebaseline();
        service.apply_batch(vec![update("BTC_USDT", 102.0)]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].momentum.up_ticks, 1);
        assert_eq!(snapshot["BTC_USDT"].momentum.down_ticks, 0);
    }
}
