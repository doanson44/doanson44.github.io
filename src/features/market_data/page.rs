use leptos::prelude::*;

use crate::features::market_data::state::MarketDataState;

#[component]
pub fn CoinPricePage(symbol: String) -> impl IntoView {
    let state = MarketDataState::new();
    state.load_coin(&symbol);

    view! {
        <pre
            class="market-data-json min-h-screen overflow-auto whitespace-pre-wrap break-words p-4 font-mono text-xs leading-relaxed text-[var(--text-primary)]"
            aria-live="polite"
            aria-label="Coin market data JSON"
        >
            {move || state.output.get()}
        </pre>
    }
}

#[component]
pub fn StockPricePage(symbol: String) -> impl IntoView {
    let state = MarketDataState::new();
    state.load_stock(&symbol);

    view! {
        <pre
            class="market-data-json min-h-screen overflow-auto whitespace-pre-wrap break-words p-4 font-mono text-xs leading-relaxed text-[var(--text-primary)]"
            aria-live="polite"
            aria-label="Stock market data JSON"
        >
            {move || state.output.get()}
        </pre>
    }
}
