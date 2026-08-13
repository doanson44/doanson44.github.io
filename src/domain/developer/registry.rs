use super::{data, encoding, formatting, generators, network, text, time, ToolId};

pub struct ToolDefinition {
    pub id: ToolId,
    pub route: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub sample_source: &'static str,
    pub sample_secondary: &'static str,
    pub secondary_label: Option<&'static str>,
    pub svg_output: bool,
    pub execute: fn(&str, &str) -> Result<String, String>,
}

macro_rules! definition {
    ($id:ident, $route:literal, $title:literal, $description:literal, $source:literal, $secondary:literal, $label:expr, $svg:expr, $executor:path) => {
        ToolDefinition {
            id: ToolId::$id,
            route: $route,
            title: $title,
            description: $description,
            sample_source: $source,
            sample_secondary: $secondary,
            secondary_label: $label,
            svg_output: $svg,
            execute: $executor,
        }
    };
}

pub static TOOLS: &[ToolDefinition] = &[
    definition!(Xml, "xml", "XML Formatter", "Format XML locally in your browser.", "<root><user><name>Son</name><active>true</active></user></root>", "", None, false, markup),
    definition!(Yaml, "yaml", "YAML Formatter", "Validate and normalize YAML locally.", "name: Developer\nactive: true\nfeatures:\n  - Rust\n  - WASM", "", None, false, formatting::format_yaml),
    definition!(Toml, "toml", "TOML Formatter", "Validate and normalize TOML configuration.", "[package]\nname = \"developer-tools\"\nversion = \"1.0.0\"", "", None, false, formatting::format_toml),
    definition!(Sql, "sql", "SQL Formatter", "Format common SQL statements locally.", "SELECT id, name FROM users WHERE active = true ORDER BY name;", "", None, false, formatting::format_sql),
    definition!(Html, "html", "HTML Formatter", "Format HTML markup with readable indentation.", "<main><section><h1>Hello</h1><p>Developer tools</p></section></main>", "", None, false, markup),
    definition!(Css, "css", "CSS Formatter", "Format CSS declarations and rules.", ".card{display:flex;gap:1rem;padding:1rem}.card:hover{opacity:.9}", "", None, false, formatting::format_braced),
    definition!(Javascript, "javascript", "JavaScript Formatter", "Format JavaScript source with basic indentation.", "function greet(name){if(name){return `Hello ${name}`;}return 'Hello';}", "", None, false, formatting::format_braced),
    definition!(Regex, "regex", "Regex Studio", "Test, inspect, and replace regular-expression matches.", r"\b[A-Z][a-z]+\b", "Son writes Rust. ChatGPT helps Son build tools.", Some("Test String"), false, text::regex_test),
    definition!(Url, "url", "URL Encoder / Decoder", "Encode and decode URL components locally.", "https://example.com/search?q=rust tools&lang=en", "decode", Some("Mode"), false, encoding::url),
    definition!(Hash, "hash", "Hash Generator", "Generate common cryptographic hashes locally.", "Hello, developer tools!", "SHA-256", Some("Algorithm"), false, encoding::hash),
    definition!(Uuid, "uuid", "UUID Generator", "Generate UUID v4 values locally.", "", "", None, false, uuid),
    definition!(Timestamp, "timestamp", "Timestamp Converter", "Convert Unix timestamps and ISO dates.", "0", "", None, false, time::timestamp),
    definition!(Color, "color", "Color Converter", "Convert HEX, RGB, and HSL color values.", "#0d6efd", "", None, false, |source, _| text::color(source)),
    definition!(Cron, "cron", "Cron Expression Studio", "Validate and explain five-field cron expressions.", "*/15 9-17 * * 1-5", "", None, false, time::cron),
    definition!(HttpStatus, "http-status", "HTTP Status Lookup", "Look up common HTTP status codes.", "404", "", None, false, network::http_status),
    definition!(Subnet, "subnet", "IP / Subnet Calculator", "Calculate IPv4 network and host ranges.", "192.168.1.0/24", "", None, false, network::subnet),
    definition!(Qr, "qr", "QR Code Generator", "Generate an SVG QR code locally.", "https://doanson44.github.io", "", None, true, qr),
    definition!(JsonDiff, "json-diff", "JSON Diff", "Compare two JSON documents structurally.", r#"{"name":"Son","age":31}"#, r#"{"name":"Son","age":32,"active":true}"#, Some("Compare With"), false, data::json_diff),
    definition!(JsonPath, "json-path", "JSONPath Tester", "Evaluate simple JSONPath expressions against JSON.", r#"{"users":[{"name":"Son","email":"son@example.com"}]}"#, "$.users[0].email", Some("JSONPath Expression"), false, data::json_path),
    definition!(JsonToType, "json-to-type", "JSON → Type Generator", "Generate Rust, C#, TypeScript, Go, or Python types from JSON.", r#"{"id":1,"name":"Son","active":true}"#, "Rust", Some("Target Language"), false, data::json_to_type),
    definition!(Curl, "curl", "cURL Builder", "Build cURL commands from a compact request definition.", "POST https://api.example.com/users\nAuthorization: Bearer TOKEN\nContent-Type: application/json\n\n{\"name\":\"Son\"}", "", None, false, curl),
    definition!(HttpHeaders, "http-headers", "HTTP Header Analyzer", "Parse and explain raw HTTP headers.", "Content-Type: application/json\nCache-Control: no-cache\nX-Content-Type-Options: nosniff", "", None, false, headers),
    definition!(OpenApi, "openapi", "OpenAPI Viewer", "Inspect OpenAPI JSON or YAML documents locally.", r#"{"openapi":"3.0.3","info":{"title":"Example","version":"1.0.0"},"paths":{"/users":{"get":{"responses":{"200":{"description":"OK"}}}}}}"#, "", None, false, data::openapi),
    definition!(SqlToEntity, "sql-to-entity", "SQL → Entity Generator", "Generate backend entities from simple SQL CREATE TABLE statements.", "CREATE TABLE users (id BIGINT NOT NULL, name VARCHAR(255), active BOOLEAN);", "Rust", Some("Target Language"), false, data::sql_to_entity),
    definition!(Git, "git", "Git Command Builder", "Generate safe Git commands without executing them.", "undo-last-commit keep-changes=true", "", None, false, git),
    definition!(Gitignore, "gitignore", ".gitignore Generator", "Generate a combined .gitignore for common stacks.", "Rust\nNode\nVSCode\nDocker\nWindows", "", None, false, gitignore),
    definition!(Chmod, "chmod", "Chmod Calculator", "Convert Unix permission bits between numeric and symbolic forms.", "755", "", None, false, chmod),
    definition!(Mime, "mime", "MIME Type Lookup", "Look up MIME types by extension or media type.", "json", "", None, false, mime),
    definition!(TextDiff, "diff", "Text Diff", "Compare two text documents and show changed lines.", "hello\nworld\nRust", "hello\nworld\nWASM", Some("Compare With"), false, text::text_diff),
    definition!(FakeData, "fake-data", "Fake Data Generator", "Generate deterministic local test data without network calls.", "count=5\ntypes=uuid,name,email,number", "", None, false, fake_data),
    definition!(MockJson, "mock-json", "JSON Mock Generator", "Generate repeated JSON records from an example object.", r#"{"id":1,"name":"Son","active":true}"#, "5", Some("Record Count"), false, mock_json),
    definition!(NumberBase, "number-base", "Number Base Converter", "Convert integers between binary, octal, decimal, and hexadecimal.", "255", "hex", Some("Target Base"), false, number_base),
    definition!(HtmlEntity, "html-entity", "HTML Entity Encoder / Decoder", "Encode or decode common HTML entities.", "<p>Hello & Son</p>", "encode", Some("Mode"), false, html_entity),
    definition!(UnicodeEscape, "unicode-escape", "Unicode Escape / Unescape", "Escape or unescape Unicode code points.", "Hello, 世界", "escape", Some("Mode"), false, unicode_escape),
];

fn markup(source: &str, _: &str) -> Result<String, String> { formatting::format_markup(source) }
fn uuid(_: &str, _: &str) -> Result<String, String> { generators::uuid() }
fn qr(source: &str, _: &str) -> Result<String, String> { generators::qr(source) }
fn curl(source: &str, _: &str) -> Result<String, String> { network::curl(source) }
fn headers(source: &str, _: &str) -> Result<String, String> { network::headers(source) }
fn git(source: &str, _: &str) -> Result<String, String> { text::git(source) }
fn gitignore(source: &str, _: &str) -> Result<String, String> { text::gitignore(source) }
fn chmod(source: &str, _: &str) -> Result<String, String> { text::chmod(source) }
fn mime(source: &str, _: &str) -> Result<String, String> { network::mime(source) }
fn fake_data(source: &str, _: &str) -> Result<String, String> { generators::fake_data(source) }
fn mock_json(source: &str, secondary: &str) -> Result<String, String> { generators::mock_json(source, secondary) }
fn number_base(source: &str, secondary: &str) -> Result<String, String> { encoding::number_base(source, secondary) }
fn html_entity(source: &str, secondary: &str) -> Result<String, String> { encoding::html_entity(source, secondary) }
fn unicode_escape(source: &str, secondary: &str) -> Result<String, String> { encoding::unicode_escape(source, secondary) }

pub fn get(id: ToolId) -> &'static ToolDefinition {
    TOOLS.iter().find(|tool| tool.id == id).expect("developer tool registry is incomplete")
}

pub fn find_by_route(route: &str) -> Option<ToolId> {
    TOOLS.iter().find(|tool| tool.route == route).map(|tool| tool.id)
}
