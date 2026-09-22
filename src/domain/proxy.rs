use serde_json::Value;

/// Controls how a JSON response is reduced for display.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseMinimizationOptions {
    /// Maximum number of elements kept in every JSON array.
    pub max_array_items: Option<usize>,
    /// Maximum number of Unicode scalar values kept in every JSON string.
    pub max_string_chars: Option<usize>,
    /// Whether the resulting JSON should be serialized without indentation.
    pub compact: bool,
}

/// Minimizes a JSON response without changing the JSON value types.
pub fn minimize_json_response(
    source: &str,
    options: &ResponseMinimizationOptions,
) -> Result<String, String> {
    let mut value: Value = serde_json::from_str(source).map_err(format_json_error)?;
    minimize_value(&mut value, options);

    if options.compact {
        serde_json::to_string(&value).map_err(format_json_error)
    } else {
        serde_json::to_string_pretty(&value).map_err(format_json_error)
    }
}

fn minimize_value(value: &mut Value, options: &ResponseMinimizationOptions) {
    match value {
        Value::Array(items) => {
            if let Some(limit) = options.max_array_items {
                items.truncate(limit);
            }
            for item in items {
                minimize_value(item, options);
            }
        }
        Value::Object(fields) => {
            for value in fields.values_mut() {
                minimize_value(value, options);
            }
        }
        Value::String(value) => {
            if let Some(limit) = options.max_string_chars {
                let truncated: String = value.chars().take(limit).collect();
                if truncated.len() < value.len() {
                    *value = truncated;
                }
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
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
    fn limits_array_items() {
        let options = ResponseMinimizationOptions {
            max_array_items: Some(2),
            max_string_chars: None,
            compact: true,
        };

        let result = minimize_json_response(r#"[{"id":1},{"id":2},{"id":3}]"#, &options).unwrap();

        assert_eq!(result, r#"[{"id":1},{"id":2}]"#);
    }

    #[test]
    fn limits_nested_arrays() {
        let options = ResponseMinimizationOptions {
            max_array_items: Some(1),
            max_string_chars: None,
            compact: true,
        };

        let result = minimize_json_response(r#"{"items":[[1,2],[3,4]]}"#, &options).unwrap();

        assert_eq!(result, r#"{"items":[[1]]}"#);
    }

    #[test]
    fn limits_string_length() {
        let options = ResponseMinimizationOptions {
            max_array_items: None,
            max_string_chars: Some(5),
            compact: true,
        };

        let result = minimize_json_response(r#"{"message":"abcdefgh"}"#, &options).unwrap();

        assert_eq!(result, r#"{"message":"abcde"}"#);
    }

    #[test]
    fn rejects_invalid_json() {
        let options = ResponseMinimizationOptions {
            max_array_items: Some(5),
            max_string_chars: Some(100),
            compact: false,
        };

        assert!(minimize_json_response("{", &options).is_err());
    }
}
