use std::collections::HashMap;

use crate::application::ports::TradingStorage;
use crate::domain::trading::{
    summarize_portfolio, Portfolio, PortfolioSummary, TradingSettings, TradingSnapshot,
};

/// Application service for client-side paper trading.
pub struct TradingService;

impl TradingService {
    /// Loads the persisted trading snapshot or creates the default paper portfolio.
    pub fn load(storage: &dyn TradingStorage) -> TradingSnapshot {
        storage
            .load()
            .ok()
            .flatten()
            .filter(|snapshot| {
                snapshot.settings.initial_capital.is_finite()
                    && snapshot.settings.initial_capital > 0.0
                    && snapshot.settings.fee_rate.is_finite()
                    && (0.0..=1.0).contains(&snapshot.settings.fee_rate)
                    && snapshot.portfolio.cash.is_finite()
                    && snapshot.portfolio.cash >= 0.0
            })
            .unwrap_or_default()
    }

    /// Persists the current trading snapshot.
    pub fn save(storage: &dyn TradingStorage, snapshot: &TradingSnapshot) -> Result<(), String> {
        storage.save(snapshot)
    }

    /// Resets the portfolio using new paper-trading settings.
    pub fn reset_with_settings(
        initial_capital: f64,
        fee_rate: f64,
        leverage: f64,
    ) -> Result<TradingSnapshot, String> {
        if !initial_capital.is_finite() || initial_capital <= 0.0 {
            return Err("Initial capital must be greater than zero.".to_string());
        }
        if !fee_rate.is_finite() || !(0.0..=1.0).contains(&fee_rate) {
            return Err("Trading fee must be between 0% and 100%.".to_string());
        }
        if !leverage.is_finite() || !(1.0..=125.0).contains(&leverage) {
            return Err("Leverage must be between 1x and 125x.".to_string());
        }

        let settings = TradingSettings {
            initial_capital,
            fee_rate,
            leverage,
        };
        Ok(TradingSnapshot {
            portfolio: Portfolio::new(initial_capital),
            settings,
        })
    }

    /// Executes a paper buy at the supplied market price.
    pub fn buy(
        snapshot: &TradingSnapshot,
        symbol: &str,
        price: f64,
        timestamp_ms: i64,
    ) -> Result<TradingSnapshot, String> {
        let mut next = snapshot.clone();
        next.portfolio
            .buy(&next.settings, symbol, price, timestamp_ms)?;
        Ok(next)
    }

    /// Executes a paper sell for the complete open position.
    pub fn sell(
        snapshot: &TradingSnapshot,
        symbol: &str,
        price: f64,
        timestamp_ms: i64,
    ) -> Result<TradingSnapshot, String> {
        let mut next = snapshot.clone();
        next.portfolio
            .sell(&next.settings, symbol, price, timestamp_ms)?;
        Ok(next)
    }

    /// Calculates portfolio metrics from the latest market prices.
    pub fn summarize(
        snapshot: &TradingSnapshot,
        prices: &HashMap<String, f64>,
    ) -> PortfolioSummary {
        summarize_portfolio(snapshot, prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::TradingStorage;

    #[derive(Default)]
    struct MemoryStorage(Option<TradingSnapshot>);

    impl TradingStorage for MemoryStorage {
        fn load(&self) -> Result<Option<TradingSnapshot>, String> {
            Ok(self.0.clone())
        }

        fn save(&self, _snapshot: &TradingSnapshot) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn load_uses_default_when_storage_is_empty() {
        let storage = MemoryStorage::default();
        let snapshot = TradingService::load(&storage);

        assert_eq!(snapshot.settings.initial_capital, 1_000.0);
        assert_eq!(snapshot.portfolio.cash, 1_000.0);
    }

    #[test]
    fn reset_with_settings_recreates_empty_portfolio() {
        let snapshot = TradingService::reset_with_settings(2_000.0, 0.002, 5.0).unwrap();

        assert_eq!(snapshot.settings.initial_capital, 2_000.0);
        assert_eq!(snapshot.settings.fee_rate, 0.002);
        assert_eq!(snapshot.settings.leverage, 5.0);
        assert!(snapshot.portfolio.positions.is_empty());
        assert_eq!(snapshot.portfolio.cash, 2_000.0);
    }
}
