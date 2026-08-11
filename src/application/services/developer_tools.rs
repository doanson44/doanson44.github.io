use crate::domain::developer_tools::{run, ToolKind};

pub struct DeveloperToolsService;

impl DeveloperToolsService {
    pub fn execute(kind: ToolKind, source: &str, secondary: &str) -> Result<String, String> {
        run(kind, source, secondary)
    }
}
