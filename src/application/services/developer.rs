use crate::domain::developer::{execute, ToolId};

pub struct DeveloperToolsService;

impl DeveloperToolsService {
    pub fn execute(tool: ToolId, source: &str, secondary: &str) -> Result<String, String> {
        execute(tool, source, secondary)
    }
}
