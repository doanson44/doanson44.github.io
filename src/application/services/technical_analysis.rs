use crate::domain::technical_analysis::{
    analyze, AdxConfig, AnalysisConfig, AnalysisInput, AnalysisResult, Asset, AssetType, AtrConfig,
    BollingerConfig, BreakoutConfig, Candle, CandleMetadata, DataQualityConfig, DataRequirements,
    DivergenceDetectionConfig, EngineConfig, IndicatorConfig, MacdConfig, MarketData,
    MarketStructureConfig, MomentumConfig, MovingAverageConfig, PatternDetectionConfig,
    PriceActionConfig, RegimeDetectionConfig, RetestConfig, RsiConfig, ScenarioEngineConfig,
    SignalEngineConfig, StochasticConfig, SupportResistanceConfig, SwingDetectionConfig,
    TrendStrengthConfig, VolatilityConfig, VolumeConfig, VolumeConfirmationConfig,
};

/// Application service for stock and crypto technical analysis.
pub struct TechnicalAnalysisService;

impl TechnicalAnalysisService {
    /// Builds a stock daily analysis input from CafeF historical price data.
    pub fn price_history_input(raw: &str, symbol: &str) -> Result<AnalysisInput, String> {
        let history = crate::domain::market::parse_price_history_response(raw, symbol)?;
        let candles = history
            .candles
            .into_iter()
            .map(|candle| Candle {
                timestamp: candle.trade_date,
                open: candle.open,
                high: candle.high,
                low: candle.low,
                close: candle.close,
                volume: candle.volume,
                metadata: CandleMetadata {
                    reference_price: Some(candle.basic_price),
                    ceiling: candle.ceiling,
                    floor: candle.floor,
                    total_value: candle.total_value,
                },
            })
            .collect();

        Ok(AnalysisInput {
            schema_version: "1.0".to_string(),
            asset: Asset {
                symbol: symbol.trim().to_ascii_uppercase(),
                asset_type: AssetType::Stock,
                exchange: "HOSE".to_string(),
                currency: "VND".to_string(),
            },
            market_data: MarketData {
                timeframe: "1D".to_string(),
                timezone: "Asia/Ho_Chi_Minh".to_string(),
                candles,
            },
        })
    }

    /// Returns the default daily stock-analysis configuration used by the Market page.
    pub fn default_stock_daily_config() -> AnalysisConfig {
        AnalysisConfig {
            schema_version: "1.0".to_string(),
            engine: EngineConfig {
                name: "technical-analysis-engine".to_string(),
                version: "1.0.0".to_string(),
            },
            data_requirements: DataRequirements {
                minimum_candles: 200,
                recommended_candles: 500,
                maximum_candles: 2000,
            },
            indicators: IndicatorConfig {
                moving_averages: MovingAverageConfig {
                    sma: vec![20, 50, 100, 200],
                    ema: vec![9, 20, 50, 200],
                },
                momentum: MomentumConfig {
                    rsi: RsiConfig { period: 14 },
                    macd: MacdConfig {
                        fast: 12,
                        slow: 26,
                        signal: 9,
                    },
                    stochastic: StochasticConfig {
                        k_period: 14,
                        d_period: 3,
                        smooth: 3,
                    },
                },
                volatility: VolatilityConfig {
                    atr: AtrConfig { period: 14 },
                    bollinger_bands: BollingerConfig {
                        period: 20,
                        stddev: 2.0,
                    },
                },
                trend_strength: TrendStrengthConfig {
                    adx: AdxConfig { period: 14 },
                },
                volume: VolumeConfig {
                    obv: true,
                    volume_average: vec![20, 50],
                },
            },
            price_action: PriceActionConfig {
                enabled: true,
                candlestick_analysis: true,
                gap_detection: true,
                consecutive_move_detection: true,
            },
            market_structure: MarketStructureConfig {
                enabled: true,
                swing_detection: SwingDetectionConfig { lookback: 5 },
                support_resistance: SupportResistanceConfig {
                    lookback: 120,
                    cluster_tolerance_percent: 1.0,
                    minimum_touches: 2,
                },
            },
            breakout_detection: BreakoutConfig {
                enabled: true,
                lookback_period: 20,
                volume_confirmation: VolumeConfirmationConfig {
                    enabled: true,
                    minimum_volume_ratio: 1.5,
                },
                retest_detection: RetestConfig { enabled: true },
            },
            pattern_detection: PatternDetectionConfig {
                enabled: true,
                candlestick_patterns: true,
                chart_patterns: vec![
                    "double_top".to_string(),
                    "double_bottom".to_string(),
                    "head_and_shoulders".to_string(),
                    "inverse_head_and_shoulders".to_string(),
                    "ascending_triangle".to_string(),
                    "descending_triangle".to_string(),
                    "symmetrical_triangle".to_string(),
                    "flag".to_string(),
                    "pennant".to_string(),
                    "cup_and_handle".to_string(),
                ],
            },
            divergence_detection: DivergenceDetectionConfig {
                enabled: true,
                indicators: vec!["rsi".to_string(), "macd".to_string(), "obv".to_string()],
                minimum_swing_distance: 5,
            },
            regime_detection: RegimeDetectionConfig {
                enabled: true,
                dimensions: vec![
                    "trend".to_string(),
                    "momentum".to_string(),
                    "volatility".to_string(),
                    "volume".to_string(),
                ],
            },
            scenario_engine: ScenarioEngineConfig {
                enabled: true,
                scenarios: vec![
                    "bullish".to_string(),
                    "bearish".to_string(),
                    "range".to_string(),
                ],
            },
            signal_engine: SignalEngineConfig {
                enabled: true,
                signal_strength_levels: vec![
                    "weak".to_string(),
                    "moderate".to_string(),
                    "strong".to_string(),
                ],
            },
            data_quality: DataQualityConfig {
                validate_ohlcv: true,
                detect_missing_candles: true,
                detect_duplicate_candles: true,
                detect_invalid_prices: true,
                detect_zero_volume: true,
            },
        }
    }

    /// Analyzes CafeF historical price data with the default stock configuration.
    pub fn analyze_price_history(
        raw: &str,
        symbol: &str,
        analysis_timestamp: impl Into<String>,
    ) -> Result<String, String> {
        let input = Self::price_history_input(raw, symbol)?;
        let config = Self::default_stock_daily_config();
        let result = Self::analyze(&input, &config, analysis_timestamp)?;
        serde_json::to_string_pretty(&result)
            .map_err(|error| format!("Failed to serialize analysis result: {error}"))
    }

    /// Analyzes CafeF history and returns both machine-readable JSON and a human-readable report.
    pub fn analyze_price_history_report(
        raw: &str,
        symbol: &str,
        analysis_timestamp: impl Into<String>,
    ) -> Result<(String, String), String> {
        let input = Self::price_history_input(raw, symbol)?;
        let config = Self::default_stock_daily_config();
        let result = Self::analyze(&input, &config, analysis_timestamp)?;
        let json = serde_json::to_string_pretty(&result)
            .map_err(|error| format!("Failed to serialize analysis result: {error}"))?;
        let report = format_analysis_report(&result);
        Ok((json, report))
    }

    /// Analyzes typed market data with the supplied configuration.
    pub fn analyze(
        input: &AnalysisInput,
        config: &AnalysisConfig,
        analysis_timestamp: impl Into<String>,
    ) -> Result<AnalysisResult, String> {
        analyze(input, config, analysis_timestamp.into())
    }

    /// Parses JSON input and configuration, then runs the analysis pipeline.
    ///
    /// # Errors
    /// Returns a human-readable error when either JSON document is invalid or
    /// the market data does not satisfy the configured requirements.
    pub fn analyze_json(
        input_json: &str,
        config_json: &str,
        analysis_timestamp: impl Into<String>,
    ) -> Result<String, String> {
        let input: AnalysisInput = serde_json::from_str(input_json)
            .map_err(|error| format!("Invalid analysis input JSON: {error}"))?;
        let config: AnalysisConfig = serde_json::from_str(config_json)
            .map_err(|error| format!("Invalid analysis config JSON: {error}"))?;
        let result = Self::analyze(&input, &config, analysis_timestamp)?;
        serde_json::to_string_pretty(&result)
            .map_err(|error| format!("Failed to serialize analysis result: {error}"))
    }

}

fn format_optional(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:.2}"))
        .unwrap_or_else(|| "N/A".to_string())
}

fn format_analysis_report(result: &AnalysisResult) -> String {
    let mut report = String::new();
    report.push_str(&format!(
        "{} ({}) — {}\\n",
        result.asset.symbol, result.asset.asset_type.as_str(), result.asset.timeframe
    ));
    report.push_str(&format!(
        "Analysis: {} {}\\n\\n",
        result.engine.name, result.engine.version
    ));

    report.push_str("SUMMARY\\n");
    report.push_str(&format!("• State: {}\\n", result.engine_summary.dominant_state));
    report.push_str(&format!("• Trend: {}\\n", result.engine_summary.trend));
    report.push_str(&format!("• Momentum: {}\\n", result.engine_summary.momentum));
    report.push_str(&format!("• Structure: {}\\n", result.engine_summary.structure));
    report.push_str(&format!(
        "• Volume confirmation: {}\\n",
        if result.engine_summary.volume_confirmation { "yes" } else { "no" }
    ));
    report.push_str(&format!("• Main risk: {}\\n\\n", result.engine_summary.main_risk));

    report.push_str("CURRENT PRICE\\n");
    report.push_str(&format!(
        "• Close: {} {}\\n• Change: {:.2} ({:.2}%)\\n• Volume: {:.0}\\n\\n",
        result.snapshot.close,
        result.asset.currency,
        result.snapshot.price_change.absolute,
        result.snapshot.price_change.percent,
        result.snapshot.volume
    ));

    report.push_str("TREND\\n");
    report.push_str(&format!(
        "• State: {}\\n• Strength: {}\\n• Alignment: {}\\n",
        result.trend.state, result.trend.strength, result.trend.alignment.description
    ));
    report.push_str(&format!(
        "• SMA: {}\\n",
        result
            .trend
            .moving_averages
            .sma
            .iter()
            .map(|item| format!("{}={}", item.period, format_optional(item.value)))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    report.push_str(&format!(
        "• EMA: {}\\n\\n",
        result
            .trend
            .moving_averages
            .ema
            .iter()
            .map(|item| format!("{}={}", item.period, format_optional(item.value)))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    report.push_str("MOMENTUM\\n");
    report.push_str(&format!(
        "• RSI: {} ({})\\n• MACD: {} / signal {} / histogram {} ({})\\n• Stochastic: K {} / D {} ({})\\n\\n",
        format_optional(result.momentum.rsi.value),
        result.momentum.rsi.state,
        format_optional(result.momentum.macd.macd),
        format_optional(result.momentum.macd.signal),
        format_optional(result.momentum.macd.histogram),
        result.momentum.macd.state,
        format_optional(result.momentum.stochastic.k),
        format_optional(result.momentum.stochastic.d),
        result.momentum.stochastic.state
    ));

    report.push_str("VOLATILITY & VOLUME\\n");
    report.push_str(&format!(
        "• ATR: {} ({})\\n• Bollinger: lower {} / middle {} / upper {}\\n• Volume: {} ({})\\n• OBV: {}\\n\\n",
        format_optional(result.volatility.atr),
        result.volatility.state,
        format_optional(result.volatility.bollinger_bands.lower),
        format_optional(result.volatility.bollinger_bands.middle),
        format_optional(result.volatility.bollinger_bands.upper),
        result.volume.current,
        result.volume.state,
        format_optional(result.volume.obv)
    ));

    report.push_str("MARKET STRUCTURE\\n");
    report.push_str(&format!(
        "• State: {}\\n• Sequence: {}\\n• Last swing high: {}\\n• Last swing low: {}\\n\\n",
        result.market_structure.state,
        if result.market_structure.structure_sequence.is_empty() {
            "N/A".to_string()
        } else {
            result.market_structure.structure_sequence.join(" → ")
        },
        result
            .market_structure
            .swing_points
            .last_swing_high
            .as_ref()
            .map(|point| format!("{:.2} ({})", point.price, point.date))
            .unwrap_or_else(|| "N/A".to_string()),
        result
            .market_structure
            .swing_points
            .last_swing_low
            .as_ref()
            .map(|point| format!("{:.2} ({})", point.price, point.date))
            .unwrap_or_else(|| "N/A".to_string())
    ));

    report.push_str("SUPPORT / RESISTANCE\\n");
    report.push_str(&format!(
        "• Immediate support: {}\\n• Major support: {}\\n• Immediate resistance: {}\\n\\n",
        format_optional(result.key_levels.immediate_support),
        format_optional(result.key_levels.major_support),
        format_optional(result.key_levels.immediate_resistance)
    ));

    report.push_str("BREAKOUT\\n");
    report.push_str(&format!(
        "• Status: {}\\n• Level: {}\\n• Direction: {}\\n• Volume confirmation: {}\\n\\n",
        result.breakout.status,
        format_optional(result.breakout.resistance_level),
        result.breakout.direction.as_deref().unwrap_or("N/A"),
        if result.breakout.volume_confirmation { "yes" } else { "no" }
    ));

    report.push_str("REGIME\\n");
    report.push_str(&format!(
        "• Overall: {}\\n• Trend: {}\\n• Momentum: {}\\n• Volatility: {}\\n• Volume: {}\\n\\n",
        result.regime.overall,
        result.regime.trend,
        result.regime.momentum,
        result.regime.volatility,
        result.regime.volume
    ));

    report.push_str("SIGNALS\\n");
    if result.signals.is_empty() {
        report.push_str("• None\\n");
    } else {
        for signal in &result.signals {
            report.push_str(&format!(
                "• {} — {} / {}\\n  Evidence: {}\\n",
                signal.direction,
                signal.category,
                signal.strength,
                signal.evidence.join(", ")
            ));
        }
    }
    report.push('\\n');

    report.push_str("PATTERNS & DIVERGENCES\\n");
    if result.patterns.is_empty() {
        report.push_str("• Patterns: none\\n");
    } else {
        for pattern in &result.patterns {
            report.push_str(&format!(
                "• Pattern: {} ({}, confidence {:.0}%)\\n",
                pattern.name,
                pattern.status,
                pattern.confidence * 100.0
            ));
        }
    }
    if result.divergences.is_empty() {
        report.push_str("• Divergences: none\\n");
    } else {
        for divergence in &result.divergences {
            report.push_str(&format!(
                "• Divergence: {} {} (confidence {:.0}%)\\n",
                divergence.indicator,
                divergence.direction,
                divergence.confidence * 100.0
            ));
        }
    }
    report.push('\\n');

    report.push_str("SCENARIOS\\n");
    for (name, scenario) in [
        ("Bullish", &result.scenarios.bullish),
        ("Bearish", &result.scenarios.bearish),
    ] {
        report.push_str(&format!("• {name}: {}\\n", scenario.status));
        if let Some(trigger) = &scenario.trigger {
            report.push_str(&format!("  Trigger: {}\\n", trigger.condition));
        }
        if let Some(invalidation) = &scenario.invalidation {
            report.push_str(&format!("  Invalidation: {}\\n", invalidation.condition));
        }
    }
    report.push_str(&format!(
        "• Range: {} — {}\\n\\n",
        result.scenarios.range.status, result.scenarios.range.condition
    ));

    report.push_str("DATA QUALITY\\n");
    report.push_str(&format!(
        "• Candles: {} used / {} received\\n• Minimum required: {}\\n• Sufficient: {}\\n",
        result.data_quality.candles_used,
        result.data_quality.candles_received,
        result.data_quality.minimum_required,
        if result.data_quality.sufficient_for_analysis { "yes" } else { "no" }
    ));
    if result.data_quality.issues.is_empty() {
        report.push_str("• Issues: none\\n");
    } else {
        report.push_str("• Issues:\\n");
        for issue in &result.data_quality.issues {
            report.push_str(&format!("  - {}: {}\\n", issue.code, issue.description));
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::technical_analysis::{
        AdxConfig, Asset, AssetType, AtrConfig, BollingerConfig, BreakoutConfig, Candle,
        CandleMetadata, DataQualityConfig, DataRequirements, DivergenceDetectionConfig,
        EngineConfig, IndicatorConfig, MacdConfig, MarketData, MarketStructureConfig,
        MomentumConfig, MovingAverageConfig, PatternDetectionConfig, PriceActionConfig,
        RegimeDetectionConfig, RsiConfig, ScenarioEngineConfig, SignalEngineConfig,
        StochasticConfig, TrendStrengthConfig, VolatilityConfig, VolumeConfig,
    };

    fn config() -> AnalysisConfig {
        AnalysisConfig {
            schema_version: "1.0".to_string(),
            engine: EngineConfig {
                name: "technical-analysis-engine".to_string(),
                version: "1.0.0".to_string(),
            },
            data_requirements: DataRequirements {
                minimum_candles: 30,
                recommended_candles: 50,
                maximum_candles: 100,
            },
            indicators: IndicatorConfig {
                moving_averages: MovingAverageConfig {
                    sma: vec![20],
                    ema: vec![20],
                },
                momentum: MomentumConfig {
                    rsi: RsiConfig { period: 14 },
                    macd: MacdConfig {
                        fast: 12,
                        slow: 26,
                        signal: 9,
                    },
                    stochastic: StochasticConfig {
                        k_period: 14,
                        d_period: 3,
                        smooth: 3,
                    },
                },
                volatility: VolatilityConfig {
                    atr: AtrConfig { period: 14 },
                    bollinger_bands: BollingerConfig {
                        period: 20,
                        stddev: 2.0,
                    },
                },
                trend_strength: TrendStrengthConfig {
                    adx: AdxConfig { period: 14 },
                },
                volume: VolumeConfig {
                    obv: true,
                    volume_average: vec![20],
                },
            },
            price_action: PriceActionConfig {
                enabled: true,
                candlestick_analysis: true,
                gap_detection: true,
                consecutive_move_detection: true,
            },
            market_structure: MarketStructureConfig {
                enabled: true,
                swing_detection: Default::default(),
                support_resistance: Default::default(),
            },
            breakout_detection: BreakoutConfig {
                enabled: true,
                lookback_period: 20,
                volume_confirmation: Default::default(),
                retest_detection: Default::default(),
            },
            pattern_detection: PatternDetectionConfig {
                enabled: true,
                candlestick_patterns: true,
                chart_patterns: vec!["ascending_structure".to_string()],
            },
            divergence_detection: DivergenceDetectionConfig {
                enabled: true,
                indicators: vec!["rsi".to_string()],
                minimum_swing_distance: 5,
            },
            regime_detection: RegimeDetectionConfig {
                enabled: true,
                dimensions: vec![
                    "trend".to_string(),
                    "momentum".to_string(),
                    "volatility".to_string(),
                    "volume".to_string(),
                ],
            },
            scenario_engine: ScenarioEngineConfig {
                enabled: true,
                scenarios: vec![
                    "bullish".to_string(),
                    "bearish".to_string(),
                    "range".to_string(),
                ],
            },
            signal_engine: SignalEngineConfig {
                enabled: true,
                signal_strength_levels: vec![
                    "weak".to_string(),
                    "moderate".to_string(),
                    "strong".to_string(),
                ],
            },
            data_quality: DataQualityConfig {
                validate_ohlcv: true,
                detect_missing_candles: true,
                detect_duplicate_candles: true,
                detect_invalid_prices: true,
                detect_zero_volume: true,
            },
        }
    }

    fn input() -> AnalysisInput {
        let candles = (0..60)
            .map(|index| {
                let close = 50.0 + index as f64 * 0.1;
                Candle {
                    timestamp: format!("2026-01-{index:02}"),
                    open: close - 0.1,
                    high: close + 0.2,
                    low: close - 0.2,
                    close,
                    volume: 1_000.0 + index as f64,
                    metadata: CandleMetadata::default(),
                }
            })
            .collect();

        AnalysisInput {
            schema_version: "1.0".to_string(),
            asset: Asset {
                symbol: "VNM".to_string(),
                asset_type: AssetType::Stock,
                exchange: "HOSE".to_string(),
                currency: "VND".to_string(),
            },
            market_data: MarketData {
                timeframe: "1D".to_string(),
                timezone: "Asia/Ho_Chi_Minh".to_string(),
                candles,
            },
        }
    }

    #[test]
    fn service_returns_analysis_result() {
        let result =
            TechnicalAnalysisService::analyze(&input(), &config(), "2026-09-23T15:42:00+07:00")
                .expect("analysis should succeed");

        assert_eq!(result.asset.symbol, "VNM");
        assert_eq!(result.data_quality.candles_used, 60);
        assert!(result.snapshot.close > 50.0);
        assert!(result.trend.moving_averages.ema[0].value.is_some());
    }

    #[test]
    fn service_serializes_json() {
        let input = serde_json::to_string(&input()).expect("input serialization should succeed");
        let config = serde_json::to_string(&config()).expect("config serialization should succeed");

        let output =
            TechnicalAnalysisService::analyze_json(&input, &config, "2026-09-23T15:42:00+07:00")
                .expect("JSON analysis should succeed");

        assert!(output.contains("\"technical-analysis-engine\""));
        assert!(output.contains("\"data_quality\""));
    }
}
