use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::trading::{Position, PositionSide, TradingSnapshot};

/// Historical fair-price candle used to reconstruct cross-margin risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalCandle {
    pub time_ms: i64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// A completed funding settlement returned by the exchange history API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingSettlement {
    pub settle_time_ms: i64,
    pub funding_rate: f64,
}

/// Historical reconstruction for one saved paper-trading portfolio.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrossMarginReplay {
    /// Positive means funding paid; negative means funding received.
    pub funding_fee: f64,
    pub funding_settlements: usize,
    pub liquidation_time_ms: Option<i64>,
    pub liquidated: bool,
    pub lowest_equity: f64,
    pub observed_lows: HashMap<String, f64>,
    pub observed_highs: HashMap<String, f64>,
    pub incomplete_symbols: Vec<String>,
}

/// Replays funding settlements and adverse candle extremes against one shared
/// cross-margin account. The maintenance-margin ratio is an explicit simulator
/// assumption because MEXC risk tiers vary by contract and position size.
pub fn replay_cross_margin(
    snapshot: &TradingSnapshot,
    candles: &HashMap<String, Vec<HistoricalCandle>>,
    funding: &HashMap<String, Vec<FundingSettlement>>,
    maintenance_margin_rate: f64,
) -> CrossMarginReplay {
    let mut result = CrossMarginReplay {
        lowest_equity: f64::INFINITY,
        ..CrossMarginReplay::default()
    };

    if snapshot.portfolio.positions.is_empty()
        || !maintenance_margin_rate.is_finite()
        || !(0.0..1.0).contains(&maintenance_margin_rate)
    {
        result.lowest_equity = snapshot.portfolio.cash;
        return result;
    }

    let mut events = Vec::<(i64, bool, String, f64, f64, f64)>::new();
    for position in &snapshot.portfolio.positions {
        let Some(symbol_candles) = candles.get(&position.symbol) else {
            result.incomplete_symbols.push(position.symbol.clone());
            continue;
        };
        for candle in symbol_candles {
            if candle.time_ms >= position.opened_at_ms
                && candle.high.is_finite()
                && candle.low.is_finite()
                && candle.close.is_finite()
                && candle.low > 0.0
                && candle.high >= candle.low
                && candle.close > 0.0
            {
                events.push((
                    candle.time_ms,
                    false,
                    position.symbol.clone(),
                    candle.high,
                    candle.low,
                    candle.close,
                ));
                result
                    .observed_lows
                    .entry(position.symbol.clone())
                    .and_modify(|low| *low = low.min(candle.low))
                    .or_insert(candle.low);
                result
                    .observed_highs
                    .entry(position.symbol.clone())
                    .and_modify(|high| *high = high.max(candle.high))
                    .or_insert(candle.high);
            }
        }

        if let Some(settlements) = funding.get(&position.symbol) {
            for settlement in settlements {
                // The position must already be open at the settlement instant.
                if settlement.settle_time_ms > position.opened_at_ms
                    && settlement.funding_rate.is_finite()
                {
                    let fair_price = symbol_candles
                        .iter()
                        .filter(|candle| candle.time_ms <= settlement.settle_time_ms)
                        .max_by_key(|candle| candle.time_ms)
                        .map(|candle| candle.close)
                        .unwrap_or(position.entry_price);
                    events.push((
                        settlement.settle_time_ms,
                        true,
                        position.symbol.clone(),
                        settlement.funding_rate,
                        fair_price,
                        fair_price,
                    ));
                }
            }
        }
    }

    events.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let mut cash = snapshot.portfolio.cash;
    let mut latest_prices = snapshot
        .portfolio
        .positions
        .iter()
        .map(|position| (position.symbol.clone(), position.entry_price))
        .collect::<HashMap<_, _>>();

    for (timestamp_ms, is_funding, symbol, high, low, close) in events {
        if result.liquidated {
            break;
        }

        if is_funding {
            let Some(position) = snapshot
                .portfolio
                .positions
                .iter()
                .find(|position| position.symbol == symbol)
            else {
                continue;
            };
            let signed_rate = match position.side {
                PositionSide::Long => high,
                PositionSide::Short => -high,
            };
            let fee = position.quantity * low * signed_rate;
            if !fee.is_finite() {
                continue;
            }
            cash -= fee;
            result.funding_fee += fee;
            result.funding_settlements += 1;
        } else {
            latest_prices.insert(symbol, close);

            // Use each open position's adverse extreme for this candle interval.
            // OHLC does not reveal the exact intrabar ordering, so this is a
            // conservative approximation rather than an exchange liquidation record.
            let mut equity = cash;
            let mut maintenance_margin = 0.0;
            for position in &snapshot.portfolio.positions {
                let mark = match latest_prices.get(&position.symbol).copied() {
                    Some(price) => price,
                    None => position.entry_price,
                };
                let adverse_mark = if position.symbol == symbol {
                    match position.side {
                        PositionSide::Long => low,
                        PositionSide::Short => high,
                    }
                } else {
                    mark
                };
                let pnl = match position.side {
                    PositionSide::Long => position.quantity * (adverse_mark - position.entry_price),
                    PositionSide::Short => position.quantity * (position.entry_price - adverse_mark),
                };
                equity += position.margin + pnl;
                maintenance_margin += position.quantity * adverse_mark * maintenance_margin_rate;
            }

            result.lowest_equity = result.lowest_equity.min(equity);
            if equity <= maintenance_margin {
                result.liquidated = true;
                result.liquidation_time_ms = Some(timestamp_ms);
                break;
            }
        }
    }

    if !result.lowest_equity.is_finite() {
        result.lowest_equity = snapshot.portfolio.cash;
    }
    result.incomplete_symbols.sort();
    result.incomplete_symbols.dedup();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::trading::{Portfolio, TradingSettings};

    fn snapshot(side: PositionSide, opened_at_ms: i64) -> TradingSnapshot {
        let settings = TradingSettings {
            initial_capital: 1_000.0,
            fee_rate: 0.001,
            leverage: 10.0,
            trade_allocation_percent: 10.0,
            position_side: side,
        };
        let mut portfolio = Portfolio::new(settings.initial_capital);
        portfolio.open(&settings, "BTC_USDT", 100.0, opened_at_ms).unwrap();
        TradingSnapshot { settings, portfolio }
    }

    #[test]
    fn funding_is_paid_by_long_and_received_by_short_when_rate_is_positive() {
        let candles = HashMap::from([(
            "BTC_USDT".to_string(),
            vec![HistoricalCandle {
                time_ms: 20,
                high: 101.0,
                low: 99.0,
                close: 100.0,
            }],
        )]);
        let funding = HashMap::from([(
            "BTC_USDT".to_string(),
            vec![FundingSettlement {
                settle_time_ms: 20,
                funding_rate: 0.001,
            }],
        )]);

        let long = replay_cross_margin(&snapshot(PositionSide::Long, 10), &candles, &funding, 0.005);
        let short = replay_cross_margin(&snapshot(PositionSide::Short, 10), &candles, &funding, 0.005);

        assert!(long.funding_fee > 0.0);
        assert!(short.funding_fee < 0.0);
        assert_eq!(long.funding_settlements, 1);
        assert_eq!(short.funding_settlements, 1);
    }

    #[test]
    fn does_not_charge_funding_before_position_was_open() {
        let candles = HashMap::from([(
            "BTC_USDT".to_string(),
            vec![HistoricalCandle {
                time_ms: 20,
                high: 101.0,
                low: 99.0,
                close: 100.0,
            }],
        )]);
        let funding = HashMap::from([(
            "BTC_USDT".to_string(),
            vec![FundingSettlement {
                settle_time_ms: 10,
                funding_rate: 0.01,
            }],
        )]);

        let replay = replay_cross_margin(&snapshot(PositionSide::Long, 10), &candles, &funding, 0.005);
        assert_eq!(replay.funding_settlements, 0);
    }

    #[test]
    fn shared_account_can_be_liquidated_by_adverse_candle_extreme() {
        let snapshot = snapshot(PositionSide::Long, 10);
        let candles = HashMap::from([(
            "BTC_USDT".to_string(),
            vec![HistoricalCandle {
                time_ms: 20,
                high: 101.0,
                low: 50.0,
                close: 80.0,
            }],
        )]);

        let replay = replay_cross_margin(&snapshot, &candles, &HashMap::new(), 0.005);
        assert!(replay.liquidated);
        assert_eq!(replay.liquidation_time_ms, Some(20));
    }
}
