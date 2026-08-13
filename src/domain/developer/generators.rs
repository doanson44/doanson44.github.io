use qrcode::{render::svg, QrCode};
use serde_json::{Map, Value};

pub fn uuid() -> Result<String, String> {
    Ok(uuid::Uuid::new_v4().to_string())
}

pub fn qr(input: &str) -> Result<String, String> {
    if input.trim().is_empty() {
        return Err("Input is empty.".into());
    }
    let code =
        QrCode::new(input.as_bytes()).map_err(|e| format!("Unable to generate QR code: {e}"))?;
    Ok(code.render::<svg::Color>().min_dimensions(180, 180).build())
}

pub fn fake_data(input: &str) -> Result<String, String> {
    let mut count = 5usize;
    let mut types = vec!["uuid", "name", "email", "number"];
    for line in input.lines() {
        if let Some(v) = line.trim().strip_prefix("count=") {
            count = v.parse().map_err(|_| "count must be an integer.")?;
        }
        if let Some(v) = line.trim().strip_prefix("types=") {
            types = v
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    if count > 1000 {
        return Err("Maximum count is 1000.".into());
    }
    let mut out = String::new();
    out.push('[');
    for i in 0..count {
        if i > 0 {
            out.push(',');
        }
        out.push('{');
        for (j, t) in types.iter().enumerate() {
            if j > 0 {
                out.push(',');
            }
            let value = match *t {
                "uuid" => format!("\"{}\"", uuid::Uuid::new_v4()),
                "name" => format!("\"Developer {}\"", i + 1),
                "email" => format!("\"developer{}@example.com\"", i + 1),
                "number" => (i + 1).to_string(),
                "boolean" => ((i % 2) == 0).to_string(),
                _ => format!("\"value-{}\"", i + 1),
            };
            out.push_str(&format!("\"{}\":{}", t, value));
        }
        out.push('}');
    }
    out.push(']');
    serde_json::to_string_pretty(&serde_json::from_str::<Value>(&out).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

pub fn mock_json(source: &str, count: &str) -> Result<String, String> {
    let template: Value = serde_json::from_str(source).map_err(|e| format!("Invalid JSON: {e}"))?;
    let count: usize = count
        .trim()
        .parse()
        .map_err(|_| "Record count must be an integer.")?;
    if count > 1000 {
        return Err("Maximum count is 1000.".into());
    }
    let mut rows = Vec::with_capacity(count);
    for i in 0..count {
        rows.push(mock_value(&template, i));
    }
    serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())
}
fn mock_value(value: &Value, index: usize) -> Value {
    match value {
        Value::Number(n) => {
            if n.is_i64() {
                Value::from(index as i64 + 1)
            } else {
                Value::from(index as f64 + 1.0)
            }
        }
        Value::String(s) => Value::String(if s.contains('@') {
            format!("developer{}@example.com", index + 1)
        } else if s.is_empty() {
            String::new()
        } else {
            format!("{} {}", s, index + 1)
        }),
        Value::Bool(_) => Value::Bool(index.is_multiple_of(2)),
        Value::Array(items) => Value::Array(items.iter().map(|v| mock_value(v, index)).collect()),
        Value::Object(obj) => {
            let mut out = Map::new();
            for (k, v) in obj {
                out.insert(k.clone(), mock_value(v, index));
            }
            Value::Object(out)
        }
        _ => value.clone(),
    }
}
