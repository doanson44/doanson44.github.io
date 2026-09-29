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
    pub fn generate(source: &str, options: &JsonGenerateOptions) -> Result<String, String> {
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
        let result = JsonService::format("{\"name\":\"Son\"}");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("\"name\": \"Son\""));
    }

    #[test]
    fn service_samples_json() {
        let result = JsonService::sample(r#"{"items":[1,2,3]}"#, 2);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("\"items\": ["));
        assert!(output.contains("1,"));
        assert!(output.contains("2"));
        assert!(!output.contains("3"));
    }

    #[test]
    fn service_minifies_json() {
        let result = JsonService::minify(r#"{ "ok": true }"#);
        assert_eq!(result.unwrap(), r#"{"ok":true}"#);
    }

    #[test]
    fn service_generates_fixed_values() {
        let options = JsonGenerateOptions {
            count: 2,
            seed: 7,
            overrides: vec![JsonOverride {
                path: "$.extId".to_string(),
                value: Value::String("123".to_string()),
            }],
        };

        let result = JsonService::generate(r#"{"extId":"string"}"#, &options);
        assert!(result.is_ok());

        let value: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let records = value.as_array().unwrap();

        assert_eq!(records.len(), 2);

        for record in records {
            assert_eq!(record["extId"], "123");
        }
    }
}
