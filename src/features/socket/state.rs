use leptos::prelude::*;

/// Reactive state for the MEXC Futures market table.
#[derive(Clone)]
pub struct SocketState {
    pub search: RwSignal<String>,
}

impl SocketState {
    /// Creates the socket feature state.
    pub fn new() -> Self {
        Self {
            search: RwSignal::new(String::new()),
        }
    }
}
