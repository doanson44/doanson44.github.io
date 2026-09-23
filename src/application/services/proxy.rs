use std::rc::Rc;

use crate::application::ports::ProxyClient;
use crate::domain::proxy::{minimize_json_response, ResponseMinimizationOptions};

/// Application service for fetching and minimizing JSON responses through a proxy.
#[derive(Debug, Clone, Copy)]
pub struct ProxyService<C> {
    client: C,
}

impl<C> ProxyService<C>
where
    C: ProxyClient + Clone + 'static,
{
    /// Creates a proxy service backed by the supplied HTTP client.
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Fetches a response without modifying the returned body.
    pub fn fetch_raw(
        &self,
        target_url: &str,
        on_result: Rc<dyn Fn(Result<String, String>)>,
    ) {
        self.client.fetch(target_url, on_result);
    }

    /// Fetches a JSON response and applies the requested display minimization.
    pub fn request_json(
        &self,
        target_url: &str,
        options: ResponseMinimizationOptions,
        on_result: Rc<dyn Fn(Result<String, String>)>,
    ) {
        let callback = on_result.clone();
        let options = options.clone();

        self.client.fetch(
            target_url,
            Rc::new(move |result| {
                callback(result.and_then(|raw| minimize_json_response(&raw, &options)));
            }),
        );
    }
}
