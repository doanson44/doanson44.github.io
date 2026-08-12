use crate::domain::developer_tools::{run, ToolKind};

pub struct DeveloperToolsService;

impl DeveloperToolsService {
    pub fn execute(kind: ToolKind, source: &str, secondary: &str) -> Result<String, String> {
        if kind == ToolKind::Url && secondary == "encode" {
            return Ok(Self::encode_url(source));
        }
        run(kind, source, secondary)
    }

    fn encode_url(input: &str) -> String {
        input
            .bytes()
            .map(|byte| {
                if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
                    (byte as char).to_string()
                } else {
                    format!("%{byte:02X}")
                }
            })
            .collect()
    }
}
