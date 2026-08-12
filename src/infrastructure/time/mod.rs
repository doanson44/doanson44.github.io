mod provider;

use js_sys::{Date, Function, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::domain::time::{millis_to_timestamp, timestamp_to_millis, TimestampUnit};

pub use provider::BrowserTimeProvider;

pub fn now_ms() -> f64 {
    Date::now()
}

pub fn local_timezone() -> String {
    let global = js_sys::global();
    let Ok(intl) = Reflect::get(&global, &JsValue::from_str("Intl")) else {
        return "UTC".into();
    };
    let Ok(constructor) = Reflect::get(&intl, &JsValue::from_str("DateTimeFormat")) else {
        return "UTC".into();
    };
    let Ok(constructor) = constructor.dyn_into::<Function>() else {
        return "UTC".into();
    };
    let Ok(formatter) = Reflect::construct(&constructor, &js_sys::Array::new()) else {
        return "UTC".into();
    };
    let Ok(options) = Reflect::get(&formatter, &JsValue::from_str("resolvedOptions")) else {
        return "UTC".into();
    };
    let Ok(options) = options.dyn_into::<Function>() else {
        return "UTC".into();
    };
    let Ok(resolved) = options.call0(&formatter) else {
        return "UTC".into();
    };
    Reflect::get(&resolved, &JsValue::from_str("timeZone"))
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or_else(|| "UTC".into())
}

pub fn timestamp_to_datetime(
    value: &str,
    unit: TimestampUnit,
    timezone: &str,
) -> Result<String, String> {
    let millis = timestamp_to_millis(value, unit)?;
    let date = Date::new(&JsValue::from_f64(millis));
    if !millis.is_finite() || date.get_time().is_nan() {
        return Err("Timestamp is outside the supported date range.".into());
    }
    format_date(&date, timezone)
}

pub fn datetime_to_timestamp(
    value: &str,
    unit: TimestampUnit,
    timezone: &str,
) -> Result<String, String> {
    let date = parse_datetime(value, timezone)?;
    Ok(millis_to_timestamp(date.get_time(), unit))
}

pub fn format_clock(millis: f64, timezone: &str) -> Result<String, String> {
    let date = Date::new(&JsValue::from_f64(millis));
    format_date(&date, timezone)
}

fn format_date(date: &Date, timezone: &str) -> Result<String, String> {
    let options = Object::new();
    set_option(&options, "timeZone", timezone)?;
    set_option(&options, "dateStyle", "medium")?;
    set_option(&options, "timeStyle", "medium")?;
    let (formatter, format) = intl_datetime_format(&options)?;
    format
        .call1(&formatter, date)
        .map_err(|_| "Unable to format date/time.".to_string())?
        .as_string()
        .ok_or_else(|| "Unable to format date/time.".into())
}

fn parse_datetime(value: &str, timezone: &str) -> Result<Date, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Enter a date and time.".into());
    }
    if timezone == "UTC" {
        let normalized = trimmed.replace(' ', "T");
        let iso = if normalized.ends_with('Z') {
            normalized
        } else {
            format!("{normalized}Z")
        };
        let date = Date::new(&JsValue::from_str(&iso));
        if date.get_time().is_nan() {
            return Err("Enter a valid date and time.".into());
        }
        return Ok(date);
    }
    let date = Date::new(&JsValue::from_str(trimmed));
    if date.get_time().is_nan() {
        return Err("Enter a valid date and time. Use YYYY-MM-DD HH:mm:ss.".into());
    }
    Ok(date)
}

fn intl_datetime_format(options: &Object) -> Result<(Object, Function), String> {
    let global = js_sys::global();
    let intl = Reflect::get(&global, &JsValue::from_str("Intl"))
        .map_err(|_| "Intl API is unavailable in this browser.".to_string())?;
    let constructor = Reflect::get(&intl, &JsValue::from_str("DateTimeFormat"))
        .map_err(|_| "Timezone formatting is unavailable in this browser.".to_string())?
        .dyn_into::<Function>()
        .map_err(|_| "Timezone formatting is unavailable in this browser.".to_string())?;
    let args = js_sys::Array::of2(&JsValue::from_str("en-US"), options);
    let formatter = Reflect::construct(&constructor, &args)
        .map_err(|_| "Invalid timezone or date formatting option.".to_string())?
        .dyn_into::<Object>()
        .map_err(|_| "Unable to create date formatter.".to_string())?;
    let format = Reflect::get(&formatter, &JsValue::from_str("format"))
        .map_err(|_| "Unable to access date formatter.".to_string())?
        .dyn_into::<Function>()
        .map_err(|_| "Unable to access date formatter.".to_string())?;
    Ok((formatter, format))
}

fn set_option(object: &Object, key: &str, value: &str) -> Result<(), String> {
    Reflect::set(object, &JsValue::from_str(key), &JsValue::from_str(value))
        .map_err(|_| "Unable to configure date formatter.".to_string())?;
    Ok(())
}
