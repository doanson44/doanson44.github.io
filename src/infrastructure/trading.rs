use crate::application::ports::TradingStorage;
use crate::domain::trading::TradingSnapshot;
use crate::infrastructure::browser;

/// Browser localStorage adapter for paper-trading state.
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalTradingStorage;

const STORAGE_KEY: &str = "socket.paper-trading.v1";

impl TradingStorage for LocalTradingStorage {
    fn load(&self) -> Result<Option<TradingSnapshot>, String> {
        let Some(raw) = browser::storage_get(STORAGE_KEY) else {
            return Ok(None);
        };

        serde_json::from_str(&raw)
            .map(Some)
            .map_err(|_| "Saved paper-trading data is invalid.".to_string())
    }

    fn save(&self, snapshot: &TradingSnapshot) -> Result<(), String> {
        let raw = serde_json::to_string(snapshot)
            .map_err(|_| "Failed to serialize paper-trading data.".to_string())?;
        browser::storage_set(STORAGE_KEY, &raw)
            .map_err(|_| "Failed to save paper-trading data.".to_string())
    }
}
