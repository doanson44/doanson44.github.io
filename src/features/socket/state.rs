use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use leptos::prelude::*;

use crate::domain::futures::{FuturesTicker, FuturesTickerRegistry};
use crate::infrastructure::mexc_futures::{
    connect_tickers, MexcFuturesConnectionStatus, MexcFuturesWsHandle,
};

/// Sortable columns in the Futures market table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Symbol,
    LastPrice,
    Change24h,
    Volume24h,
    FairPrice,
}

/// Quote asset filter for Futures contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteFilter {
    All,
    Usdt,
    Usdc,
}

/// 24-hour change direction filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeFilter {
    All,
    Positive,
    Negative,
}

/// Reactive state for the MEXC Futures market table.
#[derive(Clone)]
pub struct SocketState {
    pub tickers: RwSignal<Vec<FuturesTicker>>,
    pub search: RwSignal<String>,
    pub quote_filter: RwSignal<QuoteFilter>,
    pub change_filter: RwSignal<ChangeFilter>,
    pub sort_column: RwSignal<SortColumn>,
    pub sort_descending: RwSignal<bool>,
    pub page: RwSignal<usize>,
    pub page_size: RwSignal<usize>,
    pub connection_status: RwSignal<MexcFuturesConnectionStatus>,
    pub filtered_sorted: Memo<Vec<FuturesTicker>>,
    pub visible_rows: Memo<Vec<FuturesTicker>>,
    pub page_count: Memo<usize>,
}

impl SocketState {
    /// Creates the socket feature state and starts the all-market ticker stream.
    pub fn new() -> Self {
        let tickers = RwSignal::new(Vec::new());
        let search = RwSignal::new(String::new());
        let quote_filter = RwSignal::new(QuoteFilter::All);
        let change_filter = RwSignal::new(ChangeFilter::All);
        let sort_column = RwSignal::new(SortColumn::Symbol);
        let sort_descending = RwSignal::new(false);
        let page = RwSignal::new(1_usize);
        let page_size = RwSignal::new(25_usize);
        let connection_status = RwSignal::new(MexcFuturesConnectionStatus::Connecting);

        let filtered_sorted = Memo::new(move |_| {
            let query = search.get().trim().to_ascii_lowercase();
            let quote = quote_filter.get();
            let change = change_filter.get();
            let column = sort_column.get();
            let descending = sort_descending.get();

            let mut rows = tickers
                .get()
                .into_iter()
                .filter(|ticker| {
                    let matches_search = query.is_empty()
                        || ticker.symbol.to_ascii_lowercase().contains(query.as_str());
                    let matches_quote = match quote {
                        QuoteFilter::All => true,
                        QuoteFilter::Usdt => ticker.symbol.ends_with("_USDT"),
                        QuoteFilter::Usdc => ticker.symbol.ends_with("_USDC"),
                    };
                    let matches_change = match change {
                        ChangeFilter::All => true,
                        ChangeFilter::Positive => {
                            ticker.change_24h.is_some_and(|value| value > 0.0)
                        }
                        ChangeFilter::Negative => {
                            ticker.change_24h.is_some_and(|value| value < 0.0)
                        }
                    };
                    matches_search && matches_quote && matches_change
                })
                .collect::<Vec<_>>();

            rows.sort_unstable_by(|left, right| {
                let ordering = match column {
                    SortColumn::Symbol => left.symbol.cmp(&right.symbol),
                    SortColumn::LastPrice => {
                        compare_optional(left.last_price, right.last_price)
                    }
                    SortColumn::Change24h => {
                        compare_optional(left.change_24h, right.change_24h)
                    }
                    SortColumn::Volume24h => {
                        compare_optional(left.volume_24h, right.volume_24h)
                    }
                    SortColumn::FairPrice => {
                        compare_optional(left.fair_price, right.fair_price)
                    }
                };

                if ordering == Ordering::Equal {
                    left.symbol.cmp(&right.symbol)
                } else if descending {
                    ordering.reverse()
                } else {
                    ordering
                }
            });

            rows
        });

        let page_count = Memo::new(move |_| {
            let count = filtered_sorted.get().len();
            let size = page_size.get().max(1);
            count.div_ceil(size).max(1)
        });

        let visible_rows = Memo::new(move |_| {
            let rows = filtered_sorted.get();
            let size = page_size.get().max(1);
            let current_page = page.get().clamp(1, page_count.get());
            let start = (current_page - 1) * size;
            rows.into_iter().skip(start).take(size).collect::<Vec<_>>()
        });

        let registry = Rc::new(RefCell::new(FuturesTickerRegistry::new()));
        let registry_for_stream = registry.clone();
        let tickers_signal = tickers;
        let on_batch = Rc::new(move |updates| {
            let snapshot = registry_for_stream.borrow_mut().apply_batch(updates);
            tickers_signal.set(snapshot);
        });

        let status_signal = connection_status;
        let on_status = Rc::new(move |status| status_signal.set(status));

        match connect_tickers(on_batch, on_status) {
            Ok(handle) => register_cleanup(handle),
            Err(error) => {
                connection_status.set(MexcFuturesConnectionStatus::Error(error));
            }
        }

        Self {
            tickers,
            search,
            quote_filter,
            change_filter,
            sort_column,
            sort_descending,
            page,
            page_size,
            connection_status,
            filtered_sorted,
            visible_rows,
            page_count,
        }
    }
}

fn register_cleanup(mut handle: MexcFuturesWsHandle) {
    on_cleanup(move || handle.close());
}

fn compare_optional(left: Option<f64>, right: Option<f64>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}
