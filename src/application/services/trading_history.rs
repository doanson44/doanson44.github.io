use crate::domain::trading::TradingSnapshot;
use crate::domain::trading_history::{replay_cross_margin, CrossMarginReplay, TradingMarketHistory};

/// Default simulator maintenance-margin assumption (0.5% of marked notional).
///
/// MEXC uses symbol- and risk-tier-specific maintenance margins. This default is
/// intentionally exposed as an assumption, not represented as an exchange rule.
pub const DEFAULT_SIMULATED_MAINTENANCE_MARGIN_RATE: f64 = 0.005;

/// Application service for replaying a paper-trading account against MEXC history.
pub struct TradingHistoryService;

impl TradingHistoryService {
    /// Reconstructs account-wide cross-margin risk and funding from historical data.
    pub fn replay(snapshot: &TradingSnapshot, history: &TradingMarketHistory) -> CrossMarginReplay {
        replay_cross_margin(
            snapshot,
            &history.candles,
            &history.funding,
            DEFAULT_SIMULATED_MAINTENANCE_MARGIN_RATE,
        )
    }
}
