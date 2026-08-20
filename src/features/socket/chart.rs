use leptos::prelude::*;

use crate::infrastructure::tradingview::initialize_advanced_chart;

const DEFAULT_SYMBOL: &str = "MEXC:BTCUSDT.P";

/// Renders a TradingView Advanced Chart using TradingView's own market data.
#[component]
pub fn TradingViewChart() -> impl IntoView {
    let container_ref = NodeRef::<Div>::new();
    let error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        let Some(container) = container_ref.get() else {
            return;
        };

        let theme = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .and_then(|element| element.get_attribute("data-bs-theme"))
            .unwrap_or_else(|| "dark".to_string());

        let trading_view_theme = if theme == "light" { "light" } else { "dark" };

        if let Err(message) = initialize_advanced_chart(
            &container,
            DEFAULT_SYMBOL,
            trading_view_theme,
        ) {
            error.set(Some(message));
        }
    });

    view! {
        <section class="card bg-body-tertiary border-secondary socket-chart-panel" aria-labelledby="socket-chart-title">
            <div class="card-header d-flex align-items-center justify-content-between gap-2 border-secondary">
                <div>
                    <h3 id="socket-chart-title" class="h6 mb-0">
                        <i class="bi bi-graph-up me-2 text-primary" aria-hidden="true"></i>
                        "TradingView Chart"
                    </h3>
                    <span class="small text-body-secondary">
                        "Live market data from TradingView"
                    </span>
                </div>
                <span class="badge bg-primary bg-opacity-25 text-primary-emphasis border border-primary-subtle">
                    "TradingView"
                </span>
            </div>
            <div class="card-body p-0 socket-chart-body" node_ref=container_ref>
                {move || error.get().map(|message| view! {
                    <div class="d-flex align-items-center justify-content-center h-100 p-3 text-body-secondary" role="status">
                        <span>
                            <i class="bi bi-exclamation-triangle me-2" aria-hidden="true"></i>
                            {message}
                        </span>
                    </div>
                })}
            </div>
        </section>
    }
}
