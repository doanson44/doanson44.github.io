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

/// Normalized market data returned by the CafeF market-data endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketResponse {
    pub total_items: usize,
    pub displayed_items: usize,
    pub data: Vec<MarketStock>,
}

#[derive(Debug, Deserialize)]
struct CafeFMarketResponse {
    #[serde(rename = "Data")]
    data: Vec<MarketStock>,
    #[serde(rename = "Success")]
    success: bool,
    #[serde(rename = "Message")]
    message: Option<String>,
}

/// Parses a CafeF market-data response into normalized domain data.
pub fn parse_market_response(raw: &str) -> Result<MarketResponse, String> {
    let response: CafeFMarketResponse = serde_json::from_str(raw)
        .map_err(|error| format!("Invalid CafeF market response: {error}"))?;

    if !response.success {
        return Err(response
            .message
            .unwrap_or_else(|| "CafeF market request was unsuccessful".to_string()));
    }

    let total_items = response.data.len();

    Ok(MarketResponse {
        total_items,
        displayed_items: total_items,
        data: response.data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cafef_market_response() {
        let response = parse_market_response(
            r#"{"Data":[{"Symbol":"TCB","Result":17998400,"Color":1,"Price":33.05,"ChangePercent":2.32,"Change":0.75,"Name":"Techcombank","TotalVolume":17998400,"TotalValue":33.05,"MarketCap":234200245682699.97}],"Success":true,"Message":null}"#,
        )
        .expect("valid market response should parse");

        assert_eq!(response.total_items, 1);
        assert_eq!(response.displayed_items, 1);
        assert_eq!(response.data[0].symbol, "TCB");
        assert_eq!(response.data[0].price, 33.05);
        assert_eq!(response.data[0].change_percent, 2.32);
        assert_eq!(response.data[0].total_volume, 17998400.0);
    }

    #[test]
    fn rejects_unsuccessful_response() {
        let result = parse_market_response(
            r#"{"Data":[],"Success":false,"Message":"Market data unavailable"}"#,
        );

        assert_eq!(result.unwrap_err(), "Market data unavailable");
    }

    #[test]
    fn rejects_invalid_response() {
        assert!(parse_market_response("not-json").is_err());
    }
}
