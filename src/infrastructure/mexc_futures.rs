use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{CloseEvent, Event, MessageEvent, WebSocket, Window};

use crate::domain::futures::FuturesTickerUpdate;

const WS_ENDPOINT: &str = "wss://contract.mexc.com/edge";
const PING_INTERVAL_MS: i32 = 15_000;
const INITIAL_RECONNECT_MS: i32 = 500;
const MAX_RECONNECT_MS: i32 = 30_000;

/// Connection state reported by the MEXC Futures public stream.
#[derive(Debug, Clone, PartialEq)]
pub enum MexcFuturesConnectionStatus {
    Connecting,
    Connected,
    Reconnecting,
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

type StatusCallback = Rc<dyn Fn(MexcFuturesConnectionStatus)>;
type BatchCallback = Rc<dyn Fn(Vec<FuturesTickerUpdate>)>;
type Callback = Closure<dyn FnMut()>;
type MessageCallback = Closure<dyn FnMut(MessageEvent)>;
type ErrorCallback = Closure<dyn FnMut(Event)>;
type CloseCallback = Closure<dyn FnMut(CloseEvent)>;

struct Runtime {
    socket: Option<WebSocket>,
    ping_interval: Option<i32>,
    retry_timeout: Option<i32>,
    retry_attempt: u32,
    closed: bool,
    on_open: Option<Callback>,
    on_message: Option<MessageCallback>,
    on_error: Option<ErrorCallback>,
    on_close: Option<CloseCallback>,
    ping_callback: Option<Callback>,
    retry_callback: Option<Callback>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            socket: None,
            ping_interval: None,
            retry_timeout: None,
            retry_attempt: 0,
            closed: false,
            on_open: None,
            on_message: None,
            on_error: None,
            on_close: None,
            ping_callback: None,
            retry_callback: None,
        }
    }
}

/// Handle for a reconnecting MEXC Futures public WebSocket connection.
pub struct MexcFuturesWsHandle {
    runtime: Rc<RefCell<Runtime>>,
}

// In a single-threaded WASM environment, the runtime never crosses an actual thread.
// Leptos cleanup requires a Send + Sync handle type.
unsafe impl Send for MexcFuturesWsHandle {}
unsafe impl Sync for MexcFuturesWsHandle {}

impl MexcFuturesWsHandle {
    /// Closes the stream, cancels retries, and releases keepalive resources.
    pub fn close(&mut self) {
        close_runtime(&self.runtime);
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
    let runtime = Rc::new(RefCell::new(Runtime::default()));
    open_connection(&runtime, on_batch, on_status, false)?;
    Ok(MexcFuturesWsHandle { runtime })
}

fn open_connection(
    runtime: &Rc<RefCell<Runtime>>,
    on_batch: BatchCallback,
    on_status: StatusCallback,
    reconnecting: bool,
) -> Result<(), String> {
    if runtime.borrow().closed {
        return Ok(());
    }

    clear_active_connection(runtime);

    if reconnecting {
        on_status(MexcFuturesConnectionStatus::Reconnecting);
    } else {
        on_status(MexcFuturesConnectionStatus::Connecting);
    }

    let socket = WebSocket::new(WS_ENDPOINT)
        .map_err(|error| format!("Failed to create MEXC WebSocket: {}", js_error(&error)))?;

    let weak_runtime = Rc::downgrade(runtime);
    let subscribe_socket = socket.clone();
    let open_status = on_status.clone();
    let on_open = Closure::<dyn FnMut()>::new(move || {
        let message = r#"{"method":"sub.tickers","param":{},"gzip":false}"#;
        let _ = subscribe_socket.send_with_str(message);
        if let Some(runtime) = weak_runtime.upgrade() {
            let mut state = runtime.borrow_mut();
            state.retry_attempt = 0;
            state.retry_timeout = None;
            state.retry_callback = None;
        }
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

    let weak_runtime = Rc::downgrade(runtime);
    let close_batch = on_batch.clone();
    let close_status = on_status.clone();
    let on_close = Closure::<dyn FnMut(CloseEvent)>::new(move |event: CloseEvent| {
        let reason = event.reason();
        if let Some(runtime) = weak_runtime.upgrade() {
            if reason.is_empty() {
                close_status(MexcFuturesConnectionStatus::Disconnected);
            } else {
                close_status(MexcFuturesConnectionStatus::Error(format!(
                    "MEXC WebSocket closed: {reason}"
                )));
            }
            schedule_reconnect(&runtime, close_batch.clone(), close_status.clone());
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

    let mut state = runtime.borrow_mut();
    state.socket = Some(socket);
    state.ping_interval = Some(ping_interval);
    state.on_open = Some(on_open);
    state.on_message = Some(on_message);
    state.on_error = Some(on_error);
    state.on_close = Some(on_close);
    state.ping_callback = Some(ping_callback);

    Ok(())
}

fn schedule_reconnect(runtime: &Rc<RefCell<Runtime>>, on_batch: BatchCallback, on_status: StatusCallback) {
    let (delay, weak_runtime) = {
        let mut state = runtime.borrow_mut();
        if state.closed || state.retry_timeout.is_some() {
            return;
        }
        let exponent = state.retry_attempt.min(6);
        let delay = (INITIAL_RECONNECT_MS.saturating_mul(1_i32 << exponent)).min(MAX_RECONNECT_MS);
        state.retry_attempt = state.retry_attempt.saturating_add(1);
        (delay, Rc::downgrade(runtime))
    };

    let callback = Closure::<dyn FnMut()>::new(move || {
        if let Some(runtime) = weak_runtime.upgrade() {
            runtime.borrow_mut().retry_timeout = None;
            runtime.borrow_mut().retry_callback = None;
            if let Err(error) = open_connection(&runtime, on_batch.clone(), on_status.clone(), true) {
                on_status(MexcFuturesConnectionStatus::Error(error));
                schedule_reconnect(&runtime, on_batch.clone(), on_status.clone());
            }
        }
    });

    let Some(window) = web_sys::window() else {
        return;
    };
    let timeout = match window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        delay,
    ) {
        Ok(handle) => handle,
        Err(_) => return,
    };

    let mut state = runtime.borrow_mut();
    state.retry_timeout = Some(timeout);
    state.retry_callback = Some(callback);
}

fn clear_active_connection(runtime: &Rc<RefCell<Runtime>>) {
    let mut state = runtime.borrow_mut();
    if let Some(window) = web_sys::window() {
        if let Some(handle) = state.ping_interval.take() {
            window.clear_interval_with_handle(handle);
        }
    }
    if let Some(socket) = state.socket.take() {
        socket.set_onopen(None);
        socket.set_onmessage(None);
        socket.set_onerror(None);
        socket.set_onclose(None);
        let _ = socket.close();
    }
    state.on_open = None;
    state.on_message = None;
    state.on_error = None;
    state.on_close = None;
    state.ping_callback = None;
}

fn close_runtime(runtime: &Rc<RefCell<Runtime>>) {
    let mut state = runtime.borrow_mut();
    state.closed = true;
    if let Some(window) = web_sys::window() {
        if let Some(handle) = state.ping_interval.take() {
            window.clear_interval_with_handle(handle);
        }
        if let Some(handle) = state.retry_timeout.take() {
            window.clear_timeout_with_handle(handle);
        }
    }
    if let Some(socket) = state.socket.take() {
        socket.set_onopen(None);
        socket.set_onmessage(None);
        socket.set_onerror(None);
        socket.set_onclose(None);
        let _ = socket.close();
    }
    state.on_open = None;
    state.on_message = None;
    state.on_error = None;
    state.on_close = None;
    state.ping_callback = None;
    state.retry_callback = None;
}

fn js_error(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Unknown browser error".into())
}
