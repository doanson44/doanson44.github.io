use serde_json::Value;

/// Format valid JSON using two-space indentation.
pub fn format_json(source: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(source).map_err(format_json_error)?;
    serde_json::to_string_pretty(&value).map_err(format_json_error)
}

/// Minify valid JSON by removing insignificant whitespace.
pub fn minify_json(source: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(source).map_err(format_json_error)?;
    serde_json::to_string(&value).map_err(format_json_error)
}

/// Create a response sample by recursively limiting every JSON array to the first
/// `max_items` elements while preserving the complete object structure.
pub fn sample_json(source: &str, max_items: usize) -> Result<String, String> {
    let value: Value = serde_json::from_str(source).map_err(format_json_error)?;
    let sampled = sample_value(value, max_items);
    serde_json::to_string_pretty(&sampled).map_err(format_json_error)
}

fn sample_value(value: Value, max_items: usize) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .take(max_items)
                .map(|item| sample_value(item, max_items))
                .collect(),
        ),
        Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(key, value)| (key, sample_value(value, max_items)))
                .collect(),
        ),
        value => value,
    }
}

fn format_json_error(error: serde_json::Error) -> String {
    format!(
        "Invalid JSON at line {}, column {}: {}",
        error.line(),
        error.column(),
        error
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_valid_json() {
        let result = format_json(r#"{"name":"Son","items":[1,2]}"#).unwrap();
        // serde_json::Value sorts keys alphabetically by default, so "items" comes before "name"
        assert!(result.contains("\n  \"items\": ["));
        assert!(result.contains("\n    1,"));
        assert!(result.contains("\n  \"name\": \"Son\""));
    }

    #[test]
    fn minifies_valid_json() {
        let result = minify_json("{ \"name\": \"Son\", \"active\": true }").unwrap();
        // Alphabetical sort: active, then name
        assert_eq!(result, r#"{"active":true,"name":"Son"}"#);
    }

    #[test]
    fn samples_nested_arrays() {
        let source = r#"{
  "items": [1, 2, 3, 4, 5, 6],
  "nested": {
    "values": ["a", "b", "c"],
    "empty": []
  }
}"#;

        let result = sample_json(source, 2).unwrap();

        assert_eq!(
            result,
            r#"{
  "items": [
    1,
    2
  ],
  "nested": {
    "empty": [],
    "values": [
      "a",
      "b"
    ]
  }
}"#
        );
    }

    #[test]
    fn samples_root_array() {
        let result = sample_json(r#"[1, 2, 3, 4]"#, 3).unwrap();
        assert_eq!(result, "[\n  1,\n  2,\n  3\n]");
    }

    #[test]
    fn samples_with_zero_items() {
        let result = sample_json(r#"[1, 2]"#, 0).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn rejects_invalid_json() {
        let result = format_json("{\"name\":}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON"));
    }
}
