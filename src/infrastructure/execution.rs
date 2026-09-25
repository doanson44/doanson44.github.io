use crate::application::ports::RealTradingStorage;
use crate::domain::trading::ExecutionSettings;
use crate::infrastructure::browser;

const STORAGE_KEY: &str = "socket.execution-settings.v1";

/// Browser localStorage adapter for execution mode and client-side real-trading settings.
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalExecutionStorage;

impl RealTradingStorage for LocalExecutionStorage {
    fn load(&self) -> Result<Option<ExecutionSettings>, String> {
        let Some(raw) = browser::storage_get(STORAGE_KEY) else {
            return Ok(None);
        };

        serde_json::from_str(&raw)
            .map(Some)
            .map_err(|_| "Saved execution settings are invalid.".to_string())
    }

    fn save(&self, settings: &ExecutionSettings) -> Result<(), String> {
        let raw = serde_json::to_string(settings)
            .map_err(|_| "Failed to serialize execution settings.".to_string())?;
        browser::storage_set(STORAGE_KEY, &raw)
            .map_err(|_| "Failed to save execution settings.".to_string())
    }
}
