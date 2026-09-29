use leptos::prelude::*;

use crate::application::services::json::JsonService;
use crate::domain::json::{JsonGenerateOptions, JsonOverride};

const STORAGE_KEY: &str = "json-content";
const GENERATOR_STORAGE_KEY: &str = "json-generator-options";
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

#[derive(Clone, Debug, PartialEq)]
pub struct JsonOverrideDraft {
    pub path: String,
    pub value: String,
    pub value_type: String,
}

impl Default for JsonOverrideDraft {
    fn default() -> Self {
        Self {
            path: String::new(),
            value: String::new(),
            value_type: "string".to_string(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PersistedGenerator {
    count: usize,
    seed: u64,
    overrides: Vec<PersistedOverride>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PersistedOverride {
    path: String,
    value: String,
    value_type: String,
}

/// Reactive state for the JSON Formatter and dummy-data generator.
#[derive(Clone, Copy)]
pub struct JsonState {
    pub source: RwSignal<String>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub copied: RwSignal<bool>,
    pub sample_limit: RwSignal<usize>,
    pub generator_open: RwSignal<bool>,
    pub generate_count: RwSignal<usize>,
    pub generate_seed: RwSignal<String>,
    pub overrides: RwSignal<Vec<JsonOverrideDraft>>,
}

impl Default for JsonState {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonState {
    /// Create a new state, restoring saved JSON and generator settings from localStorage.
    pub fn new() -> Self {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
        let initial_content = storage
            .as_ref()
            .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
            .unwrap_or_else(|| SAMPLE_JSON.to_string());

        let persisted = storage
            .as_ref()
            .and_then(|s| s.get_item(GENERATOR_STORAGE_KEY).ok().flatten())
            .and_then(|value| serde_json::from_str::<PersistedGenerator>(&value).ok());

        let (generate_count, generate_seed, overrides) = persisted
            .map(|value| {
                (
                    value.count.clamp(1, 1000),
                    value.seed.to_string(),
                    value
                        .overrides
                        .into_iter()
                        .map(|item| JsonOverrideDraft {
                            path: item.path,
                            value: item.value,
                            value_type: item.value_type,
                        })
                        .collect(),
                )
            })
            .unwrap_or((10, "12345".to_string(), Vec::new()));

        Self {
            source: RwSignal::new(initial_content),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            copied: RwSignal::new(false),
            sample_limit: RwSignal::new(5),
            generator_open: RwSignal::new(false),
            generate_count: RwSignal::new(generate_count),
            generate_seed: RwSignal::new(generate_seed),
            overrides: RwSignal::new(overrides),
        }
    }

    fn storage() -> Option<web_sys::Storage> {
        web_sys::window().and_then(|w| w.local_storage().ok().flatten())
    }

    fn save_content(content: &str) {
        if let Some(storage) = Self::storage() {
            let _ = storage.set_item(STORAGE_KEY, content);
        }
    }

    fn save_generator(&self) {
        let seed = self.generate_seed.get().parse::<u64>().unwrap_or(12345);
        let persisted = PersistedGenerator {
            count: self.generate_count.get().clamp(1, 1000),
            seed,
            overrides: self
                .overrides
                .get()
                .into_iter()
                .map(|item| PersistedOverride {
                    path: item.path,
                    value: item.value,
                    value_type: item.value_type,
                })
                .collect(),
        };

        if let Ok(value) = serde_json::to_string(&persisted) {
            if let Some(storage) = Self::storage() {
                let _ = storage.set_item(GENERATOR_STORAGE_KEY, &value);
            }
        }
    }

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

    pub fn generate(&self) {
        self.copied.set(false);
        let seed = self.generate_seed.get().parse::<u64>().unwrap_or(12345);
        let drafts = self.overrides.get();
        let mut overrides = Vec::new();

        for draft in drafts {
            if draft.path.trim().is_empty() {
                continue;
            }

            let value = match draft.value_type.as_str() {
                "number" => match draft.value.trim().parse::<f64>() {
                    Ok(value) => match serde_json::Number::from_f64(value) {
                        Some(number) => serde_json::Value::Number(number),
                        None => {
                            self.output.set(String::new());
                            self.error
                                .set(Some(format!("Invalid number for {}.", draft.path.trim())));
                            return;
                        }
                    },
                    Err(_) => {
                        self.output.set(String::new());
                        self.error
                            .set(Some(format!("Invalid number for {}.", draft.path.trim())));
                        return;
                    }
                },
                "boolean" => match draft.value.trim().parse::<bool>() {
                    Ok(value) => serde_json::Value::Bool(value),
                    Err(_) => {
                        self.output.set(String::new());
                        self.error.set(Some(format!(
                            "Boolean value for {} must be true or false.",
                            draft.path.trim()
                        )));
                        return;
                    }
                },
                "null" => serde_json::Value::Null,
                "json" => match serde_json::from_str::<serde_json::Value>(&draft.value) {
                    Ok(value) => value,
                    Err(error) => {
                        self.output.set(String::new());
                        self.error.set(Some(format!(
                            "Invalid JSON value for {}: {}",
                            draft.path.trim(),
                            error
                        )));
                        return;
                    }
                },
                _ => serde_json::Value::String(draft.value),
            };

            overrides.push(JsonOverride {
                path: draft.path.trim().to_string(),
                value,
            });
        }

        let options = JsonGenerateOptions {
            count: self.generate_count.get().clamp(1, 1000),
            seed,
            overrides,
        };

        self.save_generator();

        match JsonService::generate(&self.source.get(), &options) {
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

    pub fn set_generate_count(&self, count: usize) {
        self.generate_count.set(count.clamp(1, 1000));
        self.save_generator();
    }

    pub fn set_generate_seed(&self, seed: String) {
        self.generate_seed.set(seed);
        self.save_generator();
    }

    pub fn add_override(&self) {
        self.overrides
            .update(|items| items.push(JsonOverrideDraft::default()));
        self.save_generator();
    }

    pub fn update_override(
        &self,
        index: usize,
        path: Option<String>,
        value: Option<String>,
        value_type: Option<String>,
    ) {
        self.overrides.update(|items| {
            if let Some(item) = items.get_mut(index) {
                if let Some(path) = path {
                    item.path = path;
                }
                if let Some(value) = value {
                    item.value = value;
                }
                if let Some(value_type) = value_type {
                    item.value_type = value_type;
                }
            }
        });
        self.save_generator();
    }

    pub fn remove_override(&self, index: usize) {
        self.overrides.update(|items| {
            if index < items.len() {
                items.remove(index);
            }
        });
        self.save_generator();
    }

    pub fn clear_overrides(&self) {
        self.overrides.set(Vec::new());
        self.save_generator();
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
        self.set_content(String::new());
        self.output.set(String::new());
        self.error.set(None);
    }

    pub fn reset(&self) {
        self.set_content(SAMPLE_JSON.to_string());
        self.output.set(String::new());
        self.error.set(None);
    }
}
