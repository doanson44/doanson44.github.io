use std::rc::Rc;

use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::application::ports::ProxyClient;
use crate::domain::trading::RealAccountSnapshot;

/// Application service for reading a MEXC Futures account through the configured proxy.
#[derive(Debug, Clone, Copy)]
pub struct MexcFuturesAccountService<C> {
    client: C,
}

impl<C> MexcFuturesAccountService<C>
where
    C: ProxyClient + Copy + 'static,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Fetches the USDT Futures asset using MEXC Contract API v1 authentication.
    pub fn fetch_usdt_asset(
        &self,
        api_url: &str,
        api_key: &str,
        api_secret: &str,
        request_time_ms: i64,
        on_result: Rc<dyn Fn(Result<RealAccountSnapshot, String>)>,
    ) {
        let api_url = api_url.trim().trim_end_matches('/').to_string();
        let api_key = api_key.trim().to_string();
        let api_secret = api_secret.trim().to_string();

        if api_url.is_empty() || api_key.is_empty() || api_secret.is_empty() {
            on_result(Err("API URL, API key, and API secret are required.".to_string()));
            return;
        }

        let target_url = format!("{api_url}/api/v1/private/account/assets");
        let request_time = request_time_ms.to_string();
        let signature = hmac_sha256_hex(
            api_secret.as_bytes(),
            format!("{api_key}{request_time}").as_bytes(),
        );

        let headers = vec![
            ("ApiKey".to_string(), api_key),
            ("Request-Time".to_string(), request_time),
            ("Signature".to_string(), signature),
            ("Content-Type".to_string(), "application/json".to_string()),
        ];

        self.client.fetch_with_headers(
            &target_url,
            headers,
            Rc::new(move |result| on_result(result.and_then(parse_account_response))),
        );
    }
}

#[derive(Debug, Deserialize)]
struct AccountResponse {
    success: bool,
    code: i64,
    message: Option<String>,
    data: Vec<AccountAsset>,
}

#[derive(Debug, Deserialize)]
struct AccountAsset {
    currency: String,
    #[serde(rename = "positionMargin")]
    position_margin: f64,
    #[serde(rename = "availableBalance")]
    available_balance: f64,
    #[serde(rename = "cashBalance")]
    cash_balance: f64,
    #[serde(rename = "frozenBalance")]
    frozen_balance: f64,
    equity: f64,
    unrealized: f64,
}

fn parse_account_response(raw: String) -> Result<RealAccountSnapshot, String> {
    let response: AccountResponse = serde_json::from_str(&raw)
        .map_err(|error| format!("Invalid MEXC Futures account response: {error}"))?;

    if !response.success || response.code != 0 {
        return Err(response.message.unwrap_or_else(|| {
            format!("MEXC Futures account request failed with code {}.", response.code)
        }));
    }

    let usdt = response
        .data
        .into_iter()
        .find(|asset| asset.currency.eq_ignore_ascii_case("USDT"))
        .ok_or_else(|| "MEXC Futures account did not return a USDT asset.".to_string())?;

    if !usdt.equity.is_finite() || usdt.equity < 0.0 {
        return Err("MEXC returned an invalid USDT equity value.".to_string());
    }

    Ok(RealAccountSnapshot {
        equity: usdt.equity,
        available_balance: usdt.available_balance,
        cash_balance: usdt.cash_balance,
        unrealized: usdt.unrealized,
        position_margin: usdt.position_margin,
        frozen_balance: usdt.frozen_balance,
    })
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
    let digest = outer_hasher.finalize();

    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_usdt_equity_from_account_assets() {
        let raw = r#"{
            "success": true,
            "code": 0,
            "data": [
                {
                    "currency": "BTC",
                    "positionMargin": 0,
                    "availableBalance": 0,
                    "cashBalance": 0,
                    "frozenBalance": 0,
                    "equity": 0,
                    "unrealized": 0
                },
                {
                    "currency": "USDT",
                    "positionMargin": 12.5,
                    "availableBalance": 90,
                    "cashBalance": 100,
                    "frozenBalance": 10,
                    "equity": 112.5,
                    "unrealized": 2.5
                }
            ]
        }"#;

        let snapshot = parse_account_response(raw.to_string()).unwrap();
        assert_eq!(snapshot.equity, 112.5);
        assert_eq!(snapshot.available_balance, 90.0);
        assert_eq!(snapshot.cash_balance, 100.0);
    }
}
