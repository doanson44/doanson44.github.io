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

fn format_json_error(error: serde_json::Error) -> String {
    format!("Invalid JSON at line {}, column {}: {}", error.line(), error.column(), error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_valid_json() {
        let result = format_json(r#"{"name":"Son","items":[1,2]}"#).unwrap();
        assert!(result.contains("\n  \"name\": \"Son\","));
        assert!(result.contains("\n    1,"));
    }

    #[test]
    fn minifies_valid_json() {
        let result = minify_json("{ \"name\": \"Son\", \"active\": true }").unwrap();
        assert_eq!(result, r#"{"name":"Son","active":true}"#);
    }

    #[test]
    fn rejects_invalid_json() {
        let result = format_json("{\"name\":}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON"));
    }
}
