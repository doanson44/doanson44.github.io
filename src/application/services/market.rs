use std::rc::Rc;

use crate::application::ports::ProxyClient;
use crate::domain::market::parse_market_response;

const MARKET_URL: &str = "https://cafef.vn/du-lieu/ajax/mobile/smart/ajaxbandothitruong.ashx";

/// Application service for retrieving CafeF market data through the configured proxy.
#[derive(Debug, Clone, Copy)]
pub struct MarketService<C> {
    client: C,
}

impl<C> MarketService<C>
where
    C: ProxyClient + Clone + 'static,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn load(
        &self,
        on_result: Rc<dyn Fn(Result<crate::domain::market::MarketResponse, String>)>,
    ) {
        self.client.fetch(
            MARKET_URL,
            Rc::new(move |result| {
                on_result(result.and_then(|raw| parse_market_response(&raw)));
            }),
        );
    }
}
