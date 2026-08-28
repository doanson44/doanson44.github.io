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
        registry::get(self).map(|tool| tool.route).unwrap_or("")
    }
    pub fn title(self) -> &'static str {
        registry::get(self).map(|tool| tool.title).unwrap_or("")
    }
    pub fn description(self) -> &'static str {
        registry::get(self)
            .map(|tool| tool.description)
            .unwrap_or("")
    }

    pub fn sample(self) -> (&'static str, &'static str) {
        registry::get(self)
            .map(|tool| (tool.sample_source, tool.sample_secondary))
            .unwrap_or(("", ""))
    }

    pub fn secondary_label(self) -> Option<&'static str> {
        registry::get(self).and_then(|tool| tool.secondary_label)
    }
    pub fn secondary_options(self) -> Option<&'static [(&'static str, &'static str)]> {
        registry::get(self).and_then(|tool| tool.secondary_options)
    }
    pub fn is_svg_output(self) -> bool {
        registry::get(self).is_some_and(|tool| tool.svg_output)
    }
    pub fn all() -> impl Iterator<Item = Self> {
        registry::TOOLS.iter().map(|definition| definition.id)
    }

    pub fn execute(self, source: &str, secondary: &str) -> Result<String, String> {
        let tool =
            registry::get(self).ok_or_else(|| "Developer tool is not registered.".to_string())?;
        (tool.execute)(source, secondary)
    }
}

pub fn execute(tool: ToolId, source: &str, secondary: &str) -> Result<String, String> {
    tool.execute(source, secondary)
}
