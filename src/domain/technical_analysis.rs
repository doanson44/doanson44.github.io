//! Technical analysis domain engine shared by stock and crypto assets.

use serde::{Deserialize, Serialize};

/// Supported asset classes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    /// Exchange-listed equity.
    Stock,
    /// Continuously traded digital asset.
    Crypto,
}

/// Asset identity and market metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Asset ticker or trading symbol.
    pub symbol: String,
    /// Asset class.
    pub asset_type: AssetType,
    /// Trading venue.
    pub exchange: String,
    /// Quote currency.
    pub currency: String,
}

/// Optional exchange-specific candle metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CandleMetadata {
    /// Reference price when supplied by the venue.
    #[serde(default)]
    pub reference_price: Option<f64>,
    /// Daily upper price limit when applicable.
    #[serde(default)]
    pub ceiling: Option<f64>,
    /// Daily lower price limit when applicable.
    #[serde(default)]
    pub floor: Option<f64>,
    /// Total traded value when supplied by the venue.
    #[serde(default)]
    pub total_value: Option<f64>,
}

/// One normalized OHLCV candle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    /// Candle timestamp in the source timezone.
    pub timestamp: String,
    /// Opening price.
    pub open: f64,
    /// Highest price.
    pub high: f64,
    /// Lowest price.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Traded volume.
    pub volume: f64,
    /// Optional venue-specific metadata.
    #[serde(default)]
    pub metadata: CandleMetadata,
}

/// Market data supplied to the engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketData {
    /// Candle timeframe such as 1D or 4H.
    pub timeframe: String,
    /// Source timezone.
    pub timezone: String,
    /// Historical candles.
    pub candles: Vec<Candle>,
}

/// Analysis input contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisInput {
    /// Input schema version.
    pub schema_version: String,
    /// Asset metadata.
    pub asset: Asset,
    /// Market data.
    pub market_data: MarketData,
}

/// Engine metadata configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Engine name.
    pub name: String,
    /// Engine version.
    pub version: String,
}

/// Data requirement configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataRequirements {
    /// Minimum candles required for analysis.
    pub minimum_candles: usize,
    /// Recommended candle count.
    pub recommended_candles: usize,
    /// Maximum candles accepted by the engine.
    pub maximum_candles: usize,
}

/// Moving-average configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovingAverageConfig {
    /// SMA periods.
    #[serde(default)]
    pub sma: Vec<usize>,
    /// EMA periods.
    #[serde(default)]
    pub ema: Vec<usize>,
}

/// Momentum configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MomentumConfig {
    /// RSI settings.
    pub rsi: RsiConfig,
    /// MACD settings.
    pub macd: MacdConfig,
    /// Stochastic settings.
    pub stochastic: StochasticConfig,
}

/// RSI configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RsiConfig {
    /// RSI period.
    pub period: usize,
}

/// MACD configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacdConfig {
    /// Fast EMA period.
    pub fast: usize,
    /// Slow EMA period.
    pub slow: usize,
    /// Signal EMA period.
    pub signal: usize,
}

/// Stochastic configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StochasticConfig {
    /// K period.
    pub k_period: usize,
    /// D period.
    pub d_period: usize,
    /// Smoothing period.
    pub smooth: usize,
}

/// Volatility configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityConfig {
    /// ATR settings.
    pub atr: AtrConfig,
    /// Bollinger-band settings.
    pub bollinger_bands: BollingerConfig,
}

/// ATR configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtrConfig {
    /// ATR period.
    pub period: usize,
}

/// Bollinger-band configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BollingerConfig {
    /// Bollinger period.
    pub period: usize,
    /// Standard-deviation multiplier.
    pub stddev: f64,
}

/// Trend-strength configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendStrengthConfig {
    /// ADX settings.
    pub adx: AdxConfig,
}

/// ADX configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdxConfig {
    /// ADX period.
    pub period: usize,
}

/// Volume configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeConfig {
    /// Whether OBV is enabled.
    pub obv: bool,
    /// Volume-average periods.
    #[serde(default)]
    pub volume_average: Vec<usize>,
}

/// Indicator configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndicatorConfig {
    /// Moving averages.
    pub moving_averages: MovingAverageConfig,
    /// Momentum settings.
    pub momentum: MomentumConfig,
    /// Volatility settings.
    pub volatility: VolatilityConfig,
    /// Trend-strength settings.
    pub trend_strength: TrendStrengthConfig,
    /// Volume settings.
    pub volume: VolumeConfig,
}

/// Price-action configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceActionConfig {
    /// Enables price-action analysis.
    pub enabled: bool,
    /// Enables candlestick classification.
    pub candlestick_analysis: bool,
    /// Enables gap detection.
    pub gap_detection: bool,
    /// Enables consecutive move detection.
    pub consecutive_move_detection: bool,
}

/// Market-structure configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketStructureConfig {
    /// Enables structure analysis.
    pub enabled: bool,
    /// Swing-pivot settings.
    #[serde(default)]
    pub swing_detection: SwingDetectionConfig,
    /// Support/resistance settings.
    #[serde(default)]
    pub support_resistance: SupportResistanceConfig,
}

/// Swing-pivot configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SwingDetectionConfig {
    /// Pivot lookback.
    #[serde(default = "default_swing_lookback")]
    pub lookback: usize,
}

/// Support/resistance configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SupportResistanceConfig {
    /// Historical lookback.
    #[serde(default = "default_support_resistance_lookback")]
    pub lookback: usize,
    /// Cluster tolerance in percent.
    #[serde(default = "default_cluster_tolerance")]
    pub cluster_tolerance_percent: f64,
    /// Minimum touches.
    #[serde(default = "default_minimum_touches")]
    pub minimum_touches: usize,
}

/// Breakout configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakoutConfig {
    /// Enables breakout analysis.
    pub enabled: bool,
    /// Breakout lookback.
    pub lookback_period: usize,
    /// Volume-confirmation settings.
    #[serde(default)]
    pub volume_confirmation: VolumeConfirmationConfig,
    /// Retest settings.
    #[serde(default)]
    pub retest_detection: RetestConfig,
}

/// Volume-confirmation configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VolumeConfirmationConfig {
    /// Whether volume confirmation is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Required volume ratio.
    #[serde(default = "default_volume_ratio")]
    pub minimum_volume_ratio: f64,
}

/// Retest configuration.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RetestConfig {
    /// Whether retest detection is enabled.
    #[serde(default)]
    pub enabled: bool,
}

/// Complete analysis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Configuration schema version.
    pub schema_version: String,
    /// Engine metadata.
    pub engine: EngineConfig,
    /// Data requirements.
    pub data_requirements: DataRequirements,
    /// Indicator settings.
    pub indicators: IndicatorConfig,
    /// Price-action settings.
    pub price_action: PriceActionConfig,
    /// Market-structure settings.
    pub market_structure: MarketStructureConfig,
    /// Breakout settings.
    pub breakout_detection: BreakoutConfig,
}

/// Data-quality issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataQualityIssue {
    /// Stable issue code.
    pub code: String,
    /// Human-readable issue description.
    pub description: String,
}

/// Candle coverage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coverage {
    /// First candle timestamp.
    pub from: Option<String>,
    /// Last candle timestamp.
    pub to: Option<String>,
}

/// Data-quality report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataQualityReport {
    /// Number of candles received.
    pub candles_received: usize,
    /// Number of candles used.
    pub candles_used: usize,
    /// Minimum required candle count.
    pub minimum_required: usize,
    /// Whether minimum data requirements are met.
    pub sufficient_for_analysis: bool,
    /// Validation issues.
    pub issues: Vec<DataQualityIssue>,
    /// Data coverage.
    pub coverage: Coverage,
}

/// Price-change snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceChange {
    /// Absolute change.
    pub absolute: f64,
    /// Percentage change.
    pub percent: f64,
}

/// Current market snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Current candle timestamp.
    pub timestamp: String,
    /// Opening price.
    pub open: f64,
    /// Highest price.
    pub high: f64,
    /// Lowest price.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Current volume.
    pub volume: f64,
    /// One-period price change.
    pub price_change: PriceChange,
}

/// Candle anatomy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandleAnalysis {
    /// Candle direction.
    pub candle_type: String,
    /// Absolute body size.
    pub body: f64,
    /// Total range.
    pub range: f64,
    /// Upper wick size.
    pub upper_wick: f64,
    /// Lower wick size.
    pub lower_wick: f64,
    /// Body as a percentage of range.
    pub body_percent_of_range: f64,
}

/// Price-action result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceActionAnalysis {
    /// Candle analysis.
    pub candle: CandleAnalysis,
    /// Recent percentage performance.
    pub recent_performance: RecentPerformance,
}

/// Recent returns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentPerformance {
    /// One-period return.
    #[serde(rename = "1D")]
    pub one_day: Option<f64>,
    /// Five-period return.
    #[serde(rename = "5D")]
    pub five_day: Option<f64>,
    /// Twenty-period return.
    #[serde(rename = "20D")]
    pub twenty_day: Option<f64>,
    /// Sixty-period return.
    #[serde(rename = "60D")]
    pub sixty_day: Option<f64>,
}

/// Trend state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Directional state.
    pub state: String,
    /// Strength classification.
    pub strength: String,
    /// Requested moving-average values.
    pub moving_averages: MovingAverageSnapshot,
    /// Current alignment.
    pub alignment: Alignment,
}

/// Moving-average snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovingAverageSnapshot {
    /// SMA values keyed by period.
    pub sma: Vec<PeriodValue>,
    /// EMA values keyed by period.
    pub ema: Vec<PeriodValue>,
}

/// Generic period/value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodValue {
    /// Indicator period.
    pub period: usize,
    /// Current value.
    pub value: Option<f64>,
}

/// Moving-average alignment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alignment {
    /// Whether the configured EMA chain is bullish.
    pub bullish: bool,
    /// Whether the configured EMA chain is bearish.
    pub bearish: bool,
    /// Human-readable description.
    pub description: String,
}

/// Momentum result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MomentumAnalysis {
    /// RSI analysis.
    pub rsi: RsiAnalysis,
    /// MACD analysis.
    pub macd: MacdAnalysis,
    /// Stochastic analysis.
    pub stochastic: StochasticAnalysis,
}

/// RSI analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RsiAnalysis {
    /// Current RSI.
    pub value: Option<f64>,
    /// RSI state.
    pub state: String,
    /// RSI above 50.
    pub above_50: bool,
    /// RSI overbought.
    pub overbought: bool,
    /// RSI oversold.
    pub oversold: bool,
}

/// MACD analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacdAnalysis {
    /// MACD value.
    pub macd: Option<f64>,
    /// Signal value.
    pub signal: Option<f64>,
    /// Histogram.
    pub histogram: Option<f64>,
    /// Directional state.
    pub state: String,
    /// Histogram direction.
    pub histogram_direction: String,
    /// Recent crossover.
    pub recent_cross: String,
}

/// Stochastic analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StochasticAnalysis {
    /// Smoothed K.
    pub k: Option<f64>,
    /// D value.
    pub d: Option<f64>,
    /// Directional state.
    pub state: String,
}

/// Volatility result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityAnalysis {
    /// ATR value.
    pub atr: Option<f64>,
    /// ATR as a percentage of price.
    pub atr_percent: Option<f64>,
    /// Volatility state.
    pub state: String,
    /// Bollinger analysis.
    pub bollinger_bands: BollingerAnalysis,
}

/// Bollinger analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BollingerAnalysis {
    /// Upper band.
    pub upper: Option<f64>,
    /// Middle band.
    pub middle: Option<f64>,
    /// Lower band.
    pub lower: Option<f64>,
    /// Bandwidth percentage.
    pub bandwidth: Option<f64>,
    /// Price position inside the band.
    pub price_position: Option<f64>,
}

/// Volume result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeAnalysis {
    /// Current volume.
    pub current: f64,
    /// Configured volume averages.
    pub averages: Vec<PeriodValue>,
    /// Ratio versus the first configured average.
    pub ratio_vs_primary: Option<f64>,
    /// Volume state.
    pub state: String,
    /// Current OBV.
    pub obv: Option<f64>,
    /// OBV trend.
    pub obv_trend: String,
}

/// Complete analysis result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Output schema version.
    pub schema_version: String,
    /// Engine metadata.
    pub engine: EngineOutput,
    /// Asset metadata.
    pub asset: AssetOutput,
    /// Data quality.
    pub data_quality: DataQualityReport,
    /// Current snapshot.
    pub snapshot: Snapshot,
    /// Price action.
    pub price_action: PriceActionAnalysis,
    /// Trend.
    pub trend: TrendAnalysis,
    /// Momentum.
    pub momentum: MomentumAnalysis,
    /// Volatility.
    pub volatility: VolatilityAnalysis,
    /// Volume.
    pub volume: VolumeAnalysis,
}

/// Engine output metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineOutput {
    /// Engine name.
    pub name: String,
    /// Engine version.
    pub version: String,
    /// Analysis timestamp supplied by the caller.
    pub analysis_timestamp: String,
}

/// Asset output metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetOutput {
    /// Symbol.
    pub symbol: String,
    /// Asset type.
    pub asset_type: AssetType,
    /// Exchange.
    pub exchange: String,
    /// Currency.
    pub currency: String,
    /// Timeframe.
    pub timeframe: String,
}

fn default_swing_lookback() -> usize { 5 }

fn default_support_resistance_lookback() -> usize { 120 }

fn default_cluster_tolerance() -> f64 { 1.0 }

fn default_minimum_touches() -> usize { 2 }

fn default_volume_ratio() -> f64 { 1.5 }

/// Validates the input and returns a normalized candle vector.
pub fn validate_input(
    input: &AnalysisInput,
    requirements: &DataRequirements,
) -> Result<(Vec<Candle>, DataQualityReport), String> {
    if input.schema_version.trim().is_empty() {
        return Err("Input schema_version cannot be empty".to_string());
    }
    if input.asset.symbol.trim().is_empty() {
        return Err("Asset symbol cannot be empty".to_string());
    }
    if input.market_data.timeframe.trim().is_empty() {
        return Err("Market timeframe cannot be empty".to_string());
    }
    if requirements.minimum_candles == 0 {
        return Err("minimum_candles must be greater than zero".to_string());
    }
    if requirements.maximum_candles < requirements.minimum_candles {
        return Err("maximum_candles cannot be below minimum_candles".to_string());
    }

    let received = input.market_data.candles.len();
    let mut issues = Vec::new();

    for (index, candle) in input.market_data.candles.iter().enumerate() {
        if candle.timestamp.trim().is_empty() {
            issues.push(issue("MISSING_TIMESTAMP", format!("Candle {index} has no timestamp")));
        }
        if ![candle.open, candle.high, candle.low, candle.close, candle.volume]
            .iter()
            .all(|value| value.is_finite())
        {
            issues.push(issue("INVALID_NUMBER", format!("Candle {index} contains a non-finite value")));
        }
        if candle.open <= 0.0
            || candle.high <= 0.0
            || candle.low <= 0.0
            || candle.close <= 0.0
        {
            issues.push(issue("INVALID_PRICE", format!("Candle {index} contains a non-positive price")));
        }
        if candle.high < candle.low
            || candle.high < candle.open
            || candle.high < candle.close
            || candle.low > candle.open
            || candle.low > candle.close
        {
            issues.push(issue("INVALID_OHLC", format!("Candle {index} violates OHLC bounds")));
        }
        if candle.volume < 0.0 {
            issues.push(issue("INVALID_VOLUME", format!("Candle {index} has negative volume")));
        }
    }

    let mut candles = input.market_data.candles.clone();
    candles.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    for pair in candles.windows(2) {
        if pair[0].timestamp == pair[1].timestamp {
            issues.push(issue(
                "DUPLICATE_CANDLE",
                format!("Duplicate candle timestamp: {}", pair[0].timestamp),
            ));
        }
    }

    if received > requirements.maximum_candles {
        issues.push(issue(
            "MAXIMUM_CANDLES_EXCEEDED",
            format!(
                "Received {received} candles; maximum is {}",
                requirements.maximum_candles
            ),
        ));
        let start = received - requirements.maximum_candles;
        candles = candles[start..].to_vec();
    }

    let coverage = Coverage {
        from: candles.first().map(|candle| candle.timestamp.clone()),
        to: candles.last().map(|candle| candle.timestamp.clone()),
    };

    Ok((
        candles.clone(),
        DataQualityReport {
            candles_received: received,
            candles_used: candles.len(),
            minimum_required: requirements.minimum_candles,
            sufficient_for_analysis: candles.len() >= requirements.minimum_candles && issues.is_empty(),
            issues,
            coverage,
        },
    ))
}

/// Calculates a simple moving average series.
pub fn sma(values: &[f64], period: usize) -> Result<Vec<Option<f64>>, String> {
    validate_period(values, period)?;
    let mut output = vec![None; values.len()];
    let mut sum = 0.0;

    for index in 0..values.len() {
        sum += values[index];
        if index >= period {
            sum -= values[index - period];
        }
        if index + 1 >= period {
            output[index] = Some(sum / period as f64);
        }
    }

    Ok(output)
}

/// Calculates an exponential moving average series.
pub fn ema(values: &[f64], period: usize) -> Result<Vec<Option<f64>>, String> {
    validate_period(values, period)?;
    let mut output = vec![None; values.len()];
    let seed = values[..period].iter().sum::<f64>() / period as f64;
    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut previous = seed;
    output[period - 1] = Some(seed);

    for index in period..values.len() {
        previous = (values[index] - previous) * multiplier + previous;
        output[index] = Some(previous);
    }

    Ok(output)
}

/// Calculates Wilder RSI.
pub fn rsi(values: &[f64], period: usize) -> Result<Vec<Option<f64>>, String> {
    validate_period(values, period)?;
    if values.len() <= period {
        return Ok(vec![None; values.len()]);
    }

    let mut output = vec![None; values.len()];
    let mut gains = 0.0;
    let mut losses = 0.0;

    for index in 1..=period {
        let delta = values[index] - values[index - 1];
        if delta >= 0.0 {
            gains += delta;
        } else {
            losses -= delta;
        }
    }

    let mut average_gain = gains / period as f64;
    let mut average_loss = losses / period as f64;
    output[period] = Some(rsi_value(average_gain, average_loss));

    for index in (period + 1)..values.len() {
        let delta = values[index] - values[index - 1];
        let gain = delta.max(0.0);
        let loss = (-delta).max(0.0);
        average_gain = (average_gain * (period as f64 - 1.0) + gain) / period as f64;
        average_loss = (average_loss * (period as f64 - 1.0) + loss) / period as f64;
        output[index] = Some(rsi_value(average_gain, average_loss));
    }

    Ok(output)
}

/// Calculates true-range values.
pub fn true_ranges(candles: &[Candle]) -> Result<Vec<f64>, String> {
    if candles.is_empty() {
        return Err("At least one candle is required".to_string());
    }
    let mut result = Vec::with_capacity(candles.len());
    for (index, candle) in candles.iter().enumerate() {
        let value = if index == 0 {
            candle.high - candle.low
        } else {
            let previous_close = candles[index - 1].close;
            (candle.high - candle.low)
                .max((candle.high - previous_close).abs())
                .max((candle.low - previous_close).abs())
        };
        result.push(value);
    }
    Ok(result)
}

/// Calculates ATR using Wilder smoothing.
pub fn atr(candles: &[Candle], period: usize) -> Result<Vec<Option<f64>>, String> {
    let ranges = true_ranges(candles)?;
    validate_period(&ranges, period)?;
    let mut output = vec![None; ranges.len()];
    let mut value = ranges[..period].iter().sum::<f64>() / period as f64;
    output[period - 1] = Some(value);

    for index in period..ranges.len() {
        value = (value * (period as f64 - 1.0) + ranges[index]) / period as f64;
        output[index] = Some(value);
    }

    Ok(output)
}

/// Calculates Bollinger bands from a close-price series.
pub fn bollinger(
    values: &[f64],
    period: usize,
    stddev: f64,
) -> Result<(Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>), String> {
    validate_period(values, period)?;
    if !stddev.is_finite() || stddev <= 0.0 {
        return Err("Bollinger stddev must be positive and finite".to_string());
    }

    let middle = sma(values, period)?;
    let mut upper = vec![None; values.len()];
    let mut lower = vec![None; values.len()];

    for index in (period - 1)..values.len() {
        let mean = middle[index].unwrap_or(0.0);
        let variance = values[index + 1 - period..=index]
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / period as f64;
        let deviation = variance.sqrt() * stddev;
        upper[index] = Some(mean + deviation);
        lower[index] = Some(mean - deviation);
    }

    Ok((upper, middle, lower))
}

/// Calculates OBV.
pub fn obv(candles: &[Candle]) -> Result<Vec<f64>, String> {
    if candles.is_empty() {
        return Err("At least one candle is required".to_string());
    }
    let mut result = vec![0.0; candles.len()];
    for index in 1..candles.len() {
        result[index] = result[index - 1]
            + if candles[index].close > candles[index - 1].close {
                candles[index].volume
            } else if candles[index].close < candles[index - 1].close {
                -candles[index].volume
            } else {
                0.0
            };
    }
    Ok(result)
}

/// Calculates percentage change from a historical offset.
pub fn percentage_change(values: &[f64], offset: usize) -> Option<f64> {
    if values.len() <= offset {
        return None;
    }
    let current = *values.last()?;
    let previous = values[values.len() - 1 - offset];
    if previous == 0.0 {
        None
    } else {
        Some((current - previous) / previous * 100.0)
    }
}

/// Runs the supported analysis pipeline.
pub fn analyze(
    input: &AnalysisInput,
    config: &AnalysisConfig,
    analysis_timestamp: String,
) -> Result<AnalysisResult, String> {
    let (candles, data_quality) = validate_input(input, &config.data_requirements)?;
    if !data_quality.sufficient_for_analysis {
        return Err("Market data does not satisfy the configured analysis requirements".to_string());
    }

    let closes: Vec<f64> = candles.iter().map(|c| c.close).collect();
    let latest = candles
        .last()
        .ok_or_else(|| "No candles available after validation".to_string())?;
    let previous = candles
        .get(candles.len().saturating_sub(2))
        .unwrap_or(latest);

    let snapshot = Snapshot {
        timestamp: latest.timestamp.clone(),
        open: latest.open,
        high: latest.high,
        low: latest.low,
        close: latest.close,
        volume: latest.volume,
        price_change: PriceChange {
            absolute: latest.close - previous.close,
            percent: if previous.close == 0.0 {
                0.0
            } else {
                (latest.close - previous.close) / previous.close * 100.0
            },
        },
    };

    let candle = candle_analysis(latest);
    let price_action = PriceActionAnalysis {
        candle,
        recent_performance: RecentPerformance {
            one_day: percentage_change(&closes, 1),
            five_day: percentage_change(&closes, 5),
            twenty_day: percentage_change(&closes, 20),
            sixty_day: percentage_change(&closes, 60),
        },
    };

    let sma_values = config
        .indicators
        .moving_averages
        .sma
        .iter()
        .map(|period| {
            Ok(PeriodValue {
                period: *period,
                value: sma(&closes, *period)?.last().copied().flatten(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let ema_values = config
        .indicators
        .moving_averages
        .ema
        .iter()
        .map(|period| {
            Ok(PeriodValue {
                period: *period,
                value: ema(&closes, *period)?.last().copied().flatten(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let ema9 = find_period(&ema_values, 9);
    let ema20 = find_period(&ema_values, 20);
    let ema50 = find_period(&ema_values, 50);
    let ema200 = find_period(&ema_values, 200);
    let bullish_alignment = ema20.is_some_and(|value| latest.close > value)
        && ema20.zip(ema50).is_some_and(|(a, b)| a > b)
        && ema50.zip(ema200).is_some_and(|(a, b)| a > b);
    let bearish_alignment = ema20.is_some_and(|value| latest.close < value)
        && ema20.zip(ema50).is_some_and(|(a, b)| a < b)
        && ema50.zip(ema200).is_some_and(|(a, b)| a < b);

    let trend = TrendAnalysis {
        state: if bullish_alignment {
            "bullish".to_string()
        } else if bearish_alignment {
            "bearish".to_string()
        } else {
            "neutral".to_string()
        },
        strength: "moderate".to_string(),
        moving_averages: MovingAverageSnapshot {
            sma: sma_values,
            ema: ema_values,
        },
        alignment: Alignment {
            bullish: bullish_alignment,
            bearish: bearish_alignment,
            description: alignment_description(latest.close, ema20, ema50, ema200),
        },
    };

    let rsi_values = rsi(&closes, config.indicators.momentum.rsi.period)?;
    let rsi_current = rsi_values.last().copied().flatten();
    let rsi_analysis = RsiAnalysis {
        value: rsi_current,
        state: rsi_current.map_or_else(|| "unavailable".to_string(), |value| {
            if value >= 50.0 { "positive" } else { "negative" }.to_string()
        }),
        above_50: rsi_current.is_some_and(|value| value >= 50.0),
        overbought: rsi_current.is_some_and(|value| value >= 70.0),
        oversold: rsi_current.is_some_and(|value| value <= 30.0),
    };

    let fast = ema(&closes, config.indicators.momentum.macd.fast)?;
    let slow = ema(&closes, config.indicators.momentum.macd.slow)?;
    let macd_line: Vec<f64> = fast
        .iter()
        .zip(slow.iter())
        .map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => a - b,
            _ => 0.0,
        })
        .collect();
    let signal_line = ema(&macd_line, config.indicators.momentum.macd.signal)?;
    let macd_current = macd_line.last().copied();
    let signal_current = signal_line.last().copied().flatten();
    let histogram = macd_current.zip(signal_current).map(|(a, b)| a - b);
    let previous_histogram = if macd_line.len() >= 2 {
        macd_line[macd_line.len() - 2]
            .partial_cmp(&0.0)
            .map(|_| {
                signal_line
                    .get(signal_line.len() - 2)
                    .and_then(|value| *value)
                    .map(|signal| macd_line[macd_line.len() - 2] - signal)
            })
            .flatten()
    } else {
        None
    };
    let recent_cross = if macd_line.len() >= 2 {
        let previous_signal = signal_line.get(signal_line.len() - 2).and_then(|v| *v);
        match (macd_current, signal_current, previous_signal) {
            (Some(current), Some(signal), Some(previous_signal))
                if current > signal && macd_line[macd_line.len() - 2] <= previous_signal =>
            {
                "bullish".to_string()
            }
            (Some(current), Some(signal), Some(previous_signal))
                if current < signal && macd_line[macd_line.len() - 2] >= previous_signal =>
            {
                "bearish".to_string()
            }
            _ => "none".to_string(),
        }
    } else {
        "none".to_string()
    };

    let stochastic_k = stochastic_k(&candles, config.indicators.momentum.stochastic.k_period)?;
    let stochastic_d = stochastic_d(
        &stochastic_k,
        config.indicators.momentum.stochastic.d_period,
    )?;
    let k = stochastic_k.last().copied().flatten();
    let d = stochastic_d.last().copied().flatten();
    let momentum = MomentumAnalysis {
        rsi: rsi_analysis,
        macd: MacdAnalysis {
            macd: macd_current,
            signal: signal_current,
            histogram,
            state: if macd_current.zip(signal_current).is_some_and(|(a, b)| a > b) {
                "bullish".to_string()
            } else {
                "bearish".to_string()
            },
            histogram_direction: match (histogram, previous_histogram) {
                (Some(current), Some(previous)) if current > previous => "increasing".to_string(),
                (Some(_), Some(_)) => "decreasing".to_string(),
                _ => "unavailable".to_string(),
            },
            recent_cross,
        },
        stochastic: StochasticAnalysis {
            k,
            d,
            state: if k.zip(d).is_some_and(|(a, b)| a > b) {
                "positive".to_string()
            } else {
                "negative".to_string()
            },
        },
    };

    let atr_values = atr(&candles, config.indicators.volatility.atr.period)?;
    let atr_current = atr_values.last().copied().flatten();
    let atr_percent = atr_current.map(|value| value / latest.close * 100.0);
    let (upper, middle, lower) = bollinger(
        &closes,
        config.indicators.volatility.bollinger_bands.period,
        config.indicators.volatility.bollinger_bands.stddev,
    )?;
    let upper_current = upper.last().copied().flatten();
    let middle_current = middle.last().copied().flatten();
    let lower_current = lower.last().copied().flatten();
    let bandwidth = upper_current
        .zip(lower_current)
        .zip(middle_current)
        .map(|((upper, lower), middle)| (upper - lower) / middle * 100.0);
    let price_position = upper_current.zip(lower_current).and_then(|(upper, lower)| {
        let width = upper - lower;
        if width == 0.0 {
            None
        } else {
            Some((latest.close - lower) / width)
        }
    });
    let volatility = VolatilityAnalysis {
        atr: atr_current,
        atr_percent,
        state: "normal".to_string(),
        bollinger_bands: BollingerAnalysis {
            upper: upper_current,
            middle: middle_current,
            lower: lower_current,
            bandwidth,
            price_position,
        },
    };

    let averages = config
        .indicators
        .volume
        .volume_average
        .iter()
        .map(|period| {
            Ok(PeriodValue {
                period: *period,
                value: sma(
                    &candles.iter().map(|candle| candle.volume).collect::<Vec<_>>(),
                    *period,
                )?
                .last()
                .copied()
                .flatten(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let primary_average = averages.first().and_then(|value| value.value);
    let ratio = primary_average.and_then(|average| {
        if average == 0.0 {
            None
        } else {
            Some(latest.volume / average)
        }
    });
    let obv_values = if config.indicators.volume.obv {
        Some(obv(&candles)?)
    } else {
        None
    };
    let obv_current = obv_values.as_ref().and_then(|values| values.last().copied());
    let obv_trend = obv_values.as_ref().map_or_else(
        || "unavailable".to_string(),
        |values| {
            if values.len() >= 2 && values[values.len() - 1] >= values[values.len() - 2] {
                "rising".to_string()
            } else {
                "falling".to_string()
            }
        },
    );
    let volume = VolumeAnalysis {
        current: latest.volume,
        averages,
        ratio_vs_primary: ratio,
        state: ratio.map_or_else(
            || "unavailable".to_string(),
            |value| {
                if value >= 1.0 {
                    "above_average".to_string()
                } else {
                    "below_average".to_string()
                }
            },
        ),
        obv: obv_current,
        obv_trend,
    };

    Ok(AnalysisResult {
        schema_version: config.schema_version.clone(),
        engine: EngineOutput {
            name: config.engine.name.clone(),
            version: config.engine.version.clone(),
            analysis_timestamp,
        },
        asset: AssetOutput {
            symbol: input.asset.symbol.clone(),
            asset_type: input.asset.asset_type.clone(),
            exchange: input.asset.exchange.clone(),
            currency: input.asset.currency.clone(),
            timeframe: input.market_data.timeframe.clone(),
        },
        data_quality,
        snapshot,
        price_action,
        trend,
        momentum,
        volatility,
        volume,
    })
}

fn issue(code: &str, description: String) -> DataQualityIssue {
    DataQualityIssue {
        code: code.to_string(),
        description,
    }
}

fn validate_period(values: &[f64], period: usize) -> Result<(), String> {
    if period == 0 {
        return Err("Indicator period must be greater than zero".to_string());
    }
    if values.len() < period {
        return Err(format!(
            "Indicator period {period} requires at least {period} values, received {}",
            values.len()
        ));
    }
    Ok(())
}

fn rsi_value(gain: f64, loss: f64) -> f64 {
    if loss == 0.0 {
        100.0
    } else {
        100.0 - 100.0 / (1.0 + gain / loss)
    }
}

fn candle_analysis(candle: &Candle) -> CandleAnalysis {
    let range = candle.high - candle.low;
    let body = (candle.close - candle.open).abs();
    let upper_wick = candle.high - candle.open.max(candle.close);
    let lower_wick = candle.open.min(candle.close) - candle.low;
    CandleAnalysis {
        candle_type: if candle.close > candle.open {
            "bullish".to_string()
        } else if candle.close < candle.open {
            "bearish".to_string()
        } else {
            "doji".to_string()
        },
        body,
        range,
        upper_wick,
        lower_wick,
        body_percent_of_range: if range == 0.0 {
            0.0
        } else {
            body / range * 100.0
        },
    }
}

fn find_period(values: &[PeriodValue], period: usize) -> Option<f64> {
    values
        .iter()
        .find(|value| value.period == period)
        .and_then(|value| value.value)
}

fn alignment_description(
    price: f64,
    ema20: Option<f64>,
    ema50: Option<f64>,
    ema200: Option<f64>,
) -> String {
    match (ema20, ema50, ema200) {
        (Some(a), Some(b), Some(c)) if price > a && a > b && b > c => {
            "Price > EMA20 > EMA50 > EMA200".to_string()
        }
        (Some(a), Some(b), Some(c)) if price < a && a < b && b < c => {
            "Price < EMA20 < EMA50 < EMA200".to_string()
        }
        _ => "No complete EMA20/EMA50/EMA200 alignment".to_string(),
    }
}

fn stochastic_d(
    values: &[Option<f64>],
    period: usize,
) -> Result<Vec<Option<f64>>, String> {
    if period == 0 {
        return Err("Stochastic D period must be greater than zero".to_string());
    }
    let mut result = vec![None; values.len()];
    for index in 0..values.len() {
        if index + 1 < period {
            continue;
        }
        let window = &values[index + 1 - period..=index];
        if window.iter().all(Option::is_some) {
            let sum = window.iter().filter_map(|value| *value).sum::<f64>();
            result[index] = Some(sum / period as f64);
        }
    }
    Ok(result)
}

fn stochastic_k(candles: &[Candle], period: usize) -> Result<Vec<Option<f64>>, String> {
    if period == 0 {
        return Err("Stochastic K period must be greater than zero".to_string());
    }
    if candles.len() < period {
        return Err("Insufficient candles for stochastic calculation".to_string());
    }

    let mut result = vec![None; candles.len()];
    for index in (period - 1)..candles.len() {
        let window = &candles[index + 1 - period..=index];
        let highest = window
            .iter()
            .map(|candle| candle.high)
            .fold(f64::NEG_INFINITY, f64::max);
        let lowest = window
            .iter()
            .map(|candle| candle.low)
            .fold(f64::INFINITY, f64::min);
        let range = highest - lowest;
        result[index] = Some(if range == 0.0 {
            50.0
        } else {
            (candles[index].close - lowest) / range * 100.0
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candles(values: &[f64]) -> Vec<Candle> {
        values
            .iter()
            .enumerate()
            .map(|(index, close)| Candle {
                timestamp: format!("2026-09-{index:02}"),
                open: *close,
                high: close + 1.0,
                low: close - 1.0,
                close: *close,
                volume: 100.0,
                metadata: CandleMetadata::default(),
            })
            .collect()
    }

    #[test]
    fn sma_calculates_expected_values() {
        let result = sma(&[1.0, 2.0, 3.0, 4.0], 2).expect("valid SMA");
        assert_eq!(result, vec![None, Some(1.5), Some(2.5), Some(3.5)]);
    }

    #[test]
    fn ema_calculates_expected_seed() {
        let result = ema(&[1.0, 2.0, 3.0], 2).expect("valid EMA");
        assert_eq!(result[1], Some(1.5));
    }

    #[test]
    fn rsi_rises_after_positive_move() {
        let result = rsi(&[1.0, 2.0, 3.0, 4.0], 2).expect("valid RSI");
        assert_eq!(result[2], Some(100.0));
    }

    #[test]
    fn rejects_invalid_ohlc() {
        let mut input = AnalysisInput {
            schema_version: "1.0".to_string(),
            asset: Asset {
                symbol: "TEST".to_string(),
                asset_type: AssetType::Stock,
                exchange: "TEST".to_string(),
                currency: "USD".to_string(),
            },
            market_data: MarketData {
                timeframe: "1D".to_string(),
                timezone: "UTC".to_string(),
                candles: candles(&[10.0, 11.0]),
            },
        };
        input.market_data.candles[0].high = 5.0;
        let requirements = DataRequirements {
            minimum_candles: 1,
            recommended_candles: 2,
            maximum_candles: 10,
        };
        let (_, quality) = validate_input(&input, &requirements).expect("validation should return a report");
        assert!(!quality.issues.is_empty());
        assert!(!quality.sufficient_for_analysis);
    }
}
