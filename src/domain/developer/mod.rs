mod data;
mod encoding;
mod formatting;
mod generators;
mod network;
mod registry;
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
    pub fn from_route(route: &str) -> Option<Self> {
        registry::find_by_route(route)
    }

    pub fn route(self) -> &'static str {
        registry::get(self).route
    }

    pub fn title(self) -> &'static str {
        registry::get(self).title
    }

    pub fn description(self) -> &'static str {
        registry::get(self).description
    }

    pub fn sample(self) -> (&'static str, &'static str) {
        let definition = registry::get(self);
        (definition.sample_source, definition.sample_secondary)
    }

    pub fn secondary_label(self) -> Option<&'static str> {
        registry::get(self).secondary_label
    }

    pub fn is_svg_output(self) -> bool {
        registry::get(self).svg_output
    }

    pub fn all() -> impl Iterator<Item = Self> {
        registry::TOOLS.iter().map(|definition| definition.id)
    }

    pub fn execute(self, source: &str, secondary: &str) -> Result<String, String> {
        (registry::get(self).execute)(source, secondary)
    }
}

pub fn execute(tool: ToolId, source: &str, secondary: &str) -> Result<String, String> {
    tool.execute(source, secondary)
}
