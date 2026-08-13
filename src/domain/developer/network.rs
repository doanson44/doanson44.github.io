use std::net::Ipv4Addr;

pub fn http_status(input: &str) -> Result<String, String> {
    let code: u16 = input.trim().parse().map_err(|_| "Enter an HTTP status code.")?;
    let text = match code { 100 => "Continue", 101 => "Switching Protocols", 200 => "OK", 201 => "Created", 202 => "Accepted", 204 => "No Content", 301 => "Moved Permanently", 302 => "Found", 304 => "Not Modified", 307 => "Temporary Redirect", 308 => "Permanent Redirect", 400 => "Bad Request", 401 => "Unauthorized", 403 => "Forbidden", 404 => "Not Found", 405 => "Method Not Allowed", 409 => "Conflict", 410 => "Gone", 415 => "Unsupported Media Type", 422 => "Unprocessable Content", 429 => "Too Many Requests", 500 => "Internal Server Error", 501 => "Not Implemented", 502 => "Bad Gateway", 503 => "Service Unavailable", 504 => "Gateway Timeout", _ => "Unknown / non-common status", };
    let category = match code { 100..=199 => "Informational", 200..=299 => "Success", 300..=399 => "Redirection", 400..=499 => "Client Error", 500..=599 => "Server Error", _ => "Non-standard" };
    Ok(format!("{code} {text}\nCategory: {category}"))
}

pub fn subnet(input: &str) -> Result<String, String> {
    let (ip, prefix) = input.trim().split_once('/').ok_or("Use CIDR notation such as 192.168.1.0/24.")?;
    let ip: Ipv4Addr = ip.parse().map_err(|_| "Invalid IPv4 address.")?; let prefix: u32 = prefix.parse().map_err(|_| "Invalid CIDR prefix.")?;
    if prefix > 32 { return Err("CIDR prefix must be between 0 and 32.".into()); }
    let ipn = u32::from(ip); let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) }; let network = ipn & mask; let broadcast = network | !mask;
    let hosts = if prefix >= 31 { 2u64.pow(32 - prefix) } else { (2u64.pow(32 - prefix)).saturating_sub(2) };
    Ok(format!("Network: {}\nBroadcast: {}\nSubnet mask: {}\nPrefix: /{prefix}\nUsable host count: {hosts}", Ipv4Addr::from(network), Ipv4Addr::from(broadcast), Ipv4Addr::from(mask)))
}

pub fn curl(input: &str) -> Result<String, String> {
    let mut lines = input.lines(); let first = lines.next().ok_or("Enter a request line.")?; let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_uppercase(); let url = parts.next().ok_or("Request line must contain METHOD URL.")?;
    let mut headers = Vec::new(); let mut body = Vec::new(); let mut in_body = false;
    for line in lines { if line.trim().is_empty() { in_body = true; continue; } if in_body { body.push(line); } else if let Some((k, v)) = line.split_once(':') { headers.push((k.trim(), v.trim())); } }
    let mut out = format!("curl -X {method} \\\n  '{url}'"); for (k, v) in headers { out.push_str(&format!(" \\\n  -H '{}: {}'", k.replace('\'', "'\\''"), v.replace('\'', "'\\''"))); }
    if !body.is_empty() { out.push_str(&format!(" \\\n  -d '{}'", body.join("\n").replace('\'', "'\\''"))); } Ok(out)
}

pub fn headers(input: &str) -> Result<String, String> {
    if input.trim().is_empty() { return Err("Input is empty.".into()); }
    let mut out = String::from("Header\tValue\tNotes\n");
    for line in input.lines() { if let Some((name, value)) = line.split_once(':') { let name = name.trim(); let note = match name.to_ascii_lowercase().as_str() { "content-type" => "Describes the representation format.", "cache-control" => "Controls caching behavior.", "authorization" => "Carries authentication credentials; treat values as secrets.", "x-content-type-options" => "nosniff helps prevent MIME sniffing.", "strict-transport-security" => "Enforces HTTPS in supporting browsers.", _ => "HTTP header." }; out.push_str(&format!("{name}\t{}\t{note}\n", value.trim())); } else { return Err(format!("Invalid header line: {line}")); } }
    Ok(out.trim_end().into())
}

pub fn mime(input: &str) -> Result<String, String> {
    let value = input.trim().trim_start_matches('.').to_ascii_lowercase(); let table = [("json", "application/json"), ("xml", "application/xml"), ("yaml", "application/yaml"), ("yml", "application/yaml"), ("toml", "application/toml"), ("html", "text/html"), ("css", "text/css"), ("js", "text/javascript"), ("wasm", "application/wasm"), ("pdf", "application/pdf"), ("png", "image/png"), ("jpg", "image/jpeg"), ("jpeg", "image/jpeg"), ("svg", "image/svg+xml"), ("webp", "image/webp"), ("txt", "text/plain")];
    if let Some((ext, mime)) = table.iter().find(|(ext, mime)| *ext == value || *mime == input.trim()) { Ok(format!("Extension: .{ext}\nMIME type: {mime}")) } else { Err("MIME type not found in the built-in reference table.".into()) }
}
