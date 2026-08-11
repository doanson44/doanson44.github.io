use leptos::prelude::*;

use crate::application::services::base64::Base64Service;

const STORAGE_KEY: &str = "base64-content";
const SAMPLE_TEXT: &str = "Hello, world!\nXin chào, Son!";

/// Reactive state for the Base64 tool.
#[derive(Clone, Copy)]
pub struct Base64State {
    pub source: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
}

impl Default for Base64State {
    fn default() -> Self {
        Self::new()
    }
}

impl Base64State {
    /// Create state from localStorage or the default sample.
    pub fn new() -> Self {
        let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten());
        let initial_content = storage
            .as_ref()
            .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
            .unwrap_or_else(|| SAMPLE_TEXT.to_string());

        let state = Self {
            source: RwSignal::new(initial_content),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
        };
        state.encode();
        state
    }

    fn save_content(content: &str) {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            let _ = storage.set_item(STORAGE_KEY, content);
        }
    }

    /// Update the source text and persist it locally.
    pub fn set_content(&self, content: String) {
        Self::save_content(&content);
        self.source.set(content);
        self.encode();
    }

    /// Encode the current source as Base64.
    pub fn encode(&self) {
        self.output
            .set(Base64Service::encode(&self.source.get_untracked()));
        self.error.set(None);
    }

    /// Decode the current source from Base64.
    pub fn decode(&self) {
        match Base64Service::decode(&self.source.get_untracked()) {
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

    /// Clear the source and output.
    pub fn clear(&self) {
        self.set_content(String::new());
        self.output.set(String::new());
        self.error.set(None);
    }

    /// Restore the default sample and encode it.
    pub fn reset(&self) {
        self.set_content(SAMPLE_TEXT.to_string());
    }
}
