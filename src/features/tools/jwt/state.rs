use leptos::prelude::*;
use serde_json::Value;

use crate::application::services::jwt::JwtService;

const STORAGE_KEY: &str = "jwt-content";
const SAMPLE_JWT: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

#[derive(Clone, Copy)]
pub struct JwtState {
    pub source: RwSignal<String>,
    pub header: RwSignal<Option<Value>>,
    pub payload: RwSignal<Option<Value>>,
    pub signature: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
}

impl Default for JwtState {
    fn default() -> Self {
        Self::new()
    }
}

impl JwtState {
    pub fn new() -> Self {
        let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten());
        let initial_content = storage
            .as_ref()
            .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
            .unwrap_or_else(|| SAMPLE_JWT.to_string());

        let state = Self {
            source: RwSignal::new(initial_content),
            header: RwSignal::new(None),
            payload: RwSignal::new(None),
            signature: RwSignal::new(None),
            error: RwSignal::new(None),
        };
        state.decode();
        state
    }

    fn save_content(content: &str) {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            let _ = storage.set_item(STORAGE_KEY, content);
        }
    }

    pub fn set_content(&self, content: String) {
        Self::save_content(&content);
        self.source.set(content);
        self.decode();
    }

    pub fn decode(&self) {
        match JwtService::decode(&self.source.get_untracked()) {
            Ok(decoded) => {
                self.header.set(Some(decoded.header));
                self.payload.set(Some(decoded.payload));
                self.signature.set(Some(decoded.signature));
                self.error.set(None);
            }
            Err(error) => {
                self.header.set(None);
                self.payload.set(None);
                self.signature.set(None);
                self.error.set(Some(error));
            }
        }
    }

    pub fn clear(&self) {
        self.set_content(String::new());
    }

    pub fn reset(&self) {
        self.set_content(SAMPLE_JWT.to_string());
    }
}
