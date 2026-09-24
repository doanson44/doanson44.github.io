pub mod base64;
pub mod developer;
pub mod finance;
pub mod games;
pub mod json;
pub mod jwt;
pub mod market;
pub mod proxy;
pub mod technical_analysis;
pub mod time;
pub mod trading;

use std::collections::HashMap;

use crate::domain::futures::{
    FuturesTickerRanking, FuturesTickerRegistry, FuturesTickerUpdate, TrackedFuturesTicker,
};
use crate::domain::markdown::{render_markdown, RenderedMarkdown};

/// Application service that owns live Futures market state and short-term ranking.
#[derive(Debug, Default)]
pub struct FuturesMarketService {
    registry: FuturesTickerRegistry,
    ranking: HashMap<String, FuturesTickerRanking>,
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
            let ranking = self
                .ranking
                .entry(update.symbol.clone())
                .or_insert_with(|| FuturesTickerRanking::baseline(None));
            ranking.observe_at(update.last_price, update.updated_at_ms);
        }
        self.registry.apply_batch(updates);
    }

    /// Creates a point-in-time projection for the UI flush cycle.
    pub fn snapshot(&self) -> HashMap<String, TrackedFuturesTicker> {
        self.registry
            .snapshot()
            .map(|(symbol, ticker)| {
                let ranking = self
                    .ranking
                    .get(symbol)
                    .cloned()
                    .unwrap_or_else(|| FuturesTickerRanking::baseline(ticker.last_price));
                (
                    symbol.clone(),
                    TrackedFuturesTicker {
                        ticker: ticker.clone(),
                        ranking,
                    },
                )
            })
            .collect()
    }

    /// Resets short-term ranking history for all known tickers.
    ///
    /// Current market prices are preserved as baselines so the next live update
    /// starts a fresh measurement window without creating a synthetic tick.
    pub fn reset_metrics(&mut self) {
        let baselines = self
            .registry
            .snapshot()
            .map(|(symbol, ticker)| (symbol.clone(), ticker.last_price))
            .collect::<Vec<_>>();

        self.ranking.clear();
        for (symbol, price) in baselines {
            self.ranking
                .insert(symbol, FuturesTickerRanking::baseline(price));
        }
    }

    /// Re-baselines known tickers after reconnect without creating synthetic ticks.
    pub fn rebaseline(&mut self) {
        for (symbol, ticker) in self.registry.snapshot() {
            if let Some(ranking) = self.ranking.get_mut(symbol) {
                *ranking = FuturesTickerRanking::baseline(ticker.last_price);
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

        assert_eq!(snapshot["BTC_USDT"].ranking.ranking_score(), 0);
    }

    #[test]
    fn ranking_requires_enough_history() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![
            update("BTC_USDT", 100.0),
            update("BTC_USDT", 100.2),
            update("BTC_USDT", 100.5),
        ]);
        let snapshot = service.snapshot();

        assert_eq!(snapshot["BTC_USDT"].ranking.ranking_score(), 0);
    }

    #[test]
    fn ranking_detects_a_fast_directional_move() {
        let mut service = FuturesMarketService::new();
        for (timestamp, price) in [
(0, 100.0),
            (15_000, 100.8),
            (30_000, 101.7),
            (45_000, 102.8),
            (60_000, 104.0),
            (75_000, 105.0),
            (90_000, 106.0),
            (120_000, 108.0),
            (180_000, 110.0),
            (240_000, 112.0),
            (300_000, 114.0)
        ] {
            service.apply_batch(vec![FuturesTickerUpdate {
                symbol: "BTC_USDT".into(),
                last_price: Some(price),
                volume_24h: None,
                change_24h: None,
                fair_price: None,
                updated_at_ms: Some(timestamp),
            }]);
        }

        let ranking = service.snapshot()["BTC_USDT"].ranking.ranking_score();
        assert!(
            ranking >= 60,
            "ranking should identify a strong move: {ranking}"
        );
        assert_eq!(
            service.snapshot()["BTC_USDT"].ranking.ranking_direction(),
            1
        );
    }

    #[test]
    fn reset_preserves_price_baseline_but_clears_ranking_history() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![
            FuturesTickerUpdate {
                symbol: "BTC_USDT".into(),
                last_price: Some(100.0),
                volume_24h: None,
                change_24h: None,
                fair_price: None,
                updated_at_ms: Some(0),
            },
            FuturesTickerUpdate {
                symbol: "BTC_USDT".into(),
                last_price: Some(101.0),
                volume_24h: None,
                change_24h: None,
                fair_price: None,
                updated_at_ms: Some(60_000),
            },
        ]);

        service.reset_metrics();
        service.apply_batch(vec![FuturesTickerUpdate {
            symbol: "BTC_USDT".into(),
            last_price: Some(102.0),
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: Some(120_000),
        }]);

        assert_eq!(service.snapshot()["BTC_USDT"].ranking.ranking_score(), 0);
    }

    #[test]
    fn reconnect_rebaseline_does_not_create_a_synthetic_move() {
        let mut service = FuturesMarketService::new();
        service.apply_batch(vec![
            FuturesTickerUpdate {
                symbol: "BTC_USDT".into(),
                last_price: Some(100.0),
                volume_24h: None,
                change_24h: None,
                fair_price: None,
                updated_at_ms: Some(0),
            },
            FuturesTickerUpdate {
                symbol: "BTC_USDT".into(),
                last_price: Some(101.0),
                volume_24h: None,
                change_24h: None,
                fair_price: None,
                updated_at_ms: Some(60_000),
            },
        ]);
        service.rebaseline();
        service.apply_batch(vec![FuturesTickerUpdate {
            symbol: "BTC_USDT".into(),
            last_price: Some(102.0),
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: Some(120_000),
        }]);

        assert_eq!(service.snapshot()["BTC_USDT"].ranking.ranking_score(), 0);
    }
}
