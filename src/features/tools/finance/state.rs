use leptos::prelude::*;

use crate::domain::finance::{FinanceResult, FinanceTool};

/// Reactive state for a finance calculator.
pub struct FinanceState {
    /// Current numeric input strings.
    pub inputs: RwSignal<Vec<String>>,
    /// Optional comma/newline-separated cash flows.
    pub series: RwSignal<String>,
    /// Last successful result.
    pub result: RwSignal<Option<FinanceResult>>,
    /// Current validation error.
    pub error: RwSignal<Option<String>>,
}

impl FinanceState {
    /// Creates state initialized for a calculator.
    pub fn new(tool: FinanceTool) -> Self {
        Self { inputs: RwSignal::new(vec![String::new(); tool.fields().len()]), series: RwSignal::new(String::new()), result: RwSignal::new(None), error: RwSignal::new(None) }
    }
}
