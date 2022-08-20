//! A crate for interacting with the Lemon market_data API

#![deny(missing_docs)]
pub mod api;
/// API client for the Lemon market trading API
pub mod client;
/// Data client for the Lemon market data API
pub mod data_client;
/// Error type for the Lemon market_data API
pub mod error;
/// Module for utilities
mod util;
