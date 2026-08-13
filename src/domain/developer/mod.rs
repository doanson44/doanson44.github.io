mod data;
mod encoding;
mod formatting;
mod generators;
mod network;
mod text;
mod time;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolId {
    Xml,
    Yaml,
    Toml,
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
    JsonDiff,
    JsonPath,
    JsonToType,
    Curl,
    HttpHeaders,
    OpenApi,
    SqlToEntity,
    Git,
    Gitignore,
    Chmod,
    Mime,
    TextDiff,
    FakeData,
    MockJson,
    NumberBase,
    HtmlEntity,
    UnicodeEscape,
}

impl ToolId {
    pub const ALL: &[Self] = &[
        Self::Xml,
        Self::Yaml,
        Self::Toml,
        Self::Sql,
        Self::Html,
        Self::Css,
        Self::Javascript,
        Self::Regex,
        Self::Url,
        Self::Hash,
        Self::Uuid,
        Self::Timestamp,
        Self::Color,
        Self::Cron,
        Self::HttpStatus,
        Self::Subnet,
        Self::Qr,
        Self::JsonDiff,
        Self::JsonPath,
        Self::JsonToType,
        Self::Curl,
        Self::HttpHeaders,
        Self::OpenApi,
        Self::SqlToEntity,
        Self::Git,
        Self::Gitignore,
        Self::Chmod,
        Self::Mime,
        Self::TextDiff,
        Self::FakeData,
        Self::MockJson,
        Self::NumberBase,
        Self::HtmlEntity,
        Self::UnicodeEscape,
    ];

    pub fn from_route(route: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|tool| tool.route() == route)
    }

    pub fn route(self) -> &'static str {
        match self {
            Self::Xml => "xml",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
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
            Self::JsonDiff => "json-diff",
            Self::JsonPath => "json-path",
            Self::JsonToType => "json-to-type",
            Self::Curl => "curl",
            Self::HttpHeaders => "http-headers",
            Self::OpenApi => "openapi",
            Self::SqlToEntity => "sql-to-entity",
            Self::Git => "git",
            Self::Gitignore => "gitignore",
            Self::Chmod => "chmod",
            Self::Mime => "mime",
            Self::TextDiff => "diff",
            Self::FakeData => "fake-data",
            Self::MockJson => "mock-json",
            Self::NumberBase => "number-base",
            Self::HtmlEntity => "html-entity",
            Self::UnicodeEscape => "unicode-escape",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Xml => "XML Formatter",
            Self::Yaml => "YAML Formatter",
            Self::Toml => "TOML Formatter",
            Self::Sql => "SQL Formatter",
            Self::Html => "HTML Formatter",
            Self::Css => "CSS Formatter",
            Self::Javascript => "JavaScript Formatter",
            Self::Regex => "Regex Studio",
            Self::Url => "URL Encoder / Decoder",
            Self::Hash => "Hash Generator",
            Self::Uuid => "UUID Generator",
            Self::Timestamp => "Timestamp Converter",
            Self::Color => "Color Converter",
            Self::Cron => "Cron Expression Studio",
            Self::HttpStatus => "HTTP Status Lookup",
            Self::Subnet => "IP / Subnet Calculator",
            Self::Qr => "QR Code Generator",
            Self::JsonDiff => "JSON Diff",
            Self::JsonPath => "JSONPath Tester",
            Self::JsonToType => "JSON → Type Generator",
            Self::Curl => "cURL Builder",
            Self::HttpHeaders => "HTTP Header Analyzer",
            Self::OpenApi => "OpenAPI Viewer",
            Self::SqlToEntity => "SQL → Entity Generator",
            Self::Git => "Git Command Builder",
            Self::Gitignore => ".gitignore Generator",
            Self::Chmod => "Chmod Calculator",
            Self::Mime => "MIME Type Lookup",
            Self::TextDiff => "Text Diff",
            Self::FakeData => "Fake Data Generator",
            Self::MockJson => "JSON Mock Generator",
            Self::NumberBase => "Number Base Converter",
            Self::HtmlEntity => "HTML Entity Encoder / Decoder",
            Self::UnicodeEscape => "Unicode Escape / Unescape",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Xml => "Format XML locally in your browser.",
            Self::Yaml => "Format YAML with readable indentation.",
            Self::Toml => "Validate and normalize TOML configuration.",
            Self::Sql => "Format common SQL statements locally.",
            Self::Html => "Format HTML markup with readable indentation.",
            Self::Css => "Format CSS declarations and rules.",
            Self::Javascript => "Format JavaScript source with basic indentation.",
            Self::Regex => "Test, inspect, and replace regular-expression matches.",
            Self::Url => "Encode and decode URL components locally.",
            Self::Hash => "Generate common cryptographic hashes locally.",
            Self::Uuid => "Generate UUID v4 values locally.",
            Self::Timestamp => "Convert Unix timestamps and ISO dates.",
            Self::Color => "Convert HEX, RGB, and HSL color values.",
            Self::Cron => "Validate and explain five-field cron expressions.",
            Self::HttpStatus => "Look up common HTTP status codes.",
            Self::Subnet => "Calculate IPv4 network and host ranges.",
            Self::Qr => "Generate an SVG QR code locally.",
            Self::JsonDiff => "Compare two JSON documents structurally.",
            Self::JsonPath => "Evaluate simple JSONPath expressions against JSON.",
            Self::JsonToType => "Generate Rust, C#, TypeScript, Go, or Python types from JSON.",
            Self::Curl => "Build cURL commands from a compact request definition.",
            Self::HttpHeaders => "Parse and explain raw HTTP headers.",
            Self::OpenApi => "Inspect OpenAPI JSON or YAML documents locally.",
            Self::SqlToEntity => {
                "Generate backend entities from simple SQL CREATE TABLE statements."
            }
            Self::Git => "Generate safe Git commands without executing them.",
            Self::Gitignore => "Generate a combined .gitignore for common stacks.",
            Self::Chmod => "Convert Unix permission bits between numeric and symbolic forms.",
            Self::Mime => "Look up MIME types by extension or media type.",
            Self::TextDiff => "Compare two text documents and show changed lines.",
            Self::FakeData => "Generate deterministic local test data without network calls.",
            Self::MockJson => "Generate repeated JSON records from an example object.",
            Self::NumberBase => "Convert integers between binary, octal, decimal, and hexadecimal.",
            Self::HtmlEntity => "Encode or decode common HTML entities.",
            Self::UnicodeEscape => "Escape or unescape Unicode code points.",
        }
    }

    pub fn sample(self) -> (&'static str, &'static str) {
        match self {
            Self::Xml => ("<root><user><name>Son</name><active>true</active></user></root>", ""),
            Self::Yaml => ("name: Developer\nactive: true\nfeatures:\n  - Rust\n  - WASM", ""),
            Self::Toml => ("[package]\nname = \"developer-tools\"\nversion = \"1.0.0\"", ""),
            Self::Sql => ("SELECT id, name FROM users WHERE active = true ORDER BY name;", ""),
            Self::Html => ("<main><section><h1>Hello</h1><p>Developer tools</p></section></main>", ""),
            Self::Css => (".card{display:flex;gap:1rem;padding:1rem}.card:hover{opacity:.9}", ""),
            Self::Javascript => ("function greet(name){if(name){return `Hello ${name}`;}return 'Hello';}", ""),
            Self::Regex => (r"\b[A-Z][a-z]+\b", "Son writes Rust. ChatGPT helps Son build tools."),
            Self::Url => ("https://example.com/search?q=rust tools&lang=en", "decode"),
            Self::Hash => ("Hello, developer tools!", "SHA-256"), Self::Uuid => ("", ""),
            Self::Timestamp => ("0", ""), Self::Color => ("#0d6efd", ""), Self::Cron => ("*/15 9-17 * * 1-5", ""),
            Self::HttpStatus => ("404", ""), Self::Subnet => ("192.168.1.0/24", ""), Self::Qr => ("https://doanson44.github.io", ""),
            Self::JsonDiff => (r#"{"name":"Son","age":31}"#, r#"{"name":"Son","age":32,"active":true}"#),
            Self::JsonPath => (r#"{"users":[{"name":"Son","email":"son@example.com"}]}"#, "$.users[0].email"),
            Self::JsonToType => (r#"{"id":1,"name":"Son","active":true}"#, "Rust"),
            Self::Curl => ("POST https://api.example.com/users\nAuthorization: Bearer TOKEN\nContent-Type: application/json\n\n{\"name\":\"Son\"}", ""),
            Self::HttpHeaders => ("Content-Type: application/json\nCache-Control: no-cache\nX-Content-Type-Options: nosniff", ""),
            Self::OpenApi => (r#"{"openapi":"3.0.3","info":{"title":"Example","version":"1.0.0"},"paths":{"/users":{"get":{"responses":{"200":{"description":"OK"}}}}}}"#, ""),
            Self::SqlToEntity => ("CREATE TABLE users (id BIGINT NOT NULL, name VARCHAR(255), active BOOLEAN);", "Rust"),
            Self::Git => ("undo-last-commit keep-changes=true", ""), Self::Gitignore => ("Rust\nNode\nVSCode\nDocker\nWindows", ""),
            Self::Chmod => ("755", ""), Self::Mime => ("json", ""), Self::TextDiff => ("hello\nworld\nRust", "hello\nworld\nWASM"),
            Self::FakeData => ("count=5\ntypes=uuid,name,email,number", ""), Self::MockJson => (r#"{"id":1,"name":"Son","active":true}"#, "5"),
            Self::NumberBase => ("255", "hex"), Self::HtmlEntity => ("<p>Hello & Son</p>", "encode"), Self::UnicodeEscape => ("Hello, 世界", "escape"),
        }
    }

    pub fn secondary_label(self) -> Option<&'static str> {
        match self {
            Self::Regex => Some("Test String"),
            Self::JsonDiff | Self::TextDiff => Some("Compare With"),
            Self::JsonPath => Some("JSONPath Expression"),
            Self::JsonToType | Self::SqlToEntity => Some("Target Language"),
            Self::NumberBase => Some("Target Base"),
            Self::HtmlEntity | Self::UnicodeEscape => Some("Mode"),
            Self::Url => Some("Mode"),
            Self::Hash => Some("Algorithm"),
            Self::MockJson => Some("Record Count"),
            _ => None,
        }
    }

    pub fn is_svg_output(self) -> bool {
        matches!(self, Self::Qr)
    }

    pub fn execute(self, source: &str, secondary: &str) -> Result<String, String> {
        match self {
            Self::Xml | Self::Html => formatting::format_markup(source),
            Self::Yaml => formatting::format_yaml(source),
            Self::Toml => formatting::format_toml(source),
            Self::Sql => formatting::format_sql(source),
            Self::Css | Self::Javascript => formatting::format_braced(source),
            Self::Regex => text::regex_test(source, secondary),
            Self::Url => encoding::url(source, secondary),
            Self::Hash => encoding::hash(source, secondary),
            Self::Uuid => generators::uuid(),
            Self::Timestamp => time::timestamp(source),
            Self::Color => text::color(source),
            Self::Cron => time::cron(source),
            Self::HttpStatus => network::http_status(source),
            Self::Subnet => network::subnet(source),
            Self::Qr => generators::qr(source),
            Self::JsonDiff => data::json_diff(source, secondary),
            Self::JsonPath => data::json_path(source, secondary),
            Self::JsonToType => data::json_to_type(source, secondary),
            Self::Curl => network::curl(source),
            Self::HttpHeaders => network::headers(source),
            Self::OpenApi => data::openapi(source),
            Self::SqlToEntity => data::sql_to_entity(source, secondary),
            Self::Git => text::git(source),
            Self::Gitignore => text::gitignore(source),
            Self::Chmod => text::chmod(source),
            Self::Mime => network::mime(source),
            Self::TextDiff => text::text_diff(source, secondary),
            Self::FakeData => generators::fake_data(source),
            Self::MockJson => generators::mock_json(source, secondary),
            Self::NumberBase => encoding::number_base(source, secondary),
            Self::HtmlEntity => encoding::html_entity(source, secondary),
            Self::UnicodeEscape => encoding::unicode_escape(source, secondary),
        }
    }
}

pub fn execute(tool: ToolId, source: &str, secondary: &str) -> Result<String, String> {
    tool.execute(source, secondary)
}
