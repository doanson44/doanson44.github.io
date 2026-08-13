use leptos::prelude::*;

use crate::application::services::developer::DeveloperToolsService;
use crate::domain::developer::ToolId;
use crate::infrastructure::browser::{storage_get, storage_set};

#[derive(Clone, Copy)]
pub struct DeveloperToolsState {
    pub source: RwSignal<String>,
    pub secondary: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub copied: RwSignal<bool>,
}

impl DeveloperToolsState {
    pub fn new(tool: ToolId) -> Self {
        let (source, secondary) = tool.sample();
        let source_key = format!("developer-tool-{}-source", tool.route());
        let secondary_key = format!("developer-tool-{}-secondary", tool.route());
        Self {
            source: RwSignal::new(storage_get(&source_key).unwrap_or_else(|| source.to_string())),
            secondary: RwSignal::new(storage_get(&secondary_key).unwrap_or_else(|| secondary.to_string())),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            copied: RwSignal::new(false),
        }
    }

    pub fn set_source(&self, tool: ToolId, value: String) {
        let key = format!("developer-tool-{}-source", tool.route());
        let _ = storage_set(&key, &value);
        self.source.set(value);
        self.copied.set(false);
    }

    pub fn set_secondary(&self, tool: ToolId, value: String) {
        let key = format!("developer-tool-{}-secondary", tool.route());
        let _ = storage_set(&key, &value);
        self.secondary.set(value);
        self.copied.set(false);
    }

    pub fn run(&self, tool: ToolId) {
        self.copied.set(false);
        match DeveloperToolsService::execute(tool, &self.source.get(), &self.secondary.get()) {
            Ok(output) => { self.output.set(output); self.error.set(None); }
            Err(error) => { self.output.set(String::new()); self.error.set(Some(error)); }
        }
    }

    pub fn reset(&self, tool: ToolId) {
        let (source, secondary) = tool.sample();
        self.set_source(tool, source.to_string());
        self.set_secondary(tool, secondary.to_string());
        self.output.set(String::new());
        self.error.set(None);
    }

    pub fn clear(&self, tool: ToolId) {
        self.set_source(tool, String::new());
        self.set_secondary(tool, String::new());
        self.output.set(String::new());
        self.error.set(None);
    }
}
