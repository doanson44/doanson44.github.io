use leptos::prelude::*;

use crate::application::services::JsonService;

/// Reactive state for the JSON Formatter feature.
#[derive(Clone, Copy)]
pub struct JsonState {
    pub source: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub copied: RwSignal<bool>,
}

impl Default for JsonState {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonState {
    pub fn new() -> Self {
        Self {
            source: RwSignal::new(String::new()),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            copied: RwSignal::new(false),
        }
    }

    pub fn format(&self) {
        self.run(JsonService::format);
    }

    pub fn minify(&self) {
        self.run(JsonService::minify);
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

    pub fn clear(&self) {
        self.source.set(String::new());
        self.output.set(String::new());
        self.error.set(None);
        self.copied.set(false);
    }
}
