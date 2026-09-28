use leptos::prelude::*;

use crate::application::services::json::JsonService;

const STORAGE_KEY: &str = "json-content";
const SAMPLE_JSON: &str = r#"{
  "name": "JSON Formatter",
  "description": "A client-side JSON formatting tool",
  "features": [
    "Format",
    "Minify",
    "Validate"
  ],
  "active": true
}"#;

/// Reactive state for the JSON Formatter feature.
#[derive(Clone, Copy)]
pub struct JsonState {
    pub source: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub copied: RwSignal<bool>,
    pub sample_limit: RwSignal<usize>,
}

impl Default for JsonState {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonState {
    /// Create a new state, restoring saved content from localStorage or using the sample JSON.
    pub fn new() -> Self {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
        let initial_content = storage
            .as_ref()
            .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
            .unwrap_or_else(|| SAMPLE_JSON.to_string());

        Self {
            source: RwSignal::new(initial_content),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            copied: RwSignal::new(false),
            sample_limit: RwSignal::new(5),
        }
    }

    /// Save the current JSON source to localStorage.
    fn save_content(content: &str) {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(STORAGE_KEY, content);
        }
    }

    /// Update the JSON source and persist it to localStorage.
    pub fn set_content(&self, content: String) {
        Self::save_content(&content);
        self.source.set(content);
        self.copied.set(false);
    }

    pub fn format(&self) {
        self.run(JsonService::format);
    }

    pub fn minify(&self) {
        self.run(JsonService::minify);
    }

    /// Generate a response sample by recursively limiting every array to the selected size.
    pub fn sample(&self) {
        self.copied.set(false);
        let limit = self.sample_limit.get().max(1);
        match JsonService::sample(&self.source.get(), limit) {
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

    pub fn set_sample_limit(&self, limit: usize) {
        self.sample_limit.set(limit.clamp(1, 100));
    }

    fn run(&self, operation: fn(&str) -> Result<String, String>) {
        self.copied.set(false);
        match operation(&self.source.get()) {
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

    /// Clear the editor and persist the empty state.
    pub fn clear(&self) {
        self.set_content(String::new());
        self.output.set(String::new());
        self.error.set(None);
    }

    /// Restore the default sample JSON and clear the current result.
    pub fn reset(&self) {
        self.set_content(SAMPLE_JSON.to_string());
        self.output.set(String::new());
        self.error.set(None);
    }
}
