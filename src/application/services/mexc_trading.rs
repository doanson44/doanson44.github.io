use std::rc::Rc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::application::ports::ProxyClient;
use crate::domain::trading::{PositionSide, RealFundingRecord, RealPosition};

#[derive(Debug, Clone, Copy)]
pub struct MexcFuturesTradingService<C> {
    client: C,
}

impl<C> MexcFuturesTradingService<C>
where
    C: ProxyClient + Copy + 'static,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn fetch_positions(
        &self,
        api_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        on_result: Rc<dyn Fn(Result<Vec<RealPosition>, String>)>,
    ) {
        let target_url = format!(
            "{}/api/v1/private/position/open_positions",
            api_url.trim().trim_end_matches('/')
        );
        self.authenticated_request(
            &target_url,
            api_key,
            api_secret,
            request_time_ms,
            None,
            Rc::new(move |result| on_result(result.and_then(parse_positions_response))),
        );
    }

    /// Fetches open positions and the actual funding settlements recorded for each one.
    pub fn fetch_positions_with_funding(
        &self,
        api_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        on_result: Rc<dyn Fn(Result<Vec<RealPosition>, String>)>,
    ) {
        let service = *self;
        let api_url_owned = api_url.to_string();
        let api_key_owned = api_key.to_string();
        let api_secret_owned = api_secret.to_string();
        self.fetch_positions(
            api_url,
            api_key,
            api_secret,
            request_time_ms,
            Rc::new(move |result| match result {
                Ok(positions) => fetch_position_funding_records(
                    service,
                    api_url_owned.clone(),
                    api_key_owned.clone(),
                    api_secret_owned.clone(),
                    request_time_ms,
                    positions,
                    0,
                    on_result.clone(),
                ),
                Err(error) => on_result(Err(error)),
            }),
        );
    }

    /// Fetches all funding-record pages for one MEXC position.
    pub fn fetch_funding_records(
        &self,
        api_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        position_id: i64,
        symbol: &str,
        on_result: Rc<dyn Fn(Result<Vec<RealFundingRecord>, String>)>,
    ) {
        fetch_funding_records_page(
            *self,
            api_url.to_string(),
            api_key.to_string(),
            api_secret.to_string(),
            request_time_ms,
            position_id,
            symbol.to_string(),
            1,
            0,
            Vec::new(),
            on_result,
        );
    }

    pub fn fetch_contract(
        &self,
        api_url: &str,
        symbol: &str,
        on_result: Rc<dyn Fn(Result<ContractDetail, String>)>,
    ) {
        let target_url = format!(
            "{}/api/v1/contract/detail?symbol={}",
            api_url.trim().trim_end_matches('/'),
            symbol
        );
        self.client.fetch(
            &target_url,
            Rc::new(move |result| on_result(result.and_then(parse_contract_response))),
        );
    }

    pub fn submit_market_order(
        &self,
        api_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        request: MarketOrderRequest,
        on_result: Rc<dyn Fn(Result<i64, String>)>,
    ) {
        let target_url = format!(
            "{}/api/v1/private/order/submit",
            api_url.trim().trim_end_matches('/')
        );
        let body = match serde_json::to_string(&request) {
            Ok(body) => body,
            Err(error) => {
                on_result(Err(format!("Failed to encode MEXC order: {error}")));
                return;
            }
        };

        self.authenticated_request(
            &target_url,
            api_key,
            api_secret,
            request_time_ms,
            Some(body),
            Rc::new(move |result| on_result(result.and_then(parse_order_response))),
        );
    }

    fn authenticated_request(
        &self,
        target_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        body: Option<String>,
        on_result: Rc<dyn Fn(Result<String, String>)>,
    ) {
        let api_key = api_key.trim().to_string();
        let request_time = request_time_ms.to_string();
        let signature_payload = match &body {
            Some(body) => format!("{api_key}{request_time}{body}"),
            None => format!("{api_key}{request_time}"),
        };
        let signature = hmac_sha256_hex(api_secret.trim().as_bytes(), signature_payload.as_bytes());
        let headers = vec![
            ("ApiKey".to_string(), api_key),
            ("Request-Time".to_string(), request_time),
            ("Signature".to_string(), signature),
            ("Content-Type".to_string(), "application/json".to_string()),
        ];
        let method = if body.is_some() { "POST" } else { "GET" };
        self.client
            .request(target_url, method, headers, body, on_result);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MarketOrderRequest {
    pub symbol: String,
    pub price: f64,
    pub vol: f64,
    pub leverage: u32,
    pub side: u8,
    #[serde(rename = "type")]
    pub order_type: u8,
    #[serde(rename = "openType")]
    pub open_type: u8,
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ContractDetail {
    #[serde(rename = "contractSize")]
    pub contract_size: f64,
    #[serde(rename = "volUnit")]
    pub vol_unit: f64,
    #[serde(rename = "minVol")]
    pub min_vol: f64,
    #[serde(rename = "maxVol")]
    pub max_vol: f64,
    pub state: i32,
    #[serde(rename = "apiAllowed", default = "default_api_allowed")]
    pub api_allowed: bool,
}

#[derive(Debug, Deserialize)]
struct ContractResponse {
    success: bool,
    code: i64,
    message: Option<String>,
    data: ContractDetail,
}

#[derive(Debug, Deserialize)]
struct PositionsResponse {
    success: bool,
    code: i64,
    message: Option<String>,
    data: Vec<MexcPosition>,
}

#[derive(Debug, Deserialize)]
struct MexcPosition {
    #[serde(rename = "positionId")]
    position_id: i64,
    symbol: String,
    #[serde(rename = "positionType")]
    position_type: i32,
    #[serde(rename = "holdVol")]
    hold_vol: f64,
    #[serde(rename = "holdAvgPrice")]
    hold_avg_price: f64,
    #[serde(rename = "closeAvgPrice")]
    close_avg_price: f64,
    #[serde(rename = "liquidatePrice")]
    liquidate_price: f64,
    im: f64,
    leverage: f64,
    realised: f64,
    #[serde(default)]
    unrealized: f64,
}

#[derive(Debug, Deserialize)]
struct FundingRecordsResponse {
    success: bool,
    code: i64,
    message: Option<String>,
    data: FundingRecordsPage,
}

#[derive(Debug, Deserialize)]
struct FundingRecordsPage {
    #[serde(rename = "totalPage", default)]
    total_page: u32,
    #[serde(rename = "resultList", default)]
    result_list: Vec<MexcFundingRecord>,
}

#[derive(Debug, Deserialize)]
struct MexcFundingRecord {
    id: i64,
    symbol: String,
    #[serde(rename = "positionId")]
    position_id: i64,
    #[serde(rename = "positionType")]
    position_type: i32,
    #[serde(rename = "positionValue")]
    position_value: f64,
    funding: f64,
    rate: f64,
    #[serde(rename = "settleTime")]
    settle_time_ms: i64,
}

#[derive(Debug, Deserialize)]
struct OrderResponse {
    success: bool,
    code: i64,
    message: Option<String>,
    data: Option<i64>,
}

fn parse_positions_response(raw: String) -> Result<Vec<RealPosition>, String> {
    let response: PositionsResponse = serde_json::from_str(&raw)
        .map_err(|error| format!("Invalid MEXC Futures position response: {error}"))?;
    if !response.success || response.code != 0 {
        return Err(response.message.unwrap_or_else(|| {
            format!(
                "MEXC Futures position request failed with code {}.",
                response.code
            )
        }));
    }
    Ok(response
        .data
        .into_iter()
        .filter(|position| position.hold_vol > 0.0)
        .map(|position| RealPosition {
            position_id: position.position_id,
            symbol: position.symbol,
            side: if position.position_type == 2 {
                PositionSide::Short
            } else {
                PositionSide::Long
            },
            hold_volume: position.hold_vol,
            open_average_price: position.hold_avg_price,
            close_average_price: position.close_avg_price,
            liquidation_price: position.liquidate_price,
            initial_margin: position.im,
            leverage: position.leverage,
            realized_pnl: position.realised,
            unrealized_pnl: position.unrealized,
            funding_records: Vec::new(),
        })
        .collect())
}

fn fetch_position_funding_records<C>(
    service: MexcFuturesTradingService<C>,
    api_url: String,
    api_key: String,
    api_secret: String,
    request_time_ms: i64,
    mut positions: Vec<RealPosition>,
    index: usize,
    on_result: Rc<dyn Fn(Result<Vec<RealPosition>, String>)>,
)
where
    C: ProxyClient + Copy + 'static,
{
    if index >= positions.len() {
        on_result(Ok(positions));
        return;
    }

    let position_id = positions[index].position_id;
    let symbol = positions[index].symbol.clone();
    let next_api_url = api_url.clone();
    let next_api_key = api_key.clone();
    let next_api_secret = api_secret.clone();
    service.fetch_funding_records(
        &api_url,
        &api_key,
        &api_secret,
        request_time_ms,
        position_id,
        &symbol,
        Rc::new(move |result| match result {
            Ok(records) => {
                let mut next_positions = positions.clone();
                next_positions[index].funding_records = records;
                fetch_position_funding_records(
                    service,
                    next_api_url,
                    next_api_key,
                    next_api_secret,
                    request_time_ms,
                    next_positions,
                    index + 1,
                    on_result.clone(),
                );
            }
            Err(error) => on_result(Err(error)),
        }),
    );
}

fn fetch_funding_records_page<C>(
    service: MexcFuturesTradingService<C>,
    api_url: String,
    api_key: String,
    api_secret: String,
    request_time_ms: i64,
    position_id: i64,
    symbol: String,
    page: u32,
    total_pages: u32,
    mut records: Vec<RealFundingRecord>,
    on_result: Rc<dyn Fn(Result<Vec<RealFundingRecord>, String>)>,
)
where
    C: ProxyClient + Copy + 'static,
{
    let target_url = format!(
        "{}/api/v1/private/position/funding_records?symbol={}&position_id={}&page_num={}&page_size=100",
        api_url.trim().trim_end_matches('/'),
        symbol,
        position_id,
        page
    );
    service.authenticated_request(
        &target_url,
        &api_key,
        &api_secret,
        request_time_ms,
        None,
        Rc::new(move |result| match result {
            Ok(raw) => match parse_funding_records_response(raw, &symbol) {
                Ok((reported_pages, page_records)) => {
                    let mut all_records = records.clone();
                    all_records.extend(page_records);
                    let pages = reported_pages.max(total_pages).max(page);
                    if page < pages {
                        fetch_funding_records_page(
                            service,
                            api_url.clone(),
                            api_key.clone(),
                            api_secret.clone(),
                            request_time_ms,
                            position_id,
                            symbol.clone(),
                            page + 1,
                            pages,
                            all_records,
                            on_result.clone(),
                        );
                    } else {
                        all_records.sort_by_key(|record| record.settle_time_ms);
                        all_records.dedup_by_key(|record| record.id);
                        on_result(Ok(all_records));
                    }
                }
                Err(error) => on_result(Err(error)),
            },
            Err(error) => on_result(Err(error)),
        }),
    );
}

fn parse_funding_records_response(
    raw: String,
    symbol: &str,
) -> Result<(u32, Vec<RealFundingRecord>), String> {
    let response: FundingRecordsResponse = serde_json::from_str(&raw)
        .map_err(|error| format!("Invalid MEXC funding records response for {symbol}: {error}"))?;
    if !response.success || response.code != 0 {
        return Err(response.message.unwrap_or_else(|| {
            format!(
                "MEXC funding records request for {symbol} failed with code {}.",
                response.code
            )
        }));
    }

    let records = response.data.result_list.into_iter().map(|record| {
        let side = if record.position_type == 2 {
            PositionSide::Short
        } else {
            PositionSide::Long
        };
        let cash_flow_direction = match side {
            PositionSide::Long => -record.rate.signum(),
            PositionSide::Short => record.rate.signum(),
        };
        RealFundingRecord {
            id: record.id,
            symbol: record.symbol,
            position_id: record.position_id,
            side,
            position_value: record.position_value,
            funding_amount: record.funding.abs() * cash_flow_direction,
            funding_rate: record.rate,
            settle_time_ms: record.settle_time_ms,
        }
    }).collect();
    Ok((response.data.total_page, records))
}

fn default_api_allowed() -> bool {
    true
}

fn parse_contract_response(raw: String) -> Result<ContractDetail, String> {
    let response: ContractResponse = serde_json::from_str(&raw)
        .map_err(|error| format!("Invalid MEXC Futures contract response: {error}"))?;
    if !response.success || response.code != 0 {
        return Err(response.message.unwrap_or_else(|| {
            format!(
                "MEXC Futures contract request failed with code {}.",
                response.code
            )
        }));
    }
    if response.data.contract_size <= 0.0
        || response.data.vol_unit <= 0.0
        || response.data.min_vol <= 0.0
        || response.data.max_vol <= 0.0
    {
        return Err("MEXC returned invalid contract sizing information.".into());
    }
    if response.data.state != 0 {
        return Err("This MEXC contract is not currently enabled.".into());
    }
    if !response.data.api_allowed {
        return Err("MEXC does not currently allow API trading for this contract.".into());
    }
    Ok(response.data)
}

fn parse_order_response(raw: String) -> Result<i64, String> {
    let response: OrderResponse = serde_json::from_str(&raw)
        .map_err(|error| format!("Invalid MEXC order response: {error}"))?;
    if !response.success || response.code != 0 {
        return Err(response.message.unwrap_or_else(|| {
            format!("MEXC order submission failed with code {}.", response.code)
        }));
    }
    response
        .data
        .ok_or_else(|| "MEXC did not return an order ID.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_funding_records_normalizes_cash_flow_by_position_side_and_rate() {
        let raw = r#"{
            "success": true,
            "code": 0,
            "data": {
                "totalPage": 1,
                "resultList": [
                    {
                        "id": 7,
                        "symbol": "BTC_USDT",
                        "positionId": 42,
                        "positionType": 1,
                        "positionValue": 2000.0,
                        "funding": 0.2,
                        "rate": -0.0001,
                        "settleTime": 1760000000000
                    },
                    {
                        "id": 8,
                        "symbol": "BTC_USDT",
                        "positionId": 42,
                        "positionType": 2,
                        "positionValue": 2000.0,
                        "funding": 0.2,
                        "rate": -0.0001,
                        "settleTime": 1760000001000
                    }
                ]
            }
        }"#;
        let (pages, records) = parse_funding_records_response(raw.to_string(), "BTC_USDT").unwrap();

        assert_eq!(pages, 1);
        assert_eq!(records.len(), 2);
        assert!((records[0].funding_amount - 0.2).abs() < 1e-9);
        assert!((records[1].funding_amount + 0.2).abs() < 1e-9);
        assert_eq!(records[0].settle_time_ms, 1_760_000_000_000);
    }
}

fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> String {
    const BLOCK_SIZE: usize = 64;
    let mut normalized_key = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        let digest = Sha256::digest(key);
        normalized_key[..digest.len()].copy_from_slice(&digest);
    } else {
        normalized_key[..key.len()].copy_from_slice(key);
    }
    let mut inner = [0u8; BLOCK_SIZE];
    let mut outer = [0u8; BLOCK_SIZE];
    for index in 0..BLOCK_SIZE {
        inner[index] = normalized_key[index] ^ 0x36;
        outer[index] = normalized_key[index] ^ 0x5c;
    }
    let mut inner_hasher = Sha256::new();
    inner_hasher.update(inner);
    inner_hasher.update(message);
    let inner_digest = inner_hasher.finalize();
    let mut outer_hasher = Sha256::new();
    outer_hasher.update(outer);
    outer_hasher.update(inner_digest);
    outer_hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
