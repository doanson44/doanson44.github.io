use serde_json::{Number, Value};

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

/// A fixed value applied to every generated record at the supplied dot-separated path.
#[derive(Clone, Debug, PartialEq)]
pub struct JsonOverride {
    pub path: String,
    pub value: Value,
}

/// Options for deterministic dummy JSON generation.
#[derive(Clone, Debug, PartialEq)]
pub struct JsonGenerateOptions {
    pub count: usize,
    pub seed: u64,
    pub overrides: Vec<JsonOverride>,
}

/// Generate dummy JSON from a JSON-shaped template.
///
/// In generator mode, the placeholders `"string"`, `0`, `true`, and `false`
/// are treated as type hints. Other literal values are preserved. A root object,
/// array, or scalar becomes an array containing `count` generated records/values.
pub fn generate_dummy_json(source: &str, options: &JsonGenerateOptions) -> Result<String, String> {
    let template: Value = serde_json::from_str(source).map_err(format_json_error)?;
    let count = options.count.clamp(1, 1000);
    let mut rng = DeterministicRng::new(options.seed);

    let item_template = match &template {
        Value::Array(items) => items
            .first()
            .ok_or_else(|| "Generator requires a non-empty array template.".to_string())?,
        value => value,
    };

    let mut records = Vec::with_capacity(count);
    for index in 0..count {
        let mut record = generate_value(item_template, &mut rng, index + 1);
        for override_value in &options.overrides {
            apply_override(&mut record, &override_value.path, &override_value.value)?;
        }
        records.push(record);
    }

    serde_json::to_string_pretty(&Value::Array(records)).map_err(format_json_error)
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

fn generate_value(value: &Value, rng: &mut DeterministicRng, index: usize) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), generate_value(value, rng, index)))
                .collect(),
        ),
        Value::Array(items) => {
            if let Some(item) = items.first() {
                Value::Array(vec![generate_value(item, rng, index)])
            } else {
                Value::Array(Vec::new())
            }
        }
        Value::String(value) if value == "string" => Value::String(format!("string-{index:03}")),
        Value::Number(number) if number.is_i64() || number.is_u64() => {
            Value::Number(Number::from(rng.range_u64(1, 1000)))
        }
        Value::Number(_) => Value::Number(
            Number::from_f64(rng.range_f64(1.0, 1000.0)).unwrap_or_else(|| Number::from(1)),
        ),
        Value::Bool(_) => Value::Bool(rng.next_bool()),
        value => value.clone(),
    }
}

fn apply_override(target: &mut Value, path: &str, override_value: &Value) -> Result<(), String> {
    let normalized = path.trim().trim_start_matches('$').trim_start_matches('.');
    if normalized.is_empty() {
        *target = override_value.clone();
        return Ok(());
    }

    let segments = normalized
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return Err("Fixed value path cannot be empty.".to_string());
    }

    let mut current = target;
    for segment in &segments[..segments.len() - 1] {
        match current {
            Value::Object(object) => {
                current = object
                    .get_mut(*segment)
                    .ok_or_else(|| format!("Fixed value path not found: $.{normalized}"))?;
            }
            _ => {
                return Err(format!(
                    "Fixed value path is not an object path: $.{normalized}"
                ));
            }
        }
    }

    match current {
        Value::Object(object) => {
            let key = segments[segments.len() - 1];
            if !object.contains_key(key) {
                return Err(format!("Fixed value path not found: $.{normalized}"));
            }
            object.insert(key.to_string(), override_value.clone());
            Ok(())
        }
        _ => Err(format!(
            "Fixed value path is not an object path: $.{normalized}"
        )),
    }
}

struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    fn range_u64(&mut self, min: u64, max: u64) -> u64 {
        min + self.next_u64() % (max - min + 1)
    }

    fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        let ratio = self.next_u64() as f64 / u64::MAX as f64;
        min + (max - min) * ratio
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
        assert!(result.contains("\n  \"items\": ["));
        assert!(result.contains("\n    1,"));
        assert!(result.contains("\n  \"name\": \"Son\""));
    }

    #[test]
    fn minifies_valid_json() {
        let result = minify_json(r#"{ "name": "Son", "active": true }"#).unwrap();
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
    fn generates_records_and_applies_fixed_values() {
        let source = r#"{
  "pli": "string",
  "name": "string",
  "extId": "string",
  "unitsPerCase": 0,
  "gratisBeerVisibility": true,
  "volume": 0,
  "unitCost": 0
}"#;

        let result = generate_dummy_json(
            source,
            &JsonGenerateOptions {
                count: 3,
                seed: 42,
                overrides: vec![JsonOverride {
                    path: "$.extId".to_string(),
                    value: Value::String("123".to_string()),
                }],
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&result).unwrap();
        let records = value.as_array().unwrap();
        assert_eq!(records.len(), 3);
        assert!(records.iter().all(|record| record["extId"] == "123"));
        assert_ne!(records[0]["name"], records[1]["name"]);
    }

    #[test]
    fn generation_is_deterministic_for_same_seed() {
        let options = JsonGenerateOptions {
            count: 5,
            seed: 12345,
            overrides: Vec::new(),
        };

        assert_eq!(
            generate_dummy_json(r#"{"value":0,"active":true}"#, &options).unwrap(),
            generate_dummy_json(r#"{"value":0,"active":true}"#, &options).unwrap()
        );
    }

    #[test]
    fn rejects_missing_override_path() {
        let result = generate_dummy_json(
            r#"{"extId":"string"}"#,
            &JsonGenerateOptions {
                count: 1,
                seed: 1,
                overrides: vec![JsonOverride {
                    path: "$.missing".to_string(),
                    value: Value::String("123".to_string()),
                }],
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_json() {
        let result = format_json("{\"name\":}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON"));
    }
}
