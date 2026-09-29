use crate::domain::json::{
    format_json, generate_dummy_json, minify_json, sample_json, JsonGenerateOptions,
};

/// Application service coordinating JSON formatting and dummy-data generation use cases.
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

    /// Generate deterministic dummy JSON from a template and apply fixed overrides.
    pub fn generate(
        source: &str,
        options: &JsonGenerateOptions,
    ) -> Result<String, String> {
        generate_dummy_json(source, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::json::JsonOverride;
    use serde_json::Value;

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
            JsonService::minify(r#"{ "ok": true }"#).unwrap(),
            r#"{"ok":true}"#
        );
    }

    #[test]
    fn service_generates_fixed_values() {
        let result = JsonService::generate(
            r#"{"extId":"string"}"#,
            &JsonGenerateOptions {
                count: 2,
                seed: 7,
                overrides: vec![JsonOverride {
                    path: "$.extId".to_string(),
                    value: Value::String("123".to_string()),
                }],
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&result).unwrap();
        let records = value.as_array().unwrap();
        assert_eq!(records.len(), 2);
        assert!(records.iter().all(|record| record["extId"] == "123"));
    }
}
