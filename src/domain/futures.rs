use std::collections::{HashMap, VecDeque};

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

const MOMENTUM_WINDOW: usize = 100;

/// Tracks percentage price movement over a bounded rolling window.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FuturesTickerMomentum {
    pub previous_price: Option<f64>,
    pub up_movement_percent: f64,
    pub down_movement_percent: f64,
    movements: VecDeque<f64>,
}

impl FuturesTickerMomentum {
    /// Creates a baseline without counting the first observed price as movement.
    pub fn baseline(price: Option<f64>) -> Self {
        Self {
            previous_price: price,
            ..Self::default()
        }
    }

    /// Applies a price observation and keeps only the latest non-zero movements.
    pub fn observe(&mut self, price: Option<f64>) {
        let Some(new_price) = price else {
            return;
        };
        let Some(previous_price) = self.previous_price else {
            self.previous_price = Some(new_price);
            return;
        };

        if previous_price <= 0.0 {
            self.previous_price = Some(new_price);
            return;
        }

        let movement = (new_price - previous_price) / previous_price * 100.0;
        if movement.abs() > f64::EPSILON {
            self.movements.push_back(movement);
            self.add_movement(movement);

            if self.movements.len() > MOMENTUM_WINDOW {
                if let Some(oldest) = self.movements.pop_front() {
                    self.remove_movement(oldest);
                }
            }
        }

        self.previous_price = Some(new_price);
    }

    /// Returns the net percentage movement in the current rolling window.
    pub fn net_movement_percent(&self) -> f64 {
        self.up_movement_percent - self.down_movement_percent
    }

    /// Returns positive momentum as a bounded 0..=100 score.
    pub fn progress(&self) -> u8 {
        self.net_movement_percent().clamp(0.0, 100.0).round() as u8
    }

    fn add_movement(&mut self, movement: f64) {
        if movement > 0.0 {
            self.up_movement_percent += movement;
        } else {
            self.down_movement_percent += -movement;
        }
    }

    fn remove_movement(&mut self, movement: f64) {
        if movement > 0.0 {
            self.up_movement_percent = (self.up_movement_percent - movement).max(0.0);
        } else {
            self.down_movement_percent = (self.down_movement_percent + movement).max(0.0);
        }
    }
}

/// A Futures ticker together with session-local percentage momentum.
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
    tickers: HashMap<String, FuturesTicker>,
}

impl FuturesTickerRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies incremental ticker updates without rebuilding a full snapshot.
    pub fn apply_batch(&mut self, updates: impl IntoIterator<Item = FuturesTickerUpdate>) {
        for update in updates {
            if let Some(ticker) = self.tickers.get_mut(&update.symbol) {
                ticker.apply(update);
            } else {
                let symbol = update.symbol.clone();
                self.tickers.insert(symbol, FuturesTicker::new(update));
            }
        }
    }

    /// Returns the current ticker collection for a UI projection.
    pub fn snapshot(&self) -> impl Iterator<Item = (&String, &FuturesTicker)> {
        self.tickers.iter()
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

        registry.apply_batch(vec![FuturesTickerUpdate {
            symbol: "BTC_USDT".into(),
            last_price: Some(102.0),
            volume_24h: None,
            change_24h: None,
            fair_price: None,
            updated_at_ms: Some(2),
        }]);
        let ticker = registry.snapshot().next().expect("ticker should exist").1;

        assert_eq!(ticker.last_price, Some(102.0));
        assert_eq!(ticker.volume_24h, Some(200.0));
        assert_eq!(ticker.change_24h, Some(0.05));
        assert_eq!(ticker.fair_price, Some(101.0));
        assert_eq!(ticker.updated_at_ms, Some(2));
    }

    #[test]
    fn creates_new_ticker_when_symbol_is_unknown() {
        let mut registry = FuturesTickerRegistry::new();
        registry.apply_batch(vec![update("ETH_USDT", 2000.0, 50.0)]);

        assert_eq!(registry.len(), 1);
        assert_eq!(
            registry
                .snapshot()
                .next()
                .map(|(_, ticker)| ticker.symbol.as_str()),
            Some("ETH_USDT")
        );
    }

    #[test]
    fn first_price_is_a_baseline() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(100.0));

        assert_eq!(momentum.up_movement_percent, 0.0);
        assert_eq!(momentum.down_movement_percent, 0.0);
        assert_eq!(momentum.progress(), 0);
    }

    #[test]
    fn movement_weight_reflects_percentage_magnitude() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(101.0));
        assert!((momentum.up_movement_percent - 1.0).abs() < 1e-10);
        assert_eq!(momentum.progress(), 1);

        momentum.observe(Some(111.0));
        assert!(momentum.up_movement_percent > 10.0);
        assert!(momentum.progress() > 10);
    }

    #[test]
    fn downward_movement_reduces_positive_momentum() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(110.0));
        let before_reversal = momentum.progress();

        momentum.observe(Some(100.0));

        assert!(momentum.progress() < before_reversal);
        assert!(momentum.down_movement_percent > 0.0);
    }

    #[test]
    fn momentum_is_capped_at_100() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(1.0));
        let mut price = 1.0;
        for _ in 0..100 {
            price *= 2.0;
            momentum.observe(Some(price));
        }

        assert_eq!(momentum.progress(), 100);
    }

    #[test]
    fn momentum_uses_a_rolling_window() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        for _ in 0..100 {
            momentum.observe(Some(101.0));
            momentum.observe(Some(100.0));
        }

        assert_eq!(momentum.movements.len(), 100);
        assert!(momentum.net_movement_percent() < 0.0);
        assert_eq!(momentum.progress(), 0);
    }

    #[test]
    fn missing_or_invalid_price_does_not_create_movement() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(None);
        momentum.observe(Some(0.0));

        assert_eq!(momentum.up_movement_percent, 0.0);
        assert_eq!(momentum.down_movement_percent, 0.0);
        assert_eq!(momentum.previous_price, Some(0.0));
    }
}
