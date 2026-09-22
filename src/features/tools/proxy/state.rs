use std::rc::Rc;

use leptos::prelude::*;

use crate::application::services::proxy::ProxyService;
use crate::domain::proxy::ResponseMinimizationOptions;
use crate::infrastructure::proxy::ProxyApi;

/// Reactive state for the HTTP proxy playground.
#[derive(Clone, Copy)]
pub struct ProxyState {
    service: ProxyService<ProxyApi>,
    pub target_url: RwSignal<String>,
    pub max_array_items: RwSignal<String>,
    pub max_string_chars: RwSignal<String>,
    pub compact: RwSignal<bool>,
    pub output: RwSignal<String>,
    pub error: RwSignal<Option<String>>,
    pub loading: RwSignal<bool>,
    pub copied: RwSignal<bool>,
}

impl ProxyState {
    /// Creates the default proxy playground state.
    pub fn new() -> Self {
        Self {
            service: ProxyService::new(ProxyApi),
            target_url: RwSignal::new("https://jsonplaceholder.typicode.com/posts".to_string()),
            max_array_items: RwSignal::new("5".to_string()),
            max_string_chars: RwSignal::new("1000".to_string()),
            compact: RwSignal::new(false),
            output: RwSignal::new(String::new()),
            error: RwSignal::new(None),
            loading: RwSignal::new(false),
            copied: RwSignal::new(false),
        }
    }

    /// Starts a GET request through the proxy.
    pub fn run(&self) {
        let target_url = self.target_url.get();
        if target_url.trim().is_empty() {
            self.error.set(Some("Target URL is required.".into()));
            self.output.set(String::new());
            return;
        }

        let options = ResponseMinimizationOptions {
            max_array_items: parse_limit(&self.max_array_items.get()),
            max_string_chars: parse_limit(&self.max_string_chars.get()),
            compact: self.compact.get(),
        };

        self.loading.set(true);
        self.error.set(None);
        self.output.set(String::new());
        self.copied.set(false);

        let output = self.output;
        let error = self.error;
        let loading = self.loading;

        self.service.request_json(
            &target_url,
            options,
            Rc::new(move |result| {
                loading.set(false);
                match result {
                    Ok(value) => {
                        output.set(value);
                        error.set(None);
                    }
                    Err(message) => {
                        output.set(String::new());
                        error.set(Some(message));
                    }
                }
            }),
        );
    }

    /// Restores the default request and minimization options.
    pub fn reset(&self) {
        self.target_url
            .set("https://jsonplaceholder.typicode.com/posts".into());
        self.max_array_items.set("5".into());
        self.max_string_chars.set("1000".into());
        self.compact.set(false);
        self.output.set(String::new());
        self.error.set(None);
        self.loading.set(false);
        self.copied.set(false);
    }
}

fn parse_limit(value: &str) -> Option<usize> {
    if value == "all" {
        return None;
    }

    value.parse::<usize>().ok().filter(|limit| *limit > 0)
}
