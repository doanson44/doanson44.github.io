use crate::domain::json::{format_json, minify_json, sample_json};

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

    /// Create a response sample by limiting every JSON array to the requested size.
    pub fn sample(source: &str, max_items: usize) -> Result<String, String> {
        sample_json(source, max_items)
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
    fn service_samples_json() {
        let result = JsonService::sample(r#"{"items":[1,2,3]}"#, 2).unwrap();
        assert!(result.contains("\"items\": ["));
        assert!(result.contains("1,"));
        assert!(result.contains("2"));
        assert!(!result.contains("3"));
    }

    #[test]
    fn service_minifies_json() {
        assert_eq!(
            JsonService::minify("{ \"ok\": true }").unwrap(),
            "{\"ok\":true}"
        );
    }
}
