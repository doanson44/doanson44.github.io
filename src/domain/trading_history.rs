use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::domain::trading::{PositionSide, TradingSnapshot};

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

/// Funding total for one currently open position.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FundingPositionSummary {
    /// Positive means funding paid; negative means funding received.
    pub fee: f64,
    pub settlements: usize,
}

/// Historical reconstruction for one saved paper-trading portfolio.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrossMarginReplay {
    /// Positive means funding paid; negative means funding received.
    pub funding_fee: f64,
    pub funding_settlements: usize,
    pub funding_by_symbol: HashMap<String, FundingPositionSummary>,
    pub liquidation_time_ms: Option<i64>,
    pub liquidated: bool,
    pub lowest_equity: f64,
    pub observed_lows: HashMap<String, f64>,
    pub observed_highs: HashMap<String, f64>,
    pub incomplete_symbols: Vec<String>,
}

/// Collected market history required to replay one cross-margin portfolio.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TradingMarketHistory {
    pub candles: HashMap<String, Vec<HistoricalCandle>>,
    pub funding: HashMap<String, Vec<FundingSettlement>>,
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
                        .filter(|candle| {
                            candle.time_ms >= position.opened_at_ms
                                && candle.time_ms < settlement.settle_time_ms
                        })
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

    // Apply funding before checking the same timestamp's price extremes.
    events.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| right.1.cmp(&left.1)));

    // Restore pre-replay cash, then release each position's reserved margin and
    // entry fee back into the timeline at its actual opening timestamp.
    let reserved_balance = snapshot.portfolio.positions.iter()
        .map(|position| position.margin + position.entry_fee)
        .sum::<f64>();
    let mut cash = snapshot.portfolio.cash
        + snapshot.portfolio.historical_funding_fee
        + reserved_balance;
    let mut active_symbols = HashSet::new();
    let mut latest_prices = snapshot
        .portfolio
        .positions
        .iter()
        .map(|position| (position.symbol.clone(), position.entry_price))
        .collect::<HashMap<_, _>>();

    let mut index = 0;
    while index < events.len() {
        let timestamp_ms = events[index].0;
        let group_start = index;
        while index < events.len() && events[index].0 == timestamp_ms {
            index += 1;
        }
        let group = &events[group_start..index];

        // Positions become part of the shared account only from their own opening time.
        for position in &snapshot.portfolio.positions {
            if position.opened_at_ms <= timestamp_ms
                && active_symbols.insert(position.symbol.clone())
            {
                cash -= position.margin + position.entry_fee;
            }
        }

        // Apply all funding settlements first, before checking price risk at the same timestamp.
        for (_, _, symbol, rate, fair_price, _) in group.iter().filter(|event| event.1) {
            if !active_symbols.contains(symbol.as_str()) {
                continue;
            }
            let Some(position) = snapshot
                .portfolio
                .positions
                .iter()
                .find(|position| position.symbol.as_str() == symbol.as_str())
            else {
                continue;
            };
            let signed_rate = match position.side {
                PositionSide::Long => *rate,
                PositionSide::Short => -*rate,
            };
            let fee = position.quantity * *fair_price * signed_rate;
            if fee.is_finite() {
                cash -= fee;
                result.funding_fee += fee;
                result.funding_settlements += 1;
                let position_summary = result
                    .funding_by_symbol
                    .entry(symbol.to_string())
                    .or_default();
                position_summary.fee += fee;
                position_summary.settlements += 1;
            }
        }

        let candle_events = group.iter().filter(|event| !event.1).collect::<Vec<_>>();
        for event in &candle_events {
            latest_prices.insert(event.2.to_string(), event.5);
        }

        // Evaluate the shared account against all positions' adverse extremes
        // from the same candle interval, not one symbol at a time.
        if active_symbols.is_empty() {
            continue;
        }

        let mut equity = cash;
        let mut maintenance_margin = 0.0;
        for position in &snapshot.portfolio.positions {
            if !active_symbols.contains(position.symbol.as_str()) {
                continue;
            }
            let candle = candle_events
                .iter()
                .find(|event| event.2.as_str() == position.symbol.as_str());
            let mark = match candle {
                Some(event) => match position.side {
                    PositionSide::Long => event.4,
                    PositionSide::Short => event.3,
                },
                None => latest_prices
                    .get(&position.symbol)
                    .copied()
                    .unwrap_or(position.entry_price),
            };
            let pnl = match position.side {
                PositionSide::Long => position.quantity * (mark - position.entry_price),
                PositionSide::Short => position.quantity * (position.entry_price - mark),
            };
            equity += position.margin + pnl;
            maintenance_margin += position.quantity * mark * maintenance_margin_rate;
        }

        result.lowest_equity = result.lowest_equity.min(equity);
        if equity <= maintenance_margin {
            result.liquidated = true;
            result.liquidation_time_ms = Some(timestamp_ms);
            break;
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
            trade_allocation_percent: 100.0,
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
    fn replaying_the_same_funding_history_does_not_double_charge_after_persistence() {
        let mut snapshot = snapshot(PositionSide::Long, 10);
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

        let first = replay_cross_margin(&snapshot, &candles, &funding, 0.005);
        snapshot.portfolio.cash -= first.funding_fee;
        snapshot.portfolio.historical_funding_fee = first.funding_fee;
        snapshot.portfolio.historical_funding_settlements = first.funding_settlements;
        snapshot.portfolio.positions[0].historical_funding_fee = first.funding_fee;
        snapshot.portfolio.positions[0].historical_funding_settlements = first.funding_settlements;

        let second = replay_cross_margin(&snapshot, &candles, &funding, 0.005);
        assert!((second.funding_fee - first.funding_fee).abs() < 1e-9);
        assert_eq!(second.funding_settlements, first.funding_settlements);
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
