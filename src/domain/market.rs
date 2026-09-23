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


/// One historical CafeF price record normalized for technical analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketPriceHistoryCandle {
    pub symbol: String,
    pub trade_date: String,
    pub basic_price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub ceiling: Option<f64>,
    pub floor: Option<f64>,
    pub total_value: Option<f64>,
}

/// Historical price data returned by CafeF for one symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketPriceHistory {
    pub symbol: String,
    pub candles: Vec<MarketPriceHistoryCandle>,
}

#[derive(Debug, Deserialize)]
struct CafeFPriceHistoryRecord {
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "TradeDate")]
    trade_date: String,
    #[serde(rename = "BasicPrice")]
    basic_price: f64,
    #[serde(rename = "OpenPrice")]
    open: f64,
    #[serde(rename = "HighPrice")]
    high: f64,
    #[serde(rename = "LowPrice")]
    low: f64,
    #[serde(rename = "ClosePrice")]
    close: f64,
    #[serde(rename = "Volume")]
    volume: f64,
    #[serde(rename = "Ceiling")]
    ceiling: Option<f64>,
    #[serde(rename = "Floor")]
    floor: Option<f64>,
    #[serde(rename = "TotalValue")]
    total_value: Option<f64>,
}

/// Parses CafeF historical price data into normalized domain data.
pub fn parse_price_history_response(raw: &str, symbol: &str) -> Result<MarketPriceHistory, String> {
    let records: Vec<CafeFPriceHistoryRecord> = serde_json::from_str(raw)
        .map_err(|error| format!("Invalid CafeF price-history response: {error}"))?;

    let requested_symbol = symbol.trim().to_ascii_uppercase();
    let mut candles = records
        .into_iter()
        .filter(|record| record.symbol.eq_ignore_ascii_case(&requested_symbol))
        .map(|record| MarketPriceHistoryCandle {
            symbol: record.symbol,
            trade_date: record.trade_date,
            basic_price: record.basic_price,
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            volume: record.volume,
            ceiling: record.ceiling,
            floor: record.floor,
            total_value: record.total_value,
        })
        .collect::<Vec<_>>();

    candles.sort_by(|left, right| left.trade_date.cmp(&right.trade_date));

    if candles.is_empty() {
        return Err(format!(
            "CafeF price history does not contain symbol {requested_symbol}"
        ));
    }

    Ok(MarketPriceHistory {
        symbol: requested_symbol,
        candles,
    })
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
    fn parses_price_history_for_requested_symbol() {
        let result = parse_price_history_response(
            r#"[{"Symbol":"VNM","TradeDate":"2026-09-22T00:00:00","BasicPrice":60.3,"OpenPrice":60.4,"HighPrice":61.3,"LowPrice":60.3,"ClosePrice":61.2,"Volume":1774600,"Ceiling":64.5,"Floor":56.1,"TotalValue":108291390000},{"Symbol":"FPT","TradeDate":"2026-09-22T00:00:00","BasicPrice":100.0,"OpenPrice":100.0,"HighPrice":101.0,"LowPrice":99.0,"ClosePrice":100.5,"Volume":1000,"Ceiling":105.0,"Floor":95.0,"TotalValue":100000}]",
            "VNM",
        )
        .expect("valid price history should parse");

        assert_eq!(result.symbol, "VNM");
        assert_eq!(result.candles.len(), 1);
        assert_eq!(result.candles[0].close, 61.2);
        assert_eq!(result.candles[0].volume, 1_774_600.0);
        assert_eq!(result.candles[0].ceiling, Some(64.5));
    }

    #[test]
    fn rejects_invalid_response() {
        assert!(parse_market_response("not-json").is_err());
    }
}
