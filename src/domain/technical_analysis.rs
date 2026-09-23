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

/// Pattern-detection configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PatternDetectionConfig {
    /// Whether pattern detection is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Whether candlestick patterns are enabled.
    #[serde(default)]
    pub candlestick_patterns: bool,
    /// Chart-pattern names.
    #[serde(default)]
    pub chart_patterns: Vec<String>,
}

/// Divergence-detection configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DivergenceDetectionConfig {
    /// Whether divergence detection is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Indicators to inspect.
    #[serde(default)]
    pub indicators: Vec<String>,
    /// Minimum swing distance.
    #[serde(default = "default_swing_lookback")]
    pub minimum_swing_distance: usize,
}

/// Regime-detection configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RegimeDetectionConfig {
    /// Whether regime detection is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Regime dimensions.
    #[serde(default)]
    pub dimensions: Vec<String>,
}

/// Scenario-engine configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScenarioEngineConfig {
    /// Whether scenario generation is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Scenario names.
    #[serde(default)]
    pub scenarios: Vec<String>,
}

/// Signal-engine configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalEngineConfig {
    /// Whether signal generation is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Supported signal strength levels.
    #[serde(default)]
    pub signal_strength_levels: Vec<String>,
}

/// Data-quality configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DataQualityConfig {
    /// Validate OHLCV values.
    #[serde(default)]
    pub validate_ohlcv: bool,
    /// Detect missing candles.
    #[serde(default)]
    pub detect_missing_candles: bool,
    /// Detect duplicate candles.
    #[serde(default)]
    pub detect_duplicate_candles: bool,
    /// Detect invalid prices.
    #[serde(default)]
    pub detect_invalid_prices: bool,
    /// Detect zero volume.
    #[serde(default)]
    pub detect_zero_volume: bool,
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
    /// Pattern-detection settings.
    #[serde(default)]
    pub pattern_detection: PatternDetectionConfig,
    /// Divergence-detection settings.
    #[serde(default)]
    pub divergence_detection: DivergenceDetectionConfig,
    /// Regime-detection settings.
    #[serde(default)]
    pub regime_detection: RegimeDetectionConfig,
    /// Scenario-engine settings.
    #[serde(default)]
    pub scenario_engine: ScenarioEngineConfig,
    /// Signal-engine settings.
    #[serde(default)]
    pub signal_engine: SignalEngineConfig,
    /// Data-quality settings.
    #[serde(default)]
    pub data_quality: DataQualityConfig,
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

/// Market-structure analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketStructureAnalysis {
    /// Current structure state.
    pub state: String,
    /// Latest swing points.
    pub swing_points: SwingPoints,
    /// Recent structure sequence.
    pub structure_sequence: Vec<String>,
    /// Break-of-structure result.
    pub break_of_structure: StructureBreak,
    /// Change-of-character result.
    pub change_of_character: StructureBreak,
}

/// Latest swing points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwingPoints {
    /// Latest confirmed swing high.
    pub last_swing_high: Option<LevelPoint>,
    /// Latest confirmed swing low.
    pub last_swing_low: Option<LevelPoint>,
}

/// Timestamped price level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelPoint {
    /// Price.
    pub price: f64,
    /// Candle timestamp.
    pub date: String,
}

/// Structure break result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructureBreak {
    /// Whether a break was detected.
    pub detected: bool,
    /// Break direction.
    pub direction: Option<String>,
    /// Broken level.
    pub level: Option<f64>,
}

/// Support/resistance analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportResistanceAnalysis {
    /// Support zones.
    pub supports: Vec<PriceZone>,
    /// Resistance zones.
    pub resistances: Vec<PriceZone>,
}

/// Price zone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceZone {
    /// Zone bounds.
    pub zone: ZoneBounds,
    /// Normalized strength score.
    pub strength: f64,
    /// Number of touches.
    pub touches: usize,
}

/// Zone bounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneBounds {
    /// Lower bound.
    pub low: f64,
    /// Upper bound.
    pub high: f64,
}

/// Breakout analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakoutAnalysis {
    /// Breakout status.
    pub status: String,
    /// Relevant resistance level.
    pub resistance_level: Option<f64>,
    /// Breakout direction.
    pub direction: Option<String>,
    /// Distance to the relevant level in percent.
    pub distance_percent: Option<f64>,
    /// Whether volume confirms the breakout.
    pub volume_confirmation: bool,
    /// Retest result.
    pub retest: RetestAnalysis,
}

/// Retest analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetestAnalysis {
    /// Whether a retest was detected.
    pub detected: bool,
}

/// Detected chart or candlestick pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern name.
    pub name: String,
    /// Pattern category.
    #[serde(rename = "type")]
    pub pattern_type: String,
    /// Pattern lifecycle status.
    pub status: String,
    /// Detection confidence score.
    pub confidence: f64,
}

/// Detected divergence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Divergence {
    /// Indicator name.
    pub indicator: String,
    /// Divergence direction.
    pub direction: String,
    /// Detection confidence score.
    pub confidence: f64,
}

/// Market regime analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegimeAnalysis {
    /// Trend regime.
    pub trend: String,
    /// Momentum regime.
    pub momentum: String,
    /// Volatility regime.
    pub volatility: String,
    /// Volume regime.
    pub volume: String,
    /// Overall regime.
    pub overall: String,
}

/// Signal emitted by the interpretation engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signal {
    /// Stable signal identifier.
    pub id: String,
    /// Signal direction.
    pub direction: String,
    /// Signal category.
    pub category: String,
    /// Signal strength.
    pub strength: String,
    /// Human-readable evidence.
    pub evidence: Vec<String>,
}

/// Conflicting evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    /// Stable conflict type.
    #[serde(rename = "type")]
    pub conflict_type: String,
    /// Human-readable description.
    pub description: String,
}

/// Scenario definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario state.
    pub status: String,
    /// Optional trigger.
    pub trigger: Option<ScenarioCondition>,
    /// Confirmation conditions.
    pub confirmation: Vec<String>,
    /// Optional invalidation.
    pub invalidation: Option<ScenarioCondition>,
}

/// Scenario condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioCondition {
    /// Human-readable condition.
    pub condition: String,
}

/// Range scenario.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeScenario {
    /// Scenario state.
    pub status: String,
    /// Upper boundary.
    pub upper_boundary: Option<f64>,
    /// Lower boundary.
    pub lower_boundary: Option<f64>,
    /// Range condition.
    pub condition: String,
}

/// All scenario outcomes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    /// Bullish scenario.
    pub bullish: Scenario,
    /// Bearish scenario.
    pub bearish: Scenario,
    /// Range scenario.
    pub range: RangeScenario,
}

/// Key levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyLevels {
    /// Immediate support.
    pub immediate_support: Option<f64>,
    /// Major support.
    pub major_support: Option<f64>,
    /// Immediate resistance.
    pub immediate_resistance: Option<f64>,
}

/// High-level engine summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineSummary {
    /// Dominant market state.
    pub dominant_state: String,
    /// Trend state.
    pub trend: String,
    /// Momentum state.
    pub momentum: String,
    /// Structure state.
    pub structure: String,
    /// Whether volume confirms the current structure.
    pub volume_confirmation: bool,
    /// Volatility state.
    pub volatility: String,
    /// Most important level.
    pub most_important_level: Option<f64>,
    /// Most important confirmation.
    pub most_important_confirmation: String,
    /// Main risk.
    pub main_risk: String,
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
    /// Market structure.
    pub market_structure: MarketStructureAnalysis,
    /// Support and resistance.
    pub support_resistance: SupportResistanceAnalysis,
    /// Breakout analysis.
    pub breakout: BreakoutAnalysis,
    /// Detected patterns.
    pub patterns: Vec<Pattern>,
    /// Detected divergences.
    pub divergences: Vec<Divergence>,
    /// Market regime.
    pub regime: RegimeAnalysis,
    /// Signals.
    pub signals: Vec<Signal>,
    /// Conflicting evidence.
    pub conflicts: Vec<Conflict>,
    /// Scenario analysis.
    pub scenarios: ScenarioAnalysis,
    /// Key levels.
    pub key_levels: KeyLevels,
    /// Engine summary.
    pub engine_summary: EngineSummary,
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

fn default_swing_lookback() -> usize {
    5
}

fn default_support_resistance_lookback() -> usize {
    120
}

fn default_cluster_tolerance() -> f64 {
    1.0
}

fn default_minimum_touches() -> usize {
    2
}

fn default_volume_ratio() -> f64 {
    1.5
}

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
            issues.push(issue(
                "MISSING_TIMESTAMP",
                format!("Candle {index} has no timestamp"),
            ));
        }
        if ![
            candle.open,
            candle.high,
            candle.low,
            candle.close,
            candle.volume,
        ]
        .iter()
        .all(|value| value.is_finite())
        {
            issues.push(issue(
                "INVALID_NUMBER",
                format!("Candle {index} contains a non-finite value"),
            ));
        }
        if candle.open <= 0.0 || candle.high <= 0.0 || candle.low <= 0.0 || candle.close <= 0.0 {
            issues.push(issue(
                "INVALID_PRICE",
                format!("Candle {index} contains a non-positive price"),
            ));
        }
        if candle.high < candle.low
            || candle.high < candle.open
            || candle.high < candle.close
            || candle.low > candle.open
            || candle.low > candle.close
        {
            issues.push(issue(
                "INVALID_OHLC",
                format!("Candle {index} violates OHLC bounds"),
            ));
        }
        if candle.volume < 0.0 {
            issues.push(issue(
                "INVALID_VOLUME",
                format!("Candle {index} has negative volume"),
            ));
        } else if candle.volume == 0.0 {
            issues.push(issue(
                "ZERO_VOLUME",
                format!("Candle {index} has zero volume"),
            ));
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

    let candles_used = candles.len();
    let has_fatal_issue = issues
        .iter()
        .any(|item| item.code != "MAXIMUM_CANDLES_EXCEEDED");
    let sufficient_for_analysis = candles_used >= requirements.minimum_candles && !has_fatal_issue;
    let coverage = Coverage {
        from: candles.first().map(|candle| candle.timestamp.clone()),
        to: candles.last().map(|candle| candle.timestamp.clone()),
    };

    Ok((
        candles,
        DataQualityReport {
            candles_received: received,
            candles_used,
            minimum_required: requirements.minimum_candles,
            sufficient_for_analysis,
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

pub type BollingerBands = (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>);

/// Calculates Bollinger bands from a close-price series.
pub fn bollinger(values: &[f64], period: usize, stddev: f64) -> Result<BollingerBands, String> {
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
        return Err(
            "Market data does not satisfy the configured analysis requirements".to_string(),
        );
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
        state: rsi_current.map_or_else(
            || "unavailable".to_string(),
            |value| {
                if value >= 50.0 {
                    "positive"
                } else {
                    "negative"
                }
                .to_string()
            },
        ),
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
        signal_line
            .get(signal_line.len() - 2)
            .and_then(|value| *value)
            .map(|signal| macd_line[macd_line.len() - 2] - signal)
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
                    &candles
                        .iter()
                        .map(|candle| candle.volume)
                        .collect::<Vec<_>>(),
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
    let obv_current = obv_values
        .as_ref()
        .and_then(|values| values.last().copied());
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

    let market_structure = market_structure_analysis(
        &candles,
        &config.market_structure,
        &config.breakout_detection,
    );
    let support_resistance = support_resistance_analysis(&candles, &config.market_structure);
    let breakout = breakout_analysis(
        latest,
        &support_resistance,
        &volume,
        &config.breakout_detection,
    );
    let patterns = if config.pattern_detection.enabled {
        pattern_analysis(&market_structure)
    } else {
        Vec::new()
    };
    let divergences = if config.divergence_detection.enabled {
        divergence_analysis(&candles, &rsi_values, &market_structure)
    } else {
        Vec::new()
    };
    let regime = if config.regime_detection.enabled {
        regime_analysis(&trend, &momentum, &volatility, &volume)
    } else {
        RegimeAnalysis {
            trend: "disabled".to_string(),
            momentum: "disabled".to_string(),
            volatility: "disabled".to_string(),
            volume: "disabled".to_string(),
            overall: "disabled".to_string(),
        }
    };
    let signals = if config.signal_engine.enabled {
        signal_analysis(&trend, &momentum, &volume)
    } else {
        Vec::new()
    };
    let conflicts = conflict_analysis(&trend, &volume);
    let scenarios = if config.scenario_engine.enabled {
        scenario_analysis(&support_resistance, &breakout, &momentum)
    } else {
        ScenarioAnalysis {
            bullish: Scenario {
                status: "disabled".to_string(),
                trigger: None,
                confirmation: Vec::new(),
                invalidation: None,
            },
            bearish: Scenario {
                status: "disabled".to_string(),
                trigger: None,
                confirmation: Vec::new(),
                invalidation: None,
            },
            range: RangeScenario {
                status: "disabled".to_string(),
                upper_boundary: None,
                lower_boundary: None,
                condition: "Scenario engine disabled.".to_string(),
            },
        }
    };
    let key_levels = key_levels(&support_resistance);
    let engine_summary = engine_summary(EngineSummaryInput {
        trend: &trend,
        momentum: &momentum,
        structure: &market_structure,
        volume: &volume,
        regime: &regime,
        breakout: &breakout,
        levels: &key_levels,
        conflicts: &conflicts,
    });

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
        market_structure,
        support_resistance,
        breakout,
        patterns,
        divergences,
        regime,
        signals,
        conflicts,
        scenarios,
        key_levels,
        engine_summary,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum SwingKind {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Swing {
    index: usize,
    price: f64,
    kind: SwingKind,
}

fn detect_swings(candles: &[Candle], lookback: usize) -> Vec<Swing> {
    if lookback == 0 || candles.len() < lookback * 2 + 1 {
        return Vec::new();
    }

    let mut swings = Vec::new();
    for index in lookback..candles.len() - lookback {
        let candle = &candles[index];
        let left = &candles[index - lookback..index];
        let right = &candles[index + 1..=index + lookback];
        if left.iter().all(|item| candle.high > item.high)
            && right.iter().all(|item| candle.high >= item.high)
        {
            swings.push(Swing {
                index,
                price: candle.high,
                kind: SwingKind::High,
            });
        }
        if left.iter().all(|item| candle.low < item.low)
            && right.iter().all(|item| candle.low <= item.low)
        {
            swings.push(Swing {
                index,
                price: candle.low,
                kind: SwingKind::Low,
            });
        }
    }
    swings.sort_by_key(|swing| swing.index);
    swings
}

fn market_structure_analysis(
    candles: &[Candle],
    config: &MarketStructureConfig,
    breakout_config: &BreakoutConfig,
) -> MarketStructureAnalysis {
    let swings = if config.enabled {
        detect_swings(candles, config.swing_detection.lookback)
    } else {
        Vec::new()
    };

    let highs: Vec<Swing> = swings
        .iter()
        .copied()
        .filter(|swing| swing.kind == SwingKind::High)
        .collect();
    let lows: Vec<Swing> = swings
        .iter()
        .copied()
        .filter(|swing| swing.kind == SwingKind::Low)
        .collect();

    let mut sequence = Vec::new();
    let mut previous_high = None;
    let mut previous_low = None;
    for swing in &swings {
        match swing.kind {
            SwingKind::High => {
                if let Some(previous) = previous_high {
                    sequence.push(if swing.price > previous { "HH" } else { "LH" }.to_string());
                }
                previous_high = Some(swing.price);
            }
            SwingKind::Low => {
                if let Some(previous) = previous_low {
                    sequence.push(if swing.price > previous { "HL" } else { "LL" }.to_string());
                }
                previous_low = Some(swing.price);
            }
        }
    }
    if sequence.len() > 4 {
        sequence = sequence[sequence.len() - 4..].to_vec();
    }

    let state = match sequence.as_slice() {
        sequence
            if sequence.iter().any(|value| value == "HH")
                && sequence.iter().any(|value| value == "HL")
                && !sequence.iter().any(|value| value == "LL") =>
        {
            "higher_high_higher_low".to_string()
        }
        sequence
            if sequence.iter().any(|value| value == "LH")
                && sequence.iter().any(|value| value == "LL")
                && !sequence.iter().any(|value| value == "HH") =>
        {
            "lower_high_lower_low".to_string()
        }
        _ => "range".to_string(),
    };

    let latest = candles.last().map(|candle| candle.close);
    let last_high = highs.last().copied();
    let last_low = lows.last().copied();
    let bullish_break = latest
        .zip(last_high)
        .is_some_and(|(price, swing)| price > swing.price);
    let bearish_break = latest
        .zip(last_low)
        .is_some_and(|(price, swing)| price < swing.price);
    let bos = if breakout_config.enabled && bullish_break {
        StructureBreak {
            detected: true,
            direction: Some("up".to_string()),
            level: last_high.map(|swing| swing.price),
        }
    } else if breakout_config.enabled && bearish_break {
        StructureBreak {
            detected: true,
            direction: Some("down".to_string()),
            level: last_low.map(|swing| swing.price),
        }
    } else {
        StructureBreak {
            detected: false,
            direction: None,
            level: None,
        }
    };

    MarketStructureAnalysis {
        state,
        swing_points: SwingPoints {
            last_swing_high: last_high.map(|swing| LevelPoint {
                price: swing.price,
                date: candles[swing.index].timestamp.clone(),
            }),
            last_swing_low: last_low.map(|swing| LevelPoint {
                price: swing.price,
                date: candles[swing.index].timestamp.clone(),
            }),
        },
        structure_sequence: sequence,
        break_of_structure: bos,
        change_of_character: StructureBreak {
            detected: false,
            direction: None,
            level: None,
        },
    }
}

fn support_resistance_analysis(
    candles: &[Candle],
    config: &MarketStructureConfig,
) -> SupportResistanceAnalysis {
    if !config.enabled || candles.is_empty() {
        return SupportResistanceAnalysis {
            supports: Vec::new(),
            resistances: Vec::new(),
        };
    }

    let lookback = config.support_resistance.lookback.min(candles.len());
    let start = candles.len() - lookback;
    let recent = &candles[start..];
    let swings = detect_swings(recent, config.swing_detection.lookback);
    let tolerance = config.support_resistance.cluster_tolerance_percent / 100.0;
    let mut supports = Vec::new();
    let mut resistances = Vec::new();

    for kind in [SwingKind::Low, SwingKind::High] {
        let mut levels: Vec<f64> = swings
            .iter()
            .filter(|swing| swing.kind == kind)
            .map(|swing| swing.price)
            .collect();
        levels.sort_by(|a, b| a.total_cmp(b));

        let mut clusters: Vec<Vec<f64>> = Vec::new();
        for level in levels {
            if let Some(cluster) = clusters.last_mut() {
                let reference = cluster.iter().sum::<f64>() / cluster.len() as f64;
                if reference != 0.0 && (level - reference).abs() / reference <= tolerance {
                    cluster.push(level);
                    continue;
                }
            }
            clusters.push(vec![level]);
        }

        for cluster in clusters {
            if cluster.len() < config.support_resistance.minimum_touches {
                continue;
            }
            let low = cluster.iter().copied().fold(f64::INFINITY, f64::min);
            let high = cluster.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let strength = (cluster.len() as f64 / 5.0).min(1.0);
            let zone = PriceZone {
                zone: ZoneBounds { low, high },
                strength,
                touches: cluster.len(),
            };
            if kind == SwingKind::Low {
                supports.push(zone);
            } else {
                resistances.push(zone);
            }
        }
    }

    supports.sort_by(|a, b| b.zone.high.total_cmp(&a.zone.high));
    resistances.sort_by(|a, b| a.zone.low.total_cmp(&b.zone.low));
    SupportResistanceAnalysis {
        supports,
        resistances,
    }
}

fn breakout_analysis(
    latest: &Candle,
    levels: &SupportResistanceAnalysis,
    volume: &VolumeAnalysis,
    config: &BreakoutConfig,
) -> BreakoutAnalysis {
    let resistance = levels
        .resistances
        .iter()
        .find(|level| level.zone.low > latest.close)
        .map(|level| level.zone.low);
    let Some(resistance_level) = resistance else {
        return BreakoutAnalysis {
            status: "none".to_string(),
            resistance_level: None,
            direction: None,
            distance_percent: None,
            volume_confirmation: false,
            retest: RetestAnalysis { detected: false },
        };
    };

    let distance_percent = if resistance_level == 0.0 {
        None
    } else {
        Some((resistance_level - latest.close) / resistance_level * 100.0)
    };
    let ratio = volume.ratio_vs_primary.unwrap_or(0.0);
    let confirmed = config.volume_confirmation.enabled
        && ratio >= config.volume_confirmation.minimum_volume_ratio;
    let status = if latest.close > resistance_level {
        "broken"
    } else if distance_percent.is_some_and(|distance| distance <= 2.0) {
        "approaching"
    } else {
        "none"
    };

    BreakoutAnalysis {
        status: status.to_string(),
        resistance_level: Some(resistance_level),
        direction: Some("up".to_string()),
        distance_percent,
        volume_confirmation: confirmed,
        retest: RetestAnalysis { detected: false },
    }
}

fn pattern_analysis(structure: &MarketStructureAnalysis) -> Vec<Pattern> {
    if structure.state == "higher_high_higher_low" && structure.structure_sequence.len() >= 3 {
        vec![Pattern {
            name: "ascending_structure".to_string(),
            pattern_type: "chart_pattern".to_string(),
            status: "forming".to_string(),
            confidence: 0.72,
        }]
    } else {
        Vec::new()
    }
}

fn divergence_analysis(
    candles: &[Candle],
    rsi_values: &[Option<f64>],
    structure: &MarketStructureAnalysis,
) -> Vec<Divergence> {
    let _ = candles;
    let _ = rsi_values;
    let _ = structure;
    Vec::new()
}

fn regime_analysis(
    trend: &TrendAnalysis,
    momentum: &MomentumAnalysis,
    volatility: &VolatilityAnalysis,
    volume: &VolumeAnalysis,
) -> RegimeAnalysis {
    let momentum_state = momentum.rsi.state.clone();
    let volume_state = if volume.state == "above_average" {
        "expanding".to_string()
    } else {
        "neutral".to_string()
    };
    let overall = if trend.state == "bullish" && momentum_state == "positive" {
        "bullish".to_string()
    } else if trend.state == "bearish" && momentum_state == "negative" {
        "bearish".to_string()
    } else {
        "range".to_string()
    };
    RegimeAnalysis {
        trend: trend.state.clone(),
        momentum: momentum_state,
        volatility: volatility.state.clone(),
        volume: volume_state,
        overall,
    }
}

fn signal_analysis(
    trend: &TrendAnalysis,
    momentum: &MomentumAnalysis,
    volume: &VolumeAnalysis,
) -> Vec<Signal> {
    let mut signals = Vec::new();
    if trend.state == "bullish" {
        let mut evidence = Vec::new();
        if trend.alignment.bullish {
            evidence.push("Price above EMA20".to_string());
            evidence.push("EMA20 above EMA50".to_string());
            evidence.push("EMA50 above EMA200".to_string());
        }
        signals.push(Signal {
            id: "SIG001".to_string(),
            direction: "bullish".to_string(),
            category: "trend".to_string(),
            strength: if evidence.len() >= 3 {
                "strong".to_string()
            } else {
                "moderate".to_string()
            },
            evidence,
        });
    }

    if momentum.rsi.above_50 || momentum.macd.state == "bullish" {
        let mut evidence = Vec::new();
        if momentum.rsi.above_50 {
            evidence.push("RSI above 50".to_string());
        }
        if momentum.macd.state == "bullish" {
            evidence.push("MACD bullish".to_string());
        }
        if momentum.macd.histogram_direction == "increasing" {
            evidence.push("MACD histogram increasing".to_string());
        }
        signals.push(Signal {
            id: "SIG002".to_string(),
            direction: "bullish".to_string(),
            category: "momentum".to_string(),
            strength: "moderate".to_string(),
            evidence,
        });
    }

    if volume.state == "below_average" {
        signals.push(Signal {
            id: "SIG003".to_string(),
            direction: "bearish".to_string(),
            category: "volume".to_string(),
            strength: "moderate".to_string(),
            evidence: vec!["Volume below primary moving average".to_string()],
        });
    }
    signals
}

fn conflict_analysis(trend: &TrendAnalysis, volume: &VolumeAnalysis) -> Vec<Conflict> {
    if trend.state == "bullish" && volume.state == "below_average" {
        vec![Conflict {
            conflict_type: "volume_price_mismatch".to_string(),
            description: "Price structure is bullish but volume has not expanded.".to_string(),
        }]
    } else {
        Vec::new()
    }
}

fn scenario_analysis(
    levels: &SupportResistanceAnalysis,
    breakout: &BreakoutAnalysis,
    momentum: &MomentumAnalysis,
) -> ScenarioAnalysis {
    let resistance = breakout.resistance_level;
    let support = levels.supports.first().map(|level| level.zone.low);
    let confirmation = vec![
        "Volume expands above the configured confirmation ratio".to_string(),
        "Price holds above the breakout level".to_string(),
        "RSI remains above 50".to_string(),
    ];
    ScenarioAnalysis {
        bullish: Scenario {
            status: "possible".to_string(),
            trigger: resistance.map(|level| ScenarioCondition {
                condition: format!("Daily close above {level:.2}"),
            }),
            confirmation,
            invalidation: support.map(|level| ScenarioCondition {
                condition: format!("Daily close below {level:.2}"),
            }),
        },
        bearish: Scenario {
            status: "possible".to_string(),
            trigger: support.map(|level| ScenarioCondition {
                condition: format!("Daily close below {level:.2}"),
            }),
            confirmation: vec![
                "Volume expansion".to_string(),
                "RSI below 50".to_string(),
                "Break of the recent higher low".to_string(),
            ],
            invalidation: resistance.map(|level| ScenarioCondition {
                condition: format!("Price reclaims {level:.2}"),
            }),
        },
        range: RangeScenario {
            status: "possible".to_string(),
            upper_boundary: resistance,
            lower_boundary: support,
            condition: if momentum.rsi.state == "positive" {
                "Price remains inside the established range while momentum stays positive."
                    .to_string()
            } else {
                "Price remains inside the established range.".to_string()
            },
        },
    }
}

fn key_levels(levels: &SupportResistanceAnalysis) -> KeyLevels {
    KeyLevels {
        immediate_support: levels.supports.first().map(|level| level.zone.low),
        major_support: levels.supports.get(1).map(|level| level.zone.low),
        immediate_resistance: levels.resistances.first().map(|level| level.zone.high),
    }
}

struct EngineSummaryInput<'a> {
    trend: &'a TrendAnalysis,
    momentum: &'a MomentumAnalysis,
    structure: &'a MarketStructureAnalysis,
    volume: &'a VolumeAnalysis,
    regime: &'a RegimeAnalysis,
    breakout: &'a BreakoutAnalysis,
    levels: &'a KeyLevels,
    conflicts: &'a [Conflict],
}

fn engine_summary(input: EngineSummaryInput<'_>) -> EngineSummary {
    let volume_confirmation = input
        .volume
        .ratio_vs_primary
        .is_some_and(|ratio| ratio >= 1.5);
    let dominant_state = if input.trend.state == "bullish" && !volume_confirmation {
        "bullish_but_unconfirmed"
    } else {
        input.regime.overall.as_str()
    };
    let most_important_level = input
        .breakout
        .resistance_level
        .or(input.levels.immediate_support);
    let main_risk = input
        .conflicts
        .first()
        .map(|conflict| conflict.description.clone())
        .unwrap_or_else(|| "No dominant conflict detected.".to_string());

    EngineSummary {
        dominant_state: dominant_state.to_string(),
        trend: input.trend.state.clone(),
        momentum: input.momentum.rsi.state.clone(),
        structure: input.structure.state.clone(),
        volume_confirmation,
        volatility: "normal".to_string(),
        most_important_level,
        most_important_confirmation: input
            .breakout
            .resistance_level
            .map(|level| format!("Break above {level:.2} with volume expansion"))
            .unwrap_or_else(|| "Wait for a confirmed support/resistance level.".to_string()),
        main_risk,
    }
}

fn stochastic_d(values: &[Option<f64>], period: usize) -> Result<Vec<Option<f64>>, String> {
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
        let (_, quality) =
            validate_input(&input, &requirements).expect("validation should return a report");
        assert!(!quality.issues.is_empty());
        assert!(!quality.sufficient_for_analysis);
    }
}
