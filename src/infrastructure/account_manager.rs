use crate::domain::account_manager::Project;
use crate::infrastructure::browser::{storage_get, storage_set};

const STORAGE_KEY: &str = "ms-account-manager.v1";

/// Browser localStorage adapter for account-manager data.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalAccountManagerStorage;

impl LocalAccountManagerStorage {
    pub fn load(&self) -> Result<Vec<Project>, String> {
        let Some(raw) = storage_get(STORAGE_KEY) else {
            return Ok(Vec::new());
        };
        serde_json::from_str(&raw).map_err(|error| format!("Could not read saved data: {error}"))
    }

    pub fn save(&self, projects: &[Project]) -> Result<(), String> {
        let raw = serde_json::to_string(projects)
            .map_err(|error| format!("Could not serialize data: {error}"))?;
        storage_set(STORAGE_KEY, &raw)
    }
}
