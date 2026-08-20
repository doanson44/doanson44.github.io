/// Initialize a TradingView Advanced Chart widget inside the provided element.
///
/// The widget owns its market data and renders an iframe backed by TradingView.
/// The caller is responsible for providing a stable container element.
pub fn initialize_advanced_chart(
    container: &web_sys::HtmlElement,
    symbol: &str,
    theme: &str,
) -> Result<(), String> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| "Browser document is unavailable".to_string())?;

    container.set_inner_html("");

    let widget = document
        .create_element("div")
        .map_err(|_| "Failed to create TradingView widget container".to_string())?;
    widget.set_class_name("tradingview-widget-container__widget");

    let script = document
        .create_element("script")
        .map_err(|_| "Failed to create TradingView widget script".to_string())?;
    script.set_attribute("type", "text/javascript").map_err(js_error)?;
    script
        .set_attribute(
            "src",
            "https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js",
        )
        .map_err(js_error)?;
    script.set_attribute("async", "true").map_err(js_error)?;

    let config = serde_json::json!({
        "autosize": true,
        "symbol": symbol,
        "interval": "60",
        "timezone": "exchange",
        "theme": theme,
        "style": "1",
        "locale": "en",
        "allow_symbol_change": true,
        "withdateranges": true,
        "hide_side_toolbar": true,
        "hide_top_toolbar": false,
        "hide_legend": false,
        "hide_volume": false,
        "save_image": false,
        "calendar": false,
        "support_host": "https://www.tradingview.com"
    });

    script.set_text_content(Some(&config.to_string()));
    widget.append_child(&script).map_err(js_error)?;
    container.append_child(&widget).map_err(js_error)?;

    Ok(())
}

fn js_error(_: wasm_bindgen::JsValue) -> String {
    "Failed to build TradingView chart DOM".to_string()
}
