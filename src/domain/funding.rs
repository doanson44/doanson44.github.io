use std::collections::HashMap;

/// Funding rate snapshot keyed by futures contract symbol.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FundingRateSnapshot {
    pub rates: HashMap<String, f64>,
}

impl FundingRateSnapshot {
    /// Creates a snapshot from funding rate entries.
    pub fn new(rates: HashMap<String, f64>) -> Self {
        Self { rates }
    }

    /// Returns the funding rate for a contract when available.
    pub fn get(&self, symbol: &str) -> Option<f64> {
        self.rates.get(symbol).copied()
    }
}
