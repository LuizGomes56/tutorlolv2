//! The History page: the web app's `history/` (list, details and charts).
//!
//! The desktop build shows the device's history without signing in; signing in adds the
//! account's matches (the backend merges both).
mod charts;
mod details;
mod list;

pub use details::HistoryDetails;
pub use list::HistoryList;
