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

const FAST_WINDOW_MS: u64 = 15 * 1_000;
const MEDIUM_WINDOW_MS: u64 = 60 * 1_000;
const RANKING_WINDOW_MS: u64 = 5 * 60 * 1_000;
const DIRECTION_THRESHOLD: f64 = 0.15;

/// A price observation used by the short-term market ranking engine.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct PriceSample {
    timestamp_ms: u64,
    price: f64,
}

/// Multi-horizon momentum metrics for a Futures ticker.
///
/// The score blends fast, medium, and slow returns, normalizes them by realized
/// volatility, and discounts signals that do not yet have enough history.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FuturesTickerRanking {
    #[serde(default)]
    samples: VecDeque<PriceSample>,
}

impl FuturesTickerRanking {
    /// Creates an empty ranking history.
    pub fn baseline(_price: Option<f64>) -> Self {
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
        while self.samples.len() > 1
            && self
                .samples
                .get(1)
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

    /// Returns the composite 0..=100 momentum strength score.
    pub fn ranking_score(&self) -> u8 {
        let Some(signal) = self.momentum_signal() else {
            return 0;
        };

        (signal.abs() * 100.0).round().clamp(0.0, 100.0) as u8
    }

    /// Returns the current momentum direction.
    pub fn ranking_direction(&self) -> i8 {
        let Some(signal) = self.momentum_signal() else {
            return 0;
        };
        if signal >= DIRECTION_THRESHOLD {
            1
        } else if signal <= -DIRECTION_THRESHOLD {
            -1
        } else {
            0
        }
    }

    /// Returns the fifteen-second price return.
    pub fn return_15s(&self) -> Option<f64> {
        self.return_over(FAST_WINDOW_MS)
    }

    /// Returns the one-minute price return.
    pub fn return_1m(&self) -> Option<f64> {
        self.return_over(MEDIUM_WINDOW_MS)
    }

    /// Returns the five-minute price return.
    pub fn return_5m(&self) -> Option<f64> {
        self.return_over(RANKING_WINDOW_MS)
    }

    /// Returns trend efficiency over the one-minute window.
    pub fn trend_efficiency_1m(&self) -> Option<f64> {
        self.trend_efficiency(MEDIUM_WINDOW_MS)
    }

    /// Returns the number of observations retained for the ranking window.
    pub fn observation_count(&self) -> usize {
        self.samples.len()
    }

    fn momentum_signal(&self) -> Option<f64> {
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;
        let horizons = [
            (self.return_over(FAST_WINDOW_MS), FAST_WINDOW_MS, 0.25),
            (self.return_over(MEDIUM_WINDOW_MS), MEDIUM_WINDOW_MS, 0.40),
            (self.return_over(RANKING_WINDOW_MS), RANKING_WINDOW_MS, 0.35),
        ];

        for (price_return, window_ms, weight) in horizons {
            if let Some(price_return) = price_return {
                let volatility = self.realized_volatility(window_ms).unwrap_or(0.0);
                let normalized = if volatility > f64::EPSILON {
                    let observations = (window_ms as f64 / 1_000.0).sqrt();
                    (price_return / (volatility * observations)).clamp(-3.0, 3.0) / 3.0
                } else {
                    price_return.signum().clamp(-1.0, 1.0)
                };
                weighted_sum += normalized * weight;
                weight_sum += weight;
            }
        }

        if weight_sum <= f64::EPSILON {
            return None;
        }

        let available_horizons = horizons
            .iter()
            .filter(|(price_return, _, _)| price_return.is_some())
            .count();
        let history_factor = match available_horizons {
            1 => 0.40,
            2 => 0.70,
            _ => 1.0,
        };

        let efficiency = self.trend_efficiency(MEDIUM_WINDOW_MS).unwrap_or(0.0);
        let consistency = self.direction_consistency(MEDIUM_WINDOW_MS).unwrap_or(0.0);
        let quality = (0.5 + efficiency * 0.25 + consistency * 0.25).clamp(0.5, 1.0);

        Some((weighted_sum / weight_sum) * history_factor * quality)
    }

    fn realized_volatility(&self, window_ms: u64) -> Option<f64> {
        let current = self.samples.back()?;
        let cutoff = current.timestamp_ms.saturating_sub(window_ms);
        let samples = self
            .samples
            .iter()
            .filter(|sample| sample.timestamp_ms >= cutoff)
            .copied()
            .collect::<Vec<_>>();
        if samples.len() < 3 {
            return None;
        }

        let returns = samples
            .windows(2)
            .filter_map(|pair| {
                if pair[0].price <= 0.0 {
                    None
                } else {
                    Some(pair[1].price / pair[0].price - 1.0)
                }
            })
            .collect::<Vec<_>>();
        if returns.len() < 2 {
            return None;
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns
            .iter()
            .map(|value| {
                let delta = *value - mean;
                delta * delta
            })
            .sum::<f64>()
            / (returns.len() - 1) as f64;

        Some(variance.sqrt())
    }

    fn direction_consistency(&self, window_ms: u64) -> Option<f64> {
        let current = self.samples.back()?;
        let cutoff = current.timestamp_ms.saturating_sub(window_ms);
        let samples = self
            .samples
            .iter()
            .filter(|sample| sample.timestamp_ms >= cutoff)
            .copied()
            .collect::<Vec<_>>();
        if samples.len() < 3 {
            return None;
        }

        let net_direction = (samples.last()?.price - samples.first()?.price).signum();
        if net_direction == 0.0 {
            return Some(0.0);
        }

        let directional_moves = samples
            .windows(2)
            .filter(|pair| (pair[1].price - pair[0].price).signum() == net_direction)
            .count();

        Some(directional_moves as f64 / (samples.len() - 1) as f64)
    }

    fn return_over(&self, window_ms: u64) -> Option<f64> {
        let current = self.samples.back()?;
        if current.timestamp_ms < window_ms {
            return None;
        }
        let cutoff = current.timestamp_ms - window_ms;
        let base = self
            .samples
            .iter()
            .rev()
            .find(|sample| sample.timestamp_ms <= cutoff)?;
        if base.price <= 0.0 {
            return None;
        }
        Some(current.price / base.price - 1.0)
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
}

/// A Futures ticker together with its session-local short-term ranking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedFuturesTicker {
    pub ticker: FuturesTicker,
    pub ranking: FuturesTickerRanking,
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
        let mut ranking = FuturesTickerRanking::baseline(None);
        ranking.observe(Some(100.0));

        assert_eq!(ranking.ranking_score(), 0);
        assert_eq!(ranking.ranking_direction(), 0);
    }

    #[test]
    fn ranking_warms_up_across_horizons() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(100.5), Some(15_000));
        assert!(ranking.ranking_score() > 0);

        ranking.observe_at(Some(101.0), Some(60_000));
        assert!(ranking.ranking_score() > 0);
        assert_eq!(ranking.ranking_direction(), 1);
    }

    #[test]
    fn ranking_detects_a_fast_directional_move() {
        let mut ranking = FuturesTickerRanking::default();
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
            ranking.observe_at(Some(price), Some(timestamp));
        }

        assert!(ranking.ranking_score() >= 60);
        assert_eq!(ranking.ranking_direction(), 1);
        assert!(ranking.return_15s().unwrap() > 0.0);
        assert!(ranking.return_1m().unwrap() > 0.0);
        assert!(ranking.trend_efficiency_1m().unwrap() > 0.9);
    }

    #[test]
    fn ranking_handles_a_fast_downward_move() {
        let mut ranking = FuturesTickerRanking::default();
        for (timestamp, price) in [
            (0, 100.0),
            (15_000, 99.2),
            (30_000, 98.4),
            (45_000, 97.5),
            (60_000, 96.4),
            (75_000, 95.5),
            (90_000, 94.5),
            (120_000, 92.5),
            (180_000, 90.5),
            (240_000, 88.5),
            (300_000, 86.5),
        ] {
            ranking.observe_at(Some(price), Some(timestamp));
        }

        assert!(ranking.ranking_score() >= 70);
        assert_eq!(ranking.ranking_direction(), -1);
    }

    #[test]
    fn reset_keeps_only_the_current_baseline() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(1_000));
        ranking.observe_at(Some(102.0), Some(2_000));

        ranking.reset_metrics();

        assert_eq!(ranking.observation_count(), 1);
        assert_eq!(ranking.ranking_score(), 0);
    }

    #[test]
    fn sparse_observations_keep_a_baseline_for_windowed_returns() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(4_000));
        ranking.observe_at(Some(102.0), Some(20_000));

        assert_eq!(ranking.observation_count(), 3);
        assert!(ranking.return_15s().is_some());
        assert!(ranking.return_1m().is_none());
        assert!(ranking.ranking_score() > 0);
    }

    #[test]
    fn observations_are_bounded_to_five_minutes() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(60_000));
        ranking.observe_at(Some(102.0), Some(300_000));

        assert_eq!(ranking.observation_count(), 3);
        assert!((ranking.return_5m().unwrap() - (102.0 / 100.0 - 1.0)).abs() < 1e-9);
    }

    #[test]
    fn out_of_order_observations_are_ignored() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(Some(100.0), Some(0));
        ranking.observe_at(Some(101.0), Some(15_000));
        ranking.observe_at(Some(99.0), Some(7_500));

        assert_eq!(ranking.observation_count(), 2);
        let return_15s = ranking
            .return_15s()
            .expect("fifteen-second history should exist");
        assert!((return_15s - 0.01).abs() < 1e-12);
    }

    #[test]
    fn missing_price_does_not_create_an_observation() {
        let mut ranking = FuturesTickerRanking::default();
        ranking.observe_at(None, Some(1_000));
        assert_eq!(ranking.observation_count(), 0);
    }
}
