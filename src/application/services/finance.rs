use crate::domain::finance::{FinanceResult, FinanceTool};

/// Application service coordinating finance calculations.
pub struct FinanceService;

impl FinanceService {
    /// Calculates a finance tool using validated numeric input and optional cash flows.
    pub fn calculate(tool: FinanceTool, inputs: &[f64], cash_flows: &[f64]) -> Result<FinanceResult, String> {
        tool.calculate(inputs, cash_flows)
    }
}
