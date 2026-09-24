use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Public Futures ticker state and update primitives.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

const RANKING_WINDOW_MS: u64 = 5 * 60 * 1_000;
const SHORT_WINDOW_MS: u64 = 60 * 1_000;
const MEDIUM_WINDOW_MS: u64 = 3 * 60 * 1_000;
const PREVIOUS_WINDOW_MS: u64 = 2 * 60 * 1_000;

/// A price observation used by the short-term market ranking engine.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct PriceSample {
    timestamp_ms: u64,
    price: f64,
}

/// Short-term ranking metrics for a Futures ticker.
///
/// The score favors fast, directional moves that are sustained over several
/// minutes instead of counting individual socket ticks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuturesTickerMomentum {
    #[serde(default)]
    samples: VecDeque<PriceSample>,
}

impl Default for FuturesTickerMomentum {
    fn default() -> Self {
        Self {
            samples: VecDeque::new(),
        }
    }
}

impl FuturesTickerMomentum {
    /// Creates an empty ranking history.
    pub fn baseline(_price: Option<f64>) -> Self {
        Self::default()
    }

    /// Restores no historical ranking state.
    ///
    /// Kept as a compatibility boundary for callers that previously restored
    /// tick counters. Historical tick counts are intentionally not reused.
    pub fn from_cached_counts(_up_ticks: u64, _down_ticks: u64) -> Self {
        Self::default()
    }

    /// Applies a price observation using a synthetic monotonic sequence when
    /// no exchange timestamp is available.
    pub fn observe(&mut self, price: Option<f64>) {
        let timestamp_ms = self
            .samples
            .back()
            .map(|sample| sample.timestamp_ms.saturating_add(1))
            .unwrap_or(0);
        self.observe_at(price, Some(timestamp_ms));
    }

    /// Applies a timestamped price observation and retains the latest five minutes.
    pub fn observe_at(&mut self, price: Option<f64>, timestamp_ms: Option<u64>) {
        let (Some(price), Some(timestamp_ms)) = (price, timestamp_ms) else {
            return;
        };
        if !price.is_finite() || price <= 0.0 {
            return;
        }

        if let Some(last) = self.samples.back_mut() {
            if timestamp_ms < last.timestamp_ms {
                return;
            }
            if timestamp_ms == last.timestamp_ms {
                last.price = price;
                return;
            }
        }

        self.samples.push_back(PriceSample {
            timestamp_ms,
            price,
        });
        let cutoff = timestamp_ms.saturating_sub(RANKING_WINDOW_MS);
        while self
            .samples
            .front()
            .is_some_and(|sample| sample.timestamp_ms < cutoff)
        {
            self.samples.pop_front();
        }
    }

    /// Resets ranking history while preserving the current price as a baseline.
    pub fn reset_metrics(&mut self) {
        let current = self.samples.back().copied();
        self.samples.clear();
        if let Some(sample) = current {
            self.samples.push_back(sample);
        }
    }

    /// Returns the composite 0..=100 short-term ranking score.
    pub fn ranking_score(&self) -> u8 {
        let Some(current) = self.samples.back() else {
            return 0;
        };

        let Some(return_1m) = self.return_over(SHORT_WINDOW_MS) else {
            return 0;
        };
        let Some(return_3m) = self.return_over(MEDIUM_WINDOW_MS) else {
            return 0;
        };

        let return_5m = self.return_over(RANKING_WINDOW_MS).unwrap_or(return_3m);
        let previous_1m = self
            .return_between(
                current.timestamp_ms.saturating_sub(PREVIOUS_WINDOW_MS),
                current.timestamp_ms.saturating_sub(SHORT_WINDOW_MS),
            )
            .unwrap_or(0.0);
        let acceleration = return_1m - previous_1m;
        let efficiency = self.trend_efficiency(MEDIUM_WINDOW_MS).unwrap_or(0.0);

        let same_direction = if return_1m.signum() == return_3m.signum()
            && return_3m.signum() == return_5m.signum()
            && return_1m != 0.0
        {
            1.0
        } else if return_1m.signum() == return_3m.signum() && return_1m != 0.0 {
            0.65
        } else {
            0.0
        };

        let speed_score = (return_1m.abs() / 0.005 * 40.0).min(40.0);
        let medium_score = (return_3m.abs() / 0.012 * 20.0).min(20.0);
        let acceleration_score = (acceleration.abs() / 0.003 * 15.0).min(15.0);
        let efficiency_score = efficiency * 15.0;
        let consistency_score = same_direction * 10.0;

        (speed_score + medium_score + acceleration_score + efficiency_score + consistency_score)
            .round()
            .clamp(0.0, 100.0) as u8
    }

    /// Returns the direction of the current short-term move.
    pub fn ranking_direction(&self) -> i8 {
        self.return_over(SHORT_WINDOW_MS)
            .unwrap_or(0.0)
            .signum() as i8
    }

    /// Returns the one-minute price return.
    pub fn return_1m(&self) -> Option<f64> {
        self.return_over(SHORT_WINDOW_MS)
    }

    /// Returns the three-minute price return.
    pub fn return_3m(&self) -> Option<f64> {
        self.return_over(MEDIUM_WINDOW_MS)
    }

    /// Returns the five-minute price return.
    pub fn return_5m(&self) -> Option<f64> {
        self.return_over(RANKING_WINDOW_MS)
    }

    /// Returns trend efficiency over the requested window.
    pub fn trend_efficiency_3m(&self) -> Option<f64> {
        self.trend_efficiency(MEDIUM_WINDOW_MS)
    }

    /// Returns the number of observations retained for the ranking window.
    pub fn observation_count(&self) -> usize {
        self.samples.len()
    }

    fn return_over(&self, window_ms: u64) -> Option<f64> {
        let current = self.samples.back()?;
        let cutoff = current.timestamp_ms.saturating_sub(window_ms);
        let base = self.samples.iter().find(|sample| sample.timestamp_ms >= cutoff)?;
        if base.price <= 0.0 {
            return None;
        }
        Some(current.price / base.price - 1.0)
    }

    fn return_between(&self, start_ms: u64, end_ms: u64) -> Option<f64> {
        let start = self
            .samples
            .iter()
            .find(|sample| sample.timestamp_ms >= start_ms)?;
        let end = self
            .samples
            .iter()
            .rev()
            .find(|sample| sample.timestamp_ms <= end_ms)?;
        if start.price <= 0.0 {
            return None;
        }
        Some(end.price / start.price - 1.0)
    }

    fn trend_efficiency(&self, window_ms: u64) -> Option<f64> {
        let current = self.samples.back()?;
        let cutoff = current.timestamp_ms.saturating_sub(window_ms);
        let samples = self
            .samples
            .iter()
            .filter(|sample| sample.timestamp_ms >= cutoff)
            .copied()
            .collect::<Vec<_>>();
        if samples.len() < 2 {
            return None;
        }

        let net = (samples.last()?.price - samples.first()?.price).abs();
        let path = samples
            .windows(2)
            .map(|pair| (pair[1].price - pair[0].price).abs())
            .sum::<f64>();

        if path <= f64::EPSILON {
            return Some(0.0);
        }

        Some((net / path).clamp(0.0, 1.0))
    }

    /// Legacy directional tick count. Ranking no longer uses tick counts.
    pub fn up_ticks(&self) -> u64 {
        0
    }

    /// Legacy directional tick count. Ranking no longer uses tick counts.
    pub fn down_ticks(&self) -> u64 {
        0
    }

    /// Legacy compatibility metric. The ranking score replaces momentum.
    pub fn progress(&self) -> u8 {
        self.ranking_score()
    }
}

/// A Futures ticker together with session-local directional momentum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

        assert_eq!(momentum.up_ticks, 0);
        assert_eq!(momentum.down_ticks, 0);
        assert_eq!(momentum.progress(), 0);
    }

    #[test]
    fn cached_counts_restore_without_a_previous_price() {
        let mut momentum = FuturesTickerMomentum::from_cached_counts(4, 2);
        momentum.observe(Some(100.0));

        assert_eq!(momentum.up_ticks, 4);
        assert_eq!(momentum.down_ticks, 2);
        assert_eq!(momentum.previous_price, Some(100.0));
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
    fn momentum_is_capped_at_100_and_drops_when_direction_reverses() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(0.0));
        for price in 1..=101 {
            momentum.observe(Some(price as f64));
        }

        assert_eq!(momentum.progress(), 100);

        momentum.observe(Some(102.0));
        assert_eq!(momentum.progress(), 100);

        momentum.observe(Some(101.0));
        assert_eq!(momentum.progress(), 98);
    }

    #[test]
    fn momentum_uses_a_rolling_window() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(0.0));
        for price in 1..=101 {
            momentum.observe(Some(price as f64));
        }

        assert_eq!(momentum.up_ticks, 100);
        assert_eq!(momentum.down_ticks, 0);
        assert_eq!(momentum.progress(), 100);

        momentum.observe(Some(100.0));
        assert_eq!(momentum.up_ticks, 99);
        assert_eq!(momentum.down_ticks, 1);
        assert_eq!(momentum.progress(), 98);
    }

    #[test]
    fn reset_metrics_clears_momentum_and_burst_history() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe_at(Some(100.02), Some(1_000));
        momentum.observe_at(Some(100.04), Some(2_000));
        momentum.observe_at(Some(100.08), Some(3_000));
        momentum.observe_at(Some(100.20), Some(4_000));

        momentum.reset_metrics();

        assert_eq!(momentum.previous_price, Some(100.20));
        assert_eq!(momentum.up_ticks, 0);
        assert_eq!(momentum.down_ticks, 0);
        assert_eq!(momentum.burst_score(), 0);
        assert_eq!(momentum.burst_ticks(), 0);
    }

    #[test]
    fn burst_detects_a_sudden_acceleration() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe_at(Some(100.02), Some(1_000));
        momentum.observe_at(Some(100.04), Some(2_000));
        momentum.observe_at(Some(100.08), Some(3_000));
        momentum.observe_at(Some(100.20), Some(4_000));

        assert!(momentum.is_burst());
        assert!(momentum.burst_score() >= 70);
    }

    #[test]
    fn burst_score_decays_instead_of_resetting() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe_at(Some(100.02), Some(1_000));
        momentum.observe_at(Some(100.04), Some(2_000));
        momentum.observe_at(Some(100.08), Some(3_000));
        momentum.observe_at(Some(100.20), Some(4_000));

        let initial = momentum.burst_score();
        momentum.observe_at(Some(100.21), Some(5_000));

        assert!(momentum.burst_score() < initial);
        assert!(momentum.burst_score() > 0);
    }

    #[test]
    fn burst_does_not_trigger_on_steady_moves() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe_at(Some(100.02), Some(1_000));
        momentum.observe_at(Some(100.04), Some(2_000));
        momentum.observe_at(Some(100.06), Some(3_000));
        momentum.observe_at(Some(100.08), Some(4_000));

        assert!(!momentum.is_burst());
    }

    #[test]
    fn burst_requires_timestamped_observations() {
        let mut momentum = FuturesTickerMomentum::baseline(Some(100.0));
        momentum.observe(Some(100.02));
        momentum.observe(Some(100.20));

        assert_eq!(momentum.burst_score(), 0);
        assert!(!momentum.is_burst());
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
