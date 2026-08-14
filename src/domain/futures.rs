/// Public Futures ticker state and update primitives.
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesTicker {
    pub symbol: String,
    pub last_price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub change_24h: Option<f64>,
    pub fair_price: Option<f64>,
    pub updated_at_ms: Option<u64>,
}

/// Partial update emitted by a public Futures ticker stream.
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesTickerUpdate {
    pub symbol: String,
    pub last_price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub change_24h: Option<f64>,
    pub fair_price: Option<f64>,
    pub updated_at_ms: Option<u64>,
}

/// Tracks directional price changes for one ticker during the current page session.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FuturesTickerMomentum {
    pub previous_price: Option<f64>,
    pub up_ticks: u64,
    pub down_ticks: u64,
}

impl FuturesTickerMomentum {
    /// Creates a baseline without counting the first observed price as a tick.
    pub fn baseline(price: Option<f64>) -> Self {
        Self {
            previous_price: price,
            ..Self::default()
        }
    }

    /// Applies a price observation.
    pub fn observe(&mut self, price: Option<f64>) {
        let Some(new_price) = price else {
            return;
        };
        let Some(previous_price) = self.previous_price else {
            self.previous_price = Some(new_price);
            return;
        };

        if new_price > previous_price {
            self.up_ticks = self.up_ticks.saturating_add(1);
        } else if new_price < previous_price {
            self.down_ticks = self.down_ticks.saturating_add(1);
        }
        self.previous_price = Some(new_price);
    }

    /// Returns net directional ticks, positive for upward movement.
    pub fn net_ticks(&self) -> i64 {
        self.up_ticks as i64 - self.down_ticks as i64
    }

    /// Returns the green fill percentage clamped to the range 0..=100.
    pub fn progress(&self) -> u8 {
        self.net_ticks().clamp(0, 100) as u8
    }
}

/// A Futures ticker together with session-local directional momentum.
#[derive(Debug, Clone, PartialEq)]
pub struct TrackedFuturesTicker {
    pub ticker: FuturesTicker,
    pub momentum: FuturesTickerMomentum,
}

impl FuturesTicker {
    fn new(update: FuturesTickerUpdate) -> Self {
        let mut ticker = Self {
            symbol: update.symbol.clone(),
            last_price: None,
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: None,
        };
        ticker.apply(update);
        ticker
    }

    /// Applies only fields present in the incoming update.
    pub fn apply(&mut self, update: FuturesTickerUpdate) {
        if let Some(value) = update.last_price {
            self.last_price = Some(value);
        }
        if let Some(value) = update.volume_24h {
            self.volume_24h = Some(value);
        }
        if let Some(value) = update.change_24h {
            self.change_24h = Some(value);
        }
        if let Some(value) = update.fair_price {
            self.fair_price = Some(value);
        }
        if let Some(value) = update.updated_at_ms {
            self.updated_at_ms = Some(value);
        }
    }
}

/// In-memory source of truth for the live Futures market.
#[derive(Debug, Default)]
pub struct FuturesTickerRegistry {
    tickers: std::collections::HashMap<String, FuturesTicker>,
}

impl FuturesTickerRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies a batch of ticker updates and returns a stable snapshot.
    pub fn apply_batch(&mut self, updates: Vec<FuturesTickerUpdate>) -> Vec<FuturesTicker> {
        for update in updates {
            if let Some(ticker) = self.tickers.get_mut(&update.symbol) {
                ticker.apply(update);
            } else {
                let symbol = update.symbol.clone();
                self.tickers.insert(symbol, FuturesTicker::new(update));
            }
        }

        let mut snapshot = self.tickers.values().cloned().collect::<Vec<_>>();
        snapshot.sort_unstable_by(|left, right| left.symbol.cmp(&right.symbol));
        snapshot
    }

    /// Returns the number of contracts currently known by the registry.
    pub fn len(&self) -> usize {
        self.tickers.len()
    }

    /// Returns whether the registry has no known contracts.
    pub fn is_empty(&self) -> bool {
        self.tickers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn update(symbol: &str, price: f64, volume: f64) -> FuturesTickerUpdate {
        FuturesTickerUpdate {
            symbol: symbol.into(),
            last_price: Some(price),
            volume_24h: Some(volume),
            change_24h: None,
            fair_price: None,
            updated_at_ms: None,
        }
    }

    #[test]
    fn updates_existing_ticker_without_replacing_missing_fields() {
        let mut registry = FuturesTickerRegistry::new();
        registry.apply_batch(vec![FuturesTickerUpdate {
            symbol: "BTC_USDT".into(),
            last_price: Some(100.0),
            volume_24h: Some(200.0),
            change_24h: Some(0.05),
            fair_price: Some(101.0),
            updated_at_ms: Some(1),
        }]);

        let snapshot = registry.apply_batch(vec![FuturesTickerUpdate {
            symbol: "BTC_USDT".into(),
            last_price: Some(102.0),
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: Some(2),
        }]);
        let ticker = &snapshot[0];

        assert_eq!(ticker.last_price, Some(102.0));
        assert_eq!(ticker.volume_24h, Some(200.0));
        assert_eq!(ticker.change_24h, Some(0.05));
        assert_eq!(ticker.fair_price, Some(101.0));
        assert_eq!(ticker.updated_at_ms, Some(2));
    }

    #[test]
    fn creates_new_ticker_when_symbol_is_unknown() {
        let mut registry = FuturesTickerRegistry::new();
        let snapshot = registry.apply_batch(vec![update("ETH_USDT", 2000.0, 50.0)]);

        assert_eq!(registry.len(), 1);
        assert_eq!(snapshot[0].symbol, "ETH_USDT");
    }

    #[test]
    fn snapshot_is_stable_by_symbol() {
        let mut registry = FuturesTickerRegistry::new();
        let snapshot = registry.apply_batch(vec![
            update("SOL_USDT", 10.0, 1.0),
            update("BTC_USDT", 20.0, 2.0),
        ]);

        assert_eq!(snapshot[0].symbol, "BTC_USDT");
        assert_eq!(snapshot[1].symbol, "SOL_USDT");
    }

    #[test]
    fn first_price_is_a_baseline() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(100.0));

        assert_eq!(momentum.up_ticks, 0);
        assert_eq!(momentum.down_ticks, 0);
        assert_eq!(momentum.progress(), 0);
    }

    #[test]
    fn price_changes_update_directional_ticks() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(101.0));
        momentum.observe(Some(102.0));
        momentum.observe(Some(101.0));
        momentum.observe(Some(101.0));

        assert_eq!(momentum.up_ticks, 2);
        assert_eq!(momentum.down_ticks, 1);
        assert_eq!(momentum.net_ticks(), 1);
        assert_eq!(momentum.progress(), 1);
    }

    #[test]
    fn progress_is_clamped_to_zero_and_one_hundred() {
        let mut down = FuturesTickerMomentum::baseline(Some(100.0));
        for price in (0..105).rev() {
            down.observe(Some(price as f64));
        }
        assert_eq!(down.progress(), 0);

        let mut up = FuturesTickerMomentum::baseline(Some(0.0));
        for price in 1..105 {
            up.observe(Some(price as f64));
        }
        assert_eq!(up.progress(), 100);
    }

    #[test]
    fn missing_price_does_not_create_a_tick() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(None);
        assert_eq!(momentum.up_ticks, 0);
        assert_eq!(momentum.down_ticks, 0);
        assert_eq!(momentum.previous_price, Some(100.0));
    }
}
