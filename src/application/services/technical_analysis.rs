use crate::domain::technical_analysis::{analyze, AnalysisConfig, AnalysisInput, AnalysisResult};

/// Application service for stock and crypto technical analysis.
pub struct TechnicalAnalysisService;

impl TechnicalAnalysisService {
    /// Builds a stock daily analysis input from CafeF historical price data.
    pub fn price_history_input(
        raw: &str,
        symbol: &str,
    ) -> Result<AnalysisInput, String> {
        let history = crate::domain::market::parse_price_history_response(raw, symbol)?;
        let candles = history
            .candles
            .into_iter()
            .map(|candle| crate::domain::technical_analysis::Candle {
                timestamp: candle.trade_date,
                open: candle.open,
                high: candle.high,
                low: candle.low,
                close: candle.close,
                volume: candle.volume,
                metadata: crate::domain::technical_analysis::CandleMetadata {
                    reference_price: Some(candle.basic_price),
                    ceiling: candle.ceiling,
                    floor: candle.floor,
                    total_value: candle.total_value,
                },
            })
            .collect();

        Ok(AnalysisInput {
            schema_version: "1.0".to_string(),
            asset: crate::domain::technical_analysis::Asset {
                symbol: symbol.trim().to_ascii_uppercase(),
                asset_type: crate::domain::technical_analysis::AssetType::Stock,
                exchange: "HOSE".to_string(),
                currency: "VND".to_string(),
            },
            market_data: crate::domain::technical_analysis::MarketData {
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
