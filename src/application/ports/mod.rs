use std::rc::Rc;

use crate::domain::document::MarkdownDocument;
use crate::domain::futures::FuturesTickerUpdate;

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
