use leptos::prelude::*;

use crate::application::services::developer::DeveloperToolsService;
use crate::domain::developer::ToolId;

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
        let storage_key = format!("developer-tool-{}", tool.route());
        let saved = web_sys::window().and_then(|w| w.local_storage().ok().flatten()).and_then(|s| s.get_item(&storage_key).ok().flatten());
        Self { source: RwSignal::new(saved.unwrap_or_else(|| source.to_string())), secondary: RwSignal::new(secondary.to_string()), output: RwSignal::new(String::new()), error: RwSignal::new(None), copied: RwSignal::new(false) }
    }

    pub fn set_source(&self, tool: ToolId, value: String) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) { let _ = storage.set_item(&format!("developer-tool-{}", tool.route()), &value); }
        self.source.set(value); self.copied.set(false);
    }

    pub fn set_secondary(&self, value: String) { self.secondary.set(value); self.copied.set(false); }

    pub fn run(&self, tool: ToolId) {
        self.copied.set(false);
        match DeveloperToolsService::execute(tool, &self.source.get(), &self.secondary.get()) { Ok(output) => { self.output.set(output); self.error.set(None); }, Err(error) => { self.output.set(String::new()); self.error.set(Some(error)); } }
    }

    pub fn reset(&self, tool: ToolId) { let (source, secondary) = tool.sample(); self.set_source(tool, source.to_string()); self.secondary.set(secondary.to_string()); self.output.set(String::new()); self.error.set(None); }
    pub fn clear(&self, tool: ToolId) { self.set_source(tool, String::new()); self.secondary.set(String::new()); self.output.set(String::new()); self.error.set(None); }
}
