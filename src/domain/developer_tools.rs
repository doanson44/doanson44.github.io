use regex::RegexBuilder;
use sha2::{Digest, Sha256, Sha512};
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolKind {
    Xml,
    Yaml,
    Sql,
    Html,
    Css,
    Javascript,
    Regex,
    Url,
    Hash,
    Uuid,
    Timestamp,
    Color,
    Cron,
    HttpStatus,
    Subnet,
    Qr,
}

impl ToolKind {
    pub fn from_route(route: &str) -> Option<Self> {
        match route {
            "xml" => Some(Self::Xml),
            "yaml" => Some(Self::Yaml),
            "sql" => Some(Self::Sql),
            "html" => Some(Self::Html),
            "css" => Some(Self::Css),
            "javascript" => Some(Self::Javascript),
            "regex" => Some(Self::Regex),
            "url" => Some(Self::Url),
            "hash" => Some(Self::Hash),
            "uuid" => Some(Self::Uuid),
            "timestamp" => Some(Self::Timestamp),
            "color" => Some(Self::Color),
            "cron" => Some(Self::Cron),
            "http-status" => Some(Self::HttpStatus),
            "subnet" => Some(Self::Subnet),
            "qr" => Some(Self::Qr),
            _ => None,
        }
    }

    pub fn route(self) -> &'static str {
        match self {
            Self::Xml => "xml",
            Self::Yaml => "yaml",
            Self::Sql => "sql",
            Self::Html => "html",
            Self::Css => "css",
            Self::Javascript => "javascript",
            Self::Regex => "regex",
            Self::Url => "url",
            Self::Hash => "hash",
            Self::Uuid => "uuid",
            Self::Timestamp => "timestamp",
            Self::Color => "color",
            Self::Cron => "cron",
            Self::HttpStatus => "http-status",
            Self::Subnet => "subnet",
            Self::Qr => "qr",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Xml => "XML Formatter",
            Self::Yaml => "YAML Formatter",
            Self::Sql => "SQL Formatter",
            Self::Html => "HTML Formatter",
            Self::Css => "CSS Formatter",
            Self::Javascript => "JavaScript Formatter",
            Self::Regex => "Regex Tester",
            Self::Url => "URL Encoder / Decoder",
            Self::Hash => "Hash Generator",
            Self::Uuid => "UUID Generator",
            Self::Timestamp => "Timestamp Converter",
            Self::Color => "Color Converter",
            Self::Cron => "Cron Expression Generator",
            Self::HttpStatus => "HTTP Status Lookup",
            Self::Subnet => "IP / Subnet Calculator",
            Self::Qr => "QR Code Generator",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Xml => "Format XML locally in your browser.",
            Self::Yaml => "Format YAML with readable indentation.",
            Self::Sql => "Format common SQL statements locally.",
            Self::Html => "Format HTML markup with readable indentation.",
            Self::Css => "Format CSS declarations and rules.",
            Self::Javascript => "Format JavaScript source with basic indentation.",
            Self::Regex => "Test regular expressions against sample text.",
            Self::Url => "Encode and decode URL components locally.",
            Self::Hash => "Generate SHA-256 and SHA-512 hashes locally.",
            Self::Uuid => "Generate UUID v4 values locally.",
            Self::Timestamp => "Convert Unix timestamps to UTC dates and back.",
            Self::Color => "Convert hexadecimal colors to RGB and HSL.",
            Self::Cron => "Validate and explain five-field cron expressions.",
            Self::HttpStatus => "Look up common HTTP status codes.",
            Self::Subnet => "Calculate IPv4 network and host ranges.",
            Self::Qr => "Generate an SVG QR code locally.",
        }
    }
}

pub fn sample(kind: ToolKind) -> (&'static str, &'static str) {
    match kind {
        ToolKind::Xml => ("<root><user><name>Son</name><active>true</active></user></root>", ""),
        ToolKind::Yaml => ("name: Developer Tools\nactive: true\nfeatures:\n  - Markdown\n  - JSON\n  - JWT", ""),
        ToolKind::Sql => ("SELECT id, name FROM users WHERE active = true ORDER BY name;", ""),
        ToolKind::Html => ("<main><section><h1>Hello</h1><p>Developer tools</p></section></main>", ""),
        ToolKind::Css => (".card{display:flex;gap:1rem;padding:1rem}.card:hover{opacity:.9}", ""),
        ToolKind::Javascript => ("function greet(name){if(name){return `Hello ${name}`;}return 'Hello';}", ""),
        ToolKind::Regex => (r"\b[A-Z][a-z]+\b", "Son writes Rust. ChatGPT helps Son build tools."),
        ToolKind::Url => ("https://example.com/search?q=rust tools&lang=en", ""),
        ToolKind::Hash => ("Hello, developer tools!", ""),
        ToolKind::Uuid => ("", ""),
        ToolKind::Timestamp => ("0", ""),
        ToolKind::Color => ("#0d6efd", ""),
        ToolKind::Cron => ("*/15 9-17 * * 1-5", ""),
        ToolKind::HttpStatus => ("404", ""),
        ToolKind::Subnet => ("192.168.1.0/24", ""),
        ToolKind::Qr => ("https://doanson44.github.io", ""),
    }
}

pub fn run(kind: ToolKind, source: &str, secondary: &str) -> Result<String, String> {
    match kind {
        ToolKind::Xml => format_markup(source),
        ToolKind::Yaml => format_yaml(source),
        ToolKind::Sql => format_sql(source),
        ToolKind::Html => format_markup(source),
        ToolKind::Css => format_braced(source),
        ToolKind::Javascript => format_braced(source),
        ToolKind::Regex => test_regex(source, secondary),
        ToolKind::Url => decode_url(source),
        ToolKind::Hash => hash_text(source),
        ToolKind::Uuid => Ok(uuid::Uuid::new_v4().to_string()),
        ToolKind::Timestamp => convert_timestamp(source),
        ToolKind::Color => convert_color(source),
        ToolKind::Cron => validate_cron(source),
        ToolKind::HttpStatus => lookup_status(source),
        ToolKind::Subnet => calculate_subnet(source),
        ToolKind::Qr => generate_qr(source),
    }
}

fn format_markup(source: &str) -> Result<String, String> {
    let source = source.trim();
    if source.is_empty() { return Err("Input is empty.".into()); }
    let mut out = String::new();
    let mut depth = 0usize;
    let mut token = String::new();
    let mut in_tag = false;
    for ch in source.chars() {
        if ch == '<' { if !token.trim().is_empty() { write_line(&mut out, depth, token.trim()); } token.clear(); in_tag = true; token.push(ch); }
        else if ch == '>' { token.push(ch); let t = token.trim(); if t.starts_with("</") { depth = depth.saturating_sub(1); } write_line(&mut out, depth, t); if !t.starts_with("</") && !t.starts_with("<?") && !t.starts_with("<!") && !t.ends_with("/>") && !t.contains("</") { depth += 1; } token.clear(); in_tag = false; }
        else if in_tag { token.push(ch); }
        else { token.push(ch); }
    }
    if !token.trim().is_empty() { write_line(&mut out, depth, token.trim()); }
    Ok(out.trim_end().to_string())
}

fn format_yaml(source: &str) -> Result<String, String> {
    let mut out = String::new();
    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() { continue; }
        let indent = raw.chars().take_while(|c| c.is_whitespace()).count();
        let normalized = line.replace("\t", "  ");
        write_line(&mut out, indent / 2, normalized.trim());
    }
    if out.is_empty() { Err("Input is empty.".into()) } else { Ok(out.trim_end().into()) }
}

fn format_sql(source: &str) -> Result<String, String> {
    let mut s = source.split_whitespace().collect::<Vec<_>>().join(" ");
    for keyword in ["SELECT", "FROM", "WHERE", "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "VALUES", "SET", "JOIN", "LEFT JOIN", "RIGHT JOIN", "INNER JOIN", "OUTER JOIN"] {
        let lower = keyword.to_ascii_lowercase();
        s = replace_ci(&s, &lower, &format!("\n{}", keyword));
    }
    s = s.replace(", ", ",\n  ");
    if s.trim().is_empty() { Err("Input is empty.".into()) } else { Ok(s.trim().into()) }
}

fn format_braced(source: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for ch in source.chars() {
        match ch {
            '{' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); write_line(&mut out, depth, "{"); depth += 1; }
            '}' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); depth = depth.saturating_sub(1); write_line(&mut out, depth, "}"); }
            ';' => { current.push(';'); write_line(&mut out, depth, current.trim()); current.clear(); }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); }
    if out.is_empty() { Err("Input is empty.".into()) } else { Ok(out.trim_end().into()) }
}

fn test_regex(pattern: &str, text: &str) -> Result<String, String> {
    if pattern.is_empty() { return Err("Regex pattern is empty.".into()); }
    let regex = RegexBuilder::new(pattern).build().map_err(|e| format!("Invalid regex: {e}"))?;
    let mut out = String::new();
    let matches: Vec<_> = regex.find_iter(text).collect();
    writeln!(&mut out, "{} match(es)", matches.len()).unwrap();
    for (index, m) in matches.iter().enumerate() {
        writeln!(&mut out, "{}. [{}..{}] {}", index + 1, m.start(), m.end(), m.as_str()).unwrap();
    }
    Ok(out.trim_end().into())
}

fn encode_url(input: &str) -> String {
    input.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") }).collect()
}

fn decode_url(input: &str) -> Result<String, String> {
    let bytes = input.as_bytes(); let mut out = Vec::with_capacity(bytes.len()); let mut i = 0;
    while i < bytes.len() { if bytes[i] == b'%' { if i + 2 >= bytes.len() { return Err("Invalid percent-encoding.".into()); } let hex = std::str::from_utf8(&bytes[i+1..i+3]).map_err(|_| "Invalid percent-encoding.")?; out.push(u8::from_str_radix(hex, 16).map_err(|_| "Invalid percent-encoding.")?); i += 3; } else { out.push(bytes[i]); i += 1; } }
    String::from_utf8(out).map_err(|_| "Decoded data is not valid UTF-8.".into())
}

fn hash_text(input: &str) -> Result<String, String> {
    let sha256 = Sha256::digest(input.as_bytes());
    let sha512 = Sha512::digest(input.as_bytes());
    Ok(format!("SHA-256\n{}\n\nSHA-512\n{}", hex(&sha256), hex(&sha512)))
}

fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }

fn convert_timestamp(input: &str) -> Result<String, String> {
    let value: i64 = input.trim().parse().map_err(|_| "Enter a Unix timestamp in seconds.")?;
    let (year, month, day) = unix_to_date(value)?;
    let seconds = value.rem_euclid(86_400);
    Ok(format!("UTC: {year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}Z", seconds / 3600, (seconds % 3600) / 60, seconds % 60))
}

fn unix_to_date(timestamp: i64) -> Result<(i64, i64, i64), String> {
    let days = timestamp.div_euclid(86_400);
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 }.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let d = doy - (153 * mp + 2).div_euclid(5) + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    if !(1..=9999).contains(&year) { return Err("Timestamp is outside the supported date range.".into()); }
    Ok((year, m, d))
}

fn convert_color(input: &str) -> Result<String, String> {
    let hex_value = input.trim().trim_start_matches('#');
    if hex_value.len() != 6 { return Err("Enter a six-digit hexadecimal color such as #0d6efd.".into()); }
    let r = u8::from_str_radix(&hex_value[0..2], 16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0;
    let g = u8::from_str_radix(&hex_value[2..4], 16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0;
    let b = u8::from_str_radix(&hex_value[4..6], 16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0;
    let max = r.max(g).max(b); let min = r.min(g).min(b); let l = (max + min) / 2.0; let d = max - min;
    let s = if d == 0.0 { 0.0 } else { d / (1.0 - (2.0 * l - 1.0).abs()) };
    let h = if d == 0.0 { 0.0 } else if max == r { 60.0 * (((g - b) / d) % 6.0) } else if max == g { 60.0 * (((b - r) / d) + 2.0) } else { 60.0 * (((r - g) / d) + 4.0) };
    Ok(format!("HEX: #{hex_value}\nRGB: {}, {}, {}\nHSL: {:.0}°, {:.0}%, {:.0}%", (r * 255.0).round(), (g * 255.0).round(), (b * 255.0).round(), h.rem_euclid(360.0), s * 100.0, l * 100.0))
}

fn validate_cron(input: &str) -> Result<String, String> {
    let fields: Vec<_> = input.split_whitespace().collect();
    if fields.len() != 5 { return Err("A standard cron expression must contain exactly five fields.".into()); }
    let names = ["Minute", "Hour", "Day of month", "Month", "Day of week"];
    let ranges = [0..=59, 0..=23, 1..=31, 1..=12, 0..=7];
    let mut out = String::from("Valid five-field cron expression\n");
    for i in 0..5 { if fields[i].is_empty() || !fields[i].chars().all(|c| c.is_ascii_digit() || matches!(c, '*' | '/' | '-' | ',')) { return Err(format!("Invalid {} field.", names[i])); } writeln!(&mut out, "{}: {} ({:?})", names[i], fields[i], ranges[i]).unwrap(); }
    Ok(out.trim_end().into())
}

fn lookup_status(input: &str) -> Result<String, String> {
    let code: u16 = input.trim().parse().map_err(|_| "Enter an HTTP status code such as 404.")?;
    let text = match code { 100 => "Continue", 101 => "Switching Protocols", 200 => "OK", 201 => "Created", 202 => "Accepted", 204 => "No Content", 301 => "Moved Permanently", 302 => "Found", 304 => "Not Modified", 400 => "Bad Request", 401 => "Unauthorized", 403 => "Forbidden", 404 => "Not Found", 405 => "Method Not Allowed", 409 => "Conflict", 422 => "Unprocessable Content", 429 => "Too Many Requests", 500 => "Internal Server Error", 501 => "Not Implemented", 502 => "Bad Gateway", 503 => "Service Unavailable", 504 => "Gateway Timeout", _ => return Err("Status code is not in the built-in lookup table.".into()) };
    Ok(format!("{code} — {text}"))
}

fn calculate_subnet(input: &str) -> Result<String, String> {
    let (ip, prefix) = input.trim().split_once('/').ok_or("Enter CIDR notation such as 192.168.1.0/24.")?;
    let prefix: u32 = prefix.parse().map_err(|_| "Invalid prefix length.")?;
    if prefix > 32 { return Err("IPv4 prefix length must be between 0 and 32.".into()); }
    let ip = ipv4_to_u32(ip)?; let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) }; let network = ip & mask; let broadcast = network | !mask;
    let hosts = if prefix >= 31 { 1u64 << (32 - prefix) } else { (1u64 << (32 - prefix)) - 2 };
    Ok(format!("Network: {}\nBroadcast: {}\nMask: {}\nPrefix: /{prefix}\nUsable hosts: {hosts}", u32_to_ipv4(network), u32_to_ipv4(broadcast), u32_to_ipv4(mask)))
}

fn ipv4_to_u32(input: &str) -> Result<u32, String> { let parts: Vec<_> = input.split('.').collect(); if parts.len() != 4 { return Err("Invalid IPv4 address.".into()); } let mut value = 0u32; for p in parts { let n: u8 = p.parse().map_err(|_| "Invalid IPv4 address.")?; value = (value << 8) | n as u32; } Ok(value) }
fn u32_to_ipv4(value: u32) -> String { format!("{}.{}.{}.{}", value >> 24, (value >> 16) & 255, (value >> 8) & 255, value & 255) }

fn generate_qr(input: &str) -> Result<String, String> {
    let code = qrcode::QrCode::new(input.as_bytes()).map_err(|e| format!("Unable to generate QR code: {e}"))?;
    Ok(code.render::<qrcode::render::svg::Color>().min_dimensions(220, 220).build())
}

fn write_line(out: &mut String, depth: usize, value: &str) { let _ = writeln!(out, "{}{}", "  ".repeat(depth), value); }
fn replace_ci(source: &str, needle: &str, replacement: &str) -> String { let mut out = String::new(); let mut rest = source; while let Some(index) = rest.to_ascii_lowercase().find(needle) { out.push_str(&rest[..index]); out.push_str(replacement); rest = &rest[index + needle.len()..]; } out.push_str(rest); out }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn url_round_trip() { let value = "hello world!"; assert_eq!(decode_url(&encode_url(value)).unwrap(), value); }
    #[test] fn subnet_works() { assert!(calculate_subnet("192.168.1.10/24").unwrap().contains("192.168.1.0")); }
    #[test] fn color_works() { assert!(convert_color("#ff0000").unwrap().contains("RGB: 255, 0, 0")); }
    #[test] fn regex_works() { assert!(test_regex(r"\d+", "abc 123").unwrap().contains("123")); }
}
