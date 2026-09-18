use crate::domain::document::MarkdownDocument;

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
