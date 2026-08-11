use crate::domain::json::{format_json, minify_json};

/// Application service coordinating JSON formatting use cases.
pub struct JsonService;

impl JsonService {
    /// Format valid JSON with readable indentation.
    pub fn format(source: &str) -> Result<String, String> {
        format_json(source)
    }

    /// Minify valid JSON by removing insignificant whitespace.
    pub fn minify(source: &str) -> Result<String, String> {
        minify_json(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_formats_json() {
        let result = JsonService::format("{\"name\":\"Son\"}").unwrap();
        assert!(result.contains("\"name\": \"Son\""));
    }

    #[test]
    fn service_minifies_json() {
        assert_eq!(JsonService::minify("{ \"ok\": true }").unwrap(), "{\"ok\":true}");
    }
}
