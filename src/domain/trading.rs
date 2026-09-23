use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Default paper-trading starting capital in quote currency.
pub const DEFAULT_INITIAL_CAPITAL: f64 = 1_000.0;

/// Default paper-trading fee rate, expressed as a decimal fraction.
pub const DEFAULT_FEE_RATE: f64 = 0.001;

/// Default quote-currency allocation for each new paper trade.
pub const DEFAULT_LEVERAGE: f64 = 1.0;
pub const DEFAULT_TRADE_ALLOCATION_PERCENT: f64 = 10.0;
pub const MAX_LEVERAGE: f64 = 125.0;

/// Configured direction for new paper-trading positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PositionSide {
    #[default]
    Long,
    Short,
}

/// Paper-trading configuration that is safe to persist locally.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingSettings {
    pub initial_capital: f64,
    pub fee_rate: f64,
    pub leverage: f64,
    pub trade_allocation_percent: f64,
    #[serde(default)]
    pub position_side: PositionSide,
}

impl Default for TradingSettings {
    fn default() -> Self {
        Self {
            initial_capital: DEFAULT_INITIAL_CAPITAL,
            fee_rate: DEFAULT_FEE_RATE,
            leverage: DEFAULT_LEVERAGE,
            trade_allocation_percent: DEFAULT_TRADE_ALLOCATION_PERCENT,
            position_side: PositionSide::Long,
        }
    }
}

/// A currently open paper-trading position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub entry_value: f64,
    pub entry_fee: f64,
    pub margin: f64,
    pub leverage: f64,
    #[serde(default)]
    pub side: PositionSide,
}

/// The side of a paper-trading transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

/// A completed paper-trading transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub symbol: String,
    pub side: TradeSide,
    pub price: f64,
    pub quantity: f64,
    pub value: f64,
    pub fee: f64,
    pub realized_pnl: f64,
    pub timestamp_ms: i64,
}

/// Client-side paper-trading portfolio state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: Vec<Position>,
    pub transactions: Vec<Transaction>,
    pub realized_pnl: f64,
}

impl Portfolio {
    /// Creates an empty portfolio funded with the supplied initial capital.
    pub fn new(initial_capital: f64) -> Self {
        Self {
            cash: initial_capital,
            positions: Vec::new(),
            transactions: Vec::new(),
            realized_pnl: 0.0,
        }
    }

    /// Buys a position using the default allocation of available cash.
    pub fn buy(
        &mut self,
        settings: &TradingSettings,
        symbol: &str,
        price: f64,
        timestamp_ms: i64,
    ) -> Result<(), String> {
        validate_trade_inputs(settings, symbol, price)?;

        if self
            .positions
            .iter()
            .any(|position| position.symbol == symbol)
        {
            return Err(format!("{symbol} is already held."));
        }

        let allocation_rate = settings.trade_allocation_percent / 100.0;
        let allocated_cash = self.cash * allocation_rate;
        let leverage = settings.leverage;
        let margin = allocated_cash / (1.0 + leverage * settings.fee_rate);
        let notional = margin * leverage;
        let fee = notional * settings.fee_rate;
        let total_cost = margin + fee;

        if notional <= 0.0 || total_cost > self.cash {
            return Err("Insufficient cash for the paper trade.".to_string());
        }

        let quantity = notional / price;
        self.cash -= total_cost;
        self.positions.push(Position {
            symbol: symbol.to_string(),
            quantity,
            entry_price: price,
            entry_value: notional,
            entry_fee: fee,
            margin,
            leverage,
            side: settings.position_side,
        });
        self.transactions.push(Transaction {
            symbol: symbol.to_string(),
            side: TradeSide::Buy,
            price,
            quantity,
            value: notional,
            fee,
            realized_pnl: 0.0,
            timestamp_ms,
        });

        Ok(())
    }

    /// Sells the complete open position for a symbol at the supplied market price.
    pub fn sell(
        &mut self,
        settings: &TradingSettings,
        symbol: &str,
        price: f64,
        timestamp_ms: i64,
    ) -> Result<(), String> {
        validate_trade_inputs(settings, symbol, price)?;

        let Some(index) = self
            .positions
            .iter()
            .position(|position| position.symbol == symbol)
        else {
            return Err(format!("{symbol} is not currently held."));
        };

        let position = self.positions.remove(index);
        let value = position.quantity * price;
        let fee = value * settings.fee_rate;
        let price_pnl = match position.side {
            PositionSide::Long => value - position.entry_value,
            PositionSide::Short => position.entry_value - value,
        };
        let realized_pnl = price_pnl - position.entry_fee - fee;

        self.cash += position.margin + price_pnl - fee;
        self.realized_pnl += realized_pnl;
        self.transactions.push(Transaction {
            symbol: symbol.to_string(),
            side: TradeSide::Sell,
            price,
            quantity: position.quantity,
            value,
            fee,
            realized_pnl,
            timestamp_ms,
        });

        Ok(())
    }
}

/// A holding enriched with the latest market valuation.
#[derive(Debug, Clone, PartialEq)]
pub struct HoldingSummary {
    pub symbol: String,
    pub side: PositionSide,
    pub quantity: f64,
    pub market_value: f64,
    pub pnl: f64,
}

/// Current portfolio metrics derived from live market prices.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PortfolioSummary {
    pub cash: f64,
    pub equity: f64,
    pub total_pnl: f64,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub holdings: Vec<HoldingSummary>,
}

/// Persisted paper-trading state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingSnapshot {
    pub settings: TradingSettings,
    pub portfolio: Portfolio,
}

impl Default for TradingSnapshot {
    fn default() -> Self {
        let settings = TradingSettings::default();
        Self {
            portfolio: Portfolio::new(settings.initial_capital),
            settings,
        }
    }
}

/// Calculates current portfolio metrics from live prices.
pub fn summarize_portfolio(
    snapshot: &TradingSnapshot,
    prices: &HashMap<String, f64>,
) -> PortfolioSummary {
    let mut holdings = Vec::with_capacity(snapshot.portfolio.positions.len());
    let mut equity = snapshot.portfolio.cash;
    let mut unrealized_pnl = 0.0;

    for position in &snapshot.portfolio.positions {
        let current_price = prices
            .get(&position.symbol)
            .copied()
            .unwrap_or(position.entry_price);
        let value = position.quantity * current_price;
        let price_pnl = match position.side {
            PositionSide::Long => value - position.entry_value,
            PositionSide::Short => position.entry_value - value,
        };
        let pnl = price_pnl - position.entry_fee;

        equity += position.margin + price_pnl;
        unrealized_pnl += pnl;
        holdings.push(HoldingSummary {
            symbol: position.symbol.clone(),
            side: position.side,
            quantity: position.quantity,
            market_value: value,
            pnl,
        });
    }

    let total_pnl = snapshot.portfolio.realized_pnl + unrealized_pnl;
    PortfolioSummary {
        cash: snapshot.portfolio.cash,
        equity,
        total_pnl,
        realized_pnl: snapshot.portfolio.realized_pnl,
        unrealized_pnl,
        holdings,
    }
}

fn validate_trade_inputs(
    settings: &TradingSettings,
    symbol: &str,
    price: f64,
) -> Result<(), String> {
    if symbol.trim().is_empty() {
        return Err("A market symbol is required.".to_string());
    }
    if !price.is_finite() || price <= 0.0 {
        return Err("A valid positive market price is required.".to_string());
    }
    if !settings.initial_capital.is_finite() || settings.initial_capital <= 0.0 {
        return Err("Initial capital must be greater than zero.".to_string());
    }
    if !settings.fee_rate.is_finite() || !(0.0..=1.0).contains(&settings.fee_rate) {
        return Err("Trading fee must be between 0% and 100%.".to_string());
    }
    if !settings.leverage.is_finite() || !(1.0..=MAX_LEVERAGE).contains(&settings.leverage) {
        return Err(format!(
            "Leverage must be between 1x and {MAX_LEVERAGE:.0}x."
        ));
    }
    if !settings.trade_allocation_percent.is_finite()
        || !(0.1..=100.0).contains(&settings.trade_allocation_percent)
    {
        return Err("Trade allocation must be between 0.1% and 100%.".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> TradingSettings {
        TradingSettings {
            initial_capital: 1_000.0,
            fee_rate: 0.001,
            leverage: 1.0,
            trade_allocation_percent: 10.0,
            position_side: PositionSide::Long,
        }
    }

    #[test]
    fn buy_uses_ten_percent_of_available_cash() {
        let settings = settings();
        let mut portfolio = Portfolio::new(settings.initial_capital);

        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();

        assert_eq!(portfolio.positions.len(), 1);
        assert!((portfolio.positions[0].quantity - 0.999000999000999).abs() < 1e-9);
        assert!((portfolio.cash - 900.0).abs() < 1e-9);
    }

    #[test]
    fn sell_closes_position_and_realizes_pnl() {
        let settings = settings();
        let mut portfolio = Portfolio::new(settings.initial_capital);

        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();
        portfolio.sell(&settings, "BTC_USDT", 110.0, 2).unwrap();

        assert!(portfolio.positions.is_empty());
        assert!((portfolio.cash - 1009.7802197802198).abs() < 1e-9);
        assert!((portfolio.realized_pnl - 9.7802197802198).abs() < 1e-9);
    }

    #[test]
    fn short_position_profits_when_price_falls() {
        let mut settings = settings();
        settings.position_side = PositionSide::Short;
        let mut portfolio = Portfolio::new(settings.initial_capital);

        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();
        portfolio.sell(&settings, "BTC_USDT", 90.0, 2).unwrap();

        assert!(portfolio.positions.is_empty());
        assert!((portfolio.realized_pnl - 9.8001998001998).abs() < 1e-9);
    }

    #[test]
    fn summary_accounts_for_leverage_without_counting_borrowed_notional_as_equity() {
        let mut settings = settings();
        settings.leverage = 10.0;
        let mut portfolio = Portfolio::new(settings.initial_capital);
        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();

        let snapshot = TradingSnapshot {
            settings,
            portfolio,
        };
        let prices = HashMap::from([("BTC_USDT".to_string(), 110.0)]);
        let summary = summarize_portfolio(&snapshot, &prices);

        assert!((summary.cash - 900.0).abs() < 1e-9);
        assert!((summary.equity - 1098.019801980198).abs() < 1e-9);
        assert!((summary.unrealized_pnl - 98.019801980198).abs() < 1e-9);
        assert!((summary.total_pnl - 98.019801980198).abs() < 1e-9);
    }

    #[test]
    fn duplicate_buy_is_rejected() {
        let settings = settings();
        let mut portfolio = Portfolio::new(settings.initial_capital);

        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();

        assert!(portfolio.buy(&settings, "BTC_USDT", 101.0, 2).is_err());
    }

    #[test]
    fn summary_uses_live_market_price() {
        let settings = settings();
        let mut portfolio = Portfolio::new(settings.initial_capital);
        portfolio.buy(&settings, "BTC_USDT", 100.0, 1).unwrap();

        let snapshot = TradingSnapshot {
            settings,
            portfolio,
        };
        let prices = HashMap::from([("BTC_USDT".to_string(), 110.0)]);
        let summary = summarize_portfolio(&snapshot, &prices);

        assert!((summary.equity - 1009.8901098901099).abs() < 1e-9);
        assert!((summary.total_pnl - 9.89010989010989).abs() < 1e-9);
        assert_eq!(summary.holdings.len(), 1);
    }
}
