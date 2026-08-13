use crate::domain::futures::{FuturesTicker, FuturesTickerRegistry, FuturesTickerUpdate};

/// Application service for maintaining the public Futures market state.
pub struct FuturesMarketService {
    registry: FuturesTickerRegistry,
}

impl FuturesMarketService {
    /// Creates a new empty Futures market service.
    pub fn new() -> Self {
        Self {
            registry: FuturesTickerRegistry::new(),
        }
    }

    /// Applies a complete public ticker batch and returns the latest snapshot.
    pub fn apply_ticker_batch(&mut self, updates: Vec<FuturesTickerUpdate>) -> Vec<FuturesTicker> {
        self.registry.apply_batch(updates)
    }
}

impl Default for FuturesMarketService {
    fn default() -> Self {
        Self::new()
    }
}
