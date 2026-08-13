use markdown_studio::domain::developer::ToolId;

#[test]
fn yaml_formatter_rejects_invalid_input() {
    let result = ToolId::Yaml.execute("name: [", "");
    assert!(result.is_err());
}

#[test]
fn toml_formatter_normalizes_valid_document() {
    let result = ToolId::Toml
        .execute("[package]\nname=\"demo\"\nversion=\"1.0.0\"", "")
        .expect("valid TOML should format");
    assert!(result.contains("name = \"demo\""));
}

#[test]
fn openapi_viewer_accepts_yaml() {
    let source = "openapi: 3.0.3\ninfo:\n  title: Example\n  version: 1.0.0\npaths:\n  /users:\n    get:\n      summary: List users\n      responses:\n        '200':\n          description: OK\n";
    let result = ToolId::OpenApi
        .execute(source, "")
        .expect("valid OpenAPI YAML should parse");
    assert!(result.contains("GET /users List users"));
}

#[test]
fn openapi_viewer_rejects_missing_version() {
    let result = ToolId::OpenApi.execute("info:\n  title: Example\n", "");
    assert!(result.is_err());
}

#[test]
fn sql_entity_parser_keeps_commas_inside_type_arguments() {
    let source = "CREATE TABLE payments (id BIGINT, amount DECIMAL(10,2), name VARCHAR(255));";
    let result = ToolId::SqlToEntity
        .execute(source, "Rust")
        .expect("valid CREATE TABLE should parse");
    assert!(result.contains("pub amount: f64"));
    assert!(result.contains("pub name: String"));
}
