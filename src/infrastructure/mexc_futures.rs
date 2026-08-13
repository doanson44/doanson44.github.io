use std::rc::Rc;

use serde::Deserialize;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{CloseEvent, Event, MessageEvent, WebSocket, Window};

use crate::domain::futures::FuturesTickerUpdate;

const WS_ENDPOINT: &str = "wss://contract.mexc.com/edge";
const PING_INTERVAL_MS: i32 = 15_000;

/// Connection state reported by the MEXC Futures public stream.
#[derive(Debug, Clone, PartialEq)]
pub enum MexcFuturesConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}

#[derive(Debug, Deserialize)]
struct WsEnvelope {
    channel: String,
    data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct TickerPayload {
    symbol: String,
    #[serde(rename = "lastPrice")]
    last_price: Option<f64>,
    #[serde(rename = "volume24")]
    volume_24h: Option<f64>,
    #[serde(rename = "riseFallRate")]
    change_24h: Option<f64>,
    #[serde(rename = "fairPrice")]
    fair_price: Option<f64>,
}

/// Handle for a live MEXC Futures public WebSocket connection.
#[allow(dead_code)]
pub struct MexcFuturesWsHandle {
    socket: WebSocket,
    ping_interval: Option<i32>,
    on_open: Closure<dyn FnMut()>,
    on_message: Closure<dyn FnMut(MessageEvent)>,
    on_error: Closure<dyn FnMut(Event)>,
    on_close: Closure<dyn FnMut(CloseEvent)>,
    ping_callback: Option<Closure<dyn FnMut()>>,
}

// In a single-threaded WASM environment, it is safe to send this handle across the (non-existent) threads.
// Leptos 0.7's `on_cleanup` requires `Send + Sync`.
unsafe impl Send for MexcFuturesWsHandle {}
unsafe impl Sync for MexcFuturesWsHandle {}

impl MexcFuturesWsHandle {
    /// Closes the stream and releases the keepalive interval.
    pub fn close(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Some(handle) = self.ping_interval.take() {
                window.clear_interval_with_handle(handle);
            }
        }
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        self.socket.set_onerror(None);
        self.socket.set_onclose(None);
        let _ = self.socket.close();
        self.ping_callback = None;
    }
}

impl Drop for MexcFuturesWsHandle {
    fn drop(&mut self) {
        self.close();
    }
}

/// Connects to the MEXC Futures public ticker stream for the complete market.
pub fn connect_tickers(
    on_batch: Rc<dyn Fn(Vec<FuturesTickerUpdate>)>,
    on_status: Rc<dyn Fn(MexcFuturesConnectionStatus)>,
) -> Result<MexcFuturesWsHandle, String> {
    on_status(MexcFuturesConnectionStatus::Connecting);
    let socket = WebSocket::new(WS_ENDPOINT)
        .map_err(|error| format!("Failed to create MEXC WebSocket: {}", js_error(&error)))?;

    let subscribe_socket = socket.clone();
    let open_status = on_status.clone();
    let on_open = Closure::<dyn FnMut()>::new(move || {
        let message = r#"{"method":"sub.tickers","param":{},"gzip":false}"#;
        let _ = subscribe_socket.send_with_str(message);
        open_status(MexcFuturesConnectionStatus::Connected);
    });
    socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));

    let batch_callback = on_batch.clone();
    let message_status = on_status.clone();
    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let payload: String = match event.data().as_string() {
            Some(text) => text,
            None => {
                message_status(MexcFuturesConnectionStatus::Error(
                    "MEXC returned a non-text WebSocket message".into(),
                ));
                return;
            }
        };

        let envelope = match serde_json::from_str::<WsEnvelope>(&payload) {
            Ok(value) => value,
            Err(error) => {
                message_status(MexcFuturesConnectionStatus::Error(format!(
                    "Failed to decode MEXC WebSocket message: {error}"
                )));
                return;
            }
        };

        if envelope.channel == "push.tickers" {
            match serde_json::from_value::<Vec<TickerPayload>>(envelope.data) {
                Ok(tickers) => {
                    let updates = tickers
                        .into_iter()
                        .map(|ticker| FuturesTickerUpdate {
                            symbol: ticker.symbol,
                            last_price: ticker.last_price,
                            volume_24h: ticker.volume_24h,
                            change_24h: ticker.change_24h,
                            fair_price: ticker.fair_price,
                            updated_at_ms: None,
                        })
                        .collect();
                    batch_callback(updates);
                }
                Err(error) => message_status(MexcFuturesConnectionStatus::Error(format!(
                    "Failed to decode MEXC ticker batch: {error}"
                ))),
            }
        }
    });
    socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    let error_status = on_status.clone();
    let on_error = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
        error_status(MexcFuturesConnectionStatus::Error(
            "MEXC WebSocket reported an error".into(),
        ));
    });
    socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    let close_status = on_status.clone();
    let on_close = Closure::<dyn FnMut(CloseEvent)>::new(move |event: CloseEvent| {
        let reason = event.reason();
        if reason.is_empty() {
            close_status(MexcFuturesConnectionStatus::Disconnected);
        } else {
            close_status(MexcFuturesConnectionStatus::Error(format!(
                "MEXC WebSocket closed: {reason}"
            )));
        }
    });
    socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));

    let ping_socket = socket.clone();
    let ping_callback = Closure::<dyn FnMut()>::new(move || {
        if ping_socket.ready_state() == WebSocket::OPEN {
            let _ = ping_socket.send_with_str(r#"{"method":"ping"}"#);
        }
    });

    let window: Window =
        web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let ping_interval = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            ping_callback.as_ref().unchecked_ref(),
            PING_INTERVAL_MS,
        )
        .map_err(|error| format!("Failed to start MEXC keepalive: {}", js_error(&error)))?;

    Ok(MexcFuturesWsHandle {
        socket,
        ping_interval: Some(ping_interval),
        on_open,
        on_message,
        on_error,
        on_close,
        ping_callback: Some(ping_callback),
    })
}

fn js_error(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Unknown browser error".into())
}
