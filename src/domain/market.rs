use serde::Deserialize;

/// A stock item returned by the CafeF market-data endpoint.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarketStock {
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "Price")]
    pub price: f64,
    #[serde(rename = "ChangePercent")]
    pub change_percent: f64,
    #[serde(rename = "Change")]
    pub change: f64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "TotalVolume")]
    pub total_volume: f64,
    #[serde(rename = "MarketCap")]
    pub market_cap: f64,
}

/// The market response returned by the CafeF market-data endpoint.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarketResponse {
    #[serde(rename = "totalItems")]
    pub total_items: usize,
    #[serde(rename = "displayedItems")]
    pub displayed_items: usize,
    #[serde(rename = "limitApplied")]
    pub limit_applied: usize,
    #[serde(rename = "data")]
    pub data: Vec<MarketStock>,
}

/// Parses a CafeF market-data response into domain data.
pub fn parse_market_response(raw: &str) -> Result<MarketResponse, String> {
    serde_json::from_str(raw).map_err(|error| format!("Invalid CafeF market response: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_market_response() {
        let response = parse_market_response(
            r#"{"totalItems":1,"displayedItems":1,"limitApplied":1,"data":[{"Symbol":"TCB","Result":17998400,"Color":1,"Price":33.05,"ChangePercent":2.32,"Change":0.75,"Name":"Techcombank","TotalVolume":17998400,"TotalValue":33.05,"MarketCap":234200245682699.97}]}"#,
        ).expect("valid market response should parse");
        assert_eq!(response.total_items, 1);
        assert_eq!(response.data[0].symbol, "TCB");
        assert_eq!(response.data[0].price, 33.05);
        assert_eq!(response.data[0].change_percent, 2.32);
        assert_eq!(response.data[0].total_volume, 17998400.0);
    }

    #[test]
    fn rejects_invalid_response() {
        assert!(parse_market_response("not-json").is_err());
    }
}
