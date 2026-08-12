use leptos::prelude::*;

use crate::application::services::developer_tools::DeveloperToolsService;
use crate::domain::developer_tools::{sample, ToolKind};

#[derive(Clone, Copy)]
pub struct DeveloperToolsState {
    pub source: RwSignal<String>,
    pub secondary: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub copied: RwSignal<bool>,
}

impl DeveloperToolsState {
    pub fn new(kind: ToolKind) -> Self {
        let (source, secondary) = sample(kind);
        let storage_key = format!("developer-tool-{}", kind.route());
        let saved = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item(&storage_key).ok().flatten());
        Self {
            source: RwSignal::new(saved.unwrap_or_else(|| source.to_string())),
            secondary: RwSignal::new(secondary.to_string()),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            copied: RwSignal::new(false),
        }
    }

    pub fn set_source(&self, kind: ToolKind, value: String) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(&format!("developer-tool-{}", kind.route()), &value);
        }
        self.source.set(value);
        self.copied.set(false);
    }

    pub fn set_secondary(&self, value: String) {
        self.secondary.set(value);
        self.copied.set(false);
    }

    pub fn run(&self, kind: ToolKind) {
        self.copied.set(false);
        match DeveloperToolsService::execute(kind, &self.source.get(), &self.secondary.get()) {
            Ok(output) => {
                self.output.set(output);
                self.error.set(None);
            }
            Err(error) => {
                self.output.set(String::new());
                self.error.set(Some(error));
            }
        }
    }

    pub fn reset(&self, kind: ToolKind) {
        let (source, secondary) = sample(kind);
        self.set_source(kind, source.to_string());
        self.secondary.set(secondary.to_string());
        self.output.set(String::new());
        self.error.set(None);
    }

    pub fn clear(&self, kind: ToolKind) {
        self.set_source(kind, String::new());
        self.secondary.set(String::new());
        self.output.set(String::new());
        self.error.set(None);
    }
}
