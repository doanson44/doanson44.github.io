use std::rc::Rc;

use crate::domain::document::MarkdownDocument;
use crate::domain::funding::FundingRateSnapshot;
use crate::domain::futures::FuturesTickerUpdate;
use crate::domain::trading::{ExecutionSettings, TradingSnapshot};

/// Port for document persistence.
pub trait DocumentRepository {
    fn save(&self, doc: &MarkdownDocument) -> Result<(), String>;
    fn load(&self, id: &str) -> Result<Option<MarkdownDocument>, String>;
    fn delete(&self, id: &str) -> Result<(), String>;
    fn list(&self) -> Result<Vec<(String, String)>, String>;
}

/// Port for browser-provided time and timezone formatting.
pub trait TimeProvider {
    fn now_ms(&self) -> f64;
    fn format_datetime(&self, millis: f64, timezone: &str) -> Result<String, String>;
    fn timestamp_to_datetime(
        &self,
        value: &str,
        unit: crate::domain::time::TimestampUnit,
        timezone: &str,
    ) -> Result<String, String>;
    fn datetime_to_timestamp(
        &self,
        value: &str,
        unit: crate::domain::time::TimestampUnit,
        timezone: &str,
    ) -> Result<String, String>;
}

/// Public connection states exposed by the Futures market stream.
#[derive(Debug, Clone, PartialEq)]
pub enum FuturesConnectionStatus {
    Connecting,
    Connected,
    Reconnecting,
    Disconnected,
    Error(String),
}

/// Handle for a Futures market stream lifecycle.
pub trait FuturesMarketStreamHandle {
    fn close(&mut self);
}

/// Application port for a public Futures market stream.
pub trait FuturesMarketStream {
    fn connect(
        &self,
        on_batch: Rc<dyn Fn(Vec<FuturesTickerUpdate>)>,
        on_status: Rc<dyn Fn(FuturesConnectionStatus)>,
    ) -> Result<Box<dyn FuturesMarketStreamHandle>, String>;
}

/// Application port for cached all-market funding rates.
pub trait FundingRateProvider {
    fn load_cached_or_fetch(&self, on_result: Rc<dyn Fn(Result<FundingRateSnapshot, String>)>);
}

/// Application port for a browser HTTP proxy client.
pub trait ProxyClient {
    /// Fetches a target URL through the configured proxy and returns the response text.
    fn fetch(&self, target_url: &str, on_result: Rc<dyn Fn(Result<String, String>)>);

    /// Fetches a target URL through the configured proxy with forwarded request headers.
    fn fetch_with_headers(
        &self,
        target_url: &str,
        headers: Vec<(String, String)>,
        on_result: Rc<dyn Fn(Result<String, String>)>,
    ) {
        let _ = headers;
        self.fetch(target_url, on_result);
    }
}

/// Application port for direct browser HTTP access to market data.
pub trait MarketClient {
    /// Fetches market data from the configured public endpoint.
    fn fetch(&self, on_result: Rc<dyn Fn(Result<String, String>)>);

    /// Fetches historical market data directly from a public endpoint.
    fn fetch_url(&self, target_url: &str, on_result: Rc<dyn Fn(Result<String, String>)>);
}

/// Application port for persisting market ticker pins in browser storage.
pub trait MarketPinStore {
    /// Loads the symbols currently pinned by the user.
    fn load(&self) -> Result<Vec<String>, String>;

    /// Persists the symbols currently pinned by the user.
    fn save(&self, symbols: &[String]) -> Result<(), String>;
}

/// Application port for persisting client-side execution settings.
pub trait RealTradingStorage {
    /// Loads the execution mode and real-trading settings.
    fn load(&self) -> Result<Option<ExecutionSettings>, String>;

    /// Persists the execution mode and real-trading settings.
    fn save(&self, settings: &ExecutionSettings) -> Result<(), String>;
}

/// Application port for persisting the client-side paper-trading snapshot.
pub trait TradingStorage {
    /// Loads the previously persisted paper-trading snapshot.
    fn load(&self) -> Result<Option<TradingSnapshot>, String>;

    /// Persists the current paper-trading snapshot.
    fn save(&self, snapshot: &TradingSnapshot) -> Result<(), String>;
}
