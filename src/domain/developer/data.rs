use serde_json::Value;

pub fn json_diff(left: &str, right: &str) -> Result<String, String> {
    let a: Value = serde_json::from_str(left).map_err(|e| format!("Invalid JSON A: {e}"))?;
    let b: Value = serde_json::from_str(right).map_err(|e| format!("Invalid JSON B: {e}"))?;
    let mut changes = Vec::new();
    diff_value("$", &a, &b, &mut changes);
    if changes.is_empty() { Ok("No structural differences.".into()) } else { Ok(changes.join("\n")) }
}

fn diff_value(path: &str, a: &Value, b: &Value, changes: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ao), Value::Object(bo)) => {
            for key in ao.keys() { if !bo.contains_key(key) { changes.push(format!("Removed {path}.{key}")); } }
            for key in bo.keys() { if !ao.contains_key(key) { changes.push(format!("Added {path}.{key}: {}", bo[key])); } }
            for key in ao.keys().filter(|k| bo.contains_key(*k)) { diff_value(&format!("{path}.{key}"), &ao[key], &bo[key], changes); }
        }
        (Value::Array(aa), Value::Array(ba)) => {
            let max = aa.len().max(ba.len());
            for i in 0..max { match (aa.get(i), ba.get(i)) { (Some(x), Some(y)) => diff_value(&format!("{path}[{i}]"), x, y, changes), (Some(_), None) => changes.push(format!("Removed {path}[{i}]")), (None, Some(y)) => changes.push(format!("Added {path}[{i}]: {y}")), _ => {} } }
        }
        _ if a != b => changes.push(format!("Changed {path}: {a} → {b}")),
        _ => {}
    }
}

pub fn json_path(source: &str, expression: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(source).map_err(|e| format!("Invalid JSON: {e}"))?;
    let expr = expression.trim();
    if !expr.starts_with('$') { return Err("JSONPath must start with $.".into()); }
    let mut current = &value;
    for token in expr.trim_start_matches('$').split('.').filter(|s| !s.is_empty()) {
        if let Some((name, index)) = token.split_once('[') {
            current = current.get(name).ok_or_else(|| format!("Path not found: {name}"))?;
            let index = index.trim_end_matches(']').parse::<usize>().map_err(|_| "Invalid array index.")?;
            current = current.get(index).ok_or("Array index not found.")?;
        } else if token.contains('[') { return Err("Invalid JSONPath token.".into()); }
        else { current = current.get(token).ok_or_else(|| format!("Path not found: {token}"))?; }
    }
    serde_json::to_string_pretty(current).map_err(|e| e.to_string())
}

pub fn json_to_type(source: &str, target: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(source).map_err(|e| format!("Invalid JSON: {e}"))?;
    let root = value.as_object().ok_or("Root JSON value must be an object.")?;
    match target.to_ascii_lowercase().as_str() {
        "rust" | "" => Ok(rust_struct("Root", root)),
        "c#" | "csharp" => Ok(csharp_class("Root", root)),
        "typescript" | "ts" => Ok(ts_interface("Root", root)),
        "go" => Ok(go_struct("Root", root)),
        "python" => Ok(python_dataclass("Root", root)),
        _ => Err("Supported targets: Rust, C#, TypeScript, Go, Python.".into()),
    }
}

fn type_name(key: &str) -> String {
    key.split(['_', '-', ' ']).filter(|s| !s.is_empty()).map(|s| { let mut c = s.chars(); match c.next() { Some(first) => first.to_uppercase().collect::<String>() + c.as_str(), None => String::new() } }).collect()
}
fn rust_type(v: &Value) -> &'static str { match v { Value::String(_) => "String", Value::Bool(_) => "bool", Value::Number(n) if n.is_i64() => "i64", Value::Number(_) => "f64", Value::Array(_) => "Vec<serde_json::Value>", _ => "serde_json::Value" } }
fn csharp_type(v: &Value) -> &'static str { match v { Value::String(_) => "string", Value::Bool(_) => "bool", Value::Number(n) if n.is_i64() => "long", Value::Number(_) => "double", Value::Array(_) => "List<object>", _ => "object" } }
fn ts_type(v: &Value) -> &'static str { match v { Value::String(_) => "string", Value::Bool(_) => "boolean", Value::Number(_) => "number", Value::Array(_) => "unknown[]", Value::Null => "null", _ => "Record<string, unknown>" } }
fn go_type(v: &Value) -> &'static str { match v { Value::String(_) => "string", Value::Bool(_) => "bool", Value::Number(n) if n.is_i64() => "int64", Value::Number(_) => "float64", Value::Array(_) => "[]any", _ => "any" } }
fn rust_struct(name: &str, obj: &serde_json::Map<String, Value>) -> String { let mut out = format!("#[derive(Debug, serde::Serialize, serde::Deserialize)]\npub struct {name} {{\n"); for (k,v) in obj { out.push_str(&format!("    pub {}: {},\n", k.replace('-', "_"), rust_type(v))); } out.push('}'); out }
fn csharp_class(name: &str, obj: &serde_json::Map<String, Value>) -> String { let mut out = format!("public sealed class {name}\n{{\n"); for (k,v) in obj { out.push_str(&format!("    public {} {} {{ get; set; }}\n", csharp_type(v), type_name(k))); } out.push('}'); out }
fn ts_interface(name: &str, obj: &serde_json::Map<String, Value>) -> String { let mut out = format!("interface {name} {{\n"); for (k,v) in obj { out.push_str(&format!("  {}: {};\n", k, ts_type(v))); } out.push('}'); out }
fn go_struct(name: &str, obj: &serde_json::Map<String, Value>) -> String { let mut out = format!("type {name} struct {{\n"); for (k,v) in obj { out.push_str(&format!("    {} {} `json:\"{}\"`\n", type_name(k), go_type(v), k)); } out.push('}'); out }
fn python_dataclass(name: &str, obj: &serde_json::Map<String, Value>) -> String { let mut out = format!("from dataclasses import dataclass\n\n@dataclass\nclass {name}:\n"); for (k,v) in obj { let t = match v { Value::String(_) => "str", Value::Bool(_) => "bool", Value::Number(_) => "float", _ => "object" }; out.push_str(&format!("    {k}: {t}\n")); } out }

pub fn openapi(source: &str) -> Result<String, String> {
    let value = if let Ok(json) = serde_json::from_str::<Value>(source) {
        json
    } else {
        let yaml: serde_yaml::Value = serde_yaml::from_str(source).map_err(|e| format!("Invalid OpenAPI JSON/YAML: {e}"))?;
        serde_json::to_value(yaml).map_err(|e| format!("Unable to normalize OpenAPI document: {e}"))?
    };
    let version = value.get("openapi").and_then(Value::as_str).ok_or("OpenAPI document is missing the openapi version.")?;
    let title = value.pointer("/info/title").and_then(Value::as_str).unwrap_or("Untitled API");
    let api_version = value.pointer("/info/version").and_then(Value::as_str).unwrap_or("unknown");
    let mut out = format!("{title}\nOpenAPI: {version}\nVersion: {api_version}\n\nEndpoints:\n");
    if let Some(paths) = value.get("paths").and_then(Value::as_object) {
        for (path, methods) in paths {
            if let Some(methods) = methods.as_object() {
                for (method, op) in methods {
                    if !matches!(method.as_str(), "get" | "post" | "put" | "patch" | "delete" | "head" | "options" | "trace") { continue; }
                    let summary = op.get("summary").and_then(Value::as_str).or_else(|| op.get("description").and_then(Value::as_str)).unwrap_or("");
                    out.push_str(&format!("{} {} {}\n", method.to_uppercase(), path, summary));
                }
            }
        }
    }
    Ok(out.trim_end().into())
}

pub fn sql_to_entity(source: &str, target: &str) -> Result<String, String> {
    let lower = source.to_ascii_lowercase();
    let start = lower.find("create table").ok_or("Expected CREATE TABLE statement.")?;
    let open = source[start..].find('(').ok_or("Missing column list.")? + start;
    let close = find_matching_paren(source, open).ok_or("Missing closing parenthesis.")?;
    let head = &source[start..open];
    let table = head.split_whitespace().last().ok_or("Missing table name.")?.trim_matches('`');
    let columns = split_sql_columns(&source[open + 1..close]).into_iter().filter_map(|part| {
        let mut p = part.split_whitespace();
        let name = p.next()?;
        let sql_type = p.next()?;
        Some((name.trim_matches('`').to_string(), sql_type.to_ascii_uppercase()))
    }).collect::<Vec<_>>();
    if columns.is_empty() { return Err("No columns found in CREATE TABLE statement.".into()); }
    let target = target.to_ascii_lowercase();
    if target == "c#" || target == "csharp" {
        let mut out = format!("public sealed class {}\n{{\n", type_name(table));
        for (name, ty) in columns { out.push_str(&format!("    public {} {} {{ get; set; }}\n", csharp_sql_type(&ty), type_name(&name))); }
        out.push('}'); Ok(out)
    } else if target == "rust" || target.is_empty() {
        let mut out = format!("pub struct {} {{\n", type_name(table));
        for (name, ty) in columns { out.push_str(&format!("    pub {}: {},\n", name, rust_sql_type(&ty))); }
        out.push('}'); Ok(out)
    } else { Err("Supported targets: Rust, C#, CSharp.".into()) }
}

fn find_matching_paren(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize; let mut quote = None; let mut escaped = false;
    for (offset, ch) in source[open..].char_indices() {
        if let Some(q) = quote { if escaped { escaped = false; } else if ch == '\\' { escaped = true; } else if ch == q { quote = None; } continue; }
        if ch == '\'' || ch == '"' { quote = Some(ch); continue; }
        match ch { '(' => depth += 1, ')' => { depth = depth.checked_sub(1)?; if depth == 0 { return Some(open + offset); } }, _ => {} }
    }
    None
}

fn split_sql_columns(source: &str) -> Vec<String> {
    let mut parts = Vec::new(); let mut start = 0usize; let mut depth = 0usize; let mut quote = None; let mut escaped = false;
    for (index, ch) in source.char_indices() {
        if let Some(q) = quote { if escaped { escaped = false; } else if ch == '\\' { escaped = true; } else if ch == q { quote = None; } continue; }
        if ch == '\'' || ch == '"' || ch == '`' { quote = Some(ch); continue; }
        match ch { '(' => depth += 1, ')' => depth = depth.saturating_sub(1), ',' if depth == 0 => { parts.push(source[start..index].trim().to_string()); start = index + 1; }, _ => {} }
    }
    if !source[start..].trim().is_empty() { parts.push(source[start..].trim().to_string()); }
    parts
}

fn rust_sql_type(ty: &str) -> &'static str { if ty.contains("INT") { "i64" } else if ty.contains("BOOL") { "bool" } else if ty.contains("DEC") || ty.contains("NUM") || ty.contains("FLOAT") { "f64" } else { "String" } }
fn csharp_sql_type(ty: &str) -> &'static str { if ty.contains("INT") { "long" } else if ty.contains("BOOL") { "bool" } else if ty.contains("DEC") || ty.contains("NUM") || ty.contains("FLOAT") { "decimal" } else { "string" } }
