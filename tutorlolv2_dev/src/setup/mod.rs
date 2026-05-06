use crate::MayFail;
use scraper::Selector;

pub mod client;
pub mod update;

pub fn selector(selectors: &str) -> MayFail<Selector> {
    Selector::parse(selectors)
        .map_err(|e| format!("[selector] Error parsing selector: {selectors:?}: {e:?}").into())
}
