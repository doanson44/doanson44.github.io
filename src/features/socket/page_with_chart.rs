use std::rc::Rc;

use leptos::prelude::*;

use crate::application::ports::{FundingRateProvider, FuturesMarketStream};
use crate::features::socket::chart::TradingViewChart;
use crate::features::socket::page::SocketPage as MarketSocketPage;

/// Socket page composition with the TradingView market chart.
#[component]
pub fn SocketPage(
    stream: Rc<dyn FuturesMarketStream>,
    funding_provider: Rc<dyn FundingRateProvider>,
) -> impl IntoView {
    view! {
        <div class="socket-page-with-chart d-flex flex-column flex-grow-1 overflow-auto">
            <TradingViewChart />
            <div class="socket-market-panel d-flex flex-column flex-grow-1 min-h-0">
                <MarketSocketPage
                    stream=stream
                    funding_provider=funding_provider
                />
            </div>
        </div>
    }
}
