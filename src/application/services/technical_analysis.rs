use crate::domain::technical_analysis::{analyze, AnalysisConfig, AnalysisInput, AnalysisResult};

/// Application service for stock and crypto technical analysis.
pub struct TechnicalAnalysisService;

impl TechnicalAnalysisService {
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
        let input: AnalysisInput =
            serde_json::from_str(input_json).map_err(|error| format!("Invalid analysis input JSON: {error}"))?;
        let config: AnalysisConfig =
            serde_json::from_str(config_json).map_err(|error| format!("Invalid analysis config JSON: {error}"))?;
        let result = Self::analyze(&input, &config, analysis_timestamp)?;
        serde_json::to_string_pretty(&result)
            .map_err(|error| format!("Failed to serialize analysis result: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::technical_analysis::{
        Asset, AssetType, Candle, CandleMetadata, DataRequirements, EngineConfig, IndicatorConfig,
        MarketData, MomentumConfig, MovingAverageConfig, PriceActionConfig, RsiConfig,
        MacdConfig, StochasticConfig, VolatilityConfig, AtrConfig, BollingerConfig,
        TrendStrengthConfig, AdxConfig, VolumeConfig, MarketStructureConfig, BreakoutConfig,
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
        let result = TechnicalAnalysisService::analyze(&input(), &config(), "2026-09-23T15:42:00+07:00")
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

        let output = TechnicalAnalysisService::analyze_json(
            &input,
            &config,
            "2026-09-23T15:42:00+07:00",
        )
        .expect("JSON analysis should succeed");

        assert!(output.contains("\"technical-analysis-engine\""));
        assert!(output.contains("\"data_quality\""));
    }
}
