//! Pure finance calculations used by the Finance Toolkit.

/// Identifies one of the supported finance calculators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinanceTool {
    CompoundInterest, Loan, Mortgage, InvestmentReturn, PresentFutureValue, Roi, Cagr, BreakEven,
    Budget, SavingsGoal, EmergencyFund, DebtPayoff, NetWorth, Budget503020,
    Dca, StockReturn, Dividend, PortfolioAllocation, PositionSize, RealReturn,
    ProfitMargin, MarkupMargin, Ebitda, CashFlow, BurnRate, Runway, CacLtv,
    Dcf, Npv, Irr, BondYtm, FuturesPnl, OptionsPnl, RiskReward, LeverageLiquidation,
    CurrencyConverter, Inflation, PurchasingPower, CurrencyChange, Discount, TaxPrice, PercentageChange,
}

/// A single displayed calculator metric.
#[derive(Debug, Clone, PartialEq)]
pub struct FinanceMetric { pub label: String, pub value: f64 }

/// Calculator output containing one or more metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct FinanceResult { pub metrics: Vec<FinanceMetric> }

impl FinanceResult {
    fn one(label: &str, value: f64) -> Self { Self { metrics: vec![FinanceMetric { label: label.into(), value }] } }
    fn two(a: (&str, f64), b: (&str, f64)) -> Self { Self { metrics: vec![FinanceMetric { label: a.0.into(), value: a.1 }, FinanceMetric { label: b.0.into(), value: b.1 }] } }
    fn many(values: &[(&str, f64)]) -> Self { Self { metrics: values.iter().map(|(label, value)| FinanceMetric { label: (*label).into(), value: *value }).collect() } }
}

impl FinanceTool {
    /// Returns the URL slug for this calculator.
    pub const fn route(self) -> &'static str { match self {
        Self::CompoundInterest=>"compound-interest",Self::Loan=>"loan",Self::Mortgage=>"mortgage",Self::InvestmentReturn=>"investment-return",Self::PresentFutureValue=>"present-future-value",Self::Roi=>"roi",Self::Cagr=>"cagr",Self::BreakEven=>"break-even",
        Self::Budget=>"budget",Self::SavingsGoal=>"savings-goal",Self::EmergencyFund=>"emergency-fund",Self::DebtPayoff=>"debt-payoff",Self::NetWorth=>"net-worth",Self::Budget503020=>"budget-50-30-20",
        Self::Dca=>"dca",Self::StockReturn=>"stock-return",Self::Dividend=>"dividend",Self::PortfolioAllocation=>"portfolio-allocation",Self::PositionSize=>"position-size",Self::RealReturn=>"real-return",
        Self::ProfitMargin=>"profit-margin",Self::MarkupMargin=>"markup-margin",Self::Ebitda=>"ebitda",Self::CashFlow=>"cash-flow",Self::BurnRate=>"burn-rate",Self::Runway=>"runway",Self::CacLtv=>"cac-ltv",
        Self::Dcf=>"dcf",Self::Npv=>"npv",Self::Irr=>"irr",Self::BondYtm=>"bond-ytm",Self::FuturesPnl=>"futures-pnl",Self::OptionsPnl=>"options-pnl",Self::RiskReward=>"risk-reward",Self::LeverageLiquidation=>"leverage-liquidation",
        Self::CurrencyConverter=>"currency-converter",Self::Inflation=>"inflation",Self::PurchasingPower=>"purchasing-power",Self::CurrencyChange=>"currency-change",Self::Discount=>"discount",Self::TaxPrice=>"tax-price",Self::PercentageChange=>"percentage-change",
    }}

    /// Resolves a route slug.
    pub fn from_route(route: &str) -> Option<Self> { match route {
        "compound-interest"=>Some(Self::CompoundInterest),"loan"=>Some(Self::Loan),"mortgage"=>Some(Self::Mortgage),"investment-return"=>Some(Self::InvestmentReturn),"present-future-value"=>Some(Self::PresentFutureValue),"roi"=>Some(Self::Roi),"cagr"=>Some(Self::Cagr),"break-even"=>Some(Self::BreakEven),
        "budget"=>Some(Self::Budget),"savings-goal"=>Some(Self::SavingsGoal),"emergency-fund"=>Some(Self::EmergencyFund),"debt-payoff"=>Some(Self::DebtPayoff),"net-worth"=>Some(Self::NetWorth),"budget-50-30-20"=>Some(Self::Budget503020),
        "dca"=>Some(Self::Dca),"stock-return"=>Some(Self::StockReturn),"dividend"=>Some(Self::Dividend),"portfolio-allocation"=>Some(Self::PortfolioAllocation),"position-size"=>Some(Self::PositionSize),"real-return"=>Some(Self::RealReturn),
        "profit-margin"=>Some(Self::ProfitMargin),"markup-margin"=>Some(Self::MarkupMargin),"ebitda"=>Some(Self::Ebitda),"cash-flow"=>Some(Self::CashFlow),"burn-rate"=>Some(Self::BurnRate),"runway"=>Some(Self::Runway),"cac-ltv"=>Some(Self::CacLtv),
        "dcf"=>Some(Self::Dcf),"npv"=>Some(Self::Npv),"irr"=>Some(Self::Irr),"bond-ytm"=>Some(Self::BondYtm),"futures-pnl"=>Some(Self::FuturesPnl),"options-pnl"=>Some(Self::OptionsPnl),"risk-reward"=>Some(Self::RiskReward),"leverage-liquidation"=>Some(Self::LeverageLiquidation),
        "currency-converter"=>Some(Self::CurrencyConverter),"inflation"=>Some(Self::Inflation),"purchasing-power"=>Some(Self::PurchasingPower),"currency-change"=>Some(Self::CurrencyChange),"discount"=>Some(Self::Discount),"tax-price"=>Some(Self::TaxPrice),"percentage-change"=>Some(Self::PercentageChange), _=>None,
    }}

    /// Human-readable title.
    pub const fn title(self) -> &'static str { match self {
        Self::CompoundInterest=>"Compound Interest",Self::Loan=>"Loan Calculator",Self::Mortgage=>"Mortgage Calculator",Self::InvestmentReturn=>"Investment Return",Self::PresentFutureValue=>"Present / Future Value",Self::Roi=>"ROI Calculator",Self::Cagr=>"CAGR Calculator",Self::BreakEven=>"Break-even Calculator",
        Self::Budget=>"Budget Calculator",Self::SavingsGoal=>"Savings Goal",Self::EmergencyFund=>"Emergency Fund",Self::DebtPayoff=>"Debt Payoff",Self::NetWorth=>"Net Worth",Self::Budget503020=>"50 / 30 / 20 Budget",
        Self::Dca=>"DCA Calculator",Self::StockReturn=>"Stock Return",Self::Dividend=>"Dividend Calculator",Self::PortfolioAllocation=>"Portfolio Allocation",Self::PositionSize=>"Position Size",Self::RealReturn=>"Real Return",
        Self::ProfitMargin=>"Profit Margin",Self::MarkupMargin=>"Markup vs Margin",Self::Ebitda=>"EBITDA",Self::CashFlow=>"Cash Flow",Self::BurnRate=>"Burn Rate",Self::Runway=>"Runway",Self::CacLtv=>"CAC / LTV",
        Self::Dcf=>"DCF Valuation",Self::Npv=>"NPV Calculator",Self::Irr=>"IRR Calculator",Self::BondYtm=>"Bond YTM",Self::FuturesPnl=>"Futures P&L",Self::OptionsPnl=>"Options P&L",Self::RiskReward=>"Risk / Reward",Self::LeverageLiquidation=>"Leverage / Liquidation",
        Self::CurrencyConverter=>"Currency Converter",Self::Inflation=>"Inflation Calculator",Self::PurchasingPower=>"Purchasing Power",Self::CurrencyChange=>"Currency Change",Self::Discount=>"Discount Calculator",Self::TaxPrice=>"Tax-inclusive / Tax-exclusive",Self::PercentageChange=>"Percentage Change",
    }}

    /// Category used by the Finance landing page.
    pub const fn category(self) -> &'static str { match self {
        Self::CompoundInterest|Self::Loan|Self::Mortgage|Self::InvestmentReturn|Self::PresentFutureValue|Self::Roi|Self::Cagr|Self::BreakEven=>"Core Finance",
        Self::Budget|Self::SavingsGoal|Self::EmergencyFund|Self::DebtPayoff|Self::NetWorth|Self::Budget503020=>"Personal Finance",
        Self::Dca|Self::StockReturn|Self::Dividend|Self::PortfolioAllocation|Self::PositionSize|Self::RealReturn=>"Investment",
        Self::ProfitMargin|Self::MarkupMargin|Self::Ebitda|Self::CashFlow|Self::BurnRate|Self::Runway|Self::CacLtv=>"Business Finance",
        Self::Dcf|Self::Npv|Self::Irr|Self::BondYtm=>"Valuation",
        Self::FuturesPnl|Self::OptionsPnl|Self::RiskReward|Self::LeverageLiquidation=>"Trading",
        _=>"Currency & Utilities",
    }}

    /// Returns the input labels required by this calculator.
    pub const fn fields(self) -> &'static [&'static str] { match self {
        Self::CompoundInterest=>&["Principal","Annual rate (%)","Years","Compounds / year","Contribution","Contributions / year"],
        Self::Loan|Self::DebtPayoff=>&["Principal / balance","Annual rate (%)","Term / payment (years)","Payments / year"],
        Self::Mortgage=>&["Home price","Down payment","Annual rate (%)","Term (years)","Property tax / year","Insurance / year"],
        Self::InvestmentReturn|Self::Dca=>&["Initial investment","Contribution","Annual return (%)","Years","Contributions / year","Compounds / year"],
        Self::PresentFutureValue=>&["Future value","Rate (%)","Periods"],Self::Roi=>&["Initial cost","Final value","Additional costs"],Self::Cagr=>&["Initial value","Final value","Years"],Self::BreakEven=>&["Fixed costs","Variable cost / unit","Selling price / unit"],
        Self::Budget=>&["Monthly income","Monthly expenses","Monthly savings","Other monthly costs"],Self::SavingsGoal=>&["Target amount","Current savings","Monthly contribution","Annual return (%)"],Self::EmergencyFund=>&["Monthly essentials","Target months","Current fund"],Self::NetWorth=>&["Total assets","Total liabilities"],Self::Budget503020=>&["Monthly net income"],
        Self::StockReturn=>&["Buy price","Sell price","Quantity","Fees","Dividends"],Self::Dividend=>&["Shares","Share price","Dividend / share / year","Growth (%)"],Self::PortfolioAllocation=>&["Portfolio value","Target allocation (%)","Current allocation (%)"],Self::PositionSize=>&["Account size","Risk (%)","Entry price","Stop price"],Self::RealReturn=>&["Nominal return (%)","Inflation (%)"],
        Self::ProfitMargin=>&["Revenue","Cost"],Self::MarkupMargin=>&["Cost","Selling price"],Self::Ebitda=>&["Revenue","Operating expenses","Depreciation","Amortization"],Self::CashFlow=>&["Operating cash flow","Investing cash flow","Financing cash flow"],Self::BurnRate=>&["Starting cash","Ending cash","Months"],Self::Runway=>&["Cash","Monthly burn"],Self::CacLtv=>&["Sales + marketing spend","New customers","ARPA / month","Gross margin (%)","Lifetime (months)"],
        Self::Dcf=>&["Current cash flow","Growth (%)","Forecast years","Discount rate (%)","Terminal growth (%)"],Self::Npv=>&["Initial investment","Discount rate (%)"],Self::Irr=>&[],Self::BondYtm=>&["Face value","Price","Coupon rate (%)","Years","Payments / year"],
        Self::FuturesPnl=>&["Entry price","Exit price","Contract size","Contracts","Fees"],Self::OptionsPnl=>&["Underlying","Strike","Premium","Contracts","Multiplier","Call=1 / Put=0"],Self::RiskReward=>&["Entry","Stop loss","Take profit"],Self::LeverageLiquidation=>&["Position value","Equity","Leverage","Maintenance margin (%)","Entry price","Long=1 / Short=0"],
        Self::CurrencyConverter=>&["Amount","Exchange rate (target / base)"],Self::Inflation|Self::PurchasingPower=>&["Amount","Inflation (%)","Years"],Self::CurrencyChange=>&["Initial rate","Final rate"],Self::Discount=>&["Original price","Discount (%)"],Self::TaxPrice=>&["Price","Tax rate (%)"],Self::PercentageChange=>&["Original value","New value"],
    }}

    /// Calculates a tool using finite numeric inputs and optional cash-flow series.
    pub fn calculate(self, x: &[f64], series: &[f64]) -> Result<FinanceResult, String> {
        fn req(x:&[f64], n:usize)->Result<(),String>{if x.len()<n||x[..n].iter().any(|v|!v.is_finite()){Err("Please provide valid numeric inputs.".into())}else{Ok(())}}
        fn positive(v:f64,name:&str)->Result<(),String>{if v>0.0{Ok(())}else{Err(format!("{name} must be greater than zero."))}}
        req(x, self.fields().len())?;
        let r = match self {
            Self::CompoundInterest => { positive(x[0],"Principal")?; positive(x[2],"Years")?; positive(x[3],"Compounds / year")?; let n=x[2]*x[3]; let rate=x[1]/100.0/x[3]; let fv=x[0]*(1.0+rate).powf(n); let c=x[4].max(0.0); let cf=x[5].max(0.0); let contrib=if c>0.0&&cf>0.0{c*((1.0+x[1]/100.0/cf).powf(x[2]*cf)-1.0)/(x[1]/100.0/cf)}else{0.0}; FinanceResult::many(&[("Future value",fv+contrib),("Principal",x[0]),("Contributions",contrib),("Interest earned",fv-x[0])]) }
            Self::Loan|Self::DebtPayoff => { positive(x[0],"Balance")?; positive(x[2],"Term")?; positive(x[3],"Payments / year")?; let n=x[2]*x[3]; let rate=x[1]/100.0/x[3]; let p=if rate.abs()<1e-12{x[0]/n}else{x[0]*rate/(1.0-(1.0+rate).powf(-n))}; FinanceResult::many(&[("Payment",p),("Total payments",p*n),("Total interest",p*n-x[0]),("Periods",n)]) }
            Self::Mortgage => { positive(x[0]-x[1],"Loan amount")?; positive(x[3],"Term")?; let loan=x[0]-x[1]; let n=x[3]*12.0; let m=x[2]/100.0/12.0; let p=if m.abs()<1e-12{loan/n}else{loan*m/(1.0-(1.0+m).powf(-n))}; FinanceResult::many(&[("Loan amount",loan),("Principal + interest",p),("Property tax / month",x[4]/12.0),("Insurance / month",x[5]/12.0),("Total monthly estimate",p+x[4]/12.0+x[5]/12.0),("Total interest",p*n-loan)]) }
            Self::InvestmentReturn|Self::Dca => { positive(x[3],"Years")?; positive(x[4],"Contributions / year")?; let rate=x[2]/100.0/x[5].max(1.0); let n=x[3]*x[5].max(1.0); let base=x[0]*(1.0+rate).powf(n); let c=x[1].max(0.0); let contrib=if c>0.0&&rate.abs()>1e-12{c*((1.0+rate).powf(n)-1.0)/rate}else{c*n}; FinanceResult::many(&[("Ending value",base+contrib),("Initial investment",x[0]),("Contributions",c*x[3]*x[4]),("Investment gain",base+contrib-x[0]-c*x[3]*x[4])]) }
            Self::PresentFutureValue => { positive(x[0],"Future value")?; FinanceResult::two(("Present value",x[0]/(1.0+x[1]/100.0).powf(x[2])),("Future value",x[0])) }
            Self::Roi => { positive(x[0],"Initial cost")?; let gain=x[1]-x[0]-x[2]; FinanceResult::two(("Net gain",gain),("ROI (%)",gain/x[0]*100.0)) }
            Self::Cagr => { positive(x[0],"Initial value")?; positive(x[2],"Years")?; FinanceResult::one("CAGR (%)",((x[1]/x[0]).powf(1.0/x[2])-1.0)*100.0) }
            Self::BreakEven => { let margin=x[2]-x[1]; positive(margin,"Contribution margin")?; FinanceResult::two(("Break-even units",x[0]/margin),("Break-even revenue",x[0]/margin*x[2])) }
            Self::Budget => { let total=x[1]+x[3]; FinanceResult::many(&[("Total expenses",total),("Remaining",x[0]-total),("Savings rate (%)",if x[0]>0.0{x[2]/x[0]*100.0}else{0.0})]) }
            Self::SavingsGoal => { positive(x[0]-x[1],"Remaining goal")?; positive(x[2],"Monthly contribution")?; let monthly=x[3]/100.0/12.0; let months=if monthly.abs()<1e-12{(x[0]-x[1])/x[2]}else{((x[0]*monthly+x[2])/(x[1]*monthly+x[2])).ln()/(1.0+monthly).ln()}; FinanceResult::many(&[("Estimated months",months.max(0.0)),("Years",months.max(0.0)/12.0),("Remaining target",(x[0]-x[1]).max(0.0))]) }
            Self::EmergencyFund => { positive(x[0],"Monthly essentials")?; positive(x[1],"Target months")?; FinanceResult::many(&[("Target fund",x[0]*x[1]),("Current coverage (months)",x[2]/x[0]),("Remaining",(x[0]*x[1]-x[2]).max(0.0))]) }
            Self::NetWorth => FinanceResult::two(("Net worth",x[0]-x[1]),("Debt / asset ratio (%)",if x[0]>0.0{x[1]/x[0]*100.0}else{0.0})),
            Self::Budget503020 => FinanceResult::many(&[("Needs (50%)",x[0]*0.50),("Wants (30%)",x[0]*0.30),("Savings / debt (20%)",x[0]*0.20)]),
            Self::StockReturn => { positive(x[2],"Quantity")?; let profit=(x[1]-x[0])*x[2]+x[4]-x[3]; FinanceResult::two(("Net profit",profit),("Return (%)",if x[0]*x[2]>0.0{profit/(x[0]*x[2])*100.0}else{0.0})) }
            Self::Dividend => { positive(x[0],"Shares")?; let income=x[0]*x[2]; FinanceResult::many(&[("Annual dividend",income),("Yield (%)",if x[0]*x[1]>0.0{income/(x[0]*x[1])*100.0}else{0.0}),("Next-year dividend",income*(1.0+x[3]/100.0))]) }
            Self::PortfolioAllocation => { let target=x[0]*x[1]/100.0; let current=x[0]*x[2]/100.0; FinanceResult::two(("Target amount",target),("Rebalance amount",target-current)) }
            Self::PositionSize => { positive(x[0],"Account size")?; positive((x[2]-x[3]).abs(),"Entry-to-stop distance")?; let risk=x[0]*x[1]/100.0; FinanceResult::two(("Risk amount",risk),("Units",risk/(x[2]-x[3]).abs())) }
            Self::RealReturn => FinanceResult::one("Real return (%)",((1.0+x[0]/100.0)/(1.0+x[1]/100.0)-1.0)*100.0),
            Self::ProfitMargin => { positive(x[0],"Revenue")?; FinanceResult::two(("Profit",x[0]-x[1]),("Margin (%)",(x[0]-x[1])/x[0]*100.0)) }
            Self::MarkupMargin => { positive(x[0],"Cost")?; positive(x[1],"Selling price")?; FinanceResult::many(&[("Markup (%)",(x[1]-x[0])/x[0]*100.0),("Margin (%)",(x[1]-x[0])/x[1]*100.0)]) }
            Self::Ebitda => { let e=x[0]-x[1]; FinanceResult::two(("EBITDA",e),("EBITDA margin (%)",if x[0]!=0.0{e/x[0]*100.0}else{0.0})) }
            Self::CashFlow => FinanceResult::one("Net cash flow",x[0]+x[1]+x[2]),
            Self::BurnRate => { positive(x[2],"Months")?; FinanceResult::two(("Net burn / month",(x[0]-x[1])/x[2]),("Cash consumed",x[0]-x[1])) }
            Self::Runway => { positive(x[1],"Monthly burn")?; FinanceResult::one("Runway (months)",x[0]/x[1]) }
            Self::CacLtv => { positive(x[1],"New customers")?; let cac=x[0]/x[1]; let ltv=x[2]*(x[3]/100.0)*x[4]; FinanceResult::many(&[("CAC",cac),("LTV",ltv),("LTV / CAC",if cac>0.0{ltv/cac}else{0.0})]) }
            Self::Dcf => { positive(x[2],"Forecast years")?; let mut cf=x[0]; let mut pv=0.0; for year in 1..=(x[2] as u32){cf*=1.0+x[1]/100.0; pv+=cf/(1.0+x[3]/100.0).powi(year as i32);} let terminal=cf*(1.0+x[4]/100.0)/((x[3]-x[4])/100.0); let terminal_pv=terminal/(1.0+x[3]/100.0).powf(x[2]); FinanceResult::many(&[("PV of forecast",pv),("Terminal value",terminal),("PV of terminal value",terminal_pv),("Enterprise value",pv+terminal_pv)]) }
            Self::Npv => { if series.is_empty(){return Err("Provide cash flows in the series field.".into())}; let rate=x[1]/100.0; FinanceResult::one("NPV",-x[0]+series.iter().enumerate().map(|(i,v)|v/(1.0+rate).powi((i+1) as i32)).sum::<f64>()) }
            Self::Irr => { if series.len()<2{return Err("Provide at least two cash flows in the series field.".into())}; let mut lo=-0.9999; let mut hi=10.0; let f=|rate:f64|series.iter().enumerate().map(|(i,v)|v/(1.0+rate).powi(i as i32)).sum::<f64>(); if f(lo)*f(hi)>0.0{return Err("Unable to find an IRR in the supported range.".into())}; for _ in 0..200{let mid=(lo+hi)/2.0;if f(mid).abs()<1e-10{lo=mid;hi=mid;break}if f(lo)*f(mid)<=0.0{hi=mid}else{lo=mid}} FinanceResult::one("IRR (%)",((lo+hi)/2.0)*100.0) }
            Self::BondYtm => { positive(x[0],"Face value")?; positive(x[1],"Price")?; positive(x[3],"Years")?; positive(x[4],"Payments / year")?; let periods=x[3]*x[4]; let coupon=x[0]*x[2]/100.0/x[4]; let f=|rate:f64|coupon*(1.0-(1.0+rate).powf(-periods))/rate.max(1e-12)+x[0]/(1.0+rate).powf(periods)-x[1]; let mut lo=1e-9; let mut hi=2.0; for _ in 0..100{let mid=(lo+hi)/2.0;if f(mid)>0.0{lo=mid}else{hi=mid}} FinanceResult::two(("Coupon / period",coupon),("YTM (%)",((lo+hi)/2.0)*x[4]*100.0)) }
            Self::FuturesPnl => FinanceResult::one("Net P&L",(x[1]-x[0])*x[2]*x[3]-x[4]),
            Self::OptionsPnl => { let intrinsic=if x[5]>=0.5{(x[0]-x[1]).max(0.0)}else{(x[1]-x[0]).max(0.0)}; let pnl=(intrinsic-x[2])*x[3]*x[4]; FinanceResult::two(("P&L",pnl),("Breakeven",if x[5]>=0.5{x[1]+x[2]}else{x[1]-x[2]})) }
            Self::RiskReward => { let risk=(x[0]-x[1]).abs(); let reward=(x[2]-x[0]).abs(); positive(risk,"Risk")?; FinanceResult::two(("Risk",risk),("Risk / reward",reward/risk)) }
            Self::LeverageLiquidation => { positive(x[1],"Equity")?; let margin=x[3]/100.0; let move=(1.0-margin)/x[2].max(1.0); let liq=if x[5]>=0.5{x[4]*(1.0-move)}else{x[4]*(1.0+move)}; FinanceResult::many(&[("Initial margin",x[0]/x[2].max(1.0)),("Effective leverage",x[0]/x[1]),("Approx. liquidation price",liq)]) }
            Self::CurrencyConverter => { positive(x[1],"Exchange rate")?; FinanceResult::one("Converted amount",x[0]*x[1]) }
            Self::Inflation => FinanceResult::one("Future amount",x[0]*(1.0+x[1]/100.0).powf(x[2])),
            Self::PurchasingPower => FinanceResult::one("Future purchasing power",x[0]/(1.0+x[1]/100.0).powf(x[2])),
            Self::CurrencyChange => { positive(x[0],"Initial rate")?; FinanceResult::two(("Absolute change",x[1]-x[0]),("Change (%)",(x[1]-x[0])/x[0]*100.0)) }
            Self::Discount => { let discount=x[0]*x[1]/100.0; FinanceResult::two(("Discount amount",discount),("Final price",x[0]-discount)) }
            Self::TaxPrice => { let tax=x[0]*x[1]/100.0; FinanceResult::many(&[("Net price",x[0]),("Tax",tax),("Gross price",x[0]+tax)]) }
            Self::PercentageChange => { positive(x[0],"Original value")?; FinanceResult::two(("Absolute change",x[1]-x[0]),("Percentage change (%)",(x[1]-x[0])/x[0]*100.0)) }
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn roi_is_correct(){let r=FinanceTool::Roi.calculate(&[100.0,150.0,0.0],&[]).unwrap();assert!((r.metrics[1].value-50.0).abs()<1e-9);}
    #[test] fn cagr_is_correct(){let r=FinanceTool::Cagr.calculate(&[100.0,121.0,2.0],&[]).unwrap();assert!((r.metrics[0].value-10.0).abs()<1e-9);}
    #[test] fn break_even_rejects_invalid_margin(){assert!(FinanceTool::BreakEven.calculate(&[100.0,10.0,10.0],&[]).is_err());}
    #[test] fn percentage_change_is_correct(){let r=FinanceTool::PercentageChange.calculate(&[100.0,125.0],&[]).unwrap();assert!((r.metrics[1].value-25.0).abs()<1e-9);}
    #[test] fn irr_finds_known_rate(){let r=FinanceTool::Irr.calculate(&[-100.0,110.0],&[]).unwrap();assert!((r.metrics[0].value-10.0).abs()<1e-5);}
}
